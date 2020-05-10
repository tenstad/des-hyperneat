use evolution::neat::{
    node::NodeRef,
    state::{StateCore, StateProvider},
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct State {
    pub cppn_state: StateCore,
    pub topology_state: StateCore,
    pub output_id_innovation_offset: usize,
    pub io_output_id: HashMap<NodeRef, usize>,
}

impl StateProvider<(), ()> for State {
    fn get_core(&self) -> &StateCore {
        &self.cppn_state
    }
    fn get_core_mut(&mut self) -> &mut StateCore {
        &mut self.cppn_state
    }
    fn get_node_state(&self) -> &() {
        &self.cppn_state.node_state
    }
    fn get_node_state_mut(&mut self) -> &mut () {
        &mut self.cppn_state.node_state
    }
    fn get_link_state(&self) -> &() {
        &self.cppn_state.link_state
    }
    fn get_link_state_mut(&mut self) -> &mut () {
        &mut self.cppn_state.link_state
    }
}

impl StateProvider<State, State> for State {
    fn get_core(&self) -> &StateCore {
        &self.topology_state
    }
    fn get_core_mut(&mut self) -> &mut StateCore {
        &mut self.topology_state
    }
    fn get_node_state(&self) -> &State {
        self
    }
    fn get_node_state_mut(&mut self) -> &mut State {
        self
    }
    fn get_link_state(&self) -> &State {
        self
    }
    fn get_link_state_mut(&mut self) -> &mut State {
        self
    }
}
