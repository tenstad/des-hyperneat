use crate::sideshyperneat::state::State;
use evolution::neat::{
    genome::GetNeat,
    node::{NeatNode, NodeExtension, NodeRef},
};
use rand::Rng;

#[derive(Clone, GetNeat, new)]
pub struct Node {
    #[neat]
    pub neat: NeatNode,
    pub depth: u64,
    pub cppn_output_id: u64,
}

impl NodeExtension for Node {
    type Config = ();
    type State = State;

    fn new(_: &Self::Config, neat: NeatNode, state: &mut Self::State) -> Self {
        let innovation = if let NodeRef::Hidden(id) = neat.node_ref {
            state
                .topology_state
                .innovation_log
                .hidden_node_innovations
                .get(&id)
                .unwrap()
                .innovation_number
                + state.output_id_innovation_offset
                + 2
        } else if let Some(innovation) = state.io_output_id.get(&neat.node_ref) {
            *innovation
        } else {
            let innovation = state.output_id_innovation_offset;
            state.io_output_id.insert(neat.node_ref, innovation);
            state.output_id_innovation_offset += 1;
            innovation
        };

        Self::new(neat, 1, innovation)
    }

    fn crossover(
        &self,
        _: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        assert_eq!(self.cppn_output_id, other.cppn_output_id);

        Self {
            neat: self.neat.crossover(&other.neat, fitness, other_fitness),
            depth: if rand::thread_rng().gen::<bool>() {
                self.depth
            } else {
                other.depth
            },
            cppn_output_id: self.cppn_output_id,
        }
    }

    fn distance(&self, _: &Self::Config, other: &Self) -> f64 {
        let mut distance = self.neat.distance(&other.neat);
        distance += (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
