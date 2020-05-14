use envconfig::Envconfig;
use serde::Serialize;

#[derive(Envconfig, Clone, Serialize)]
pub struct NeatConfig {
    #[envconfig(from = "ADD_NODE_PROBABILITY", default = "0.05")]
    pub add_node_probability: f64,

    #[envconfig(from = "ADD_CONNECTION_PROBABILITY", default = "0.08")]
    pub add_connection_probability: f64,

    #[envconfig(from = "INITIAL_LINK_WEIGHT_SIZE", default = "0.5")]
    pub initial_link_weight_size: f64,

    #[envconfig(from = "MUTATE_LINK_WEIGHT_PROBABILITY", default = "0.8")]
    pub mutate_link_weight_probability: f64,

    #[envconfig(from = "MUTATE_LINK_WEIGHT_SIZE", default = "0.5")]
    pub mutate_link_weight_size: f64,

    #[envconfig(from = "DISABLE_CONNECTION_PROBABILITY", default = "0.05")]
    pub disable_connection_probability: f64,

    #[envconfig(from = "ONLY_HIDDEN_NODE_DISTANCE", default = "true")]
    pub only_hidden_node_distance: bool,

    #[envconfig(from = "LINK_DISTANCE_WEIGHT", default = "0.5")]
    pub link_distance_weight: f64,
}

impl Default for NeatConfig {
    fn default() -> Self {
        Self::init().unwrap()
    }
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
