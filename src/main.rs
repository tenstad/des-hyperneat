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
use conf::MainConfig;
use cppn::cppn;
use dataset_environment::DatasetEnvironment;
use deshyperneat::deshyperneat;
use eshyperneat::eshyperneat;
use evolution::neat::neat;
use hyperneat::hyperneat;
use sideshyperneat::sideshyperneat;

fn main() {
    match &conf::CONF.method[..] {
        "NEAT" => neat::<DatasetEnvironment, MainConfig>(),
        "CPPN" => cppn::<DatasetEnvironment, MainConfig>(),
        "HyperNEAT" => hyperneat::<DatasetEnvironment, MainConfig>(),
        "ES-HyperNEAT" => eshyperneat::<DatasetEnvironment, MainConfig>(),
        "DES-HyperNEAT" => deshyperneat::<DatasetEnvironment, MainConfig>(),
        "CoDES-HyperNEAT" => codeshyperneat::<DatasetEnvironment, MainConfig>(),
        "SiDES-HyperNEAT" => sideshyperneat::<DatasetEnvironment, MainConfig>(),
        _ => println!("Unknown method method"),
    }
}
