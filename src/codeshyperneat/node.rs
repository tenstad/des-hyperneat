use crate::codeshyperneat::state::CustomState;
use crate::deshyperneat::conf::DESHYPERNEAT;
use evolution::neat::{
    genome::GetNeat,
    node::{NeatNode, NodeExtension, NodeRef},
};
use rand::Rng;

#[derive(Clone, GetNeat)]
pub struct Node {
    #[neat]
    pub neat: NeatNode,
    pub module_species: u64,
    pub depth: u64,
}

impl NodeExtension for Node {
    type Config = ();
    type State = CustomState;

    fn new(_: &Self::Config, neat: NeatNode, state: &mut Self::State) -> Self {
        let mut rng = rand::thread_rng();
        let module_species = if state.species > 0 {
            rng.gen_range(0, state.species)
        } else {
            0
        };

        let depth = 1
            .min(match neat.neat().node_ref {
                NodeRef::Input(_) => DESHYPERNEAT.max_input_substrate_depth,
                NodeRef::Hidden(_) => DESHYPERNEAT.max_hidden_substrate_depth,
                NodeRef::Output(_) => DESHYPERNEAT.max_output_substrate_depth,
            })
            .max(0);

        Self {
            neat,
            module_species,
            depth,
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
            depth: if rand::thread_rng().gen::<bool>() {
                self.depth
            } else {
                other.depth
            },
        }
    }

    fn distance(&self, _: &Self::Config, other: &Self) -> f64 {
        let mut distance = self.neat.distance(&other.neat);
        distance += 0.5 * ((self.module_species != other.module_species) as u8) as f64;
        distance += 0.5 * (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
