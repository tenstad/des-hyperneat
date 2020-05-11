use crate::codeshyperneat::state::CustomState;
use evolution::neat::{
    genome::GetNeat,
    link::{LinkExtension, NeatLink},
};
use rand::Rng;

#[derive(Clone, GetNeat)]
pub struct Link {
    #[neat]
    pub neat: NeatLink,
    pub module_species: usize,
}

impl LinkExtension for Link {
    type Config = ();
    type State = CustomState;

    fn new(_: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self {
        let mut rng = rand::thread_rng();
        let module_species = if state.species > 0 {
            rng.gen_range(0, state.species)
        } else {
            0
        };

        Self {
            neat,
            module_species,
        }
    }

    fn identity(config: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self {
        Self::new(config, neat, state)
    }

    fn clone_with(&self, _: &Self::Config, neat: NeatLink, _: &mut Self::State) -> Self {
        Self {
            neat,
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
            neat: self.neat.crossover(&other.neat, fitness, other_fitness),
            module_species: if rand::thread_rng().gen::<bool>() {
                self.module_species
            } else {
                other.module_species
            },
        }
    }

    fn distance(&self, _: &Self::Config, other: &Self) -> f64 {
        let mut distance = 0.5 * self.neat.distance(&other.neat);
        distance += 0.5 * ((self.module_species != other.module_species) as u8) as f64;
        distance
    }
}
