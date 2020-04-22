#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate derive_new;

mod eshyperneat;
mod hyperneat;
mod neat;

use eshyperneat::eshyperneat;
use hyperneat::hyperneat;
use neat::{dataset_environment::DatasetEnvironment, neat};

fn main() {
    // neat::<DatasetEnvironment>();
    // hyperneat::<DatasetEnvironment>();
    eshyperneat::<DatasetEnvironment>();
}
