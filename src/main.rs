#[macro_use]
extern crate envconfig_derive;
extern crate derive_new;
extern crate envconfig;

mod conf;
mod cppn;
mod dataset_environment;
mod eshyperneat;
mod hyperneat;

use cppn::cppn;
use dataset_environment::DatasetEnvironment;
use eshyperneat::eshyperneat;
use evolution::neat::neat;
use hyperneat::hyperneat;

fn main() {
    match &conf::CONF.method[..] {
        "NEAT" => neat::<DatasetEnvironment>(),
        "CPPN" => cppn::<DatasetEnvironment>(),
        "HyperNEAT" => hyperneat::<DatasetEnvironment>(),
        "ES-HyperNEAT" => eshyperneat::<DatasetEnvironment>(),
        _ => println!("Unknown method method"),
    }
}
