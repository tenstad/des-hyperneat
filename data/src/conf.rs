use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET", default = "datasets/generated/wine")]
    pub dataset: String,

    #[envconfig(from = "SEED", default = "0")]
    pub seed: u64,

    #[envconfig(from = "VALIDATION_FRACTION", default = "0.0")]
    pub validation_fraction: f64,

    #[envconfig(from = "TEST_FRACTION", default = "0.0")]
    pub test_fraction: f64,
}

lazy_static! {
    pub static ref DATA: Conf = Conf::init().unwrap();
}
