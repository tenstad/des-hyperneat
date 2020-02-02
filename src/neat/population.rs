use crate::conf;
use crate::data::dataset::Dimensions;
use crate::neat::environment::Environment;
use crate::neat::nodes;
use crate::neat::organism::Organism;
use crate::neat::species::Species;
use rand::Rng;
use std::collections::HashMap;

pub struct Population {
    species: Vec<Species>,
    innovation_log: InnovationLog,
    global_innovation: InnovationTime,
}

impl Population {
    pub fn new(dimensions: &Dimensions) -> Population {
        let mut population = Population {
            species: Vec::new(),
            innovation_log: InnovationLog::new(),
            global_innovation: InnovationTime::new(),
        };

        for _ in 0..conf::NEAT.population_size {
            population.push(Organism::new(0, dimensions.inputs, dimensions.outputs));
        }

        return population;
    }

    pub fn push(&mut self, organism: Organism) {
        if let Some(species) = self.compatible_species(&organism) {
            species.push(organism);
        } else {
            let mut species = Species::new();
            species.push(organism);
            self.species.push(species);
        }
    }

    fn compatible_species(&mut self, organism: &Organism) -> Option<&mut Species> {
        for species in self.species.iter_mut() {
            if species.is_compatible(&organism) {
                return Some(species);
            }
        }

        None
    }

    pub fn evolve(&mut self, environment: &dyn Environment) {
        let mut new_population = Population {
            species: Vec::new(),
            innovation_log: InnovationLog::new(),
            global_innovation: InnovationTime::new(),
        };

        for _ in 0..conf::NEAT.population_size {
            if let (Some(a), Some(b)) = (self.best_of(2), self.best_of(2)) {
                let mut child = a.crossover(b);
                child.mutate(&mut self.innovation_log, &mut self.global_innovation);
                new_population.push(child);
            }
        }

        self.species = new_population.species;

        self.evaluate(environment);
    }

    pub fn random_organism(&self) -> Option<&Organism> {
        let mut rng = rand::thread_rng();

        self.iter()
            .skip(rng.gen_range(0, conf::NEAT.population_size) as usize)
            .next()
    }

    pub fn best_of(&self, k: u64) -> Option<&Organism> {
        let mut best: Option<&Organism> = None;
        let mut best_fitness = 1000000.0;

        for _ in 0..k {
            if let Some(organism) = self.random_organism() {
                if organism.fitness < best_fitness {
                    best = Some(organism);
                    best_fitness = organism.fitness;
                }
            }
        }

        return best;
    }

    pub fn evaluate(&mut self, environment: &dyn Environment) {
        let sharing: Vec<u64> = self
            .iter()
            .map(|o1| {
                self.iter()
                    .filter(|o2| o1.distance(o2) < conf::NEAT.sharing_threshold)
                    .count() as u64
            })
            .collect();

        for (organism, sharing) in self.iter_mut().zip(sharing) {
            organism.evaluate(environment, sharing);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Organism> {
        self.species
            .iter()
            .map(|species| species.organisms.iter())
            .flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Organism> {
        self.species
            .iter_mut()
            .map(|species| species.organisms.iter_mut())
            .flatten()
    }

    pub fn best(&self) -> Option<&Organism> {
        self.iter().min_by(|a, b| a.cmp(&b))
    }
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
