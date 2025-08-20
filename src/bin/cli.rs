use clap::{Parser, Subcommand};
use bytes::Bytes;
use tklog::info;

use tiny_redis::{client::Client, DEFUALT_PORT};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[arg(id = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value_t = DEFUALT_PORT)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping { msg: Option<Bytes> },
    Get { key: String },
    Set { key: String, value: String },
}

#[tokio::main]
async fn main() -> tiny_redis::Result<()> {
    let cli = Cli::parse();

    let addr = format!("{}:{}", cli.host, cli.port);

    let mut client = Client::connect(addr).await.unwrap();

    match cli.command {
        Command::Ping { msg }=> {
            let value = client.ping(msg).await?;
            let string = str::from_utf8(&value)?;
            info!("return: {}", string);
        },
        _ => unimplemented!(),
    }

    Ok(())
}