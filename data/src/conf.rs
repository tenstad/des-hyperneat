use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET", default = "datasets/wine")]
    pub dataset: String,
}

lazy_static! {
    pub static ref DATA: Conf = Conf::init().unwrap();
}
