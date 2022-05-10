use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub mod get_asset_pair_info;
pub use get_asset_pair_info::*;

pub mod get_server_time;
pub use get_server_time::*;

pub mod get_open_orders;
pub use get_open_orders::*;