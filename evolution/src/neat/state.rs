use crate::neat::node::NodeRef;
use std::collections::HashMap;

#[derive(new)]
pub struct InitConfig {
    pub inputs: u64,
    pub outputs: u64,
}

#[derive(Default, Clone)]
pub struct InnovationLog {
    // Hidden node id -> Innovation
    pub hidden_node_innovations: HashMap<u64, Innovation>,
    // Link split innovation -> Innovation
    pub split_innovations: HashMap<u64, Innovation>,
    // Source and target node -> link connect innovation
    pub connect_innovations: HashMap<(NodeRef, NodeRef), u64>,
    // Link connect innovation -> Source and target node
    pub reverse_connect_innovations: HashMap<u64, (NodeRef, NodeRef)>,
}

#[derive(Default, Clone, new)]
pub struct Innovation {
    pub node_number: u64,
    pub innovation_number: u64,
}

#[derive(Default, Clone)]
pub struct NeatState {
    pub innovation_log: InnovationLog,
    pub next_innovation: Innovation,
    pub node_state: (),
    pub link_state: (),
}

pub trait StateProvider<N, L>: Clone + Send + Default {
    fn neat(&self) -> &NeatState;
    fn neat_mut(&mut self) -> &mut NeatState;
    fn node(&self) -> &N;
    fn node_mut(&mut self) -> &mut N;
    fn link(&self) -> &L;
    fn link_mut(&mut self) -> &mut L;
}

impl StateProvider<(), ()> for NeatState {
    fn neat(&self) -> &NeatState {
        self
    }
    fn neat_mut(&mut self) -> &mut NeatState {
        self
    }
    fn node(&self) -> &() {
        &self.node_state
    }
    fn node_mut(&mut self) -> &mut () {
        &mut self.node_state
    }
    fn link(&self) -> &() {
        &self.link_state
    }
    fn link_mut(&mut self) -> &mut () {
        &mut self.link_state
    }
}

impl NeatState {
    pub fn get_split_innovation(&mut self, link_innovation: u64) -> &Innovation {
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

    pub fn get_connect_innovation(&mut self, from: NodeRef, to: NodeRef) -> u64 {
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
