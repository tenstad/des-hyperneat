use crate::sideshyperneat::state::State;
use evolution::neat::{
    genome::Node as NeatNode,
    node::{NodeCore, NodeRef},
};
use rand::Rng;

#[derive(Clone, new)]
pub struct Node {
    pub core: NodeCore,
    pub depth: usize,
    pub cppn_output_id: usize,
}

impl NeatNode for Node {
    type State = State;

    fn new(core: NodeCore, state: &mut Self::State) -> Self {
        let innovation = if let NodeRef::Hidden(id) = core.node_ref {
            state
                .topology_state
                .innovation_log
                .hidden_node_innovations
                .get(&id)
                .unwrap()
                .innovation_number
                + state.output_id_innovation_offset
                + 2
        } else if let Some(innovation) = state.io_output_id.get(&core.node_ref) {
            *innovation
        } else {
            let innovation = state.output_id_innovation_offset;
            state.io_output_id.insert(core.node_ref, innovation);
            state.output_id_innovation_offset += 1;
            innovation
        };

        Self::new(core, 1, innovation)
    }

    fn get_core(&self) -> &NodeCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut NodeCore {
        &mut self.core
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        assert_eq!(self.cppn_output_id, other.cppn_output_id);

        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
            depth: if rand::thread_rng().gen::<bool>() {
                self.depth
            } else {
                other.depth
            },
            cppn_output_id: self.cppn_output_id,
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let mut distance = self.core.distance(&other.core);
        distance += (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
