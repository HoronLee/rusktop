use crate::builder::ConfigBuilder;

pub struct Config;

impl Config {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self
    }
}
