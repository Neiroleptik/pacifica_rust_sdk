use std::str::FromStr;

use pacifica_rust_sdk::{
    common::{
        types::{AggLevel, EquityHistoryInterval, Interval},
        utils::get_timestamp_ms,
    },
    info::info_client::InfoClient,
    logging::init_logging_once,
};
use solana_sdk::pubkey::Pubkey;
use tracing::info;

#[tokio::main]
async fn main() {
    init_logging_once("debug");
    let api_key = None; // Or = Some("YourApiKeyString".to_string())
    let client = InfoClient::new(false, false, api_key).await.unwrap();
    // Test pubkey
    let account = Pubkey::from_str("bNMBfVAEjgxhNruYTR2vSZ92UdVKP2pdZewEG7978Vt").unwrap();

    let now = get_timestamp_ms();
    let symbol = "ETH";

    match client.get_markets_info().await {
        Ok(markets) => {
            info!("--- Markets Info ---\n{:#?}", markets);
        }
        Err(e) => {
            info!("--- Markets Info ---\nError: {:?}", e);
        }
    }

    match client.prices().await {
        Ok(prices) => info!("--- Prices ---\n{:#?}", prices),
        Err(e) => info!("--- Prices ---\nError: {:?}", e),
    }

    match client
        .kline(symbol.to_string(), Interval::OneMinute, now - 60_000, now)
        .await
    {
        Ok(kline) => info!("--- Kline ---\n{:#?}", kline),
        Err(e) => info!("--- Kline ---\nError: {:?}", e),
    }

    match client.recent_trades(symbol.to_string()).await {
        Ok(trades) => info!("--- Recent Trades ---\n{:#?}", trades),
        Err(e) => info!("--- Recent Trades ---\nError: {:?}", e),
    }

    match client.order_book(symbol.to_string(), AggLevel::L1).await {
        Ok(book) => info!("--- Order Book ---\n{:#?}", book),
        Err(e) => info!("--- Order Book ---\nError: {:?}", e),
    }

    match client.funding_rate_history(symbol.to_string()).await {
        Ok(fr) => info!("--- Funding Rate History ---\n{:#?}", fr),
        Err(e) => info!("--- Funding Rate History ---\nError: {:?}", e),
    }

    match client.account(account).await {
        Ok(acc) => info!("--- Account Info ---\n{:#?}", acc),
        Err(e) => info!("--- Account Info ---\nError: {:?}", e),
    }

    match client
        .account_funding_history(account, Some(10), Some(0))
        .await
    {
        Ok(fh) => info!("--- Account Funding History ---\n{:#?}", fh),
        Err(e) => info!("--- Account Funding History ---\nError: {:?}", e),
    }

    match client.account_settings(account).await {
        Ok(settings) => info!("--- Account Settings ---\n{:#?}", settings),
        Err(e) => info!("--- Account Settings ---\nError: {:?}", e),
    }

    match client.positions(account).await {
        Ok(positions) => info!("--- Positions ---\n{:#?}", positions),
        Err(e) => info!("--- Positions ---\nError: {:?}", e),
    }

    match client
        .trade_history(
            account,
            Some(symbol.to_string()),
            Some(now - 400_000),
            Some(now),
            Some(10),
            Some(0),
        )
        .await
    {
        Ok(th) => info!("--- Trade History ---\n{:#?}", th),
        Err(e) => info!("--- Trade History ---\nError: {:?}", e),
    }

    match client.balance_history(account, Some(10), Some(0)).await {
        Ok(bh) => info!("--- Balance History ---\n{:#?}", bh),
        Err(e) => info!("--- Balance History ---\nError: {:?}", e),
    }

    match client
        .equity_history(
            account,
            EquityHistoryInterval::OneDay,
            Some(now - 1_000_000),
            Some(now),
            None,
            Some(10),
            Some(0),
        )
        .await
    {
        Ok(eh) => info!("--- Equity History ---\n{:#?}", eh),
        Err(e) => info!("--- Equity History ---\nError: {:?}", e),
    }

    match client.opened_orders(account).await {
        Ok(oo) => info!("--- Opened Orders ---\n{:#?}", oo),
        Err(e) => info!("--- Opened Orders ---\nError: {:?}", e),
    }

    match client.order_history(account, Some(10), Some(0)).await {
        Ok(oh) => info!("--- Order History ---\n{:#?}", oh),
        Err(e) => info!("--- Order History ---\nError: {:?}", e),
    }

    match client.order_history_by_id(123456789).await {
        Ok(ohid) => info!("--- Order History By ID ---\n{:#?}", ohid),
        Err(e) => info!("--- Order History By ID ---\nError: {:?}", e),
    }
}
