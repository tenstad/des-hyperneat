use crate::genome::Evolvable;
use crate::neat::genome::{GenomeComponent, NeatCore};
use crate::neat::state::PopulationState;
use std::fmt;

#[derive(Copy, Clone)]
pub struct NodeCore {
    pub node_ref: NodeRef,
}

impl NodeCore {
    pub fn new(node_ref: NodeRef) -> Self {
        Self { node_ref }
    }
}

impl NeatCore<NodeCore> for NodeCore {
    fn get_neat(&self) -> &Self {
        self
    }

    fn get_neat_mut(&mut self) -> &mut Self {
        self
    }
}

impl GenomeComponent<NodeCore> for NodeCore {
    fn new(node: Self) -> Self {
        node
    }
}

impl Evolvable for NodeCore {
    type PopulationState = PopulationState;

    fn mutate(&mut self, population_state: &mut Self::PopulationState) {}

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        assert_eq!(self.node_ref, other.node_ref);
        NodeCore {
            node_ref: self.node_ref,
        }
    }
}

/// NodeRef refers to node type and ID.
/// The ID is separate for the three types.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeRef {
    Input(usize),
    Hidden(usize),
    Output(usize),
}

impl NodeRef {
    pub fn id(&self) -> usize {
        match self {
            NodeRef::Input(id) => *id,
            NodeRef::Hidden(id) => *id,
            NodeRef::Output(id) => *id,
        }
    }
}

impl fmt::Display for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeRef::Input(id) => write!(f, "I{}", id),
            NodeRef::Hidden(id) => write!(f, "H{}", id),
            NodeRef::Output(id) => write!(f, "O{}", id),
        }
    }
}

impl std::cmp::Ord for NodeRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (NodeRef::Input(a), NodeRef::Input(b)) => a.cmp(b),
            (NodeRef::Output(a), NodeRef::Output(b)) => a.cmp(b),
            (NodeRef::Hidden(a), NodeRef::Hidden(b)) => a.cmp(b),
            (NodeRef::Input(_), NodeRef::Hidden(_)) => std::cmp::Ordering::Less,
            (NodeRef::Input(_), NodeRef::Output(_)) => std::cmp::Ordering::Less,
            (NodeRef::Hidden(_), NodeRef::Output(_)) => std::cmp::Ordering::Less,
            (NodeRef::Hidden(_), NodeRef::Input(_)) => std::cmp::Ordering::Greater,
            (NodeRef::Output(_), NodeRef::Input(_)) => std::cmp::Ordering::Greater,
            (NodeRef::Output(_), NodeRef::Hidden(_)) => std::cmp::Ordering::Greater,
        }
    }
}

impl std::cmp::PartialOrd for NodeRef {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
