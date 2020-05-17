use serde::Serialize;

pub trait Stats: Send + Serialize {}

#[derive(Serialize)]
pub struct NoStats;
impl Stats for NoStats {}

#[derive(Serialize, new)]
pub struct PopulationStats<G: Serialize, P: Serialize, E: Serialize> {
    organism: Vec<OrganismStats<G, P, E>>,
}
#[derive(Serialize, new)]
pub struct OrganismStats<G: Serialize, P: Serialize, E: Serialize> {
    pub fitness: f64,
    pub genome: G,
    pub phenotype: P,
    pub evaluation: E,
}

pub trait GetPopulationStats {
    type G: Serialize;
    type P: Serialize;
    type E: Serialize;

    fn population(&self) -> &PopulationStats<Self::G, Self::P, Self::E>;
    fn best(&self) -> Option<&OrganismStats<Self::G, Self::P, Self::E>>;
}

impl<G: Serialize, P: Serialize, E: Serialize> GetPopulationStats for PopulationStats<G, P, E> {
    type G = G;
    type P = P;
    type E = E;

    fn population(&self) -> &Self {
        self
    }

    fn best(&self) -> Option<&OrganismStats<Self::G, Self::P, Self::E>> {
        self.organism
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
    }
}
