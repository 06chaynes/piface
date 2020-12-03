mod error;
pub mod gateway;
pub mod interface;

pub use self::gateway::DefaultRoute;
pub use self::interface::PrimaryInterface;
pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;
