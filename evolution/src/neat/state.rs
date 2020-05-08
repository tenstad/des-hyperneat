use crate::neat::node::NodeRef;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct InnovationLog {
    pub split_innovations: HashMap<usize, Innovation>,
    pub connect_innovations: HashMap<(NodeRef, NodeRef), usize>,
    pub reverse_connect_innovations: HashMap<usize, (NodeRef, NodeRef)>,
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
            .split_innovations
            .contains_key(&link_innovation)
        {
            let (from, to) = *self
                .innovation_log
                .reverse_connect_innovations
                .get(&link_innovation)
                .unwrap();
            // Add the two connections of this innovation to connection log
            self.innovation_log.connect_innovations.insert(
                (from, NodeRef::Hidden(self.next_innovation.node_number)),
                self.next_innovation.innovation_number,
            );
            self.innovation_log.connect_innovations.insert(
                (NodeRef::Hidden(self.next_innovation.node_number), to),
                self.next_innovation.innovation_number + 1,
            );
            // Add the same two in reverse
            self.innovation_log.reverse_connect_innovations.insert(
                self.next_innovation.innovation_number,
                (from, NodeRef::Hidden(self.next_innovation.node_number)),
            );
            self.innovation_log.reverse_connect_innovations.insert(
                self.next_innovation.innovation_number + 1,
                (NodeRef::Hidden(self.next_innovation.node_number), to),
            );

            // Add a new innovation to log
            self.innovation_log
                .split_innovations
                .insert(link_innovation, self.next_innovation.clone());

            // Increase global node count and innovation number
            self.next_innovation.node_number += 1;
            self.next_innovation.innovation_number += 2;
        }

        &self
            .innovation_log
            .split_innovations
            .get(&link_innovation)
            .unwrap()
    }

    pub fn get_connect_innovation(&mut self, from: NodeRef, to: NodeRef) -> usize {
        if !self
            .innovation_log
            .connect_innovations
            .contains_key(&(from, to))
        {
            // Add a new innovation to log
            self.innovation_log
                .connect_innovations
                .insert((from, to), self.next_innovation.innovation_number);
            self.innovation_log
                .reverse_connect_innovations
                .insert(self.next_innovation.innovation_number, (from, to));

            // Increase global innovation number
            self.next_innovation.innovation_number += 1;
        }

        *self
            .innovation_log
            .connect_innovations
            .get(&(from, to))
            .unwrap()
    }
}
