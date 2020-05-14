use crate::sideshyperneat::state::State;
use evolution::neat::{
    genome::GetNeat,
    link::{LinkExtension, NeatLink},
};
use rand::Rng;

#[derive(Clone, GetNeat, new)]
pub struct Link {
    #[neat]
    pub neat: NeatLink,
    pub depth: u64,
    pub cppn_output_id: u64,
    pub is_identity: bool,
}

impl LinkExtension for Link {
    type Config = ();
    type State = State;

    fn new(_: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self {
        let innovation = neat.innovation;
        Self::new(
            neat,
            1,
            state.output_id_innovation_offset + innovation,
            false,
        )
    }

    fn identity(_: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self {
        let innovation = neat.innovation;
        Self::new(
            neat,
            1,
            state.output_id_innovation_offset + innovation,
            true,
        )
    }

    fn clone_with(&self, _: &Self::Config, neat: NeatLink, _: &mut Self::State) -> Self {
        Self::new(neat, self.depth, self.cppn_output_id, self.is_identity)
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
            neat: self.neat.crossover(&other.neat, fitness, other_fitness),
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
        let mut distance = self.neat.distance(&other.neat);
        distance += (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
