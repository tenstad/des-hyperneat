use envconfig::Envconfig;
use lazy_static::lazy_static;
use network::activation::Activation;
use serde::Serialize;

#[derive(Envconfig, Clone, Serialize)]
pub struct NeatConfig {
    #[envconfig(from = "ADD_NODE_PROBABILITY", default = "0.03")]
    pub add_node_probability: f64,

    #[envconfig(from = "ADD_LINK_PROBABILITY", default = "0.05")]
    pub add_link_probability: f64,

    #[envconfig(from = "INITIAL_LINK_WEIGHT_SIZE", default = "0.5")]
    pub initial_link_weight_size: f64,

    #[envconfig(from = "MUTATE_LINK_WEIGHT_PROBABILITY", default = "0.9")]
    pub mutate_link_weight_probability: f64,

    #[envconfig(from = "MUTATE_LINK_WEIGHT_SIZE", default = "0.5")]
    pub mutate_link_weight_size: f64,

    #[envconfig(from = "REMOVE_NODE_PROBABILITY", default = "0.006")]
    pub remove_node_probability: f64,

    #[envconfig(from = "REMOVE_LINK_PROBABILITY", default = "0.01")]
    pub remove_link_probability: f64,

    #[envconfig(from = "ONLY_HIDDEN_NODE_DISTANCE", default = "true")]
    pub only_hidden_node_distance: bool,

    #[envconfig(from = "LINK_DISTANCE_WEIGHT", default = "0.5")]
    pub link_distance_weight: f64,

    #[envconfig(from = "MUTATE_ONLY_ONE_LINK", default = "true")]
    pub mutate_only_one_link: bool,
}

impl Default for NeatConfig {
    fn default() -> Self {
        Self::init().unwrap()
    }
}

#[derive(Envconfig, Clone, Serialize)]
pub struct MethodConfig {
    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Sigmoid")]
    pub output_activation: Activation,
}

impl Default for MethodConfig {
    fn default() -> Self {
        Self::init().unwrap()
    }
}

lazy_static! {
    pub static ref NEAT: MethodConfig = MethodConfig::default();
}

pub trait ConfigProvider<N, L>: Clone {
    fn neat(&self) -> &NeatConfig;
    fn neat_node(&self) -> &N;
    fn neat_link(&self) -> &L;
}

impl ConfigProvider<(), ()> for NeatConfig {
    fn neat(&self) -> &NeatConfig {
        self
    }
    fn neat_node(&self) -> &() {
        &()
    }
    fn neat_link(&self) -> &() {
        &()
    }
}
