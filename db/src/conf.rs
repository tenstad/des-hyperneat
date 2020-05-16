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

    #[envconfig(from = "JOB_COLLECTION", default = "jobs")]
    pub job_collection: String,

    #[envconfig(from = "LOG_COLLECTION", default = "logs")]
    pub log_collection: String,

    #[envconfig(from = "JOB_ID", default = "0")]
    pub job_id: String,

    #[envconfig(from = "METHOD", default = "")]
    pub method: String,
}

lazy_static! {
    pub static ref DB: Conf = Conf::init().unwrap();
}
