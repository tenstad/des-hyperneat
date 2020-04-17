use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {
    #[envconfig(from = "DATASET", default = "datasets/wine")]
    pub dataset: String,

    #[envconfig(from = "THREADS", default = "8")]
    pub thread_count: usize,
}
