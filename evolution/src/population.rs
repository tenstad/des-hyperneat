use crate::conf::CONF;
use crate::evaluate;
use crate::genome::Genome;
use crate::organism::Organism;
use crate::species::Species;
use rand::Rng;
use std::f64;
use std::fmt;

pub struct Population<G: Genome> {
    population_size: usize,
    species: Vec<Species<G>>,
    state: G::PopulationState,
}

impl<G: Genome> Population<G> {
    pub fn new(population_size: usize, init_config: &G::InitConfig) -> Self {
        let mut population = Population {
            population_size,
            species: Vec::new(),
            state: G::PopulationState::default(),
        };

        for _ in 0..population_size {
            population.push(Organism::<G>::new(&init_config), false);
        }

        return population;
    }

    /// Add organism to population
    pub fn push(&mut self, organism: Organism<G>, lock_new: bool) {
        if let Some(species) = self.compatible_species(&organism) {
            species.push(organism);
        } else {
            let mut species = Species::<G>::new();
            if lock_new {
                species.lock();
            }
            species.push(organism);
            self.species.push(species);
        }
    }

    /// Find first species compatible with organism
    fn compatible_species(&mut self, organism: &Organism<G>) -> Option<&mut Species<G>> {
        for species in self.species.iter_mut() {
            if species.is_compatible(&organism) {
                return Some(species);
            }
        }

        None
    }

    /// Evolve the population
    pub fn evolve(&mut self) {
        // Adjust fitnesses based on age, stagnation and apply fitness sharing
        for species in self.species.iter_mut() {
            species.adjust_fitness();
        }

        // Average fitness of all organisms
        // Subtract 1 from pop size to make allow for potential increase of (up to) 1.0 in best fit species.
        // If not increased by (up to) 1.0, the extra individual will be added to the spicies closest to
        // reproducing an additional child.
        let avg_fitness: f64 = self
            .iter()
            .map(|organism| organism.adjusted_fitness)
            .sum::<f64>()
            / (CONF.population_size - 1) as f64;

        // Calculate number of new offsprings to produce within each new species
        for species in self.species.iter_mut() {
            species.calculate_offsprings(avg_fitness);
        }

        // Make sure best species reproduces (or survives through elitism, if enabled)
        let best_specie = self
            .species
            .iter_mut()
            .max_by(|a, b| a.best_fitness.partial_cmp(&b.best_fitness).unwrap())
            .unwrap();
        if best_specie.offsprings < 1.0 {
            best_specie.offsprings = 1.0;
        }

        // The total size of the next population before making up for floting point precicsion
        let mut new_population_size: usize = self
            .species
            .iter()
            .map(|species| species.offsprings.floor() as usize)
            .sum();

        // Sort species based on closeness to additional offspring
        let mut sorted_species: Vec<(f64, &mut Species<G>)> = self
            .species
            .iter_mut()
            .map(|species| (species.offsprings % 1.0, species))
            .collect();
        // Reversed sort (highest first)
        sorted_species.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Distribute missing offsprings amongs species
        // in order of floating distance from additional offspring
        while new_population_size < self.population_size {
            for (_, species) in sorted_species.iter_mut() {
                species.offsprings += 1.0;
                new_population_size += 1;

                if new_population_size == self.population_size {
                    break;
                }
            }
        }

        // Verify correct amount of offsprings
        assert_eq!(
            self.species
                .iter()
                .map(|species| species.offsprings.floor() as usize)
                .sum::<usize>(),
            self.population_size
        );

        // Kill individuals of low performance, not allowed to reproduce
        for species in self.species.iter_mut() {
            species.retain_best();
        }

        // Increase the age of the species, making current organisms old
        for species in self.species.iter_mut() {
            species.age();
        }

        // Evolve spiecies
        let mut rng = rand::thread_rng();
        for i in 0..self.species.len() {
            let elites = std::cmp::min(
                CONF.elitism,
                std::cmp::min(
                    self.species[i].len(),
                    self.species[i].offsprings.floor() as usize,
                ),
            );
            let reproductions = self.species[i].offsprings.floor() as usize - elites;

            // Directly copy elites, without crossover or mutation
            for j in 0..elites {
                self.push(self.species[i].organisms[j].clone(), true);
            }

            // Breed new organisms
            for _ in 0..reproductions {
                let error = "unable to gather organism";
                let father = if rng.gen::<f64>() < CONF.interspecies_reproduction_chance {
                    // Interspecies breeding
                    self.tournament_select(CONF.interspecies_tournament_size)
                        .expect(error)
                } else {
                    // Breeding within species
                    self.species[i].random_organism().expect(error)
                };
                let mother = self.species[i].random_organism().expect(error);

                let mut child = mother.crossover(father);
                child.mutate(&mut self.state);
                self.push(child, true);
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

        // Verify correct number of individuals in new population
        assert_eq!(self.iter().count(), CONF.population_size);
    }

    /// Get random organism from population
    fn random_organism(&self) -> Option<&Organism<G>> {
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
    fn tournament_select(&self, k: usize) -> Option<&Organism<G>> {
        let mut best: Option<&Organism<G>> = None;
        let mut best_fitness = f64::MIN;

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

    /// Update fitness of all organisms
    pub fn evaluate(&mut self, evaluator: &impl evaluate::Evaluate<G>) {
        for (species_index, organism_index, fitness) in evaluator
            .evaluate(
                self.enumerate()
                    .map(|(species_index, organism_index, organism)| {
                        (species_index, organism_index, organism.genome.clone())
                    }),
            )
            .iter()
        {
            self.species[*species_index].organisms[*organism_index].fitness = *fitness;
        }
    }

    /// Iterate organisms
    fn iter(&self) -> impl Iterator<Item = &Organism<G>> {
        self.species.iter().map(|species| species.iter()).flatten()
    }

    /// Iterate organisms
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Organism<G>> {
        self.species
            .iter_mut()
            .map(|species| species.iter_mut())
            .flatten()
    }

    /// Enumerate organisms
    fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &Organism<G>)> {
        self.species
            .iter()
            .enumerate()
            .map(|(species_index, species)| {
                species
                    .iter()
                    .enumerate()
                    .map(move |(genome_index, genome)| (species_index, genome_index, genome))
            })
            .flatten()
    }

    /// Gather best organism
    pub fn best(&self) -> Option<&Organism<G>> {
        self.iter().max_by(|a, b| a.cmp(&b))
    }
}

impl<G: Genome> fmt::Display for Population<G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Population(species: {}): ", self.species.len())?;
        for species in self.species.iter() {
            write!(f, "{} ", species.organisms.len())?;
        }
        Ok(())
    }
}
