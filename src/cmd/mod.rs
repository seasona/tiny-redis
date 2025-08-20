
mod ping;
pub use ping::Ping;

use crate::{frame::Frame, parse::Parse, connection::Connection};

#[derive(Debug)]
pub enum Command {
    Ping(Ping),
    Unknown(String),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;

        let command_name = parse.next_string()?.to_lowercase();

        let command = match command_name.as_str() {
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            _ => unimplemented!(),
        };

        parse.finish()?;

        Ok(command)
    }

    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {

        match self {
            Command::Ping(cmd) => cmd.apply(dst).await,
            _ => unimplemented!(),
        }
    }

    /// Returns the command name
    pub(crate) fn get_name(&self) -> &str {
        match self {
            Command::Ping(_) => "ping",
            _ => unimplemented!(),
        }
    }
}