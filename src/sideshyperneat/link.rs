use crate::sideshyperneat::state::State;
use evolution::neat::{
    genome::{GetCore, Link as NeatLink},
    link::LinkCore,
};
use rand::Rng;

#[derive(Clone, GetCore, new)]
pub struct Link {
    #[core]
    pub core: LinkCore,
    pub depth: usize,
    pub cppn_output_id: usize,
    pub is_identity: bool,
}

impl NeatLink for Link {
    type Config = ();
    type State = State;

    fn new(_: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self {
        let innovation = core.innovation;
        Self::new(
            core,
            1,
            state.output_id_innovation_offset + innovation,
            false,
        )
    }

    fn identity(_: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self {
        let innovation = core.innovation;
        Self::new(
            core,
            1,
            state.output_id_innovation_offset + innovation,
            true,
        )
    }

    fn clone_with(&self, _: &Self::Config, core: LinkCore, _: &mut Self::State) -> Self {
        Self::new(core, self.depth, self.cppn_output_id, self.is_identity)
    }

    fn crossover(
        &self,
        _: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
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

    fn distance(&self, _: &Self::Config, other: &Self) -> f64 {
        let mut distance = self.core.distance(&other.core);
        distance += (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
