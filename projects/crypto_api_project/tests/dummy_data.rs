pub const BASE_URL: &str = "https://api.kraken.com";

pub const DUMMY_SERVER_TIME: &str = r#"
        {
            "error": [],
            "result": {
            "unixtime": 1616334,
            "rfc1123": "Sun, 21 Mar 21 14:23:14 +0000"
            }
        }"#;

pub const DUMMY_ASSET_DATA: &str = r#"
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

pub const DUMMY_DATA_OPEN_ORDERS: &str = r#"
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
pub const DUMMY_ASSET_DATA_W_QUERY: &str = r#"
{
    "error": [],
    "result": {
        "XETHXXBT": {
            "altname": "ETHXBT",
            "wsname": "ETH/XBT",
            "aclass_base": "currency",
            "base": "XETH",
            "aclass_quote": "currency",
            "quote": "XXBT",
            "lot": "unit",
            "pair_decimals": 5,
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
            "ordermin": "0.001"
        },
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

pub const UNKNOWN_ASSET_PAIR_ERROR: &str = r#"
{
    "error": ["EQuery:Unknown asset pair"],
    "result: {}
}"#;

pub const INVALID_KEY_ERROR: &str = r#"
    {
        "error": ["EAPI:Invalid key"],
        "result": { "open": {}}
    }"#;

pub const INVALID_NONCE_ERROR: &str = r#"
    {
    "error": ["EAPI:Invalid nonce"],
    "result": { "open": {}}
    }"#;

pub const INVALID_SIGN_ERROR: &str = r#"
    {
    "error": ["EAPI:Invalid signature"],
    "result": { "open": {}}
    }"#;

pub const INVALID_OTP_ERROR: &str = r#"
    {
    "error": ["EGeneral:Permission denied"],
    "result": { "open": {}}
    }"#;
