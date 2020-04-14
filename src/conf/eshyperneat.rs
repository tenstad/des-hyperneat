use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "VARIANCE_THRESHOLD", default = "0.01")]
    pub variance_threshold: f64,

    #[envconfig(from = "DIVERSITY_THRESHOLD", default = "0.01")]
    pub diversity_threshold: f64,

    #[envconfig(from = "BAND_THRESHOLD", default = "0.6")]
    pub band_threshold: f64,

    #[envconfig(from = "INITIAL_DEPTH", default = "4")]
    pub initial_depth: u32,

    #[envconfig(from = "MAX_DEPTH", default = "8")]
    pub max_depth: u32,

    #[envconfig(from = "RESOLUTION", default = "1048576.0")]
    pub resolution: f64,
}
