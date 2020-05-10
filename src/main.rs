#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate neat_macro;

mod codeshyperneat;
mod conf;
mod cppn;
mod dataset_environment;
mod deshyperneat;
mod eshyperneat;
mod hyperneat;
mod sideshyperneat;

use codeshyperneat::codeshyperneat;
use cppn::cppn;
use dataset_environment::DatasetEnvironment;
use deshyperneat::deshyperneat;
use eshyperneat::eshyperneat;
use evolution::neat::neat;
use hyperneat::hyperneat;
use sideshyperneat::sideshyperneat;

fn main() {
    match &conf::CONF.method[..] {
        "NEAT" => neat::<DatasetEnvironment>(),
        "CPPN" => cppn::<DatasetEnvironment>(),
        "HyperNEAT" => hyperneat::<DatasetEnvironment>(),
        "ES-HyperNEAT" => eshyperneat::<DatasetEnvironment>(),
        "DES-HyperNEAT" => deshyperneat::<DatasetEnvironment>(),
        "CoDES-HyperNEAT" => codeshyperneat::<DatasetEnvironment>(),
        "SiDES-HyperNEAT" => sideshyperneat::<DatasetEnvironment>(),
        _ => println!("Unknown method method"),
    }
}
