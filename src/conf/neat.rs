use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET_FILENAME", default = "datasets/xor")]
    pub dataset_filename: String,

    #[envconfig(from = "POPULATION_SIZE", default = "100")]
    pub population_size: u64,

    #[envconfig(from = "ITERATIONS", default = "10000")]
    pub iterations: u64,

    #[envconfig(from = "SPECIATION_THRESHOLD", default = "0.9")]
    pub speciation_threshold: f64,

    #[envconfig(from = "INTERSPECIES_REPRODUCTION_CHANCE", default = "0.15")]
    pub interspecies_reproduction_chance: f64,

    #[envconfig(from = "ADD_NODE_PROBABILITY", default = "0.03")]
    pub add_node_probability: f64,

    #[envconfig(from = "ADD_CONNECTION_PROBABILITY", default = "0.05")]
    pub add_connection_probability: f64,

    #[envconfig(from = "DISABLE_CONNECTION_PROBABILITY", default = "0.2")]
    pub disable_connection_probability: f64,

    #[envconfig(from = "MUTATE_LINK_WEIGHT_PROBABILITY", default = "0.8")]
    pub mutate_link_weight_probability: f64,

    #[envconfig(from = "MUTATE_LINK_WEIGHT_SIZE", default = "0.2")]
    pub mutate_link_weight_size: f64,

    #[envconfig(from = "MUTATE_HIDDEN_BIAS_PROBABILIT", default = "0.8")]
    pub mutate_hidden_bias_probability: f64,

    #[envconfig(from = "MUTATE_HIDDEN_BIAS_SIZE", default = "0.2")]
    pub mutate_hidden_bias_size: f64,

    #[envconfig(from = "MUTATE_OUTPUT_BIAS_PROBABILIT", default = "0.8")]
    pub mutate_output_bias_probability: f64,

    #[envconfig(from = "MUTATE_OUTPUT_BIAS_SIZE", default = "0.2")]
    pub mutate_output_bias_size: f64,

    #[envconfig(from = "DROPOFF_AGE", default = "20")]
    pub dropoff_age: u64,

    #[envconfig(from = "YOUNG_SPECIES_FITNESS_MULTIPLIER", default = "1.05")]
    pub young_species_fitness_multiplier: f64,

    #[envconfig(from = "STAGNENT_SPECIES_FITNESS_MULTIPLIER", default = "0.01")]
    pub stagnent_species_fitness_multiplier: f64,

    #[envconfig(from = "SURVIVAL_RATO", default = "0.4")]
    pub survival_ratio: f64,

    #[envconfig(from = "ELITISM", default = "1")]
    pub elitism: u64,

    #[envconfig(from = "NORMALIZE_OUTPUT", default = "true")]
    pub normalize_output: bool,
    
}
