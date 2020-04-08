use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "VARIANCE_THRESHOLD", default = "0.03")]
    pub variance_threshold: f64,

    #[envconfig(from = "DIVERSITY_THRESHOLD", default = "0.03")]
    pub diversity_threshold: f64,

    #[envconfig(from = "BAND_THRESHOLD", default = "0.3")]
    pub band_threshold: f64,

    #[envconfig(from = "INITIAL_DEPTH", default = "4")]
    pub initial_depth: u32,

    #[envconfig(from = "MAX_DEPTH", default = "8")]
    pub max_depth: u32,
}
