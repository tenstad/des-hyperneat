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
pub struct PopulationState {
    pub innovation_log: InnovationLog,
    pub next_innovation: Innovation,
}

pub trait NeatStateProvider: Default + Clone + Send {
    fn get_neat(&self) -> &PopulationState;
    fn get_neat_mut(&mut self) -> &mut PopulationState;
}

impl NeatStateProvider for PopulationState {
    fn get_neat(&self) -> &PopulationState {
        self
    }
    fn get_neat_mut(&mut self) -> &mut PopulationState {
        self
    }
}

#[derive(new)]
pub struct InitConfig {
    pub inputs: usize,
    pub outputs: usize,
}
