use tklog::{
    LEVEL, LOG, Format,
};
use clap::Parser;
use tokio::net::TcpListener;

use tiny_redis::{server, DEFUALT_PORT};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    port: Option<u16>,
}

fn log_init() {
    LOG.set_console(true)  
       .set_level(LEVEL::Debug)  
       .set_format(Format::LevelFlag | Format::Microseconds | Format::ShortFileName)  
       .set_formatter("{level} {time} {file}: {message}\n");
}

#[tokio::main]
pub async fn main() {
    log_init();

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFUALT_PORT);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    server::run(listener).await;
}
