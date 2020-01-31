use crate::neat::organism::Organism;
use crate::neat::nodes;
use crate::neat::species::Species;
use std::cmp;
use std::collections::HashMap;

pub struct Population {
    species: Vec<Species>,
    speciation_threshold: f64,
    innovation_log: InnovationLog,
    global_innovation: InnovationTime,
}

pub struct InnovationLog {
    pub node_additions: HashMap<u64, InnovationTime>,
    pub edge_additions: HashMap<(nodes::NodeRef, nodes::NodeRef), u64>,
}

impl InnovationLog {
    pub fn new() -> InnovationLog {
        InnovationLog {
            node_additions: HashMap::<u64, InnovationTime>::new(),
            edge_additions: HashMap::<(nodes::NodeRef, nodes::NodeRef), u64>::new(),
        }
    }
}

pub struct InnovationTime {
    pub node_number: u64,
    pub innovation_number: u64,
}

impl InnovationTime {
    pub fn new() -> InnovationTime {
        InnovationTime {
            node_number: 0,
            innovation_number: 0,
        }
    }
}

impl Population {
    pub fn new(size: usize, speciation_threshold: f64, inputs: u64, outputs: u64) -> Population {
        let mut population = Population {
            species: Vec::new(),
            speciation_threshold,
            innovation_log: InnovationLog::new(),
            global_innovation: InnovationTime::new(),
        };

        for _ in 0..size {
            population.add(Organism::new(0, inputs, outputs));
        }

        return population;
    }

    pub fn add(&mut self, individual: Organism) {
        let mut added = false;

        for species in self.species.iter_mut() {
            if species.individual_compatible(&individual, self.speciation_threshold) {
                added = true;
                break;
            }
        }

        if !added {
            let mut species = Species::new();
            species.add(individual);
            self.species.push(species);
        }
    }

    pub fn evolve(&mut self) {}

    pub fn best(&self) -> Option<&Organism> {
        self.species
            .iter()
            .map(|species| species.best())
            .max_by(|a, b| match (a, b) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => cmp::Ordering::Greater,
                (None, Some(_)) => cmp::Ordering::Less,
                (None, None) => cmp::Ordering::Equal,
            })
            .unwrap_or(None)
    }
}
