#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate enum_display_derive;

mod conf;
mod data;
mod neat;
mod network;

use neat::experiments::dataset_experiment::DatasetExperiment;

fn main() {
    let environment = DatasetExperiment::init(&conf::NEAT.dataset_filename);
    if let Ok(environment) = environment {
        println!("Running NEAT with environment: {}", environment);
        neat::neat(&environment);
    } else {
        println!("ERROR: Unable to load environment");
    }
}
