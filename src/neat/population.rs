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
    pub innovation_log: InnovationLog,
    pub global_innovation: InnovationTime,
}

impl Population {
    pub fn new(dimensions: &Dimensions) -> Population {
        let mut population = Population {
            species: Vec::new(),
            innovation_log: InnovationLog::new(),
            global_innovation: InnovationTime::new(),
        };

        for _ in 0..conf::NEAT.population_size {
            population.push(
                Organism::new(0, dimensions.inputs, dimensions.outputs),
                false,
            );
        }

        return population;
    }

    pub fn push(&mut self, organism: Organism, lock_new: bool) {
        if let Some(species) = self.compatible_species(&organism) {
            species.push(organism);
        } else {
            let mut species = Species::new();
            if lock_new {
                species.lock();
            }
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
        // Adjust fitnesses based on age, stagnation and apply fitness sharing
        for species in self.species.iter_mut() {
            species.adjust_fitness();
        }

        // Average fitness of all organisms
        let avg_fitness: f64 = self
            .iter()
            .map(|organism| organism.adjusted_fitness)
            .sum::<f64>()
            / conf::NEAT.population_size as f64;

        // Calculate number of new offsprings to produce within each new species
        for species in self.species.iter_mut() {
            species.calculate_offsprings(avg_fitness);
        }

        // The total size of the next population before making up for floting point precicsion
        let mut new_population_size: u64 = self
            .species
            .iter()
            .map(|species| species.offsprings.floor() as u64)
            .sum();

        // Sort species based on closeness to additional offspring
        let mut sorted_species: Vec<(f64, &mut Species)> = self
            .species
            .iter_mut()
            .map(|species| (species.offsprings % 1.0, species))
            .collect();
        // Reversed sort (highest first)
        sorted_species.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Distribute missing offsprings amongs species
        // in order of floating distance from additional offspring
        while new_population_size < conf::NEAT.population_size {
            for (_, species) in sorted_species.iter_mut() {
                species.offsprings += 1.0;
                new_population_size += 1;

                if new_population_size == conf::NEAT.population_size {
                    break;
                }
            }
        }

        // Verify correct amount of offsprings
        assert_eq!(
            self.species
                .iter()
                .map(|species| species.offsprings.floor() as u64)
                .sum::<u64>(),
            conf::NEAT.population_size
        );

        // Kill individuals of low performance, not allowed to reproduce
        for species in self.species.iter_mut() {
            species.truncate();
        }

        // Increase the age of the species, making current organisms old
        for species in self.species.iter_mut() {
            species.age();
        }

        // Evolve spiecies
        let mut rng = rand::thread_rng();
        for i in 0..self.species.len() {
            let elites = std::cmp::min(
                conf::NEAT.elitism as usize,
                std::cmp::min(
                    self.species[i].len(),
                    self.species[i].offsprings.floor() as usize,
                ),
            );
            let reproductions = self.species[i].offsprings.floor() as usize - elites;

            for j in 0..elites {
                self.push(self.species[i].organisms[j].clone(), true);
            }

            for _ in 0..reproductions {
                let organism = if rng.gen::<f64>() < conf::NEAT.interspecies_reproduction_chance {
                    self.tournament_select(2)
                } else {
                    self.species[i].random_organism()
                };
                if let (Some(a), Some(b)) = (self.species[i].random_organism(), organism) {
                    let mut child = a.crossover(b);
                    child.mutate(&mut self.innovation_log, &mut self.global_innovation);
                    self.push(child, true);
                }
            }
        }

        // Kill old population
        for species in self.species.iter_mut() {
            species.remove_old();
        }

        // Remove empty species
        for i in (0..self.species.len()).rev() {
            if self.species[i].len() == 0 {
                self.species.swap_remove(i);
            }
        }

        print!("{}: ", self.species.len());
        for species in self.species.iter_mut() {
            print!("{} ", species.organisms.len());
        }
        println!("");

        // Verify correct number of individuals
        assert_eq!(self.iter().count(), conf::NEAT.population_size as usize);

        self.evaluate(environment);
    }

    pub fn random_organism(&self) -> Option<&Organism> {
        let mut rng = rand::thread_rng();
        let len = self.iter().count();

        self.iter().skip(rng.gen_range(0, len) as usize).next()
    }

    pub fn tournament_select(&self, k: u64) -> Option<&Organism> {
        let mut best: Option<&Organism> = None;
        let mut best_fitness = -1.0;

        for _ in 0..k {
            if let Some(organism) = self.random_organism() {
                if organism.fitness > best_fitness {
                    best = Some(organism);
                    best_fitness = organism.fitness;
                }
            }
        }

        return best;
    }

    pub fn evaluate(&mut self, environment: &dyn Environment) {
        for organism in self.iter_mut() {
            organism.evaluate(environment);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Organism> {
        self.species.iter().map(|species| species.iter()).flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Organism> {
        self.species
            .iter_mut()
            .map(|species| species.iter_mut())
            .flatten()
    }

    pub fn best(&self) -> Option<&Organism> {
        self.iter().max_by(|a, b| a.cmp(&b))
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
