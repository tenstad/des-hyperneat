#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate enum_display_derive;

mod conf;
mod data;
mod generic_neat;
mod neat;
mod network;

use crate::generic_neat::dataset_environment::DatasetEnvironment;

fn main() {
    let environment = DatasetEnvironment::init(&conf::NEAT.dataset_filename);

    if let Ok(environment) = environment {
        println!("Running NEAT with dataset: {}", environment);
        neat::neat(&environment);
    } else {
        println!("ERROR: Unable to load dataset");
    }
}
