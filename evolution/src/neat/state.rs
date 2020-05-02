use crate::neat::node;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct InnovationLog {
    pub node_additions: HashMap<usize, Innovation>,
    pub edge_additions: HashMap<(node::NodeRef, node::NodeRef), usize>,
}

#[derive(Default, Clone)]
pub struct Innovation {
    pub node_number: usize,
    pub innovation_number: usize,
}

#[derive(Default, Clone)]
pub struct StateCore {
    pub innovation_log: InnovationLog,
    pub next_innovation: Innovation,
}

pub trait NeatStateProvider: Default + Clone + Send {
    fn get_core(&self) -> &StateCore;
    fn get_core_mut(&mut self) -> &mut StateCore;
}

impl NeatStateProvider for StateCore {
    fn get_core(&self) -> &StateCore {
        self
    }
    fn get_core_mut(&mut self) -> &mut StateCore {
        self
    }
}

#[derive(new)]
pub struct InitConfig {
    pub inputs: usize,
    pub outputs: usize,
}
