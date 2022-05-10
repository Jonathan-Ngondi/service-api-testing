mod api_library;
use crate::api_library::GetServerTimeRequest;
// pub mod get_asset_pair_info;
// pub use get_asset_pair_info::*;

// pub mod get_server_time;
// pub use get_server_time::*;

mod client;
pub use client::*;

pub mod encryption;
pub use encryption::*;

pub mod error;
pub use error::*;

pub mod keys;
pub use keys::*;



#[tokio::main]
async fn main() {

    let client = Client::builder()
                .api_key(API_KEY)
                .api_secret(PRIVATE_KEY)
                .build();
    
    
    println!("{:#?}", client.get_server_time().send().await);
    println!("{:#?}", client.get_asset_pair_info().asset("XXBTZUSD".to_string()).send().await);
    println!("{:#?}", client.get_open_orders().send().await);

    
}
