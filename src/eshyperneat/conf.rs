use bson;
use envconfig::Envconfig;
use lazy_static::lazy_static;
use network::activation::Activation;
use serde::Serialize;

#[derive(Envconfig, Serialize, Clone)]
pub struct MethodConfig {
    #[envconfig(from = "INPUT_CONFIG", default = "line")]
    pub input_config: String,

    #[envconfig(from = "OUTPUT_CONFIG", default = "line")]
    pub output_config: String,

    #[envconfig(from = "VARIANCE_THRESHOLD", default = "0.2")]
    pub variance_threshold: f64,

    #[envconfig(from = "DIVISION_THRESHOLD", default = "0.2")]
    pub division_threshold: f64,

    #[envconfig(from = "BAND_THRESHOLD", default = "0.3")]
    pub band_threshold: f64,

    #[envconfig(from = "INITIAL_RESOLUTION", default = "4")]
    #[serde(with = "bson::compat::u2f")]
    pub initial_resolution: u64,

    #[envconfig(from = "MAX_RESOLUTION", default = "5")]
    #[serde(with = "bson::compat::u2f")]
    pub max_resolution: u64,

    #[envconfig(from = "ITERATION_LEVEL", default = "3")]
    #[serde(with = "bson::compat::u2f")]
    pub iteration_level: u64,

    #[envconfig(from = "RESOLUTION", default = "1048576.0")]
    pub resolution: f64,

    #[envconfig(from = "MAX_DISCOVERIES", default = "0")]
    #[serde(with = "bson::compat::u2f")]
    pub max_discoveries: u64,

    #[envconfig(from = "MAX_OUTGOING", default = "0")]
    #[serde(with = "bson::compat::u2f")]
    pub max_outgoing: u64,

    #[envconfig(from = "HIDDEN_ACTIVATION", default = "None")]
    pub hidden_activation: Activation,

    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Softmax")]
    pub output_activation: Activation,

    #[envconfig(from = "MAX_VARIANCE", default = "false")]
    pub max_variance: bool,

    #[envconfig(from = "RELATIVE_VARIANCE", default = "true")]
    pub relative_variance: bool,

    #[envconfig(from = "MEDIAN_VARIANCE", default = "false")]
    pub median_variance: bool,

    #[envconfig(from = "ONLY_LEAF_VARIANCE", default = "false")]
    pub only_leaf_variance: bool,

    #[envconfig(from = "LOG_VISUALIZATIONS", default = "false")]
    pub log_visualizations: bool,
}

impl Default for MethodConfig {
    fn default() -> Self {
        MethodConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref ESHYPERNEAT: MethodConfig = MethodConfig::default();
}
