use std::collections::HashMap;

use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use serde::{Serialize, de::DeserializeOwned};
use solana_sdk::pubkey::Pubkey;

use crate::{
    common::{
        consts,
        errors::ExchangeError,
        tick_lot::TickLot,
        types::{AggLevel, DefaultResponse, Interval},
        utils::match_both_some,
    },
    info::info_endpoint::InfoEndpoint,
    models::info::{
        params::{
            account::{
                AccountFundingHistoryParams, AccountParams, AccountSettingsParams,
                BalanceHistoryParams, EquityHistoryParams, OpenedOrdersParams,
                OrderHistoryByIdParams, OrderHistoryParams, PositionsParams, TradesHistoryParams,
            },
            market::{
                FundingRateHistoryParams, KlineParams, MarketsInfoParams, OrderBookParams,
                PricesParams, RecentTradesParams,
            },
        },
        response::{
            account::{
                AccountFundingHistoryResponse, AccountResponse, AccountSettingsResponse,
                BalanceHistoryResponse, EquityHistoryResponse, OpenedOrdersResponse,
                OrderHistoryByIdResponse, OrderHistoryResponse, PositionsResponse,
                TradesHistoryResponse,
            },
            market::{
                FundingRateHistoryResponse, KlineResponse, MarketModel, MarketsInfoResponse,
                OrderBookResponse, PricesResponse, RecentTradesResponse,
            },
        },
    },
    rest::rest_client::RestClient,
    ws::ws_client::WebSocketClient,
};

pub struct InfoClient {
    pub base_url: &'static str,
    pub market_cache: HashMap<String, MarketModel>,
    pub tick_lot_utils: TickLot,
    pub web_socket_client: Option<WebSocketClient>,
    api_key: Option<String>,
    default_headers: HeaderMap,
    http_client: RestClient,
}

impl InfoClient {
    pub async fn new(
        is_mainnet: bool,
        enable_ws: bool,
        api_key: Option<String>,
    ) -> Result<Self, ExchangeError> {
        let base_url = if is_mainnet {
            consts::REST_API_MAINNET_URL
        } else {
            consts::REST_API_TESTNET_URL
        };

        let web_socket_client: Option<WebSocketClient> = if enable_ws {
            let ws_url = if is_mainnet {
                consts::WS_MAINNET_URL
            } else {
                consts::WS_TESTNET_URL
            };
            Some(WebSocketClient::new(ws_url, api_key.clone()).await?)
        } else {
            None
        };

        let mut default_headers = HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        if let Some(ref key) = api_key {
            default_headers.insert("PF-API-KEY", HeaderValue::from_str(key)?);
        }

        let http_client = RestClient::new(base_url);

        let response = http_client
            .get::<DefaultResponse<MarketsInfoResponse>, MarketsInfoParams>(
                Some(&InfoEndpoint::MarketsInfo.get()),
                Some(&MarketsInfoParams {}),
                Some(&default_headers),
            )
            .await?;
        let market_cache: HashMap<String, MarketModel> = response
            .data
            .unwrap()
            .into_iter()
            .map(|m| (m.symbol.clone(), m))
            .collect();
        let tick_lot_utils = TickLot::new(market_cache.clone());

        Ok(Self {
            base_url,
            market_cache,
            tick_lot_utils,
            web_socket_client,
            api_key,
            default_headers,
            http_client,
        })
    }

    pub async fn set_default_api_key(&mut self, api_key: String) -> Result<(), ExchangeError> {
        self.api_key = Some(api_key);
        if let Some(ref key) = self.api_key {
            let header_value = HeaderValue::from_str(key)?;
            self.default_headers.insert("PF-API-KEY", header_value);
            if let Some(ws_client) = &mut self.web_socket_client {
                ws_client.set_api_key(key.clone()).await?;
            }
        }
        Ok(())
    }

    async fn request_info_fn<T, P>(
        &self,
        endpoint: InfoEndpoint,
        params: &P,
    ) -> Result<T, ExchangeError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // debug!("{}", serde_json::to_string(params)?);
        let response = self
            .http_client
            .get::<T, P>(Some(&endpoint.get()), Some(params), None)
            .await?;
        Ok(response)
    }

    pub async fn get_markets_info(
        &self,
    ) -> Result<DefaultResponse<MarketsInfoResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<MarketsInfoResponse>, MarketsInfoParams>(
            InfoEndpoint::MarketsInfo,
            &MarketsInfoParams {},
        )
        .await
    }

    pub async fn prices(&self) -> Result<DefaultResponse<PricesResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<PricesResponse>, PricesParams>(
            InfoEndpoint::Prices,
            &PricesParams {},
        )
        .await
    }
    pub async fn kline(
        &self,
        symbol: String,
        interval: Interval,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<DefaultResponse<KlineResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<KlineResponse>, KlineParams>(
            InfoEndpoint::Kline,
            &KlineParams {
                symbol,
                interval,
                start_time,
                end_time,
            },
        )
        .await
    }

    pub async fn recent_trades(
        &self,
        symbol: String,
    ) -> Result<DefaultResponse<RecentTradesResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<RecentTradesResponse>, RecentTradesParams>(
            InfoEndpoint::RecentTrades,
            &RecentTradesParams { symbol },
        )
        .await
    }
    pub async fn order_book(
        &self,
        symbol: String,
        agg_level: Option<AggLevel>,
    ) -> Result<DefaultResponse<OrderBookResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<OrderBookResponse>, OrderBookParams>(
            InfoEndpoint::OrderBook,
            &OrderBookParams { symbol, agg_level },
        )
        .await
    }
    pub async fn funding_rate_history(
        &self,
        symbol: String,
    ) -> Result<DefaultResponse<FundingRateHistoryResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<FundingRateHistoryResponse>, FundingRateHistoryParams>(
            InfoEndpoint::FundingRateHistory,
            &FundingRateHistoryParams { symbol },
        )
        .await
    }
    pub async fn account(
        &self,
        account: Pubkey,
    ) -> Result<DefaultResponse<AccountResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<AccountResponse>, AccountParams>(
            InfoEndpoint::Account,
            &AccountParams { account },
        )
        .await
    }
    pub async fn account_funding_history(
        &self,
        account: Pubkey,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<DefaultResponse<AccountFundingHistoryResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<AccountFundingHistoryResponse>, AccountFundingHistoryParams>(
            InfoEndpoint::AccountFundingHistory,
            &AccountFundingHistoryParams{account, limit, offset}
        ).await
    }
    pub async fn account_settings(
        &self,
        account: Pubkey,
    ) -> Result<DefaultResponse<AccountSettingsResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<AccountSettingsResponse>, AccountSettingsParams>(
            InfoEndpoint::AccountSettings,
            &AccountSettingsParams { account },
        )
        .await
    }
    pub async fn positions(
        &self,
        account: Pubkey,
    ) -> Result<DefaultResponse<PositionsResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<PositionsResponse>, PositionsParams>(
            InfoEndpoint::Positions,
            &PositionsParams { account },
        )
        .await
    }
    pub async fn trade_history(
        &self,
        account: Pubkey,
        symbol: Option<String>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<DefaultResponse<TradesHistoryResponse>, ExchangeError> {
        match_both_some(&start_time, &end_time)?;
        self.request_info_fn::<DefaultResponse<TradesHistoryResponse>, TradesHistoryParams>(
            InfoEndpoint::TradesHistory,
            &TradesHistoryParams {
                account,
                symbol,
                start_time,
                end_time,
                limit,
                offset,
            },
        )
        .await
    }
    pub async fn balance_history(
        &self,
        account: Pubkey,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<DefaultResponse<BalanceHistoryResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<BalanceHistoryResponse>, BalanceHistoryParams>(
            InfoEndpoint::BalanceHistory,
            &BalanceHistoryParams {
                account,
                limit,
                offset,
            },
        )
        .await
    }
    pub async fn equity_history(
        &self,
        equity_history_params: EquityHistoryParams,
    ) -> Result<DefaultResponse<EquityHistoryResponse>, ExchangeError> {
        match_both_some(
            &equity_history_params.start_time,
            &equity_history_params.end_time,
        )?;
        self.request_info_fn::<DefaultResponse<EquityHistoryResponse>, EquityHistoryParams>(
            InfoEndpoint::EquityHistory,
            &equity_history_params,
        )
        .await
    }
    pub async fn opened_orders(
        &self,
        account: Pubkey,
    ) -> Result<DefaultResponse<OpenedOrdersResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<OpenedOrdersResponse>, OpenedOrdersParams>(
            InfoEndpoint::OpenedOrders,
            &OpenedOrdersParams { account },
        )
        .await
    }
    pub async fn order_history(
        &self,
        account: Pubkey,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<DefaultResponse<OrderHistoryResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<OrderHistoryResponse>, OrderHistoryParams>(
            InfoEndpoint::OrderHistory,
            &OrderHistoryParams {
                account,
                limit,
                offset,
            },
        )
        .await
    }
    pub async fn order_history_by_id(
        &self,
        order_id: u64,
    ) -> Result<DefaultResponse<OrderHistoryByIdResponse>, ExchangeError> {
        self.request_info_fn::<DefaultResponse<OrderHistoryByIdResponse>, OrderHistoryByIdParams>(
            InfoEndpoint::OrderHistoryById,
            &OrderHistoryByIdParams { order_id },
        )
        .await
    }
}
