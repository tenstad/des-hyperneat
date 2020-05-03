use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "SINGLE_CPPN_STATE", default = "false")]
    pub single_cppn_state: bool,
}

lazy_static! {
    pub static ref DESHYPERNEAT: Conf = Conf::init().unwrap();
}
