mod general;
mod hyperneat;
mod neat;

use envconfig::Envconfig;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GENERAL: general::Conf = general::Conf::init().unwrap();
    pub static ref NEAT: neat::Conf = neat::Conf::init().unwrap();
    pub static ref HYPERNEAT: hyperneat::Conf = hyperneat::Conf::init().unwrap();
}
