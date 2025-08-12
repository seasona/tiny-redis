
pub mod connection;

pub mod server;

pub mod frame;


/* Self-define error type, use it to dynamic cast different error type */
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;