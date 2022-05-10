mod api_library;

mod two_factor_auth;
pub use two_factor_auth::*;

mod client;
pub use client::*;

pub mod encryption;
pub use encryption::*;

pub mod error;
pub use error::*;

pub mod funguo;
pub use funguo::*;

#[tokio::main]
async fn main() {
    // let key_vec = read_keys("api.keys");

    // let api_key: &str = key_vec.get(0).unwrap();
    // let private_key: &str = key_vec.get(1).unwrap();
    // let google_auth_secret: &str = key_vec.get(2).unwrap();
    let bytes = base64::decode(RANDOM_STRING).unwrap();
    let bytes_2 = base64::decode(RANDOM_STRING2).unwrap();
    let bytes_3 = base64::decode(RANDOM_STRING3).unwrap();

    let client = Client::builder()
        .api_key(
            String::from_utf8(bytes)
                .expect("Found invalid UTF-8")
                .as_str(),
        )
        .api_secret(
            String::from_utf8(bytes_2)
                .expect("Found invalid UTF-8")
                .as_str(),
        )
        .google_auth(
            String::from_utf8(bytes_3)
                .expect("Found invalid UTF-8")
                .as_str(),
        )
        .build();

    println!("{:#?}", client.get_server_time().send().await);
    println!(
        "{:#?}",
        client
            .get_asset_pair_info()
            .asset("XXBTZUSD".to_string())
            .send()
            .await
    );
    println!("{:#?}", client.get_open_orders().send().await);
}
