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
}

impl Default for MethodConfig {
    fn default() -> Self {
        MethodConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref HYPERNEAT: MethodConfig = MethodConfig::init().unwrap();
}
