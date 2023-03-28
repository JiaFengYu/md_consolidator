use crypto_ws_client::{BinanceLinearWSClient, DeribitWSClient, FtxWSClient, WSClient};
use std::io::{Write};
use std::fs::File;
use chrono::Utc;
use std::collections::HashMap;
mod models;
use futures::future;
use std::thread;
use std::sync::{Arc, Mutex};
use std::fmt;

#[derive(Debug)]
struct QuoteEntry {
    pub bestBid: f64,
    pub bestBidAmount: f64,
    pub bestAsk: f64,
    pub bestAskAmount: f64,
    pub timestamp: i64,    
    pub localTimestamp: i64,
}

#[tokio::main]
async fn binance_stream(orderbook: Arc<std::sync::Mutex<HashMap<std::string::String, QuoteEntry>>>) { 
    let (tx, rx) = std::sync::mpsc::channel();
    // add symbols to this vector to subscribe to other instruments
    let symbols = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]; // to showcase scalability 
                                                                      //    let mut map = HashMap::new();
                                                                      //for i in &symbols{
                                                                      //map.insert(i,format!("{i}_PERP_BINANCE"));
                                                                      //}
    tokio::task::spawn(async move {
        let ws_client = BinanceLinearWSClient::new(tx, None).await;
        ws_client.subscribe_bbo(&symbols).await;
        let _ = tokio::time::timeout(std::time::Duration::MAX, ws_client.run()).await;
        ws_client.close();
    });

    let path = "logs/binance_logs.txt"; //requires a logs folder
    let mut output = File::create(path);

    for msg in rx {
        println!("{}", msg);
        let parsed: models::BinanceDepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");
        write!(output.as_ref().expect("Failed to write log."), 
               "{}: bestBid {}, bestBitQuantity {}, bestAsk {}, bestAskQuantity {}, eventTime {}, localTimestamp {}, totalLatency {}\n",
               parsed.data.s, parsed.data.b, parsed.data.B, parsed.data.a, parsed.data.A, parsed.data.E, Utc::now().timestamp_millis(), Utc::now().timestamp_millis()-parsed.data.E
              );
        orderbook.lock().unwrap().insert("BTCUSDT_PERP_BINANCE".to_string(), QuoteEntry {
            bestBid:parsed.data.b.parse().unwrap(), bestBidAmount: parsed.data.B.parse().unwrap(), bestAsk: parsed.data.a.parse().unwrap(),
            bestAskAmount: parsed.data.A.parse().unwrap(), timestamp: parsed.data.E,localTimestamp: Utc::now().timestamp_millis()});

        //orderbook.lock().unwrap().insert(map[&parsed.data.s], QuoteEntry {
        //    bestBid:parsed.data.b.parse().unwrap(), bestBidAmount: parsed.data.B.parse().unwrap(), bestAsk: parsed.data.a.parse().unwrap(),
        //    bestAskAmount: parsed.data.A.parse().unwrap(), timestamp: parsed.data.E,localTimestamp: Utc::now().timestamp_millis()});
    };
}

#[tokio::main]
async fn ftx_stream(orderbook: Arc<std::sync::Mutex<HashMap<std::string::String, QuoteEntry>>>){
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        let symbols = vec!["BTC-PERP".to_string()];
        let ws_client = FtxWSClient::new(tx, None).await;
        ws_client.subscribe_bbo(&symbols).await;
        let _ = tokio::time::timeout(std::time::Duration::MAX, ws_client.run()).await;
        ws_client.close();
    });

    let path = "logs/ftx_logs.txt"; //requires a logs folder
    let mut output = File::create(path);
    for msg in rx {
        println!("{}", msg);
        let parsed: models::FTXStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");
        let ftx_timestamp = (parsed.data.time *1000.0).round() as i64;
        write!(output.as_ref().expect("Failed to write log."), 
               "{}: bestBid {}, bestBitQuantity {}, bestAsk {}, bestAskQuantity {}, eventTime {}, localTimestamp {}, totalLatency {}\n",
               parsed.market, parsed.data.bid, parsed.data.bidSize, parsed.data.ask, parsed.data.askSize, ftx_timestamp, Utc::now().timestamp_millis(),  Utc::now().timestamp_millis()-ftx_timestamp
              );
        orderbook.lock().unwrap().insert("BTCUSDT_PERP_FTX".to_string(), QuoteEntry {
            bestBid:parsed.data.bid, bestBidAmount: parsed.data.bidSize, bestAsk: parsed.data.ask,
            bestAskAmount: parsed.data.askSize, timestamp: ftx_timestamp, localTimestamp: Utc::now().timestamp_millis()});
    };
}

#[tokio::main]
async fn deribit_stream(orderbook: Arc<std::sync::Mutex<HashMap<std::string::String, QuoteEntry>>> ) {
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        let symbols = vec!["BTC-PERPETUAL".to_string()];
        let ws_client = DeribitWSClient::new(tx, None).await;
        ws_client.subscribe_bbo(&symbols).await;
        let _ = tokio::time::timeout(std::time::Duration::MAX, ws_client.run()).await;
        ws_client.close();
    });

    let path = "logs/deribit_logs.txt"; //requires a logs folder
    let mut output = File::create(path);
    for msg in rx {
        println!("{}", msg);
        let parsed: models::DeribitStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");
        write!(output.as_ref().expect("Failed to write log."), 
               "{}: bestBid {}, bestBitQuantity {}, bestAsk {}, bestAskQuantity {}, eventTime {}, localTimestamp {}, totalLatency {}\n",
               parsed.params.data.instrument_name, parsed.params.data.best_bid_price, parsed.params.data.best_bid_amount, parsed.params.data.best_ask_price, parsed.params.data.best_ask_amount, parsed.params.data.timestamp, Utc::now().timestamp_millis(),  Utc::now().timestamp_millis()-parsed.params.data.timestamp
              );
        orderbook.lock().unwrap().insert("BTCUSDT_PERP_DERIBIT".to_string(), QuoteEntry {
            bestBid: parsed.params.data.best_bid_price, bestBidAmount: parsed.params.data.best_bid_amount, bestAsk: parsed.params.data.best_ask_price,
            bestAskAmount: parsed.params.data.best_ask_amount, timestamp: parsed.params.data.timestamp, localTimestamp: Utc::now().timestamp_millis()});
    };
}

//#[tokio::main]
fn main() {
    let orderbook = Arc::new(Mutex::new(HashMap::new()));
    orderbook.lock().unwrap().insert("BTCUSDT_PERP_BINANCE".to_string(), QuoteEntry {bestBid: 0.0, bestBidAmount: 0.0, bestAsk: 0.0, bestAskAmount: 0.0, timestamp: 0, localTimestamp: 0});
    orderbook.lock().unwrap().insert("BTCUSDT_PERP_FTX".to_string(), QuoteEntry {bestBid: 0.0, bestBidAmount: 0.0, bestAsk: 0.0, bestAskAmount: 0.0, timestamp: 0, localTimestamp: 0});
    orderbook.lock().unwrap().insert("BTCUSDT_PERP_DERIBIT".to_string(), QuoteEntry {bestBid: 0.0, bestBidAmount: 0.0, bestAsk: 0.0, bestAskAmount: 0.0, timestamp: 0, localTimestamp: 0});

    // Clone the Arc to allow for share-state concurrency 
    let binance_orderbook_ref = Arc::clone(&orderbook);
    let ftx_orderbook_ref = Arc::clone(&orderbook);
    let deribit_orderbook_ref = Arc::clone(&orderbook);

    // Spawn a thread for each stream. Let them update orderbook concurrently
    let binance_handle = thread::spawn(move || {
        binance_stream(binance_orderbook_ref);
    }
    );
    let ftx_handle = thread::spawn(move || {
        ftx_stream(ftx_orderbook_ref);
    }
    );

    let deribit_handle = thread::spawn(move || {
        deribit_stream(deribit_orderbook_ref);
    }
    );

    loop {
        println!("{:?}", orderbook);
        //thread::sleep(std::time::Duration::from_secs(1));
    }
}
