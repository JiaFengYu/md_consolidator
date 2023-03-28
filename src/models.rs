use serde::de;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct OfferData {
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}

#[derive(Debug, Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct BinanceDepthStreamData {
    pub s: String,
    pub b: String,
    pub a: String,
    pub B: String,
    pub A: String,
    pub E: i64,
}

#[derive(Debug, Deserialize)]
pub struct BinanceTradeStreamData {
    pub s: String,
    pub E: i64,
    pub p: String,
    pub q: String,
    pub e: String,
}

#[derive(Debug, Deserialize)]
pub struct FTXStreamData {
    pub bid: f64,
    pub ask: f64,
    pub bidSize: f64,
    pub askSize: f64,
    pub time: f64,
}

#[derive(Debug, Deserialize)]
pub struct DeribitData {
    pub timestamp: i64,
    pub instrument_name: String,
    pub best_bid_price: f64,
    pub best_bid_amount: f64,
    pub best_ask_price: f64,
    pub best_ask_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct DeribitStreamData {
    pub channel: String,
    pub data: DeribitData,
}


pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct BinanceDepthStreamWrapper {
    pub stream: String,
    pub data: BinanceDepthStreamData,
}

#[derive(Debug, Deserialize)]
pub struct FTXStreamWrapper {
    pub market: String,
    pub data: FTXStreamData,
}

#[derive(Debug, Deserialize)]
pub struct DeribitStreamWrapper {
    pub params: DeribitStreamData,
}