use std::net::SocketAddr;

use clap::{Parser, Subcommand};

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
    let mut config: rusktop_core::AppConfig = konfig::Config::new()
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

    lug::init(lug::LugConfig {
        env: config.log.env,
        level: config.log.level,
        file: config.log.file.clone(),
    })?;

    match cli.mode {
        None => {
            #[cfg(feature = "ui")]
            {
                tracing::info!("Starting UI mode");
                start_ui(config);
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
            rusktop_core::run(&config.database.url, addr).await?;
        }
    }

    Ok(())
}

#[cfg(feature = "ui")]
fn start_ui(config: rusktop_core::AppConfig) {
    rusktop_ui::run(config);
}
