use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
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
}

lazy_static! {
    pub static ref NEAT: Conf = Conf::init().unwrap();
}
