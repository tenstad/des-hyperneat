use evolution::neat::state::{StateCore, StateProvider};

#[derive(Default, Clone)]
pub struct State {
    pub core: StateCore,
    pub custom: CustomState,
}

#[derive(Clone, Default)]
pub struct CustomState {
    pub species: usize,
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
