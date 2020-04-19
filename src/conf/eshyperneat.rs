use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "VARIANCE_THRESHOLD", default = "0.03")]
    pub variance_threshold: f64,

    #[envconfig(from = "DIVISION_THRESHOLD", default = "0.03")]
    pub division_threshold: f64,

    #[envconfig(from = "BAND_THRESHOLD", default = "0.3")]
    pub band_threshold: f64,

    #[envconfig(from = "INITIAL_RESOLUTION", default = "3")]
    pub initial_resolution: usize,

    #[envconfig(from = "MAX_RESOLUTION", default = "4")]
    pub max_resolution: usize,

    #[envconfig(from = "ITERATION_LEVEL", default = "2")]
    pub iteration_level: usize,

    #[envconfig(from = "RESOLUTION", default = "1048576.0")]
    pub resolution: f64,
}
