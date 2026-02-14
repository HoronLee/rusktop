pub mod toml;

#[cfg(feature = "yaml")]
pub mod yaml;

pub use self::toml::parse_toml;

#[cfg(feature = "yaml")]
pub use yaml::parse_yaml;
