use crate::api_library::Result;
use crate::Client;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

#[must_use = "Does nothing until you send or execute it"]
pub struct GetAssetPairInfoRequest {
    client: Client,
    pair: Option<String>,
}

impl GetAssetPairInfoRequest {
    pub fn asset(self, pair: impl Into<String>) -> Self {
        Self {
            pair: Some(pair.into()),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let url = if let Some(pair) = &self.pair {
            format!("/0/public/AssetPairs?pair={}", pair)
        } else {
            String::from("/0/public/AssetPairs")
        };

        self.client.make_public_api_call(&url).await
    }

    pub async fn send(self) -> Result<GetAssetPairInfoResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct FeeSchedule(f64, f64);

#[derive(Debug, Deserialize)]
pub struct PairInfo {
    pub altname: String,
    pub wsname: Option<String>,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub lot: String,
    pub pair_decimals: i32,
    pub lot_decimals: i32,
    pub lot_multiplier: i32,
    pub leverage_buy: Vec<f64>,
    pub leverage_sell: Vec<f64>,
    pub fees: Vec<FeeSchedule>,
    pub fees_maker: Option<Vec<FeeSchedule>>,
    pub fee_volume_currency: String,
    pub margin_call: f64,
    pub margin_stop: f64,
    pub ordermin: Option<String>,
}

pub type GetAssetPairInfoResponse = HashMap<String, PairInfo>;

impl Client {
    pub fn get_asset_pair_info(&self) -> GetAssetPairInfoRequest {
        GetAssetPairInfoRequest {
            client: self.clone(),
            pair: None,
        }
    }
}
