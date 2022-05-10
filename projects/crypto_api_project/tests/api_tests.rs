use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use serde::{
    Deserialize, 
    Serialize,
};

use std::{
    collections::HashMap,
    convert::Infallible,
    str::FromStr,
};

#[derive(Debug, Deserialize)]
struct Response <T> {
    pub error: Vec<String>,
    pub result: Option <T>,
}

#[derive(Debug)]
struct OpenOrdersPostPayload {
    pub _nonce: u32,
    pub _trades: bool,
    pub _userref: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerTime{
     unixtime: u32,
     rfc1123: String,
}
#[derive(Debug, Deserialize)]
pub struct GetOpenOrders {
    pub open: HashMap<String, OpenOrderInfo>,
}

type GetAssetPairInfo = HashMap<String, PairInfo>;
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

#[derive(Debug, Deserialize)]
pub struct OpenOrderInfo {
    pub status: String,
    pub cost: String,
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
    pub orderside: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub order: String,
    pub close: String,
}
#[derive(Debug)]
enum State {
    Authorized,
    Unauthorzed,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "authorized" => Self::Authorized,
            "unauthorized" => Self::Unauthorzed,
            invalid => return Err(format!("Invalid `State`: {invalid}")),
        })
    }
}

#[derive(Debug)]
struct MyClient {
    _url: String,
    api_key: Option<String>,
    api_sign: Option<String>,
    nonce: u64,
    otp: Option<u32>,
    status: Option<u32>,
    response_server_time: Option<ServerTime>,
    response_asset_data: Option<GetAssetPairInfo>,
    response_open_orders: Option<Response<GetOpenOrders>>, 
}

impl MyClient{
    fn new() -> Self{ Self {
        _url: BASE_URL.to_string(),
        api_key: None,
        api_sign: None,
        nonce: 1000,
        otp: None,
        status: None,
        response_server_time: None,
        response_asset_data: None,
        response_open_orders: None,
        }
    }

    fn make_public_api_call_to_server_time(&mut self)-> Response<ServerTime> {
        self.status = Some(200u32);
        serde_json::from_str(DUMMY_SERVER_TIME).unwrap()
    }
    
    fn make_public_api_call_for_asset_data(&mut self) -> Response<GetAssetPairInfo> {
        self.status = Some(200u32);
        serde_json::from_str(DUMMY_ASSET_DATA).unwrap()
    }

    fn make_private_api_call_for_open_orders(
        &mut self, 
        api_key: Option<String>, 
        api_sign: Option<String>,
        nonce: u64,
        ) 
        -> Response<GetOpenOrders>{

        self.status = Some(200u32);
        
        if let Some(otp) = self.otp {
            if !self.verify_otp(otp) {
                return serde_json::from_str(INVALID_NONCE_ERROR).unwrap();
            }
        }

        if !self.verify_nonce(nonce) {
            return serde_json::from_str(INVALID_NONCE_ERROR).unwrap();
        }
        
        if let Some(api_key) = api_key {
            if let Some(api_sign) = api_sign {
                match (api_key.as_str(), api_sign.as_str()){
                    ("DummyKey54321", "DummySign54321") => serde_json::from_str(DUMMY_DATA_OPEN_ORDERS).unwrap(),
                    (a,b) if (a == "DummyKey54321"&& b != "DummySign54321") => 
                                                            serde_json::from_str(INVALID_SIGN_ERROR).unwrap(),
                    (a,b) if (a != "DummyKey54321" && b == "DummySign54321") => 
                                                            serde_json::from_str(INVALID_KEY_ERROR).unwrap(),
                    (_, _) => serde_json::from_str(INVALID_KEY_ERROR).unwrap(),
                }
            } else {
                serde_json::from_str(INVALID_SIGN_ERROR).unwrap()
            }
        } else {
            serde_json::from_str(INVALID_KEY_ERROR).unwrap()
        }
    }

    fn verify_nonce(&mut self, nonce: u64) -> bool{
        nonce > self.nonce

    }

    fn verify_otp(&mut self, otp: u32) -> bool {
        otp == 654321
    }
}


#[derive(Debug, WorldInit)]
struct MyWorld{
    client: MyClient,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok (Self {
            client: MyClient::new(),
        }
    )
    }
}

#[when("user wants to get server time from the service")]
async fn get_server_time_response(world: &mut MyWorld) {
    world.client.response_server_time =  world.client.make_public_api_call_to_server_time().result;
}

#[then("the server time request status is OK")]
async fn time_data_retrieval_was_ok(world: &mut MyWorld) {
    assert_eq!(world.client.status.as_ref().unwrap(), &200u32)
}

#[then("the user retrieves the server time successfully")]
async fn server_time_status_ok(world: &mut MyWorld) {
    assert_eq!(world.client.response_server_time.as_ref().unwrap().unixtime, 1616334)
}

#[when("user wants to get XBT/USD asset data from the service")]
async fn get_xbt_usd_asset_data(world: &mut MyWorld) {
    world.client.response_asset_data = world.client.make_public_api_call_for_asset_data().result;
}

#[then("the asset data request status is OK")]
async fn asset_data_status_ok(world: &mut MyWorld) {
    assert_eq!(world.client.status.as_ref().unwrap(), &200u32)
}

#[then("the user retrieves the asset data successfully")]
async fn retrieve_asset_data(world: &mut MyWorld) {
    assert_eq!(
        world.client.response_asset_data.as_ref().unwrap().get("XXBTZUSD").unwrap().altname, "XBTUSD")
}

#[given(regex = r"^an (authorized|unauthorized) http POST request to the private service$")]
fn authorized_private_api(world: &mut MyWorld, state: State){
    match state {
        State::Authorized => {
                        world.client.api_key = Some("DummyKey54321".to_string());
                        world.client.api_sign = Some("DummySign54321".to_string());
                       },
        State::Unauthorzed => {
                        world.client.api_key = None;
                        world.client.api_sign = None;
                        }
    };
}


#[when("user retrieves open orders from the open orders endpoint with valid key and sign")]
async fn retrieve_open_orders(world: &mut MyWorld){
    let api_key = world.client.api_key.as_ref().unwrap().clone();
    let api_sign = world.client.api_sign.as_ref().unwrap().clone();
    world.client.response_open_orders = 
    Some(world.client.make_private_api_call_for_open_orders(
        Some(api_key),
        Some(api_sign), 
        1003u64,
    ));
}

#[then("the open order request status is OK")]
fn check_open_orders_status_code(world:&mut MyWorld){
    assert_eq!(world.client.status.as_ref().unwrap(), &200u32)
}

#[then("the user retrieves the open orders successfully")]
fn retrieve_open_orders_successfully(world:&mut MyWorld){
    assert_eq!(
        world.client.response_open_orders.as_ref().unwrap().result.as_ref().unwrap().open.len(), 5)
}

#[when("user retrieves open orders from the open orders endpoint with invalid key")]
async fn retrieve_open_orders_with_invalid_key(world: &mut MyWorld){
    world.client.response_open_orders = 
    Some(world.client.make_private_api_call_for_open_orders(
        Some("Boy, is this fun".to_string()),
        Some("DummySign54321".to_string()), 
        1004u64,
    ));
}

#[then("the user receives an invalid key error")]
fn get_error_invalid_key(world:&mut MyWorld){
    assert_eq!(
        world.client.response_open_orders.as_ref().unwrap().result.as_ref().unwrap().open.len(), 0);
    assert_eq!(world.client.response_open_orders
        .as_ref().unwrap().error.get(0).unwrap(), "EAPI:Invalid key")
}


#[when("user retrieves open orders from the open orders endpoint with invalid sign")]
async fn retrieve_open_orders_with_invalid_sign(world: &mut MyWorld){
    world.client.response_open_orders = 
    Some(world.client.make_private_api_call_for_open_orders(
        Some("DummyKey54321".to_string()),
        Some("Boy, is this fun".to_string()),
        1006u64,
        ));
}

#[then("the user receives an invalid sign error")]
fn get_error_invalid_sign(world:&mut MyWorld){
    assert_eq!(
        world.client.response_open_orders.as_ref().unwrap().result.as_ref().unwrap().open.len(), 0);
    assert_eq!(world.client.response_open_orders
        .as_ref().unwrap().error.get(0).unwrap(), "EAPI:Invalid signature")
}

#[when("user retrieves open orders from the open orders endpoint with invalid nonce")]
async fn retrieve_open_orders_with_invalid_nonce(world: &mut MyWorld){
    world.client.response_open_orders = 
    Some(world.client.make_private_api_call_for_open_orders(
        Some("DummyKey54321".to_string()),
        Some("DummySign54321".to_string()),
        106u64,
        ));
}

#[then("the user receives an invalid nonce error")]
fn get_error_invalid_nonce(world:&mut MyWorld){
    assert_eq!(
        world.client.response_open_orders.as_ref().unwrap().result.as_ref().unwrap().open.len(), 0);
    assert_eq!(world.client.response_open_orders
        .as_ref().unwrap().error.get(0).unwrap(), "EAPI:Invalid nonce")
}

#[when("the user tries to access a private endpoint with 2FA enabled without otp in payload")]
async fn retrieve_open_orders_with_invalid_otp(world: &mut MyWorld){
    world.client.otp = Some(987654u32);
    world.client.response_open_orders = 
    Some(world.client.make_private_api_call_for_open_orders(
        Some("DummyKey54321".to_string()),
        Some("DummySign54321".to_string()),
        106u64,
        ));
}

#[then("user receives a permission denied error")]
fn get_error_invalid_otp(world:&mut MyWorld){
    assert_eq!(
        world.client.response_open_orders.as_ref().unwrap().result.as_ref().unwrap().open.len(), 0);
    assert_eq!(world.client.response_open_orders
        .as_ref().unwrap().error.get(0).unwrap(), "EAPI:Invalid nonce")
}


mod dummy_data;
pub use crate::dummy_data::*;
#[cfg(feature = "output-json")]
#[tokio::main]
async fn main() {
    println!("----------------Starting Tests--------------");
     MyWorld::run("tests/features/public_api_tests.feature").await;
     MyWorld::run("tests/features/private_api_tests.feature").await;
     println!("---------------------End--------------------");
}
