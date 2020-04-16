use network::activation;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "WEIGHT_THRESHOLD", default = "0.1")]
    pub weight_threshold: f64,

    #[envconfig(from = "HIDDEN_ACTIVATION", default = "ReLU")]
    pub hidden_activation: activation::Activation,

    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Softmax")]
    pub output_activation: activation::Activation,
}
