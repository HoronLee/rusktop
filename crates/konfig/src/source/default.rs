use crate::error::Result;
use crate::source::traits::ConfigSource;
use crate::value::ConfigMap;

pub struct DefaultSource {
    values: ConfigMap,
}

impl DefaultSource {
    pub fn new(values: ConfigMap) -> Self {
        Self { values }
    }

    pub fn empty() -> Self {
        Self {
            values: ConfigMap::new(),
        }
    }
}

impl ConfigSource for DefaultSource {
    fn load(&self) -> Result<ConfigMap> {
        Ok(self.values.clone())
    }

    fn name(&self) -> &str {
        "default"
    }
}
