use crate::encryption;
use crate::error::{Error};
use crate::api_library::Result;
use crate::two_factor_auth::*;
use serde::{
    Deserialize,
    de::DeserializeOwned,
};
use reqwest::{
    Client as ReqwestClient,
    header,
};
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

#[derive(Debug, Deserialize)]
struct Response <T> {
    pub error: Vec<String>,
    pub result: Option <T>,
}

const BASE_URL: & str = "https://api.kraken.com";

#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    api_secret: Option<String>,
    google_auth: Option<String>,
    http_client: Option<ReqwestClient>,
}

impl ClientBuilder {
    
    pub fn base_url(mut self, base_url: &str) -> Self{
        self.base_url = Some(base_url.to_string());
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn api_secret(mut self, api_secret: &str) -> Self {
        self.api_secret = Some(api_secret.to_string());
        self
    }
    pub fn google_auth(mut self, google_auth: &str) -> Self {
        self.google_auth = Some(google_auth.to_string());
        self
    }

    pub fn http_client(mut self, http_client: ReqwestClient) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self) -> Client {
        Client {
            base_url: self
            .base_url
            .unwrap_or_else(|| BASE_URL.to_string()),
            api_key: self.api_key,
            api_secret: self.api_secret,
            google_auth: self.google_auth,
            http_client: self.http_client.unwrap_or_else(ReqwestClient::new),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    google_auth: Option<String>,
    http_client: ReqwestClient,
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str) -> Self{
        Self::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
async fn unwrap_response<Resp>(&self, response: reqwest::Response) -> Result<Resp> 
where
    Resp: DeserializeOwned,
{   

    
    let response: Response<Resp> = response.json().await?;
    

    if !response.error.is_empty(){
        return Err(Error::Api(response.error.join(",")));
    }

    if let Some(result) = response.result {
        Ok(result)
    } else {
        Err(Error::internal("No result field present in response"))
    }

    
}

pub async fn make_public_api_call<Resp>(&self, url: &str) -> Result<Resp> 
where
    Resp: DeserializeOwned
{
    let url_path = format!("{}{}", self.base_url, url);

    let response = self.http_client
                    .get(&url_path)
                    .send()
                    .await?;

    self.unwrap_response(response).await
    
}

pub async fn make_private_api_call<Resp>(&self, url: &str, query: Option<String>, two_factor_enabled: bool) -> Result<Resp>
where
    Resp: DeserializeOwned,
    {
        let response = if let Some (api_key) = &self.api_key {
            if let Some(api_secret) = &self.api_secret {
                let url_path = url;

                let url = format!("{}{}", self.base_url, url);

                let nonce = calculate_nonce().to_string();

                let mut form_data = if let Some(query) = query {
                    format!("{}&nonce={}", query, nonce)
                } else {
                    format!("nonce={}", nonce)
                };

                if two_factor_enabled {
                    let totp = get_code(self.google_auth.as_ref().unwrap()).unwrap();
                    form_data = format!("{}&otp={}", form_data, totp);
                }

                self.http_client
                    .post(&url)
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header("API-Key", api_key)
                    .header("API-Sign", encryption::get_signature(api_secret, url_path, &nonce, &form_data).unwrap()
                    )
                    .body(form_data.into_bytes())
                    .send()
                    .await?
            } else {
                return Err(Error::Unauthorzed);
            } 

        } else {
            return Err(Error::Unauthorzed);
        };

        self.unwrap_response(response).await
    }
}


pub fn calculate_nonce() -> u64 {
    let now = SystemTime::now();
    let time_from_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    time_from_epoch.as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::calculate_nonce;

    #[test]
    fn test_calculate_nonce() {
        let nonce1 = calculate_nonce();

        std::thread::sleep(std::time::Duration::from_millis(2));

        let nonce2 = calculate_nonce();

        assert!(nonce1 < nonce2);
    }
}