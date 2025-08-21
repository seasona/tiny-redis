use tokio::net::TcpListener;
use log::{info, debug, error};

use crate::{cmd::Command, connection::Connection};

/// Server listener
#[derive(Debug)]
struct Listener {
    listener: TcpListener,
}

/// Per-connection handler
#[derive(Debug)]
struct Handler {
    connection: Connection,
}

impl Listener {
    async fn run(&mut self) -> crate::Result<()> {
        info!("accepting inbound connections");

        loop {
            let socket = self.listener.accept().await?;

            let mut handler = Handler {
                connection: Connection::new(socket.0),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!("connection failed: {:?}", err);
                }
            });
        }
    }
}

impl Handler {
    async fn run(&mut self) -> crate::Result<()> {
        // TODO: we need exit if the connection is closed
        loop {
            let maybe_frame = self.connection.read_frame().await?;

            // If `None` is returned, the stream is closed.
            let frame = match maybe_frame {
                Some(frame) => frame,
                None => return Ok(()),
            };

            debug!("received frame: {:?}", frame);

            let command = Command::from_frame(frame)?;

            debug!("command: {}", command.get_name());

            command.apply(&mut self.connection).await?;
        }
    }
}

/// Run the tiny-redis server
pub async fn run(listener: TcpListener) {
    let mut server = Listener {
        listener,
    };

    let _ = server.run().await;
}