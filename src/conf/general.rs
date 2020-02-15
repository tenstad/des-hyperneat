use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "THREADS", default = "1")]
    pub threads: usize,
}
