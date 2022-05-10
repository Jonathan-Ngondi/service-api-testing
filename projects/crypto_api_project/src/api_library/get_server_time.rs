use crate::Client;
use crate::api_library::Result;
use serde::{de::DeserializeOwned, Deserialize};

//  https://api.kraken.com/0/public/Time
#[must_use = "Does nothing until you send or execute it"]
pub struct GetServerTimeRequest {
    client: Client,
}

impl GetServerTimeRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client.make_public_api_call("/0/public/Time").await
    }

    pub async fn send(self) -> Result<GetServerTimeResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct GetServerTimeResponse {
    pub unixtime: i64,
    pub rfc1123: String,
}

impl Client {
    pub fn get_server_time(&self) -> GetServerTimeRequest {
        GetServerTimeRequest {
            client: self.clone(),
        }
    }
}
