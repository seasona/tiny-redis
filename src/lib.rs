pub mod connection;
use connection::Connection;

pub mod server;

pub mod frame;
use frame::Frame;

pub mod client;

mod parse;
use parse::Parse;
use parse::ParseError;

mod cmd;
use cmd::Command;

mod db;
use db::Db;
use db::DbDropGuard;

pub const DEFUALT_PORT: u16 = 6379;

/* Self-define error type, use it to dynamic cast different error type */
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
