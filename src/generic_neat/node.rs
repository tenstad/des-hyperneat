use crate::conf;
use crate::network::activation::Activation;
use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Node<T> {
    pub node_ref: NodeRef,
    pub bias: f64,
    pub activation: Activation,
    pub custom: T,
}

pub trait Custom: Copy + Clone {
    fn new() -> Self;
    fn crossover(&self, other: &Self) -> Self;
}

impl<T: Custom> Node<T> {
    pub fn new(node_ref: NodeRef) -> Node<T> {
        Node {
            node_ref: node_ref,
            bias: 0.0,
            activation: match node_ref {
                NodeRef::Input(_) => Activation::None,
                NodeRef::Hidden(_) => conf::NEAT.hidden_activations.random(),
                NodeRef::Output(_) => conf::NEAT.output_activations.random(),
            },
            custom: T::new(),
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
            custom: self.custom.crossover(&other.custom),
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
        self.id().cmp(&other.id())
    }
}

impl std::cmp::PartialOrd for NodeRef {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
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
