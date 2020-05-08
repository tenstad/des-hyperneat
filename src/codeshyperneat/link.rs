use crate::codeshyperneat::genome::State;
use evolution::neat::{genome::Link as NeatLink, link::LinkCore};
use rand::Rng;

#[derive(Clone)]
pub struct Link {
    pub core: LinkCore,
    pub module_species: usize,
}

impl NeatLink for Link {
    type State = State;

    fn new(core: LinkCore, state: &mut State) -> Self {
        let mut rng = rand::thread_rng();
        let module_species = if state.species > 0 {
            rng.gen_range(0, state.species)
        } else {
            0
        };

        Self {
            core,
            module_species,
        }
    }

    fn identity(core: LinkCore, state: &mut State) -> Self {
        Self::new(core, state)
    }

    fn clone_with(&self, core: LinkCore, _: &mut State) -> Self {
        let mut clone = self.clone();
        clone.core = core;
        clone
    }

    fn get_core(&self) -> &LinkCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut LinkCore {
        &mut self.core
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
            module_species: if rand::thread_rng().gen::<bool>() {
                self.module_species
            } else {
                other.module_species
            },
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let mut distance = 0.5 * self.core.distance(&other.core);
        distance += 0.5 * ((self.module_species != other.module_species) as u8) as f64;
        distance
    }
}
