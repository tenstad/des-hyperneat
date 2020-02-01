#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

mod conf;
mod neat;

fn main() {
    neat::neat();
}
