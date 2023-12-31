use reqwest::Error;
use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::TimeZone;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct BtcPrice {
    price: String,
    pub time: String,
}

#[derive(Deserialize, Debug)]
pub struct TradeResponse {
    tick: TickData,
}

#[derive(Deserialize, Debug)]
pub struct TickData {
    data: Vec<TradeData>,
}

#[derive(Deserialize, Debug)]
pub struct TradeData {
    price: f64,
}

#[derive(Deserialize, Debug)]
struct Trade {
    id: f64,
    ts: u64,
    price: f64,
    amount: f64,
    direction: String,
}

#[derive(Deserialize, Debug)]
struct Tick {
    data: Vec<Trade>,
}

#[derive(Deserialize, Debug)]
struct Trades {
    tick: Tick,
}

#[derive(Serialize, Debug)]
pub struct BtcTrade {
    pub timestamp: String,
    pub price: String,
    pub amount: String,
    pub direction: String,
}

#[derive(Deserialize, Debug)]
struct DepthData {
    asks: Vec<Vec<f64>>,
    bids: Vec<Vec<f64>>,
}

#[derive(Deserialize, Debug)]
struct DepthResponse {
    tick: DepthData,
}

#[derive(Serialize, Debug)]
pub struct OrderBookEntry {
    pub price: String,
    pub quantity: String,
}

#[derive(Serialize, Debug)]
pub struct OrderBook {
    pub asks: Vec<OrderBookEntry>,
    pub bids: Vec<OrderBookEntry>,
}

pub async fn huobi_btc_price() -> Result<BtcPrice, Error> {
    let symbol = "btcusdt";
    let ticker_url = format!("https://api.huobi.pro/market/trade?symbol={}", symbol);
    let response: TradeResponse = reqwest::get(&ticker_url).await?.json().await?;
    let current_time = Utc::now().naive_utc();

    let most_recent_trade = response.tick.data.get(0);

    Ok(BtcPrice {
        price: most_recent_trade.as_ref().unwrap().price.to_string(),
        time: current_time.to_string(),
    })
}

pub async fn huobi_btc_trades() -> Result<Vec<BtcTrade>, Error> {
    let symbol = "btcusdt";
    let trades_url = format!("https://api.huobi.pro/market/trade?symbol={}", symbol);
    let response: Trades = reqwest::get(&trades_url).await?.json().await?;

    if response.tick.data.is_empty() {
        return Ok(vec![]);
    }

    let trades: Vec<BtcTrade> = response.tick.data.iter().map(|trade| {
        BtcTrade {
            timestamp: Utc.timestamp_opt((trade.ts / 1000) as i64, 0).single().unwrap_or(Utc.timestamp(0, 0)).to_string(),
            price: trade.price.to_string(),
            amount: trade.amount.to_string(),
            direction: trade.direction.clone(),
        }
    }).collect();

    Ok(trades)
}

pub async fn huobi_asks() -> Result<Vec<OrderBookEntry>, Error> {
    let symbol = "btcusdt";
    let depth_url = format!("https://api.huobi.pro/market/depth?symbol={}&type=step0", symbol);
    let response: DepthResponse = reqwest::get(&depth_url).await?.json().await?;

    let asks: Vec<OrderBookEntry> = response.tick.asks.iter().take(20).map(|ask| {
        OrderBookEntry {
            price: ask[0].to_string(),
            quantity: ask[1].to_string(),
        }
    }).collect();

    Ok(asks)
}

pub async fn huobi_bids() -> Result<Vec<OrderBookEntry>, Error> {
    let symbol = "btcusdt";
    let depth_url = format!("https://api.huobi.pro/market/depth?symbol={}&type=step0", symbol);
    let response: DepthResponse = reqwest::get(&depth_url).await?.json().await?;

    let bids: Vec<OrderBookEntry> = response.tick.bids.iter().take(20).map(|bid| {
        OrderBookEntry {
            price: bid[0].to_string(),
            quantity: bid[1].to_string(),
        }
    }).collect();

    Ok(bids)
}
