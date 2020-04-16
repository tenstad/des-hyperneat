#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate enum_display_derive;

mod conf;
mod data;
mod eshyperneat;
mod figure;
mod generic_neat;
mod hyperneat;
mod neat;
mod network;

fn main() {
    // neat::neat::<neat::dataset_environment::DatasetEnvironment>();
    // hyperneat::hyperneat::<hyperneat::dataset_environment::DatasetEnvironment>();
    eshyperneat::hyperneat::<hyperneat::dataset_environment::DatasetEnvironment>();
}
