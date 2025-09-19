use crate::common::consts::API_VERSION_ENDPOINT;

#[derive(Debug, Clone)]
pub enum InfoEndpoint {
    MarketsInfo,
    Prices,
    Kline,
    OrderBook,
    RecentTrades,
    FundingRateHistory,
    Account,
    AccountSettings,
    Positions,
    TradesHistory,
    AccountFundingHistory,
    BalanceHistory,
    EquityHistory,
    OpenedOrders,
    OrderHistory,
    OrderHistoryById,
}

impl InfoEndpoint {
    pub fn get(&self) -> String {
        let path = match self {
            InfoEndpoint::MarketsInfo => "/info",
            InfoEndpoint::Prices => "/info/prices",
            InfoEndpoint::Kline => "/kline",
            InfoEndpoint::RecentTrades => "/trades",
            InfoEndpoint::OrderBook => "/book",
            InfoEndpoint::FundingRateHistory => "/funding_rate/history",
            InfoEndpoint::Account => "/account",
            InfoEndpoint::AccountSettings => "/account/settings",
            InfoEndpoint::Positions => "/positions",
            InfoEndpoint::TradesHistory => "/positions/history",
            InfoEndpoint::AccountFundingHistory => "/funding/history",
            InfoEndpoint::BalanceHistory => "/account/balance/history",
            InfoEndpoint::EquityHistory => "/portfolio",
            InfoEndpoint::OpenedOrders => "/orders",
            InfoEndpoint::OrderHistory => "/orders/history",
            InfoEndpoint::OrderHistoryById => "/orders/history_by_id",
        };

        // *Empty endpoints hasn't API_VERSION in path
        // let no_version = matches!(
        //     self,
        // );

        let no_version = false;

        if no_version {
            path.to_string()
        } else {
            format!("{API_VERSION_ENDPOINT}{path}")
        }
    }
}
