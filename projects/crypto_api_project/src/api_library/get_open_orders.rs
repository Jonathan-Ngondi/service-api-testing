use crate::Client;
use crate::api_library::Result;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;


// - https://api.kraken.com/0/private/OpenOrders
#[must_use = "Does nothing until you send or execute it"]
#[allow(dead_code)]
pub struct GetOpenOrdersRequest {
    client: Client,
    trades: Option<bool>,
    userref: Option<i32>,
    otp: Option<u32>,
}

impl GetOpenOrdersRequest {
    pub fn trades(self, trades: bool) -> Self {
        Self {
            trades: Some(trades),
            ..self
        }
    }

    pub fn userref(self, userref: i32) -> Self {
        Self {
            userref: Some(userref),
            ..self
        }
    }

    pub fn otp(self, otp: u32) -> Self {
        Self {
            otp: Some(otp),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query: Vec<String> = Vec::new();

        if let Some(true) = self.trades {
            query.push(String::from("trades=true"));
        }

        if let Some(userref) = self.userref {
            query.push(format!("userref={}", userref));
        }

        let query = if query.is_empty() {
            None
        } else {
            Some(query.join("&"))
        };

        self.client
            .make_private_api_call("/0/private/OpenOrders", query, false)
            .await
    }

    pub async fn send(self) -> Result<GetOpenOrdersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenOrderInfo {
    pub status: String,
    pub cost: String,
    pub descr: OrderDescription,
    pub opentm: f64,
    pub oflags: String,
    pub fee: String,
    pub vol: String,
    pub vol_executed: Option<String>,
    pub userref: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct OrderDescription {
    pub pair: String,
    #[serde(rename(deserialize = "type"))]
    pub orderside: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub order: String,
    pub close: String,
}

#[derive(Debug, Deserialize)]
pub struct GetOpenOrdersResponse {
    pub open: HashMap<String, OpenOrderInfo>,
}

impl Client {
    pub fn get_open_orders(&self) -> GetOpenOrdersRequest {
        GetOpenOrdersRequest {
            client: self.clone(),
            trades: None,
            userref: None,
            otp: None,
        }
    }
}