use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET_FILENAME", default = "datasets/xor")]
    pub dataset_filename: String,

    #[envconfig(from = "POPULATION_SIZE", default = "100")]
    pub population_size: u64,

    #[envconfig(from = "ITERATIONS", default = "500")]
    pub iterations: u64,

    #[envconfig(from = "SPECIATION_THRESHOLD", default = "0.5")]
    pub speciation_threshold: f64,

    #[envconfig(from = "SHARING_THRESHOLD", default = "0.5")]
    pub sharing_threshold: f64,

    #[envconfig(from = "ADD_NODE_PROBABILITY", default = "0.05")]
    pub add_node_probability: f64,

    #[envconfig(from = "ADD_CONNECTION_PROBABILITY", default = "0.1")]
    pub add_connection_probability: f64,

    #[envconfig(from = "MUTATE_LINK_WEIGHT_PROBABILITY", default = "1.0")]
    pub mutate_link_weight_probability: f64,

    #[envconfig(from = "MUTATE_HIDDEN_BIAS_PROBABILIT", default = "1.0")]
    pub mutate_hidden_bias_probability: f64,

    #[envconfig(from = "MUTATE_OUTPUT_BIAS_PROBABILIT", default = "1.0")]
    pub mutate_output_bias_probability: f64,
}
