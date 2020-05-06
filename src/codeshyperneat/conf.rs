use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "BLUEPRINT_DEVELOPMENTS", default = "5")]
    pub blueprint_developments: usize,
}

lazy_static! {
    pub static ref CODESHYPERNEAT: Conf = Conf::init().unwrap();
}
