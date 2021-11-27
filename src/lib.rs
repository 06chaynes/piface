//! This library will attempt to determine the local machine's default route to the internet and pull related info
mod error;
pub mod gateway;
pub mod interface;

pub use self::error::Error;
pub use self::gateway::DefaultRoute;
pub use self::interface::PrimaryInterface;
pub type Result<T> = std::result::Result<T, Error>;
