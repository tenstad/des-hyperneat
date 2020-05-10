use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "TOPOLOGY_MUTATION_PROBABILITY", default = "0.2")]
    pub topology_mutation_probability: f64,

    #[envconfig(from = "CPPN_MUTATION_PROBABILITY", default = "0.8")]
    pub cppn_mutation_probability: f64,
}

lazy_static! {
    pub static ref SIDESHYPERNEAT: Conf = Conf::init().unwrap();
}
