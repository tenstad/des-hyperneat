#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate derive_new;

mod conf;
mod dataset_environment;
mod eshyperneat;
mod hyperneat;
mod neat;

use dataset_environment::DatasetEnvironment;
use eshyperneat::eshyperneat;
use hyperneat::hyperneat;
use neat::neat;

fn main() {
    match &conf::CONF.method[..] {
        "NEAT" => neat::<DatasetEnvironment>(),
        "HyperNEAT" => hyperneat::<DatasetEnvironment>(),
        "ES-HyperNEAT" => eshyperneat::<DatasetEnvironment>(),
        _ => println!("Unknown method method"),
    }
}
