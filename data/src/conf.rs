use bson;
use envconfig::Envconfig;
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Envconfig, Serialize)]
pub struct DatasetConfig {
    #[envconfig(from = "DATASET", default = "datasets/generated/wine")]
    pub dataset: String,

    #[envconfig(from = "SEED", default = "0")]
    #[serde(with = "bson::compat::u2f")]
    pub seed: u64,

    #[envconfig(from = "VALIDATION_FRACTION", default = "0.0")]
    pub validation_fraction: f64,

    #[envconfig(from = "TEST_FRACTION", default = "0.0")]
    pub test_fraction: f64,

    #[envconfig(from = "ADD_BIAS_INPUT", default = "false")]
    pub add_bias_input: bool,
}

impl Default for DatasetConfig {
    fn default() -> Self {
        Self::init().unwrap()
    }
}

lazy_static! {
    pub static ref DATA: DatasetConfig = DatasetConfig::init().unwrap();
}
