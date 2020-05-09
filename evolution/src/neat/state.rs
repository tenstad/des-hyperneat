use crate::neat::node::NodeRef;
use std::collections::HashMap;

#[derive(new)]
pub struct InitConfig {
    pub inputs: usize,
    pub outputs: usize,
}

#[derive(Default, Clone)]
pub struct InnovationLog {
    // Hidden node id -> Innovation
    pub hidden_node_innovations: HashMap<usize, Innovation>,
    // Link split innovation -> Innovation
    pub split_innovations: HashMap<usize, Innovation>,
    // Source and target node -> link connect innovation
    pub connect_innovations: HashMap<(NodeRef, NodeRef), usize>,
    // Link connect innovation -> Source and target node
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
    pub node_state: (),
    pub link_state: (),
}

pub trait StateProvider<N, L>: Clone + Send + Default {
    fn get_core(&self) -> &StateCore;
    fn get_core_mut(&mut self) -> &mut StateCore;
    fn get_node_state(&self) -> &N;
    fn get_node_state_mut(&mut self) -> &mut N;
    fn get_link_state(&self) -> &L;
    fn get_link_state_mut(&mut self) -> &mut L;
}

impl StateProvider<(), ()> for StateCore {
    fn get_core(&self) -> &StateCore {
        self
    }
    fn get_core_mut(&mut self) -> &mut StateCore {
        self
    }
    fn get_node_state(&self) -> &() {
        &self.node_state
    }
    fn get_node_state_mut(&mut self) -> &mut () {
        &mut self.node_state
    }
    fn get_link_state(&self) -> &() {
        &self.link_state
    }
    fn get_link_state_mut(&mut self) -> &mut () {
        &mut self.link_state
    }
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
            // Add hidden node innovation
            self.innovation_log.hidden_node_innovations.insert(
                self.next_innovation.node_number,
                self.next_innovation.clone(),
            );

            // Increase global node count and innovation number
            self.next_innovation.node_number += 1;
            // link soruce - hidden; link hidden - target; hidden node
            self.next_innovation.innovation_number += 3;
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
