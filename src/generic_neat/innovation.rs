use crate::generic_neat::node;
use std::collections::HashMap;

pub struct InnovationLog {
    pub node_additions: HashMap<u64, InnovationTime>,
    pub edge_additions: HashMap<(node::NodeRef, node::NodeRef), u64>,
}

pub struct InnovationTime {
    pub node_number: u64,
    pub innovation_number: u64,
}

impl InnovationLog {
    pub fn new() -> InnovationLog {
        InnovationLog {
            node_additions: HashMap::new(),
            edge_additions: HashMap::new(),
        }
    }
}

impl InnovationTime {
    pub fn new() -> InnovationTime {
        InnovationTime {
            node_number: 0,
            innovation_number: 0,
        }
    }
}
