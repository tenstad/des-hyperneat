use evolution::neat::{
    node::NodeRef,
    state::{StateCore, StateProvider},
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct State {
    pub core: StateCore,
    pub custom: CustomState,
}

#[derive(Clone, Default)]
pub struct CustomState {
    pub single_cppn_state: StateCore,
    pub unique_cppn_states: HashMap<(NodeRef, NodeRef), StateCore>,
    pub cppn_state_redirects: HashMap<(NodeRef, NodeRef), (NodeRef, NodeRef)>,
}

impl StateProvider<CustomState, CustomState> for State {
    fn get_core(&self) -> &StateCore {
        &self.core
    }
    fn get_core_mut(&mut self) -> &mut StateCore {
        &mut self.core
    }
    fn get_node_state(&self) -> &CustomState {
        &self.custom
    }
    fn get_node_state_mut(&mut self) -> &mut CustomState {
        &mut self.custom
    }
    fn get_link_state(&self) -> &CustomState {
        &self.custom
    }
    fn get_link_state_mut(&mut self) -> &mut CustomState {
        &mut self.custom
    }
}
