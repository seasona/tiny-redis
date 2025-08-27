mod ping;
pub use ping::Ping;

mod set;
pub use set::Set;

mod get;
pub use get::Get;

use crate::{Connection, Db, Frame, Parse};

#[derive(Debug)]
pub enum Command {
    Ping(Ping),
    Set(Set),
    Get(Get),
    Unknown(String),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;

        let command_name = parse.next_string()?.to_lowercase();

        let command = match command_name.as_str() {
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            _ => unimplemented!(),
        };

        parse.finish()?;

        Ok(command)
    }

    pub(crate) async fn apply(self, db: &mut Db, dst: &mut Connection) -> crate::Result<()> {
        match self {
            Command::Ping(cmd) => cmd.apply(dst).await,
            Command::Set(cmd) => cmd.apply(&db, dst).await,
            Command::Get(cmd) => cmd.apply(&db, dst).await,
            _ => unimplemented!(),
        }
    }

    /// Returns the command name
    pub(crate) fn get_name(&self) -> &str {
        match self {
            Command::Ping(_) => "ping",
            Command::Set(_) => "set",
            Command::Get(_) => "get",
            _ => unimplemented!(),
        }
    }
}
