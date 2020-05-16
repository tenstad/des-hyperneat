use envconfig::Envconfig;
use evolution::neat::conf::{ConfigProvider, NeatConfig};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Envconfig, Serialize)]
pub struct MethodConfig {
    #[envconfig(from = "TOPOLOGY_MUTATION_PROBABILITY", default = "0.2")]
    pub topology_mutation_probability: f64,

    #[envconfig(from = "CPPN_MUTATION_PROBABILITY", default = "0.8")]
    pub cppn_mutation_probability: f64,
}

impl Default for MethodConfig {
    fn default() -> Self {
        MethodConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref SIDESHYPERNEAT: MethodConfig = MethodConfig::init().unwrap();
}

#[derive(Default, Clone, Serialize)]
pub struct GenomeConfig {
    pub cppn: NeatConfig,
    pub topology: NeatConfig,
}

impl ConfigProvider<(), ()> for GenomeConfig {
    fn neat(&self) -> &NeatConfig {
        &self.topology
    }
    fn neat_node(&self) -> &() {
        &()
    }
    fn neat_link(&self) -> &() {
        &()
    }
}
