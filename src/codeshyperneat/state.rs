use evolution::neat::state::{NeatState, StateProvider};

#[derive(Default, Clone)]
pub struct State {
    pub neat: NeatState,
    pub custom: CustomState,
}

#[derive(Clone, Default)]
pub struct CustomState {
    pub species: usize,
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
