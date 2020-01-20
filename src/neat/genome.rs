pub struct Link {
    pub from: i64,
    pub to: i64,
    pub weight: f64,
    pub enabled: bool,
    pub innovation: i64,
}

pub struct Genome {
    pub inputs: i64,
    pub outputs: i64,
    pub nodes: i64,
    pub links: Vec<Link>,
}

impl Genome {
    pub fn generate(inputs: i64, outputs: i64) -> Genome {
        return Genome {
            inputs: inputs,
            outputs: outputs,
            nodes: 10,
            links: Vec::new(),
        };
    }
}
