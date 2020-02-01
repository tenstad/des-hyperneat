use crate::neat::organism::Organism;
use crate::conf;

pub struct Species {
    pub organisms: Vec<Organism>,
}

impl Species {
    pub fn new() -> Species {
        Species {
            organisms: Vec::new(),
        }
    }

    pub fn is_compatible(&mut self, organism: &Organism) -> bool {
        if let Some(first_organism) = self.organisms.get(0) {
            first_organism.distance(organism) < conf::NEAT.speciation_threshold
        } else {
            true
        }
    }

    pub fn push(&mut self, individual: Organism) {
        self.organisms.push(individual);
    }
}
