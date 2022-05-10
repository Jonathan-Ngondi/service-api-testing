use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use reqwest::{
    Client as ReqwestClient,
    header,
};
use serde::{
    Deserialize, 
    Serialize,
};
use serde_json::{
    Value,
    json,
};
use std::{
    collections::HashMap,
    convert::Infallible,
    fmt::Display,
    str::FromStr,
};

#[derive(Debug, Deserialize)]
struct Response <T> {
    pub error: Vec<String>,
    pub result: Option <T>,
}

#[derive(Debug)]
struct OpenOrdersPostPayload {
    pub nonce: u32,
    pub trades: bool,
    pub userref: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerTime{
     unixtime: u32,
     rfc1123: String,
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
    url: String,
    api_key: Option<String>,
    api_sign: Option<String>,
    status: Option<u64>,
    response_server_time: Option<ServerTime>,
    response_asset_data: Option<GetAssetPairInfo>,
    response_open_orders: Option<OpenOrderInfo>, 
}

impl MyClient{
    fn new() -> Self{ Self {
        url: BASE_URL.to_string(),
        api_key: None,
        api_sign: None,
        status: None,
        response_server_time: None,
        response_asset_data: None,
        response_open_orders: None,
        }
    }

    fn make_public_api_call_to_server_time(&mut self)-> Response<ServerTime> {
        self.status = Some(200u64);
        serde_json::from_str(DUMMY_SERVER_TIME).unwrap()
    }
    
    fn make_public_api_call_for_asset_data(&mut self) -> Response<GetAssetPairInfo> {
        self.status = Some(200u64);
        serde_json::from_str(DUMMY_ASSET_DATA).unwrap()
    }

    fn make_private_api_call_for_open_orders(&mut self, api_key: Option<String>, api_sign: Option<String>) -> Response<OpenOrderInfo>{
        self.status = Some(200u64);
        if let Some(api_key) = api_key {
            if let Some(api_sign) = api_sign {
                match (api_key, api_sign){
                    ("DummyKey54321", "DummySign54321") => serde_json::from_str(DUMMY_DATA_OPEN_ORDERS),
                    (a,b) if (a.as_str() == "DummyKey54321"&& b.as_str() != "DummySign54321") => serde_json::from_str(INVALID_SIGN_ERROR),
                    (a,b) if (a.as_str() != "DummyKey54321" && b.as_str() == "DummySign54321") => serde_json::from_str(INVALID_KEY_ERROR),
                    (_, _) => serde_json::from_str(INVALID_KEY_ERROR),
                }
            } else {
                serde_json::from_str(INVALID_SIGN_ERROR)
            }
        } else {
            serde_json::from_str(INVALID_KEY_ERROR)
        }
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
    assert_eq!(world.client.status.as_ref().unwrap(), &200u64)
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
    assert_eq!(world.client.status.as_ref().unwrap(), &200u64)
}

#[then("the user retrieves the asset data successfully")]
async fn retrieve_asset_data(world: &mut MyWorld) {
    assert_eq!(
        world.client.response_asset_data.as_ref().unwrap().get("XXBTZUSD").unwrap().altname, "XBTUSD")
}

#[given(r"^an (authorized | unauthorized) http POST request to the private service$")]
fn authorized_private_api(world: &mut MyWorld, state: State){
    match state {
        Authorized => {
                        world.api_key = Some("DummyKey54321");
                        world.api_sign = Some("DummySign54321");
                       },
        Unauthorzed => {
                        world.api_key = None;
                        world.api_sign = None;
                        }
    };
}


#[when("the user wants to retrieve open orders")]
async fn retrieve_open_orders(world: &mut MyWorld){
    world.client.response_open_orders = world.client.make_private_api_call_for_open_orders(&world.api_key, &world.api_sign);
}

#[then("the status code is OK")]
fn check_open_orders_status_code(world:&mut MyWorld){
    assert_eq!(world.client.status.as_ref().unwrap(), 200u64)
}

#[then("the user retrieves the open orders successfully")]
fn retrieve_open_orders_successfully(world:&mut World){
    
}

#[tokio::main]
async fn main() {

     MyWorld::run("tests/features/public_api_tests.feature").await;
}

const BASE_URL: & str = "https://api.kraken.com";

const DUMMY_SERVER_TIME: &str = r#"
        {
            "error": [],
            "result": {
            "unixtime": 1616334,
            "rfc1123": "Sun, 21 Mar 21 14:23:14 +0000"
            }
        }"#;

const DUMMY_ASSET_DATA: &str = r#"
        {
            "error": [],
            "result": {
                "XXBTZUSD": {
                    "altname": "XBTUSD",
                    "wsname": "XBT/USD",
                    "aclass_base": "currency",
                    "base": "XXBT",
                    "aclass_quote": "currency",
                    "quote": "ZUSD",
                    "lot": "unit",
                    "pair_decimals": 1,
                    "lot_decimals": 8,
                    "lot_multiplier": 1,
                    "leverage_buy": [
                        2,
                        3,
                        4,
                        5
                    ],
                    "leverage_sell": [
                        2,
                        3,
                        4,
                        5
                    ],
                    "fees": [
                        [
                            0,
                            0.26
                        ],
                        [
                            50000,
                            0.24
                        ],
                        [
                            100000,
                            0.22
                        ],
                        [
                            250000,
                            0.2
                        ],
                        [
                            500000,
                            0.18
                        ],
                        [
                            1000000,
                            0.16
                        ],
                        [
                            2500000,
                            0.14
                        ],
                        [
                            5000000,
                            0.12
                        ],
                        [
                            10000000,
                            0.1
                        ]
                    ],
                    "fees_maker": [
                        [
                            0,
                            0.16
                        ],
                        [
                            50000,
                            0.14
                        ],
                        [
                            100000,
                            0.12
                        ],
                        [
                            250000,
                            0.1
                        ],
                        [
                            500000,
                            0.08
                        ],
                        [
                            1000000,
                            0.06
                        ],
                        [
                            2500000,
                            0.04
                        ],
                        [
                            5000000,
                            0.02
                        ],
                        [
                            10000000,
                            0.0
                        ]
                    ],
                    "fee_volume_currency": "ZUSD",
                    "margin_call": 80,
                    "margin_stop": 40,
                    "ordermin": "0.0001"
                }
            }
        }"#;

const DUMMY_ASSET_DATA_W_QUERY: &str = r#"
{
    "error": [],
    "result": {
        "XETHXXBT": {
            "fees": [
                [
                    0,
                    0.26
                ],
                [
                    50000,
                    0.24
                ],
                [
                    100000,
                    0.22
                ],
                [
                    250000,
                    0.2
                ],
                [
                    500000,
                    0.18
                ],
                [
                    1000000,
                    0.16
                ],
                [
                    2500000,
                    0.14
                ],
                [
                    5000000,
                    0.12
                ],
                [
                    10000000,
                    0.1
                ]
            ],
            "fees_maker": [
                [
                    0,
                    0.16
                ],
                [
                    50000,
                    0.14
                ],
                [
                    100000,
                    0.12
                ],
                [
                    250000,
                    0.1
                ],
                [
                    500000,
                    0.08
                ],
                [
                    1000000,
                    0.06
                ],
                [
                    2500000,
                    0.04
                ],
                [
                    5000000,
                    0.02
                ],
                [
                    10000000,
                    0.0
                ]
            ],
            "fee_volume_currency": "ZUSD"
        },
        "XXBTZUSD": {
            "fees": [
                [
                    0,
                    0.26
                ],
                [
                    50000,
                    0.24
                ],
                [
                    100000,
                    0.22
                ],
                [
                    250000,
                    0.2
                ],
                [
                    500000,
                    0.18
                ],
                [
                    1000000,
                    0.16
                ],
                [
                    2500000,
                    0.14
                ],
                [
                    5000000,
                    0.12
                ],
                [
                    10000000,
                    0.1
                ]
            ],
            "fees_maker": [
                [
                    0,
                    0.16
                ],
                [
                    50000,
                    0.14
                ],
                [
                    100000,
                    0.12
                ],
                [
                    250000,
                    0.1
                ],
                [
                    500000,
                    0.08
                ],
                [
                    1000000,
                    0.06
                ],
                [
                    2500000,
                    0.04
                ],
                [
                    5000000,
                    0.02
                ],
                [
                    10000000,
                    0.0
                ]
            ],
            "fee_volume_currency": "ZUSD"
        }
    }
}"#;

const DUMMY_DATA_OPEN_ORDERS: &str = r#"
        {
            "error": [ ],
            "result": {
            "open": {
            "OQCLML-BW3P3-BUCMWZ": {
            "refid": null,
            "userref": 0,
            "status": "open",
            "opentm": 1616666559.8974,
            "starttm": 0,
            "expiretm": 0,
            "descr": {},
            "vol": "1.25000000",
            "vol_exec": "0.37500000",
            "cost": "11253.7",
            "fee": "0.00000",
            "price": "30010.0",
            "stopprice": "0.00000",
            "limitprice": "0.00000",
            "misc": "",
            "oflags": "fciq",
            "trades": []
            },
            "OB5VMB-B4U2U-DK2WRW": {
            "refid": null,
            "userref": 120,
            "status": "open",
            "opentm": 1616665899.5699,
            "starttm": 0,
            "expiretm": 0,
            "descr": {
            "pair": "XBTUSD",
            "type": "buy",
            "ordertype": "limit",
            "price": "14500.0",
            "price2": "0",
            "leverage": "5:1",
            "order": "buy 0.27500000 XBTUSD @ limit 14500.0 with 5:1 leverage",
            "close": ""
            },
            "vol": "0.27500000",
            "vol_exec": "0.00000000",
            "cost": "0.00000",
            "fee": "0.00000",
            "price": "0.00000",
            "stopprice": "0.00000",
            "limitprice": "0.00000",
            "misc": "",
            "oflags": "fciq"
            },
            "OXHXGL-F5ICS-6DIC67": {
            "refid": null,
            "userref": 120,
            "status": "open",
            "opentm": 1616665894.0036,
            "starttm": 0,
            "expiretm": 0,
            "descr": {},
            "vol": "0.27500000",
            "vol_exec": "0.00000000",
            "cost": "0.00000",
            "fee": "0.00000",
            "price": "0.00000",
            "stopprice": "0.00000",
            "limitprice": "0.00000",
            "misc": "",
            "oflags": "fciq"
            },
            "OLQCVY-B27XU-MBPCL5": {
            "refid": null,
            "userref": 251,
            "status": "open",
            "opentm": 1616665556.7646,
            "starttm": 0,
            "expiretm": 0,
            "descr": {},
            "vol": "0.27500000",
            "vol_exec": "0.00000000",
            "cost": "0.00000",
            "fee": "0.00000",
            "price": "0.00000",
            "stopprice": "0.00000",
            "limitprice": "0.00000",
            "misc": "",
            "oflags": "fciq"
            },
            "OQCGAF-YRMIQ-AMJTNJ": {
            "refid": null,
            "userref": 0,
            "status": "open",
            "opentm": 1616665511.0373,
            "starttm": 0,
            "expiretm": 0,
            "descr": {},
            "vol": "1.25000000",
            "vol_exec": "0.00000000",
            "cost": "0.00000",
            "fee": "0.00000",
            "price": "0.00000",
            "stopprice": "0.00000",
            "limitprice": "0.00000",
            "misc": "",
            "oflags": "fciq",
            "trigger": "index"
            }
            }
            }
}"#;

const INVALID_KEY_ERROR: &str = r#"
    {
        "error": [EAPI:Invalid key],
        "result": {}
    "#;

const INVALID_SIGN_ERROR: &str = r#"
    {
    "error": [EAPI:Invalid signature],
    "result": {}
    "#;

const INVALID_NONCE_ERROR: &str = r#"
    {
        "error": [EAPI:Invalid signature],
        "result": {}
    "#;