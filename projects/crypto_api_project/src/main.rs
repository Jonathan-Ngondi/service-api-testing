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

pub const API_KEY: &str = "D4wq8jvaEWFB5UV/EwpYC/uxGHEbasopeK+s2FHFYs4vb5bvuFmir7x5";

pub const PRIVATE_KEY: &str = "LX8846Ho0Wa0OZ/lMH7uxzoadhrrOrJqNCLPv3KnquywGXfhyaqmzhQqt10BPLxeSMY16Ty2i3Jxo3ZrZhIVpQ==";

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
