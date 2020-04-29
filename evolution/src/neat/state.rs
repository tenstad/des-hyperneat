use crate::neat::node;
use std::collections::HashMap;

#[derive(Default)]
pub struct InnovationLog {
    pub node_additions: HashMap<usize, Innovation>,
    pub edge_additions: HashMap<(node::NodeRef, node::NodeRef), usize>,
}

#[derive(Default)]
pub struct Innovation {
    pub node_number: usize,
    pub innovation_number: usize,
}

#[derive(Default)]
pub struct PopulationState {
    pub innovation_log: InnovationLog,
    pub next_innovation: Innovation,
}

pub trait NeatStateProvider: Default {
    fn get_neat(&mut self) -> &mut PopulationState;
}

impl NeatStateProvider for PopulationState {
    fn get_neat(&mut self) -> &mut PopulationState {
        self
    }
}

#[derive(new)]
pub struct InitConfig {
    pub inputs: usize,
    pub outputs: usize,
}
