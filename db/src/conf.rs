use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DB_HOST", default = "localhost")]
    pub host: String,

    #[envconfig(from = "DB_USERNAME", default = "admin")]
    pub username: String,

    #[envconfig(from = "DB_PASSWORD", default = "")]
    pub password: String,

    #[envconfig(from = "DATABASE", default = "deshyperneat")]
    pub database: String,

    #[envconfig(from = "COLLECTION", default = "logs")]
    pub collection: String,

    #[envconfig(from = "JOB_ID", default = "-1")]
    pub job_id: String,
}

lazy_static! {
    pub static ref DB: Conf = Conf::init().unwrap();
}
