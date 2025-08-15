use tokio::net::{TcpStream, ToSocketAddrs};

use crate::connection::{self, Connection};
use crate::frame::Frame;

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
        let socket = TcpStream::connect(addr).await?;

        let connection = Connection::new(socket);

        Ok(Client { connection })
    }

    pub async fn ping(&mut self) -> crate::Result<()> {
        let frame = Frame::Simple("PING".into());

        self.connection.write_frame(&frame).await?;

        Ok(())
    }
}