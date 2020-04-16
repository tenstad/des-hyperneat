use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET", default = "datasets/iris")]
    pub dataset: String,

    #[envconfig(from = "THREADS", default = "8")]
    pub thread_count: usize,
}
