use std::fmt::Debug;

use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Serialize, de::DeserializeOwned};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use tokio::sync::mpsc;
use tracing::debug;
use uuid::Uuid;

use crate::{
    common::{
        errors::ExchangeError,
        signing::sign_message,
        types::{
            DefaultFinalHeaders, DefaultResponse, DefaultSignatureHeaders, FinalRequest,
            OperationFinalHeaders, PacificSignature, SubAccountFinalHeaders,
        },
        utils::{get_timestamp_ms, prepare_final_request},
    },
    exchange::operations::{Operation, SubaccountCreateAction},
    info::info_client::InfoClient,
    models::exchange::{
        payload::{
            account::WithdrawPayload,
            agent_wallet::BindAgentWalletPayload,
            api_key::{CreateApiKeyPayload, ListApiKeysPayload, RevokeApiKeyPayload},
            batch_order::{
                BatchOrderActionPayload, BatchOrderActionType, BatchOrderActionsFinalHeaders,
                BatchOrderFinalRequest,
            },
            market_settings::{UpdateLeveragePayload, UpdateMarginModePayload},
            order::{
                CancelAllOrdersPayload, CancelOrderPayload, CancelStopOrderPayload,
                CreateMarketOrderPayload, CreateOrderPayload, CreateStopOrderPayload,
                SetPositionTpslPayload,
            },
            subaccount::{
                SubaccountConfirmPayload, SubaccountInitiatePayload, SubaccountTransferPayload,
            },
        },
        response::{
            account::WithdrawResponse,
            agent_wallet::BindAgentWalletResponse,
            api_key::{CreateApiKeyResponse, ListApiKeysResponse, RevokeApiKeyResponse},
            batch_order::BatchOrderResponse,
            market_settings::{UpdateLeverageResponse, UpdateMarginModeResponse},
            order::{
                CancelAllOrdersResponse, CancelOrderResponse, CancelStopOrderResponse,
                CreateMarketOrderResponse, CreateOrderResponse, CreateStopOrderResponse,
                SetPositionTPSLResponse,
            },
            subaccount::{SubaccountCreateResponse, SubaccountTransferResponse},
        },
    },
    rest::rest_client::RestClient,
};

pub struct ExchangeClient {
    pub base_url: &'static str,
    pub info_client: InfoClient,
    signer_keypair: Keypair,
    main_pubkey: Pubkey,
    agent_pubkey: Option<Pubkey>,
    pub api_key: Option<String>,
    http_client: RestClient,
    default_headers: HeaderMap,
}

impl ExchangeClient {
    pub async fn new(
        is_mainnet: bool,
        enable_ws: bool,
        api_key: Option<String>,
        signer_keypair: Keypair,
        main_pubkey: Pubkey,
        agent_pubkey: Option<Pubkey>,
    ) -> Result<Self, ExchangeError> {
        if agent_pubkey.is_some() && agent_pubkey != Some(signer_keypair.pubkey()) {
            return Err(ExchangeError::Custom(
                "Signer account and Agent pubkey cannot be the different".to_string(),
            ));
        }

        let base_url = if is_mainnet {
            crate::common::consts::REST_API_MAINNET_URL
        } else {
            crate::common::consts::REST_API_TESTNET_URL
        };

        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(ref key) = api_key {
            default_headers.insert("PF-API-KEY", HeaderValue::from_str(key)?);
        }

        let info_client = InfoClient::new(is_mainnet, enable_ws, api_key.clone()).await?;
        let http_client = RestClient::new(base_url);

        Ok(Self {
            base_url,
            info_client,
            api_key,
            signer_keypair,
            main_pubkey,
            agent_pubkey,
            http_client,
            default_headers,
        })
    }

    pub async fn set_default_api_key(&mut self, api_key: String) -> Result<(), ExchangeError> {
        self.api_key = Some(api_key);

        if let Some(ref key) = self.api_key {
            let header_value = HeaderValue::from_str(key)?;
            self.default_headers.insert("PF-API-KEY", header_value);
        }
        self.info_client
            .set_default_api_key(self.api_key.clone().unwrap())
            .await
    }

    async fn send_request<T, P>(
        &self,
        operation: Operation,
        final_request: FinalRequest<P>,
    ) -> Result<T, ExchangeError>
    where
        T: DeserializeOwned + Debug,
        P: Serialize + Debug,
    {
        debug!("send_request: operation={:?}", operation);
        debug!("send_request: final_request={:?}", &final_request);

        let response = self
            .http_client
            .post::<T, FinalRequest<P>>(Some(&operation.endpoint()), Some(&final_request), None)
            .await?;

        debug!("send_request: response={:?}", &response);

        Ok(response)
    }

    pub async fn request_ws_exchange_fn<P>(
        &self,
        request_method: &str,
        sign_payload: P,
        expiry_window: Option<u32>,
    ) -> Result<mpsc::Receiver<serde_json::Value>, ExchangeError>
    where
        P: Serialize + Debug,
    {
        if self.info_client.web_socket_client.is_none() {
            return Err(ExchangeError::NotInitialized(
                "WebSocket client not initialized".into(),
            ));
        }
        let final_request = prepare_final_request(
            request_method,
            sign_payload,
            expiry_window,
            &self.signer_keypair,
            &self.main_pubkey,
            &self.agent_pubkey,
        )
        .await?;
        let request_id = Uuid::new_v4();
        let rx = match self.info_client.web_socket_client.as_ref() {
            Some(ws_client) => {
                ws_client
                    .send_exchange_request(Some(request_id), request_method, final_request)
                    .await?
            }
            None => {
                return Err(ExchangeError::Custom(
                    "WebSocket client not initialized".into(),
                ));
            }
        };
        Ok(rx)
    }

    pub async fn request_exchange_fn<T, P>(
        &self,
        operation: Operation,
        sign_payload: P,
        expiry_window: Option<u32>,
    ) -> Result<T, ExchangeError>
    where
        T: DeserializeOwned + Debug,
        P: Serialize + Debug,
    {
        let final_request = prepare_final_request(
            operation.name().as_deref().unwrap(),
            sign_payload,
            expiry_window,
            &self.signer_keypair,
            &self.main_pubkey,
            &self.agent_pubkey,
        )
        .await?;

        self.send_request::<T, P>(operation, final_request).await
    }

    // pub async fn get_points(&self, user: Pubkey) -> Result<GetPointsResponse, ExchangeError> {
    //     let response = self
    //         .request_exchange_fn::<GetPointsResponse, GetPointsPayload>(
    //             Operation::GetPoints,
    //             GetPointsPayload { user },
    //             None,
    //         )
    //         .await?;
    //     Ok(response)
    // }

    pub async fn order(
        &self,
        sign_payload: CreateOrderPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<CreateOrderResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn::<DefaultResponse<CreateOrderResponse>, CreateOrderPayload>(
                Operation::CreateOrder,
                sign_payload,
                expiry_window,
            )
            .await?;
        Ok(response)
    }

    pub async fn batch_order(
        &self,
        orders: Vec<BatchOrderActionPayload>,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<BatchOrderResponse>, ExchangeError> {
        let mut actions: Vec<BatchOrderActionsFinalHeaders> = Vec::new();
        for order in orders {
            let (order_type, operation_name) = match order {
                BatchOrderActionPayload::CreateOrder(_) => (
                    BatchOrderActionType::Create,
                    Operation::CreateOrder.name().unwrap(),
                ),
                BatchOrderActionPayload::CancelOrder(_) => (
                    BatchOrderActionType::Cancel,
                    Operation::CancelOrder.name().unwrap(),
                ),
                BatchOrderActionPayload::CreateMarketOrder(_) => (
                    BatchOrderActionType::CreateMarket,
                    Operation::CreateMarketOrder.name().unwrap(),
                ),
            };
            let sign_headers = DefaultSignatureHeaders {
                timestamp: get_timestamp_ms(),
                expiry_window,
                type_field: operation_name,
            };
            let (_message, signature) = sign_message(&sign_headers, &order, &self.signer_keypair)?;
            let final_headers = OperationFinalHeaders::Default(DefaultFinalHeaders {
                account: self.main_pubkey,
                agent_wallet: self.agent_pubkey,
                signature: PacificSignature::Simple(signature),
                expiry_window: sign_headers.expiry_window,
                timestamp: sign_headers.timestamp,
            });
            let final_request = FinalRequest {
                headers: final_headers,
                payload: order.clone(),
            };

            actions.push(BatchOrderActionsFinalHeaders {
                type_field: order_type,
                data: final_request,
            });
        }

        let final_request = BatchOrderFinalRequest { actions };

        let response = self
            .http_client
            .post::<DefaultResponse<BatchOrderResponse>, BatchOrderFinalRequest>(
                Some(&Operation::BatchOrder.endpoint()),
                Some(&final_request),
                Some(&self.default_headers),
            )
            .await?;

        Ok(response)
    }

    pub async fn market_order(
        &self,
        sign_payload: CreateMarketOrderPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<CreateMarketOrderResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<CreateMarketOrderResponse>, CreateMarketOrderPayload>(
                Operation::CreateMarketOrder,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn stop_order(
        &self,
        sign_payload: CreateStopOrderPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<CreateStopOrderResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<CreateStopOrderResponse>, CreateStopOrderPayload>(
                Operation::CreateStopOrder,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn set_position_tpsl(
        &self,
        sign_payload: SetPositionTpslPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<SetPositionTPSLResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<SetPositionTPSLResponse>, SetPositionTpslPayload>(
                Operation::SetPositionTpsl,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn cancel_order(
        &self,
        sign_payload: CancelOrderPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<CancelOrderResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn::<DefaultResponse<CancelOrderResponse>, CancelOrderPayload>(
                Operation::CancelOrder,
                sign_payload,
                expiry_window,
            )
            .await?;
        Ok(response)
    }

    pub async fn cancel_all_orders(
        &self,
        sign_payload: CancelAllOrdersPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<CancelAllOrdersResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<CancelAllOrdersResponse>, CancelAllOrdersPayload>(
                Operation::CancelAllOrders,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn cancel_stop_order(
        &self,
        sign_payload: CancelStopOrderPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<CancelStopOrderResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<CancelStopOrderResponse>, CancelStopOrderPayload>(
                Operation::CancelStopOrder,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn update_margin_mode(
        &self,
        sign_payload: UpdateMarginModePayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<UpdateMarginModeResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<UpdateMarginModeResponse>, UpdateMarginModePayload>(
                Operation::UpdateMarginMode,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn update_leverage(
        &self,
        sign_payload: UpdateLeveragePayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<UpdateLeverageResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn::<DefaultResponse<UpdateLeverageResponse>, UpdateLeveragePayload>(
                Operation::UpdateLeverage,
                sign_payload,
                expiry_window,
            )
            .await?;
        Ok(response)
    }

    pub async fn withdraw(
        &self,
        sign_payload: WithdrawPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<WithdrawResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn::<DefaultResponse<WithdrawResponse>, WithdrawPayload>(
                Operation::Withdraw,
                sign_payload,
                expiry_window,
            )
            .await?;
        Ok(response)
    }

    pub async fn subaccount_create(
        &self,
        subaccount: &Keypair,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<SubaccountCreateResponse>, ExchangeError> {
        if self.signer_keypair.pubkey() == subaccount.pubkey() {
            return Err(ExchangeError::Custom(
                "Main account and subaccount cannot be the same".to_string(),
            ));
        }
        if Some(self.signer_keypair.pubkey()) == self.agent_pubkey {
            return Err(ExchangeError::Custom(
                "Main account and agent pubkey cannot be the same".to_string(),
            ));
        }
        let timestamp = get_timestamp_ms();

        let sub_headers = DefaultSignatureHeaders {
            timestamp,
            expiry_window,
            type_field: Operation::SubaccountCreate(SubaccountCreateAction::Initiate)
                .name()
                .unwrap(),
        };
        let sub_payload = SubaccountInitiatePayload {
            account: self.main_pubkey,
        };
        let (_sub_msg, sub_signature) = sign_message(&sub_headers, &sub_payload, subaccount)?;

        let main_headers = DefaultSignatureHeaders {
            timestamp,
            expiry_window,
            type_field: Operation::SubaccountCreate(SubaccountCreateAction::Confirm)
                .name()
                .unwrap(),
        };
        let main_payload = SubaccountConfirmPayload {
            signature: sub_signature.clone(),
        };
        let (_main_msg, main_signature) =
            sign_message(&main_headers, &main_payload, &self.signer_keypair)?;

        let final_headers = OperationFinalHeaders::SubAccountCreate(SubAccountFinalHeaders {
            main_account: self.main_pubkey,
            subaccount: subaccount.pubkey(),
            main_signature: PacificSignature::Simple(main_signature),
            sub_signature: PacificSignature::Simple(sub_signature),
            expiry_window,
            timestamp,
        });

        let final_request = FinalRequest {
            headers: final_headers,
            payload: (),
        };

        let response = self
            .http_client
            .post::<DefaultResponse<SubaccountCreateResponse>, _>(
                Some(&Operation::SubaccountCreate(SubaccountCreateAction::Confirm).endpoint()),
                Some(&final_request),
                None,
            )
            .await?;

        Ok(response)
    }

    pub async fn subaccount_transfer(
        &self,
        sign_payload: SubaccountTransferPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<SubaccountTransferResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<SubaccountTransferResponse>, SubaccountTransferPayload>(
                Operation::SubaccountTransfer,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn bind_agent_wallet(
        &self,
        sign_payload: BindAgentWalletPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<BindAgentWalletResponse>, ExchangeError> {
        if Some(self.signer_keypair.pubkey()) == self.agent_pubkey {
            return Err(ExchangeError::Custom(
                "Main account and agent pubkey cannot be the same".to_string(),
            ));
        }
        let response = self
            .request_exchange_fn
                ::<DefaultResponse<BindAgentWalletResponse>, BindAgentWalletPayload>(
                Operation::BindAgentWallet,
                sign_payload,
                expiry_window
            ).await?;
        Ok(response)
    }

    pub async fn create_api_key(
        &self,
        sign_payload: CreateApiKeyPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<CreateApiKeyResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn::<DefaultResponse<CreateApiKeyResponse>, CreateApiKeyPayload>(
                Operation::CreateApiKey,
                sign_payload,
                expiry_window,
            )
            .await?;
        Ok(response)
    }

    pub async fn revoke_api_key(
        &self,
        sign_payload: RevokeApiKeyPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<RevokeApiKeyResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn::<DefaultResponse<RevokeApiKeyResponse>, RevokeApiKeyPayload>(
                Operation::RevokeApiKey,
                sign_payload,
                expiry_window,
            )
            .await?;
        Ok(response)
    }

    pub async fn get_list_api_keys(
        &self,
        sign_payload: ListApiKeysPayload,
        expiry_window: Option<u32>,
    ) -> Result<DefaultResponse<ListApiKeysResponse>, ExchangeError> {
        let response = self
            .request_exchange_fn::<DefaultResponse<ListApiKeysResponse>, ListApiKeysPayload>(
                Operation::ListApiKeys,
                sign_payload,
                expiry_window,
            )
            .await?;
        Ok(response)
    }
}
