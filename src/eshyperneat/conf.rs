use envconfig::Envconfig;
use lazy_static::lazy_static;
use network::activation::Activation;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "VARIANCE_THRESHOLD", default = "0.2")]
    pub variance_threshold: f64,

    #[envconfig(from = "DIVISION_THRESHOLD", default = "0.2")]
    pub division_threshold: f64,

    #[envconfig(from = "BAND_THRESHOLD", default = "0.3")]
    pub band_threshold: f64,

    #[envconfig(from = "INITIAL_RESOLUTION", default = "4")]
    pub initial_resolution: usize,

    #[envconfig(from = "MAX_RESOLUTION", default = "5")]
    pub max_resolution: usize,

    #[envconfig(from = "ITERATION_LEVEL", default = "3")]
    pub iteration_level: usize,

    #[envconfig(from = "RESOLUTION", default = "1048576.0")]
    pub resolution: f64,

    #[envconfig(from = "MAX_DISCOVERIES", default = "512")]
    pub max_discoveries: usize,

    #[envconfig(from = "MAX_OUTGOING", default = "16")]
    pub max_outgoing: usize,

    #[envconfig(from = "HIDDEN_ACTIVATION", default = "None")]
    pub hidden_activation: Activation,

    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Softmax")]
    pub output_activation: Activation,

    #[envconfig(from = "MAX_VARIANCE", default = "true")]
    pub max_variance: bool,

    #[envconfig(from = "RELATIVE_VARIANCE", default = "true")]
    pub relative_variance: bool,

    #[envconfig(from = "MEDIAN_VARIANCE", default = "true")]
    pub median_variance: bool,

    #[envconfig(from = "ONLY_LEAF_VARIANCE", default = "false")]
    pub only_leaf_variance: bool,
}

lazy_static! {
    pub static ref ESHYPERNEAT: Conf = Conf::init().unwrap();
}
