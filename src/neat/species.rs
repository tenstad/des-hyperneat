use crate::conf;
use crate::neat::organism::Organism;
use rand::Rng;

pub struct Species {
    pub age: u64,
    pub best_fitness: f64,
    pub lifetime_best_fitness: f64,
    pub last_improvement: u64,
    pub offsprings: f64,
    iter_locked: bool,
    locked_organisms: usize,
    pub organisms: Vec<Organism>,
}

impl Species {
    pub fn new() -> Species {
        Species {
            age: 0,
            best_fitness: 0.0,
            lifetime_best_fitness: 0.0,
            last_improvement: 0,
            offsprings: 0.0,
            iter_locked: false,
            locked_organisms: 0,
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

    pub fn len(&self) -> usize {
        if self.iter_locked {
            self.locked_organisms
        } else {
            self.organisms.len()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Organism> {
        self.organisms.iter().take(self.len())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Organism> {
        let len = self.len();

        self.organisms.iter_mut().take(len)
    }

    pub fn random_organism(&self) -> Option<&Organism> {
        let mut rng = rand::thread_rng();

        self.iter()
            .skip(rng.gen_range(0, self.len()) as usize)
            .next()
    }

    pub fn adjust_fitness(&mut self) {
        let is_stagnent = self.age - self.last_improvement > conf::NEAT.dropoff_age;
        let is_young = self.age < 10;
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

            if organism.adjusted_fitness <= 0.0 {
                organism.adjusted_fitness = 0.0001;
            }
        }

        // Sort organisms by descending adjusted fitness
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
    pub fn truncate(&mut self) {
        // Assumes the individuals are sorted in descending fitness order
        self.organisms.truncate(std::cmp::max(
            (self.organisms.len() as f64 * conf::NEAT.survival_ratio).floor() as usize,
            2, // Keep a minimum of two individuals for sexual reproduction
        ));
    }

    pub fn lock(&mut self) {
        self.iter_locked = true;
        self.locked_organisms = self.organisms.len();
    }

    pub fn age(&mut self) {
        self.lock();
        self.age += 1;
    }

    pub fn remove_old(&mut self) {
        self.organisms = self.organisms.split_off(self.locked_organisms);
        self.iter_locked = false;
    }

    pub fn calculate_offsprings(&mut self, avg_fitness: f64) {
        self.offsprings = self
            .organisms
            .iter()
            .map(|organism| organism.adjusted_fitness / avg_fitness)
            .sum();
    }
}
