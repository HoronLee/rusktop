use crate::error::{ConfigError, Result};
use crate::format;
use crate::source::traits::ConfigSource;
use crate::value::ConfigMap;
use serde_json::Value;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
    Json,
}

impl FileFormat {
    pub fn from_path(path: &Path) -> Result<Self> {
        let extension = path.extension().and_then(|s| s.to_str()).ok_or_else(|| {
            ConfigError::ParseError(format!(
                "Cannot determine file format from path: {}",
                path.display()
            ))
        })?;

        match extension.to_lowercase().as_str() {
            "toml" => Ok(FileFormat::Toml),
            #[cfg(feature = "yaml")]
            "yaml" | "yml" => Ok(FileFormat::Yaml),
            "json" => Ok(FileFormat::Json),
            _ => Err(ConfigError::ParseError(format!(
                "Unsupported file format: {}",
                extension
            ))),
        }
    }

    pub fn parse(&self, content: &str) -> Result<Value> {
        match self {
            FileFormat::Toml => format::parse_toml(content),
            #[cfg(feature = "yaml")]
            FileFormat::Yaml => format::parse_yaml(content),
            FileFormat::Json => serde_json::from_str(content).map_err(ConfigError::SerdeJsonError),
        }
    }
}

pub struct FileSource {
    path: PathBuf,
    format: FileFormat,
    required: bool,
}

impl FileSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        let format = FileFormat::from_path(&path_buf).unwrap_or(FileFormat::Toml);

        Self {
            path: path_buf,
            format,
            required: true,
        }
    }

    pub fn with_format<P: AsRef<Path>>(path: P, format: FileFormat) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            format,
            required: true,
        }
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl ConfigSource for FileSource {
    fn load(&self) -> Result<ConfigMap> {
        let content = match std::fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(e) => {
                if !self.required && e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(ConfigMap::new());
                }
                return Err(ConfigError::FileRead(e));
            }
        };

        let value = self.format.parse(&content)?;

        if let Value::Object(map) = value {
            let config_map: ConfigMap = map.into_iter().collect();
            Ok(config_map)
        } else {
            Err(ConfigError::ParseError(
                "Config file root must be an object".to_string(),
            ))
        }
    }

    fn name(&self) -> &str {
        self.path.to_str().unwrap_or("file")
    }
}
