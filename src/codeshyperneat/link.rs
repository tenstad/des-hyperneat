use crate::codeshyperneat::state::CustomState;
use evolution::neat::{
    genome::{GetCore, Link as NeatLink},
    link::LinkCore,
};
use rand::Rng;

#[derive(Clone, GetCore)]
pub struct Link {
    #[core]
    pub core: LinkCore,
    pub module_species: usize,
}

impl NeatLink for Link {
    type Config = ();
    type State = CustomState;

    fn new(_: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self {
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

    fn identity(config: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self {
        Self::new(config, core, state)
    }

    fn clone_with(&self, _: &Self::Config, core: LinkCore, _: &mut Self::State) -> Self {
        Self {
            core,
            module_species: self.module_species,
        }
    }

    fn crossover(
        &self,
        _: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
            module_species: if rand::thread_rng().gen::<bool>() {
                self.module_species
            } else {
                other.module_species
            },
        }
    }

    fn distance(&self, _: &Self::Config, other: &Self) -> f64 {
        let mut distance = 0.5 * self.core.distance(&other.core);
        distance += 0.5 * ((self.module_species != other.module_species) as u8) as f64;
        distance
    }
}
