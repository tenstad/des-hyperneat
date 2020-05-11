use envconfig::Envconfig;
use evolution::neat::conf::{ConfigProvider, NeatConfig};
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "TOPOLOGY_MUTATION_PROBABILITY", default = "0.2")]
    pub topology_mutation_probability: f64,

    #[envconfig(from = "CPPN_MUTATION_PROBABILITY", default = "0.8")]
    pub cppn_mutation_probability: f64,
}

lazy_static! {
    pub static ref SIDESHYPERNEAT: Conf = Conf::init().unwrap();
}

#[derive(Default, Clone)]
pub struct Config {
    pub cppn: NeatConfig,
    pub topology: NeatConfig,
}

impl ConfigProvider<(), ()> for Config {
    fn get_core(&self) -> &NeatConfig {
        &self.topology
    }
    fn get_node_config(&self) -> &() {
        &()
    }
    fn get_link_config(&self) -> &() {
        &()
    }
}
