use crate::codeshyperneat::state::CustomState;
use evolution::neat::{genome::Node as NeatNode, node::NodeCore};
use rand::Rng;

#[derive(Clone)]
pub struct Node {
    pub core: NodeCore,
    pub module_species: usize,
    pub depth: usize,
}

impl NeatNode for Node {
    type State = CustomState;

    fn new(core: NodeCore, state: &mut Self::State) -> Self {
        let mut rng = rand::thread_rng();
        let module_species = if state.species > 0 {
            rng.gen_range(0, state.species)
        } else {
            0
        };

        Self {
            core,
            module_species,
            depth: 1,
        }
    }

    fn get_core(&self) -> &NodeCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut NodeCore {
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
            depth: if rand::thread_rng().gen::<bool>() {
                self.depth
            } else {
                other.depth
            },
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let mut distance = self.core.distance(&other.core);
        distance += 0.5 * ((self.module_species != other.module_species) as u8) as f64;
        distance += 0.5 * (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
