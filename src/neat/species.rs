use crate::neat::organism::Organism;

pub struct Species {
    individuals: Vec<Organism>,
}

impl Species {
    pub fn new() -> Species {
        Species {
            individuals: Vec::new(),
        }
    }

    pub fn individual_compatible(&mut self, individual: &Organism, threshold: f64) -> bool {
        self.individuals.len() == 0 || self.individuals[0].distance(individual) < threshold
    }

    pub fn add(&mut self, individual: Organism) {
        self.individuals.push(individual);
    }

    pub fn best(&self) -> Option<&Organism> {
        self.individuals.iter().max_by(|a, b| a.cmp(&b))
    }
}
