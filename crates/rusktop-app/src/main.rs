use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            log: LogConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:./rusktop.db?mode=rwc".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct LogConfig {
    pub level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
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
    use gpui::*;
    use rusktop_ui::CounterView;

    let app = Application::new();

    app.run(|cx: &mut App| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(400.0), px(300.0)),
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some("Rusktop".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| cx.new(|_| CounterView::new()))
            .expect("Failed to open window");
    });
}
