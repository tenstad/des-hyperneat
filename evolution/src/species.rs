use crate::conf::PopulationConfig;
use crate::genome::Genome;
use crate::organism::Organism;
use rand::Rng;
use std::f64;

/// Collection of similar organisms
// The lock is used to add new organisms without affecting the reproduction of the previous generation.
// It is unlocked after reproduction, which will remove the previous generation and keep the new.
pub struct Species<G> {
    age: u64,
    pub best_fitness: f64,
    lifetime_best_fitness: f64,
    last_improvement: u64,
    pub offsprings: f64,
    pub elites: u64,
    pub organisms: Vec<Organism<G>>,
    pub extinct: bool,
    locked: bool, // When locked new organisms may be added, but the len() and iter() functions will remain unchanged after addition
    locked_organisms: usize, // The number of organisms when species was locked
}

impl<G: Genome> Species<G> {
    pub fn new() -> Self {
        Self {
            age: 0,
            best_fitness: 0.0,
            lifetime_best_fitness: 0.0,
            last_improvement: 0,
            offsprings: 0.0,
            elites: 0,
            extinct: false,
            locked: false,
            locked_organisms: 0,
            organisms: Vec::new(),
        }
    }

    /// Determine wether a new organism is compatible
    pub fn is_compatible(
        &mut self,
        population_config: &PopulationConfig,
        genome_config: &G::Config,
        other: &Organism<G>,
    ) -> bool {
        if let Some(organism) = self.organisms.first() {
            organism.distance(genome_config, other) < population_config.speciation_threshold
        } else {
            true // All organisms are compatible if the species is empty
        }
    }

    /// Add an organism
    pub fn push(&mut self, individual: Organism<G>) {
        self.organisms.push(individual);
    }

    /// Number of organisms. Adheres to lock.
    pub fn len(&self) -> usize {
        if self.locked {
            self.locked_organisms
        } else {
            self.organisms.len()
        }
    }

    /// Iterate organisms. Adheres to lock.
    pub fn iter(&self) -> impl Iterator<Item = &Organism<G>> {
        self.organisms.iter().take(self.len())
    }

    /// Iterate mutable organisms. Adheres to lock.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Organism<G>> {
        let len = self.len(); // Must read len before iter_mut
        self.organisms.iter_mut().take(len)
    }

    /// Get random organism. Adheres to lock.
    pub fn random_organism(&self) -> Option<&Organism<G>> {
        self.iter()
            .skip(rand::thread_rng().gen_range(0, self.len()))
            .next()
    }

    /// Adjust fintesses of all organims
    pub fn adjust_fitness(&mut self, config: &PopulationConfig) {
        assert!(!self.locked);

        let is_stagnent = self.age - self.last_improvement > config.dropoff_age;
        let is_young = self.age < config.young_age_limit;
        let size: f64 = self.organisms.len() as f64;

        for organism in self.organisms.iter_mut() {
            let mut adjusted_fitness = organism.fitness.expect("organism does not have fitness");

            // Greatly penalize stagnent species
            if is_stagnent {
                adjusted_fitness *= config.stagnent_species_fitness_multiplier;
            }

            // Boost young species
            if is_young {
                adjusted_fitness *= config.young_species_fitness_multiplier;
            }

            // Share fitness within species
            adjusted_fitness /= size;

            // Avoid zero fitness
            if adjusted_fitness <= 0.0 || !adjusted_fitness.is_finite() {
                adjusted_fitness = 0.0001;
            }

            organism.adjusted_fitness = Some(adjusted_fitness);
        }

        // Sort organisms descendingly by adjusted fitness
        self.organisms
            .sort_by(|a, b| b.adjusted_fitness.partial_cmp(&a.adjusted_fitness).unwrap());

        // Update best fitness and last improvement if currently best in lifetime
        self.best_fitness = self
            .organisms
            .first()
            .map(|organism| organism.fitness.unwrap())
            .unwrap_or(0.0);
        if self.best_fitness > self.lifetime_best_fitness {
            self.lifetime_best_fitness = self.best_fitness;
            self.last_improvement = self.age;
        }
    }

    /// Retain only the best individuals
    pub fn retain_best(&mut self, config: &PopulationConfig) {
        assert!(!self.locked);

        let new_size = (self.organisms.len() as f64 * config.survival_ratio).round() as usize;
        // Assumes the individuals are sorted in descending fitness order
        // Keep a minimum of two individuals for sexual reproduction
        self.organisms
            .truncate(new_size.max(self.elites as usize).max(2));
    }

    /// Lock the species, so that next generation organisms are not used for reproduction
    pub fn lock(&mut self) {
        assert!(!self.locked);

        self.locked_organisms = self.organisms.len();
        self.locked = true;
    }

    /// Increase age and prepare for addition of new organisms
    pub fn age(&mut self) {
        self.lock();
        self.age += 1;
    }

    /// Remove all the locked organisms (the old generation), and retain the organisms pushed after lock (next generation)
    pub fn remove_old(&mut self) {
        assert!(self.locked);
        self.locked = false;

        if self.locked_organisms < self.organisms.len() {
            self.organisms = self.organisms.split_off(self.locked_organisms);
        } else {
            // No new individuals were added, species is now extinct
            self.organisms.truncate(1);
            self.extinct = true;
        }
    }

    /// Calculate number of offsprings based on adjusted fitness of organisms
    pub fn calculate_offsprings(&mut self, avg_fitness: f64, config: &PopulationConfig) {
        assert!(!self.locked);

        self.offsprings = self
            .organisms
            .iter()
            .map(|organism| organism.adjusted_fitness.unwrap() / avg_fitness)
            .sum();
        self.elites = config.guaranteed_elites;
    }

    /// Use tournament selection to select an organism
    pub fn tournament_select(&self, k: u64) -> Option<&Organism<G>> {
        let mut best: Option<&Organism<G>> = None;
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
}
