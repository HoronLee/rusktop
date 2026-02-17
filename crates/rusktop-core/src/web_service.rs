#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running { port: u16 },
    Stopping,
    Error(String),
}

pub struct WebServiceConfig {
    pub port: u16,
}

impl WebServiceConfig {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn default_port() -> u16 {
        8080
    }

    pub fn is_valid_port(port: u16) -> bool {
        port >= 1024
    }
}

impl Default for WebServiceConfig {
    fn default() -> Self {
        Self {
            port: Self::default_port(),
        }
    }
}
