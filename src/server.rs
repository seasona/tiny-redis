use log::{debug, error, info};
use tokio::net::TcpListener;

use crate::{Command, Connection, Db, DbDropGuard};

/// Server listener
#[derive(Debug)]
struct Listener {
    listener: TcpListener,
    db_holder: DbDropGuard,
}

/// Per-connection handler
#[derive(Debug)]
struct Handler {
    connection: Connection,
    db: Db,
}

impl Listener {
    async fn run(&mut self) -> crate::Result<()> {
        info!("accepting inbound connections");

        loop {
            let socket = self.listener.accept().await?;

            let mut handler = Handler {
                connection: Connection::new(socket.0),
                // get a clone of shared db
                db: self.db_holder.db(),
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

            command.apply(&mut self.db, &mut self.connection).await?;
        }
    }
}

/// Run the tiny-redis server
pub async fn run(listener: TcpListener) {
    let mut server = Listener {
        listener,
        db_holder: DbDropGuard::new(),
    };

    let _ = server.run().await;
}
