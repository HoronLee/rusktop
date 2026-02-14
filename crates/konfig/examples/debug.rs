use konfig::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
struct SimpleConfig {
    port: u16,
}

fn main() -> konfig::Result<()> {
    println!("Environment variables:");
    for (key, value) in std::env::vars() {
        if key.starts_with("TEST_") {
            println!("  {} = {}", key, value);
        }
    }

    let config: SimpleConfig = Config::new().env_prefix("TEST").load()?;

    println!("\nLoaded config:");
    println!("  Port: {}", config.port);

    Ok(())
}
