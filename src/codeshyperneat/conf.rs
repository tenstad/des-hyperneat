use bson;
use envconfig::Envconfig;
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Envconfig, Serialize, Clone)]
pub struct MethodConfig {
    #[envconfig(from = "BLUEPRINT_DEVELOPMENTS", default = "5")]
    #[serde(with = "bson::compat::u2f")]
    pub blueprint_developments: u64,
}

impl Default for MethodConfig {
    fn default() -> Self {
        MethodConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref CODESHYPERNEAT: MethodConfig = MethodConfig::default();
}
