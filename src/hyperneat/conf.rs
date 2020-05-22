use envconfig::Envconfig;
use lazy_static::lazy_static;
use network::activation::Activation;
use serde::Serialize;

#[derive(Envconfig, Serialize)]
pub struct MethodConfig {
    #[envconfig(from = "WEIGHT_THRESHOLD", default = "0.1")]
    pub weight_threshold: f64,

    #[envconfig(from = "HIDDEN_ACTIVATION", default = "ReLU")]
    pub hidden_activation: Activation,

    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Softmax")]
    pub output_activation: Activation,

    #[envconfig(from = "LOG_VISUALIZATIONS", default = "false")]
    pub log_visualizations: bool,

    #[envconfig(from = "INPUT_CONFIG", default = "line")]
    pub input_config: String,

    #[envconfig(from = "OUTPUT_CONFIG", default = "line")]
    pub output_config: String,

    #[envconfig(from = "HIDDEN_LAYER_SIZES", default = "[4, 4]")]
    pub hidden_layer_sizes: String,

    #[envconfig(from = "HIDDEN_LAYERS", default = "")]
    pub hidden_layers: String,
}

impl Default for MethodConfig {
    fn default() -> Self {
        MethodConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref HYPERNEAT: MethodConfig = MethodConfig::default();
}
