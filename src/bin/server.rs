use clap::Parser;
use tokio::net::TcpListener;
use log::info;

use tiny_redis::{server, DEFUALT_PORT};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    port: Option<u16>,
}

// Use beijing time (UTC+8)
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
pub async fn main() {
    init_env_logger();

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFUALT_PORT);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    server::run(listener).await;
}
