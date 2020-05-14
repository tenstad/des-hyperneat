use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "HOST", default = "localhost")]
    pub host: String,

    #[envconfig(from = "USERNAME", default = "admin")]
    pub username: String,

    #[envconfig(from = "PASSWORD", default = "")]
    pub password: String,

    #[envconfig(from = "DATABASE", default = "des-hyperneat")]
    pub database: String,

    #[envconfig(from = "COLLECTION", default = "log")]
    pub collection: String,
}

lazy_static! {
    pub static ref DB: Conf = Conf::init().unwrap();
}
