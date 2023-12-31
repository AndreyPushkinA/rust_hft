mod binance;
mod huobi;
mod kraken;

use binance::*;
use huobi::*;
use kraken::*;
use tokio::time::{sleep, Duration};
use rocket::{get, routes, http::Status};
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use rocket::serde::json::Json as RocketJson; 
use serde_json::to_value;

#[get("/kraken_btc_price")]
async fn get_kraken_btc_price() -> Result<Json, Status> {
    match kraken_btc_price().await {
        Ok(price) => {
            let _timestamp = NaiveDateTime::parse_from_str(&price.time, "%Y-%m-%d %H:%M:%S%.9f")
                .map_err(|_| Status::InternalServerError)?;

            match to_value(price) {
                Ok(json_value) => Ok(json_value),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/kraken_btc_trades")]
async fn get_kraken_btc_trades() -> Result<RocketJson<Vec<Trade>>, Status> {
    match kraken_btc_trades().await {
        Ok(data) => Ok(RocketJson(data)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[get("/huobi_btc_price")]
async fn get_huobi_btc_price() -> Result<Json, Status> {
    match huobi_btc_price().await {
        Ok(price) => {
            let timestamp = NaiveDateTime::parse_from_str(&price.time, "%Y-%m-%d %H:%M:%S%.9f")
                .expect("Failed to parse datetime");

            match to_value(price) {
                Ok(json_value) => Ok(json_value),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/huobi_btc_trades")]
async fn get_huobi_btc_trades() -> Result<Json, Status> {
    match huobi_btc_trades().await {
        Ok(trades) => {
            match to_value(trades) {
                Ok(json_value) => Ok(json_value),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(e) => {
            eprintln!("Error fetching Huobi BTC trades: {:?}", e);
            Err(Status::InternalServerError)
        },
    }
}

#[get("/huobi_btc_asks")]
async fn get_huobi_btc_asks() -> Result<Json, Status> {
    match huobi_asks().await {
        Ok(asks) => {
            match to_value(asks) {
                Ok(json_value) => Ok(json_value),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(e) => {
            eprintln!("Error fetching Huobi asks: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/huobi_btc_bids")]
async fn get_huobi_btc_bids() -> Result<Json, Status> {
    match huobi_bids().await {
        Ok(bids) => {
            match to_value(bids) {
                Ok(json_value) => Ok(json_value),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(e) => {
            eprintln!("Error fetching Huobi bids: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}


#[get("/binance_btc_price")]
async fn get_binance_btc_price() -> Result<Json, Status> {
    match binance_btc_price().await {
        Ok(price) => {
            let timestamp = NaiveDateTime::parse_from_str(&price.time, "%Y-%m-%d %H:%M:%S%.9f").expect("Failed to parse datetime");
            
            match to_value(price) {
                Ok(json_value) => Ok(json_value),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/binance_btc_trades")]
async fn get_binance_btc_trades() -> Result<Json, Status> {
    match binance_btc_trades().await {
        Ok(trades) => {
            match to_value(trades) {
                Ok(Json::Object(map)) => Ok(Json::Object(map)),
                _ => Err(Status::InternalServerError),
            }
        },
        Err(e) => {
            eprintln!("Error fetching BTC trades: {:?}", e);
            Err(Status::InternalServerError)
        },
    }
}
#[get("/binance_btc_asks")]
async fn get_binance_btc_asks() -> Result<Json, Status> {
    match binance_order_book_data().await {
        Ok((_, asks_data)) => {
            match to_value(asks_data) {
                Ok(Json::Object(map)) => Ok(Json::Object(map)),
                _ => Err(Status::InternalServerError),
            }
        },
        Err(e) => {
            eprintln!("Error fetching BTC asks: {:?}", e);
            Err(Status::InternalServerError)
        },
    }
}

#[get("/binance_btc_bids")]
async fn get_binance_btc_bids() -> Result<Json, Status> {
    match binance_order_book_data().await {
        Ok((bids_data, _)) => {
            match to_value(bids_data) {
                Ok(Json::Object(map)) => Ok(Json::Object(map)),
                _ => Err(Status::InternalServerError),
            }
        },
        Err(e) => {
            eprintln!("Error fetching BTC bids: {:?}", e);
            Err(Status::InternalServerError)
        },
    }
}

async fn fetch_binance_btc_price() {
    loop {
        match binance_btc_price().await {
            Ok(price) => {
                let timestamp = match NaiveDateTime::parse_from_str(&price.time, "%Y-%m-%d %H:%M:%S%.9f") {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Failed to parse datetime: {:?}. Original string: {}", e, &price.time);
                        continue;
                    }
                };

                if let Err(e) = binance_price_clickhouse(&timestamp, &price.price).await {
                    eprintln!("Failed to insert into ClickHouse: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch price from Binance: {:?}", e);
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}

async fn fetch_huobi_btc_price() {
    loop {
        match huobi_btc_price().await {
            Ok(price) => {
                let timestamp = match NaiveDateTime::parse_from_str(&price.time, "%Y-%m-%d %H:%M:%S%.9f") {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Failed to parse datetime: {:?}. Original string: {}", e, &price.time);
                        continue;
                    }
                };

                if let Err(e) = binance_price_clickhouse(&timestamp, &price.price).await {
                    eprintln!("Failed to insert into ClickHouse: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch price from Huobi: {:?}", e);
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}

#[rocket::main]
async fn main() {
    tokio::spawn(fetch_binance_btc_price());
    tokio::spawn(fetch_huobi_btc_price());

    let result = rocket::build()
        .mount("/", routes![get_binance_btc_price, get_binance_btc_trades, get_binance_btc_asks, get_binance_btc_bids, get_huobi_btc_price, get_huobi_btc_trades, get_huobi_btc_asks, get_huobi_btc_bids, get_kraken_btc_price, get_kraken_btc_trades])
        .launch()
        .await;

    match result {
            Ok(_) => println!("Server started successfully!"),
            Err(err) => eprintln!("Server failed to start: {:?}", err),
    }
}
