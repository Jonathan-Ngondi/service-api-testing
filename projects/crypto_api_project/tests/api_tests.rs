use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use serde::{
    Deserialize, 
    Serialize,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    convert::Infallible,
    fmt::Display,
};


pub const DUMMY_DATA_SERVER_TIME: &str = r#"
        {
            "error": [],
            "result": {
            "unixtime": 1616336594,
            "rfc1123": "Sun, 21 Mar 21 14:23:14 +0000"
            }
        }"#;

pub const DUMMY_DATA_ASSET_DATA: &str = r#"
        {
            "error": [ ],
            "result": {
            "XBT": {
            "aclass": "currency",
            "altname": "XBT",
            "decimals": 10,
            "display_decimals": 5
            },
            "ZEUR": {
            "aclass": "currency",
            "altname": "EUR",
            "decimals": 4,
            "display_decimals": 2
            },
            "ZUSD": {
            "aclass": "currency",
            "altname": "USD",
            "decimals": 4,
            "display_decimals": 2
            }
            }
            }
        "#;

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    fn to_str(&self) -> &str{
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        }

    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}


#[derive(Debug)]
struct HttpRequest {
    pub url: String,
    pub api_key: Option<String>,
    pub path: Option<String>,
    pub query_params: Option<String>,
    pub payload: Option<HttpPostPayload>,
    pub status: Option<usize>,
    pub header: Option<HashMap<String, String>>,
    pub method: Option<HttpMethod>,
    

}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    pub error: Vec<String>,
    pub result: Value,
}

#[derive(Debug)]
struct HttpPostPayload {
    pub nonce: u32,
    pub trades: bool,
    pub userref: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerTime{
     unixtime: u32,
     rfc1123: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetData {
    pub aclass: String,
    pub altname: String,
    pub decimals: u32,
    pub display_decimals: u32,
}

#[derive(Debug, WorldInit)]
struct HttpWorld{
    request: HttpRequest,
    response: Option<Response>,
}

#[async_trait(?Send)]
impl World for HttpWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok (Self {
            request: HttpRequest {
                url: String::from("https://api.kraken.com/0/public"),
                api_key: None,
                path: None,
                query_params: None,
                payload: None,
                status: None,
                header: None,
                method: None,
                },
            response: None, 
            }

         )
    }
}

#[given("an authorized http GET request to the public service")]
fn authorized_public_api(world: &mut HttpWorld){
    world.request.api_key= Some(String::from("DummyKey1234"))

}

#[when("user wants to get server time from the service")]
async fn get_server_time_response(world: &mut HttpWorld) {

    world.request.path = Some(String::from("/Time"));

    let mock_server = MockServer::start().await;
    let dummy_key = String::from("DummyKey");
    let key = world.request.api_key.as_ref().unwrap_or(&dummy_key);

   
    if key == "DummyKey1234"{ 
        Mock::given(method("GET"))
        .and(path("/Time"))
        .respond_with(ResponseTemplate::new(200).set_body_string(DUMMY_DATA_SERVER_TIME))
        .mount(&mock_server)
        .await;
    } else {
        Mock::given(method("GET"))
        .and(path("/Time"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;
    }



    let url = format!("{}{}", world.request.url, world.request.path.as_ref().unwrap()); 
    let response = surf::get(url)
        .await
        .unwrap();
    
    world.request.status= Some(response.status() as usize);
    world.response = Some(serde_json::from_str(response.body()).unwrap());
}

#[then("the server time request status is OK")]
async fn server_time_status_ok(world: &mut HttpWorld) {

    assert_eq!(world.request.status.unwrap(), 200 as usize);
    world.request.status = None;
}

#[then("the user retrieves the server time successfully")]
async fn retrieve_time_data(world: &mut HttpWorld) {
    let server_time = world.response.as_ref().unwrap();
    assert_eq!(server_time.result["unixtime"], 1616336594)

}

#[when("user wants to get XBT/USD asset data from the service")]
async fn get_xbt_usd_asset_data(world: &mut HttpWorld) {

    world.request.path = Some(String::from("/Assets"));

    let mock_server = MockServer::start().await;
    let dummy_key = String::from("DummyKey");
    let key = world.request.api_key.as_ref().unwrap_or(&dummy_key);
    
    if key == "DummyKey1234"{ 
        Mock::given(method("GET"))
        .and(path("/Assets"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let val = serde_json::from_str(DUMMY_DATA_ASSET_DATA).unwrap();

    world.response = Some(val);
    } else {
        Mock::given(method("GET"))
        .and(path("/Assets"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;
    }



    let url = format!("{}{}", world.request.url, world.request.path.as_ref().unwrap()); 
    let response = surf::get(url)
        .await
        .unwrap();
    
    world.request.status= Some(response.status() as usize);
}

#[then("the asset data request status is OK")]
async fn asset_data_status_ok(world: &mut HttpWorld) {
    assert_eq!(world.request.status.unwrap(), 200 as usize);
}

#[then("the user retrieves the asset data successfully")]
async fn retrieve_asset_data(world: &mut HttpWorld) {
    let asset_data = world.response.as_ref().unwrap();
    assert_eq!(asset_data.result["XBT"]["altname"], "XBT")

}

#[given("an authorized http POST request to the private service")]
fn authorized_public_api(world: &mut HttpWorld){
    world.request.api_key= Some(String::from("DummyKey1234"))
    world.payload = Some (HttpPostPayload{
        pub nonce: 231001238,
        pub trades: false,
        pub userref: 1234,
    });

}





#[tokio::main]
async fn main() {

    HttpWorld::run("tests/features/api_tests.feature").await;
    
}