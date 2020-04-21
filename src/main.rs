#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate derive_new;

mod data;
mod eshyperneat;
mod hyperneat;
mod neat;

use eshyperneat::eshyperneat;
use hyperneat::hyperneat;
use neat::neat;

fn main() {
    // neat::<neat::dataset_environment::DatasetEnvironment>();
    // hyperneat::<hyperneat::dataset_environment::DatasetEnvironment>();
    eshyperneat::<hyperneat::dataset_environment::DatasetEnvironment>();
}
