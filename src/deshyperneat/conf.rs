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

    #[envconfig(from = "MUTATE_NODE_DEPTH_PROBABILITY", default = "0.1")]
    pub mutate_node_depth_probability: f64,

    #[envconfig(from = "MUTATE_ALL_COMPONENTS", default = "true")]
    pub mutate_all_components: bool,

    #[envconfig(from = "LOG_VISUALIZATIONS", default = "false")]
    pub log_visualizations: bool,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "MAX_INPUT_SUBSTRATE_DEPTH", default = "0")]
    pub max_input_substrate_depth: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "MAX_OUTPUT_SUBSTRATE_DEPTH", default = "0")]
    pub max_output_substrate_depth: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "MAX_HIDDEN_SUBSTRATE_DEPTH", default = "5")]
    pub max_hidden_substrate_depth: u64,

    #[envconfig(from = "ENABLE_IDENTITY_MAPPING", default = "true")]
    pub enable_identity_mapping: bool,

    #[envconfig(from = "STATIC_SUBSTRATE_DEPH", default = "-1")]
    pub static_substrate_depth: i64,
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
