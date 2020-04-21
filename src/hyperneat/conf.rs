use envconfig::Envconfig;
use lazy_static::lazy_static;
use network::activation::Activation;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "WEIGHT_THRESHOLD", default = "0.1")]
    pub weight_threshold: f64,

    #[envconfig(from = "HIDDEN_ACTIVATION", default = "ReLU")]
    pub hidden_activation: Activation,

    #[envconfig(from = "OUTPUT_ACTIVATION", default = "Softmax")]
    pub output_activation: Activation,
}

lazy_static! {
    pub static ref HYPERNEAT: Conf = Conf::init().unwrap();
}
