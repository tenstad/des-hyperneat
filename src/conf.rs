use envconfig::Envconfig;
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Envconfig, Serialize)]
pub struct MainConfig {
    #[envconfig(from = "METHOD", default = "DES-HyperNEAT")]
    pub method: String,
}

impl Default for MainConfig {
    fn default() -> Self {
        MainConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref CONF: MainConfig = MainConfig::init().unwrap();
}
