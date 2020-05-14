use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "METHOD", default = "DES-HyperNEAT")]
    pub method: String,

    #[envconfig(from = "DEBUG", default = "true")]
    pub debug: bool,
}

lazy_static! {
    pub static ref CONF: Conf = Conf::init().unwrap();
}
