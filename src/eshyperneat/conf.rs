use envconfig::Envconfig;
use lazy_static::lazy_static;
use network::activation::Activation;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "VARIANCE_THRESHOLD", default = "0.03")]
    pub variance_threshold: f64,

    #[envconfig(from = "DIVISION_THRESHOLD", default = "0.03")]
    pub division_threshold: f64,

    #[envconfig(from = "BAND_THRESHOLD", default = "0.3")]
    pub band_threshold: f64,

    #[envconfig(from = "INITIAL_RESOLUTION", default = "4")]
    pub initial_resolution: usize,

    #[envconfig(from = "MAX_RESOLUTION", default = "5")]
    pub max_resolution: usize,

    #[envconfig(from = "ITERATION_LEVEL", default = "2")]
    pub iteration_level: usize,

    #[envconfig(from = "RESOLUTION", default = "1048576.0")]
    pub resolution: f64,

    #[envconfig(from = "MAX_DISCOVERIES", default = "256")]
    pub max_discoveries: usize,

    #[envconfig(from = "MAX_OUTGOING", default = "12")]
    pub max_outgoing: usize,

    #[envconfig(from = "HIDDEN_ACTIVATION", default = "None")]
    pub hidden_activation: Activation,

    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Softmax")]
    pub output_activation: Activation,
}

lazy_static! {
    pub static ref ESHYPERNEAT: Conf = Conf::init().unwrap();
}
