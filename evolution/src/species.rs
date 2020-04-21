use crate::conf::EVOLUTION;
use crate::genome::Genome;
use crate::organism::Organism;
use rand::Rng;

/// Collection of similar organisms
// The lock is used to add new organisms without affecting the reproduction of the previous generation.
// It is unlocked after reproduction, which will remove the previous generation and keep the new.
pub struct Species<G> {
    age: u64,
    pub best_fitness: f64,
    lifetime_best_fitness: f64,
    last_improvement: u64,
    pub offsprings: f64,
    pub organisms: Vec<Organism<G>>,
    locked: bool, // When locked new organisms may be added, but the len() and iter() functions will remain unchanged after addition
    locked_organisms: usize, // The number of locked organisms, this is the length and number of iterated organisms when species is locked
}

impl<G: Genome> Species<G> {
    pub fn new() -> Self {
        Self {
            age: 0,
            best_fitness: 0.0,
            lifetime_best_fitness: 0.0,
            last_improvement: 0,
            offsprings: 0.0,
            locked: false,
            locked_organisms: 0,
            organisms: Vec::new(),
        }
    }

    /// Determine wether a new organism is compatible
    pub fn is_compatible(&mut self, other: &Organism<G>) -> bool {
        if let Some(organism) = self.organisms.first() {
            organism.distance(other) < EVOLUTION.speciation_threshold
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

    /// Get a random organism. Adheres to lock.
    pub fn random_organism(&self) -> Option<&Organism<G>> {
        self.iter()
            .skip(rand::thread_rng().gen_range(0, self.len()))
            .next()
    }

    /// Adjust fintesses of all organims
    pub fn adjust_fitness(&mut self) {
        assert!(!self.locked);

        let is_stagnent = self.age - self.last_improvement > EVOLUTION.dropoff_age;
        let is_young = self.age < EVOLUTION.young_age_limit;
        let size: f64 = self.organisms.len() as f64;

        for organism in self.organisms.iter_mut() {
            organism.adjusted_fitness = organism.fitness;

            // Greatly penalize stagnent species
            if is_stagnent {
                organism.adjusted_fitness *= EVOLUTION.stagnent_species_fitness_multiplier;
            }

            // Boost young species
            if is_young {
                organism.adjusted_fitness *= EVOLUTION.young_species_fitness_multiplier;
            }

            // Share fitness within species
            organism.adjusted_fitness /= size;

            // Avoid zero fitness
            if organism.adjusted_fitness <= 0.0 || !organism.adjusted_fitness.is_finite() {
                organism.adjusted_fitness = 0.0001;
            }
        }

        // Sort organisms descendingly by adjusted fitness
        self.organisms
            .sort_by(|a, b| b.adjusted_fitness.partial_cmp(&a.adjusted_fitness).unwrap());

        // Update best fitness and last improvement if currently best in lifetime
        self.best_fitness = self
            .organisms
            .first()
            .map(|organism| organism.fitness)
            .unwrap_or(0.0);
        if self.best_fitness > self.lifetime_best_fitness {
            self.lifetime_best_fitness = self.best_fitness;
            self.last_improvement = self.age;
        }
    }

    /// Retain only the best individuals
    pub fn retain_best(&mut self) {
        assert!(!self.locked);

        // Assumes the individuals are sorted in descending fitness order
        self.organisms.truncate(std::cmp::max(
            (self.organisms.len() as f64 * EVOLUTION.survival_ratio).floor() as usize,
            2, // Keep a minimum of two individuals for sexual reproduction
        ));
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
        self.organisms = self.organisms.split_off(self.locked_organisms);
        self.locked = false;
    }

    /// Calculate number of offsprings based on adjusted fitness of organisms
    pub fn calculate_offsprings(&mut self, avg_fitness: f64) {
        assert!(!self.locked);

        self.offsprings = self
            .organisms
            .iter()
            .map(|organism| organism.adjusted_fitness / avg_fitness)
            .sum();
    }
}
