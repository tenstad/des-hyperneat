use crate::network::activation;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "HIDDEN_ACTIVATION", default = "0.05")]
    pub weight_threshold: f64,

    #[envconfig(from = "HIDDEN_ACTIVATION", default = "ReLU")]
    pub hidden_activation: activation::Activation,

    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Softmax")]
    pub output_activation: activation::Activation,
}
