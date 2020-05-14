use evolution::neat::{
    node::NodeRef,
    state::{NeatState, StateProvider},
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct State {
    pub cppn_state: NeatState,
    pub topology_state: NeatState,
    pub output_id_innovation_offset: u64,
    pub io_output_id: HashMap<NodeRef, u64>,
}

impl StateProvider<(), ()> for State {
    fn neat(&self) -> &NeatState {
        &self.cppn_state
    }
    fn neat_mut(&mut self) -> &mut NeatState {
        &mut self.cppn_state
    }
    fn node(&self) -> &() {
        &self.cppn_state.node_state
    }
    fn node_mut(&mut self) -> &mut () {
        &mut self.cppn_state.node_state
    }
    fn link(&self) -> &() {
        &self.cppn_state.link_state
    }
    fn link_mut(&mut self) -> &mut () {
        &mut self.cppn_state.link_state
    }
}

impl StateProvider<State, State> for State {
    fn neat(&self) -> &NeatState {
        &self.topology_state
    }
    fn neat_mut(&mut self) -> &mut NeatState {
        &mut self.topology_state
    }
    fn node(&self) -> &State {
        self
    }
    fn node_mut(&mut self) -> &mut State {
        self
    }
    fn link(&self) -> &State {
        self
    }
    fn link_mut(&mut self) -> &mut State {
        self
    }
}
