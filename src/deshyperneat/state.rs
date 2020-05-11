use evolution::neat::{
    node::NodeRef,
    state::{NeatState, StateProvider},
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct State {
    pub neat: NeatState,
    pub custom: CustomState,
}

#[derive(Clone, Default)]
pub struct CustomState {
    pub single_cppn_state: NeatState,
    pub unique_cppn_states: HashMap<(NodeRef, NodeRef), NeatState>,
    pub cppn_state_redirects: HashMap<(NodeRef, NodeRef), (NodeRef, NodeRef)>,
}

impl StateProvider<CustomState, CustomState> for State {
    fn neat(&self) -> &NeatState {
        &self.neat
    }
    fn neat_mut(&mut self) -> &mut NeatState {
        &mut self.neat
    }
    fn node(&self) -> &CustomState {
        &self.custom
    }
    fn node_mut(&mut self) -> &mut CustomState {
        &mut self.custom
    }
    fn link(&self) -> &CustomState {
        &self.custom
    }
    fn link_mut(&mut self) -> &mut CustomState {
        &mut self.custom
    }
}
