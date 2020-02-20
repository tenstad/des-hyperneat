use crate::conf;
use crate::generic_neat::organism::Organism;
use rand::Rng;

pub struct Species {
    age: u64,
    pub best_fitness: f64,
    lifetime_best_fitness: f64,
    last_improvement: u64,
    pub offsprings: f64,
    pub organisms: Vec<Organism>,
    locked: bool, // When iter_locked new organisms may be added, but the len() and iter() functions will remain unchanged after addition
    locked_organisms: usize, // The number of locked organisms, this is the length and number of iterated organisms when species is locked
}

impl Species {
    pub fn new() -> Species {
        Species {
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
    pub fn is_compatible(&mut self, organism: &Organism) -> bool {
        if let Some(first_organism) = self.organisms.get(0) {
            first_organism.distance(organism) < conf::NEAT.speciation_threshold
        } else {
            true
        }
    }

    /// Add an organism
    pub fn push(&mut self, individual: Organism) {
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
    pub fn iter(&self) -> impl Iterator<Item = &Organism> {
        self.organisms.iter().take(self.len())
    }

    /// Iterate organisms. Adheres to lock.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Organism> {
        let len = self.len();

        self.organisms.iter_mut().take(len)
    }

    /// Gather a random organism. Adheres to lock.
    pub fn random_organism(&self) -> Option<&Organism> {
        let mut rng = rand::thread_rng();

        self.iter()
            .skip(rng.gen_range(0, self.len()) as usize)
            .next()
    }

    /// Adjust fintesses of all organims
    pub fn adjust_fitness(&mut self) {
        assert!(!self.locked);

        let is_stagnent = self.age - self.last_improvement > conf::NEAT.dropoff_age;
        let is_young = self.age < conf::NEAT.young_age_limit;
        let size: f64 = self.organisms.len() as f64;

        for organism in self.organisms.iter_mut() {
            organism.adjusted_fitness = organism.fitness;

            // Greatly penalize stagnent species
            if is_stagnent {
                organism.adjusted_fitness *= conf::NEAT.stagnent_species_fitness_multiplier;
            }

            // Boost young species
            if is_young {
                organism.adjusted_fitness *= conf::NEAT.young_species_fitness_multiplier;
            }

            // Share fitness within species
            organism.adjusted_fitness /= size;

            // Avoid zero fitness
            if organism.adjusted_fitness <= 0.0 {
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
            (self.organisms.len() as f64 * conf::NEAT.survival_ratio).floor() as usize,
            2, // Keep a minimum of two individuals for sexual reproduction
        ));
    }

    /// Lock the species, so that next generation organisms are not used for reproduction
    pub fn lock(&mut self) {
        assert!(!self.locked);

        self.locked = true;
        self.locked_organisms = self.organisms.len();
    }

    /// Increase age and prepare for addition of new organisms
    pub fn age(&mut self) {
        self.lock();
        self.age += 1;
    }

    /// Remove all the locked organisms (the old organisms), and retain the (next generation) organisms pushed after lock
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
