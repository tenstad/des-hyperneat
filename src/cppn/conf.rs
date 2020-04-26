use envconfig::Envconfig;
use lazy_static::lazy_static;
use network::activation::Activations;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "MUTATE_HIDDEN_BIAS_PROBABILITY", default = "0.8")]
    pub mutate_hidden_bias_probability: f64,

    #[envconfig(from = "MUTATE_HIDDEN_BIAS_SIZE", default = "0.1")]
    pub mutate_hidden_bias_size: f64,

    #[envconfig(from = "MUTATE_HIDDEN_ACTIVATION_PROBABILITY", default = "0.1")]
    pub mutate_hidden_activation_probability: f64,

    #[envconfig(from = "MUTATE_OUTPUT_BIAS_PROBABILITY", default = "0.8")]
    pub mutate_output_bias_probability: f64,

    #[envconfig(from = "MUTATE_OUTPUT_BIAS_SIZE", default = "0.1")]
    pub mutate_output_bias_size: f64,

    #[envconfig(from = "MUTATE_OUTPUT_ACTIVATION_PROBABILITY", default = "0.05")]
    pub mutate_output_activation_probability: f64,

    #[envconfig(
        from = "HIDDEN_ACTIVATIONS",
        default = "None ReLU Sigmoid Normal Sine Square Exp"
    )]
    pub hidden_activations: Activations,

    #[envconfig(
        from = "OUTPUT_ACTIVATIONS",
        default = "None ReLU Sigmoid Normal Sine Square Exp"
    )]
    pub output_activations: Activations,
}

lazy_static! {
    pub static ref CPPN: Conf = Conf::init().unwrap();
}
