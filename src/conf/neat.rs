use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET_FILENAME", default = "datasets/xor")]
    pub dataset_filename: String,

    #[envconfig(from = "POPULATION_SIZE", default = "50")]
    pub population_size: u64,

    #[envconfig(from = "ITERATIONS", default = "100")]
    pub iterations: u64,

    #[envconfig(from = "SPECIATION_THRESHOLD", default = "0.5")]
    pub speciation_threshold: f64,

    #[envconfig(from = "SHARING_THRESHOLD", default = "0.5")]
    pub sharing_threshold: f64,
}
