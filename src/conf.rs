use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "METHOD", default = "ES-HyperNEAT")]
    pub method: String,
}

lazy_static! {
    pub static ref CONF: Conf = Conf::init().unwrap();
}
