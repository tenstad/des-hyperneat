#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate enum_display_derive;

mod conf;
mod data;
mod generic_neat;
mod hyperneat;
mod neat;
mod network;

use crate::generic_neat::environment::Environment;
use crate::hyperneat::substrate;
use crate::neat::dataset_environment::DatasetEnvironment;

fn main() {
    let environment = DatasetEnvironment::init(&conf::NEAT.dataset_filename);

    if let Ok(environment) = environment {
        let network = substrate::Network::layered(vec![
            environment.get_dimensions().inputs as usize,
            20,
            20,
            20,
            environment.get_dimensions().outputs as usize,
        ]);

        hyperneat::hyperneat(&environment, network);
        // neat::neat(&environment);
    } else {
        println!("ERROR: Unable to load dataset");
    }
}
