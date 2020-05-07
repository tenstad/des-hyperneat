use crate::neat::node::NodeRef;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct InnovationLog {
    pub node_additions: HashMap<usize, Innovation>,
    pub edge_additions: HashMap<(NodeRef, NodeRef), usize>,
}

#[derive(Default, Clone, new)]
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

impl StateCore {
    pub fn get_split_innovation(&mut self, link_innovation: usize) -> &Innovation {
        if !self
            .innovation_log
            .node_additions
            .contains_key(&link_innovation)
        {
            // Add a new innovation to log
            self.innovation_log
                .node_additions
                .insert(link_innovation, self.next_innovation.clone());

            // Increase global node count and innovation number
            self.next_innovation.node_number += 1;
            self.next_innovation.innovation_number += 2;
        }

        &self
            .innovation_log
            .node_additions
            .get(&link_innovation)
            .unwrap()
    }

    pub fn get_connect_innovation(&mut self, from: NodeRef, to: NodeRef) -> usize {
        if !self.innovation_log.edge_additions.contains_key(&(from, to)) {
            // Add a new innovation to log
            self.innovation_log
                .edge_additions
                .insert((from, to), self.next_innovation.innovation_number);

            // Increase global innovation number
            self.next_innovation.innovation_number += 1;
        }

        *self.innovation_log.edge_additions.get(&(from, to)).unwrap()
    }
}
