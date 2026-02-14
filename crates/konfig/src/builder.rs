use crate::error::Result;
use crate::merge::merge_deep;
use crate::source::{ConfigSource, EnvSource, FileSource};
use crate::value::ConfigMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::path::Path;

pub struct ConfigBuilder {
    sources: Vec<Box<dyn ConfigSource>>,
    env_prefix: Option<String>,
    env_separator: String,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            env_prefix: None,
            env_separator: "_".to_string(),
        }
    }

    pub fn env_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.env_prefix = Some(prefix.into());
        self
    }

    pub fn env_separator(mut self, separator: impl Into<String>) -> Self {
        self.env_separator = separator.into();
        self
    }

    pub fn file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.sources.push(Box::new(FileSource::new(path)));
        self
    }

    pub fn optional_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.sources
            .push(Box::new(FileSource::new(path).optional()));
        self
    }

    pub fn add_source(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.sources.push(source);
        self
    }

    pub fn load<T>(self) -> Result<T>
    where
        T: DeserializeOwned + Default + Serialize,
    {
        let mut merged_data = ConfigMap::new();

        let default_value = serde_json::to_value(T::default())?;
        if let Value::Object(map) = default_value {
            for (k, v) in map {
                merged_data.insert(k, v);
            }
        }

        for source in &self.sources {
            let data = source.load()?;
            merge_deep(&mut merged_data, data);
        }

        if self.env_prefix.is_some() || !self.env_separator.is_empty() {
            let env_source =
                EnvSource::new(self.env_prefix.clone()).with_separator(self.env_separator.clone());
            let env_data = env_source.load()?;
            merge_deep(&mut merged_data, env_data);
        }

        let value = Value::Object(merged_data.into_iter().collect());
        serde_json::from_value(value).map_err(Into::into)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
