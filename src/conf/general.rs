use crate::network::activation;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET", default = "datasets/iris")]
    pub dataset: String,

    #[envconfig(from = "THREADS", default = "4")]
    pub thread_count: usize,

    #[envconfig(from = "OUTPUT_ACTIVATIONS", default = "Sigmoid")]
    pub output_activations: activation::Activation,
}
