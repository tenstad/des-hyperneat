use envconfig::Envconfig;
use evolution::neat::conf::{ConfigProvider, NeatConfig};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Envconfig, Serialize)]
pub struct MethodConfig {
    #[envconfig(from = "SINGLE_CPPN_STATE", default = "false")]
    pub single_cppn_state: bool,

    #[envconfig(from = "INPUT_CONFIG", default = "line")]
    pub input_config: String,

    #[envconfig(from = "OUTPUT_CONFIG", default = "line")]
    pub output_config: String,
}

impl Default for MethodConfig {
    fn default() -> Self {
        MethodConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref DESHYPERNEAT: MethodConfig = MethodConfig::default();
}

#[derive(Default, Clone, Serialize)]
pub struct GenomeConfig {
    pub cppn: NeatConfig,
    pub topology: NeatConfig,
}

impl ConfigProvider<NeatConfig, NeatConfig> for GenomeConfig {
    fn neat(&self) -> &NeatConfig {
        &self.topology
    }
    fn neat_node(&self) -> &NeatConfig {
        &self.cppn
    }
    fn neat_link(&self) -> &NeatConfig {
        &self.cppn
    }
}
