use crate::conf;
use network::activation::Activation;
use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Node {
    pub node_ref: NodeRef,
    pub bias: f64,
    pub activation: Activation,
}

impl Node {
    pub fn new(node_ref: NodeRef) -> Node {
        Node {
            node_ref: node_ref,
            bias: 0.0,
            activation: match node_ref {
                NodeRef::Input(_) => Activation::None,
                NodeRef::Hidden(_) => conf::NEAT.hidden_activations.random(),
                NodeRef::Output(_) => conf::NEAT.output_activations.random(),
            },
        }
    }

    pub fn crossover(&self, other: &Self) -> Self {
        assert_eq!(self.node_ref, other.node_ref);

        Node {
            node_ref: self.node_ref,
            bias: (self.bias + other.bias) / 2.0,
            activation: if rand::thread_rng().gen::<bool>() {
                self.activation
            } else {
                other.activation
            },
        }
    }
}

/// NodeRef refers to node type and ID.
/// The ID is separate for the three types.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeRef {
    Input(u64),
    Hidden(u64),
    Output(u64),
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
        match (self, other) {
            (NodeRef::Input(a), NodeRef::Input(b)) => a.partial_cmp(b),
            (NodeRef::Output(a), NodeRef::Output(b)) => a.partial_cmp(b),
            (NodeRef::Hidden(a), NodeRef::Hidden(b)) => a.partial_cmp(b),
            (NodeRef::Input(_), NodeRef::Hidden(_)) => Some(std::cmp::Ordering::Less),
            (NodeRef::Input(_), NodeRef::Output(_)) => Some(std::cmp::Ordering::Less),
            (NodeRef::Hidden(_), NodeRef::Output(_)) => Some(std::cmp::Ordering::Less),
            (NodeRef::Hidden(_), NodeRef::Input(_)) => Some(std::cmp::Ordering::Greater),
            (NodeRef::Output(_), NodeRef::Input(_)) => Some(std::cmp::Ordering::Greater),
            (NodeRef::Output(_), NodeRef::Hidden(_)) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl NodeRef {
    pub fn id(&self) -> u64 {
        match self {
            NodeRef::Input(id) => *id,
            NodeRef::Hidden(id) => *id,
            NodeRef::Output(id) => *id,
        }
    }
}
