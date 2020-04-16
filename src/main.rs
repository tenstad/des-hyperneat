#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

mod conf;
mod data;
mod eshyperneat;
mod generic_neat;
mod hyperneat;
mod neat;

fn main() {
    // neat::neat::<neat::dataset_environment::DatasetEnvironment>();
    // hyperneat::hyperneat::<hyperneat::dataset_environment::DatasetEnvironment>();
    eshyperneat::hyperneat::<hyperneat::dataset_environment::DatasetEnvironment>();
}
