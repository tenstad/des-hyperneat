use envconfig::Envconfig;
use evolution::neat::conf::{ConfigProvider, NeatConfig};
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "SINGLE_CPPN_STATE", default = "false")]
    pub single_cppn_state: bool,
}

lazy_static! {
    pub static ref DESHYPERNEAT: Conf = Conf::init().unwrap();
}

#[derive(Default, Clone)]
pub struct Config {
    pub cppn: NeatConfig,
    pub topology: NeatConfig,
}

impl ConfigProvider<NeatConfig, NeatConfig> for Config {
    fn get_core(&self) -> &NeatConfig {
        &self.topology
    }
    fn get_node_config(&self) -> &NeatConfig {
        &self.cppn
    }
    fn get_link_config(&self) -> &NeatConfig {
        &self.cppn
    }
}
