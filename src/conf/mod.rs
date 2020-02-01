mod neat;

use envconfig::Envconfig;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NEAT: neat::Conf = neat::Conf::init().unwrap();
}
