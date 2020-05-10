use crate::sideshyperneat::state::State;
use evolution::neat::{genome::Link as NeatLink, link::LinkCore};
use rand::Rng;

#[derive(Clone, new)]
pub struct Link {
    pub core: LinkCore,
    pub depth: usize,
    pub cppn_output_id: usize,
    pub is_identity: bool,
}

impl NeatLink for Link {
    type State = State;

    fn new(core: LinkCore, state: &mut Self::State) -> Self {
        let innovation = core.innovation;
        Self::new(
            core,
            1,
            state.output_id_innovation_offset + innovation,
            false,
        )
    }

    fn identity(core: LinkCore, state: &mut Self::State) -> Self {
        let innovation = core.innovation;
        Self::new(
            core,
            1,
            state.output_id_innovation_offset + innovation,
            true,
        )
    }

    fn clone_with(&self, core: LinkCore, _: &mut Self::State) -> Self {
        Self::new(core, self.depth, self.cppn_output_id, self.is_identity)
    }

    fn get_core(&self) -> &LinkCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut LinkCore {
        &mut self.core
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        assert_eq!(self.cppn_output_id, other.cppn_output_id);
        assert_eq!(self.is_identity, other.is_identity);

        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
            depth: if rand::thread_rng().gen::<bool>() {
                self.depth
            } else {
                other.depth
            },
            cppn_output_id: self.cppn_output_id,
            is_identity: self.is_identity,
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let mut distance = self.core.distance(&other.core);
        distance += (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
