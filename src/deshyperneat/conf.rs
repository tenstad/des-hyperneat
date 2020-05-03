use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {}

lazy_static! {
    pub static ref DESHYPERNEAT: Conf = Conf::init().unwrap();
}
