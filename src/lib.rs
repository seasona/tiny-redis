
pub mod connection;

pub mod server;

pub mod frame;

pub mod client;

pub const DEFUALT_PORT: u16 = 6379;

/* Self-define error type, use it to dynamic cast different error type */
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;