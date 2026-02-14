use crate::error::Result;
use crate::value::ConfigMap;

pub trait ConfigSource: Send + Sync {
    fn load(&self) -> Result<ConfigMap>;

    fn name(&self) -> &str;
}
