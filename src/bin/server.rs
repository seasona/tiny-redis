use tklog::{trace, debug, error, fatal, info, warn};
use clap::Parser;
use tokio::net::TcpListener;

use tiny_redis::{server, DEFUALT_PORT};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    port: Option<u16>,
}

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFUALT_PORT);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    server::run(listener).await;
}
