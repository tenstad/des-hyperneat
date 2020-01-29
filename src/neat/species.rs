use crate::neat::individual::Individual;

pub struct Species {
    individuals: Vec<Individual>,
}

impl Species {
    pub fn new() -> Species {
        Species {
            individuals: Vec::new(),
        }
    }

    pub fn individual_compatible(&mut self, individual: &Individual, threshold: f64) -> bool {
        self.individuals.len() == 0 || self.individuals[0].distance(individual) < threshold
    }

    pub fn add(&mut self, individual: Individual) {
        self.individuals.push(individual);
    }
}
