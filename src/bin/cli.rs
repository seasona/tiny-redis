use bytes::Bytes;
use clap::{Parser, Subcommand};
use log::info;

use tiny_redis::{DEFUALT_PORT, client::Client};

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

fn init_env_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();

    info!("env_logger initialized.");
}

#[tokio::main]
async fn main() -> tiny_redis::Result<()> {
    init_env_logger();

    let cli = Cli::parse();

    let addr = format!("{}:{}", cli.host, cli.port);

    let mut client = Client::connect(addr).await.unwrap();

    match cli.command {
        Command::Ping { msg } => {
            let value = client.ping(msg).await?;
            let string = str::from_utf8(&value)?;
            info!("return: {}", string);
        }
        _ => unimplemented!(),
    }

    Ok(())
}
