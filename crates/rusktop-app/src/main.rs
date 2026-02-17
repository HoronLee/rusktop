use std::net::SocketAddr;

use clap::{Parser, Subcommand};

mod config;
use config::AppConfig;

#[derive(Parser)]
#[command(name = "rusktop")]
#[command(about = "Rusktop - Dual-mode Rust application")]
struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    Web {
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config: AppConfig = konfig::Config::new()
        .env_prefix("RUSKTOP")
        .file("config.toml")
        .load()?;

    let cli = Cli::parse();

    if let Some(Mode::Web { host, port }) = &cli.mode {
        if let Some(h) = host {
            config.server.host = h.clone();
        }
        if let Some(p) = port {
            config.server.port = *p;
        }
    }

    let log_level = match config.log.level.as_str() {
        "debug" => lug::Level::Debug,
        "warn" => lug::Level::Warn,
        "error" => lug::Level::Error,
        "trace" => lug::Level::Trace,
        _ => lug::Level::Info,
    };

    lug::init(lug::LugConfig {
        env: lug::Environment::Dev,
        level: log_level,
        file: None,
    })?;

    match cli.mode {
        None => {
            #[cfg(feature = "ui")]
            {
                tracing::info!("Starting UI mode");
                start_ui();
            }
            #[cfg(not(feature = "ui"))]
            {
                eprintln!("UI mode not available (compiled without 'ui' feature)");
                eprintln!("Run with 'web' subcommand instead: rusktop-app web");
                std::process::exit(1);
            }
        }
        Some(Mode::Web { .. }) => {
            tracing::info!("Starting Web mode");
            let addr: SocketAddr =
                format!("{}:{}", config.server.host, config.server.port).parse()?;
            rusktop_web::run(&config.database.url, addr).await?;
        }
    }

    Ok(())
}

#[cfg(feature = "ui")]
fn start_ui() {
    rusktop_ui::run();
}
