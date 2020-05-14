use crate::conf::PopulationConfig;
use crate::environment::Stats;
use crate::evaluate;
use crate::genome::Genome;
use crate::organism::Organism;
use crate::species::Species;
use rand::Rng;
use std::{collections::HashMap, f64, fmt};

pub struct Population<G: Genome, S> {
    pub population_config: PopulationConfig,
    pub genome_config: G::Config,
    pub species: HashMap<u64, Species<G, S>>,
    pub extinct_species: HashMap<u64, Species<G, S>>,
    pub next_id: u64,
    pub state: G::State,
}

impl<G: Genome, S> Population<G, S> {
    pub fn new(
        population_config: PopulationConfig,
        genome_config: G::Config,
        init_config: &G::InitConfig,
    ) -> Self {
        let genome_config_clone = genome_config.clone();
        let mut population = Population {
            population_config,
            genome_config,
            species: HashMap::new(),
            extinct_species: HashMap::new(),
            next_id: 0,
            state: G::State::default(), // temp value, replaced after inserting all organisms
        };

        let mut state = G::State::default();
        for _ in 0..population.population_config.population_size {
            population.push(
                Organism::<G, S>::new(&genome_config_clone, &init_config, &mut state),
                false,
            );
        }
        population.state = state;

        return population;
    }

    /// Add organism to population
    pub fn push(&mut self, organism: Organism<G, S>, lock_new: bool) {
        if let Some(species) = self.compatible_species(&organism) {
            species.push(organism);
        } else {
            // New organism is not compatible with any existing species, create a new one
            let mut species = Species::<G, S>::new();
            // If during reproduction, the species is locked so that the new organism avoids parent selection
            if lock_new {
                species.lock();
            }
            species.push(organism);
            self.species.insert(self.next_id, species);
            self.next_id += 1;
        }
    }

    /// Find first species compatible with organism
    fn compatible_species(&mut self, organism: &Organism<G, S>) -> Option<&mut Species<G, S>> {
        for species in self.species.values_mut() {
            if species.is_compatible(&self.population_config, &self.genome_config, &organism) {
                return Some(species);
            }
        }

        None
    }

    /// Evolve the population
    pub fn evolve(&mut self) {
        // Adjust fitnesses based on age, stagnation and apply fitness sharing
        // Also sorts organisms by descending fitness
        for species in self.species.values_mut() {
            species.adjust_fitness(&self.population_config);
        }

        // Average fitness of all organisms
        let elites = self.population_config.global_elites
            + self.population_config.guaranteed_elites * self.species.len() as u64;
        // Subtract number of guaranteed elites from pop size, reserving these slots for elites.
        let avg_fitness: f64 = self
            .iter()
            .map(|organism| organism.adjusted_fitness.unwrap())
            .sum::<f64>()
            / (self.population_config.population_size - elites) as f64;

        // Calculate number of new offsprings to produce within each new species
        for species in self.species.values_mut() {
            species.calculate_offsprings(avg_fitness, &self.population_config);
        }

        // The total size of the next population before making up for floting point precicsion
        let mut new_population_size = self
            .species
            .values()
            .map(|species| species.offsprings.floor() as u64)
            .sum::<u64>()
            + elites;

        let mut species_ids = self.species.keys().cloned().collect::<Vec<u64>>();

        // Sort species based on closeness to additional offspring
        species_ids.sort_by(|a, b| {
            (1.0 - self.species[a].offsprings % 1.0)
                .partial_cmp(&(1.0 - self.species[b].offsprings % 1.0))
                .unwrap()
        });

        // Distribute missing offsprings amongs species
        // in order of floating distance from additional offspring
        while new_population_size < self.population_config.population_size {
            for species_id in species_ids.iter() {
                let mut species = self.species.get_mut(species_id).unwrap();
                species.offsprings = species.offsprings.floor() + 1.0;
                new_population_size += 1;

                if new_population_size == self.population_config.population_size {
                    break;
                }
            }
        }

        // Sort species based on best_fitness (best first)
        species_ids.sort_by(|a, b| {
            self.species[b]
                .best_fitness
                .partial_cmp(&self.species[a].best_fitness)
                .unwrap()
        });
        let mut elites_distributed = 0;

        // Distribute elites
        while elites_distributed < self.population_config.global_elites {
            for species_id in species_ids.iter() {
                let mut species = self.species.get_mut(species_id).unwrap();
                if species.elites < species.len() as u64 {
                    species.elites += 1;
                    elites_distributed += 1;

                    if elites_distributed == self.population_config.global_elites {
                        break;
                    }
                }
            }
        }

        assert_eq!(
            self.species
                .values()
                .map(|s| s.offsprings.floor() as u64 + s.elites)
                .sum::<u64>(),
            self.population_config.population_size,
            "wrong number of planned individuals in next population"
        );

        // Kill individuals of low performance, not allowed to reproduce
        for species in self.species.values_mut() {
            species.retain_best(&self.population_config);
        }

        // Increase the age of and lock all species, making current organisms old
        for species in self.species.values_mut() {
            species.age();
        }

        for i in species_ids.iter() {
            let mut species = self.species.get_mut(i).unwrap();
            // Steal elites from number of offsprings
            let elites_taken_from_offspring = self
                .population_config
                .elites_from_offspring
                .min(species.offsprings.floor() as u64)
                .min(species.len() as u64);
            species.elites += elites_taken_from_offspring;
            species.offsprings -= elites_taken_from_offspring as f64;
            drop(species);

            // Directly copy elites, without crossover or mutation
            for j in 0..(self.species[i].elites as usize) {
                self.push(self.species[i].organisms[j].as_elite(), true);
            }
        }

        // Evolve spiecies
        let mut rng = rand::thread_rng();
        for i in species_ids.iter() {
            let reproductions = self.species[i].offsprings.floor() as u64;

            // Breed new organisms
            for _ in 0..reproductions {
                let error = "unable to gather organism";
                let father = if rng.gen::<f64>()
                    < self.population_config.interspecies_reproduction_probability
                {
                    // Interspecies breeding
                    self.tournament_select(self.population_config.interspecies_tournament_size)
                        .expect(error)
                } else {
                    // Breeding within species
                    self.species[i]
                        .tournament_select(self.population_config.tournament_size)
                        .expect(error)
                };

                let mut child =
                    if rng.gen::<f64>() < self.population_config.asexual_reproduction_probability {
                        father.as_elite()
                    } else {
                        let mother = self.species[i]
                            .tournament_select(self.population_config.tournament_size)
                            .expect(error);
                        mother.crossover(&self.genome_config, father)
                    };

                child.mutate(&self.genome_config, &mut self.state);
                self.push(child, true);
            }
        }

        // Kill old population
        for species in self.species.values_mut() {
            species.remove_old();
        }

        // Remove extinct species
        for i in species_ids.iter() {
            if self.species[i].extinct {
                self.extinct_species
                    .insert(*i, self.species.remove(i).unwrap());
            }
        }

        // Verify correct number of individuals in new population
        assert_eq!(
            self.iter().count() as u64,
            self.population_config.population_size,
            "wrong number of individuals in population"
        );
    }

    pub fn mutate(&mut self) {
        for species in self.species.values_mut() {
            for organism in species.iter_mut() {
                organism.mutate(&self.genome_config, &mut self.state);
            }
        }
    }

    /// Get random organism from population
    fn random_organism(&self) -> Option<&Organism<G, S>> {
        let len = self.iter().count();

        if len == 0 {
            None
        } else {
            self.iter()
                .skip(rand::thread_rng().gen_range(0, len))
                .next()
        }
    }

    /// Use tournament selection to select an organism
    fn tournament_select(&self, k: u64) -> Option<&Organism<G, S>> {
        let mut best: Option<&Organism<G, S>> = None;
        let mut best_fitness = f64::MIN;

        for _ in 0..k {
            if let Some(organism) = self.random_organism() {
                if let Some(fitness) = organism.fitness {
                    if fitness > best_fitness {
                        best = Some(organism);
                        best_fitness = fitness;
                    }
                }
            }
        }

        return best;
    }

    /// Update fitness of all organisms
    pub fn evaluate(&mut self, evaluator: &impl evaluate::Evaluate<G, S>) {
        for (species_index, organism_index, fitness, stats) in evaluator
            .evaluate(
                self.enumerate()
                    .map(|(species_index, organism_index, organism)| {
                        (species_index, organism_index, organism.genome.clone())
                    }),
            )
            .drain(..)
        {
            self.species.get_mut(&species_index).unwrap().organisms[organism_index].fitness =
                Some(fitness);
            self.species.get_mut(&species_index).unwrap().organisms[organism_index].stats =
                Some(stats);
        }
    }

    /// Iterate organisms
    pub fn iter(&self) -> impl Iterator<Item = &Organism<G, S>> {
        self.species
            .values()
            .map(|species| species.iter())
            .flatten()
    }

    /// Iterate organisms
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Organism<G, S>> {
        self.species
            .values_mut()
            .map(|species| species.iter_mut())
            .flatten()
    }

    /// Enumerate organisms
    pub fn enumerate(&self) -> impl Iterator<Item = (u64, usize, &Organism<G, S>)> {
        self.species
            .iter()
            .map(|(species_index, species)| {
                species
                    .iter()
                    .enumerate()
                    .map(move |(genome_index, genome)| (*species_index, genome_index, genome))
            })
            .flatten()
    }

    /// Gather best organism
    pub fn best(&self) -> Option<&Organism<G, S>> {
        self.iter().max_by(|a, b| a.cmp(&b))
    }
}

impl<G: Genome, S: Stats> fmt::Display for Population<G, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Population(species: {}, extinct: {}): ",
            self.species.len(),
            self.extinct_species.len()
        )?;
        for species in self.species.values() {
            write!(f, "{} ", species.organisms.len())?;
        }
        Ok(())
    }
}
