use rand::Rng;
use std::fmt::Display;
use std::fmt;

#[derive(Copy, Clone)]
pub struct InputNode {
    pub id: u64, // ID of node. Global for all input nodes
}

impl InputNode {
    pub fn new(id: u64) -> InputNode {
        InputNode { id: id }
    }
}

#[derive(Copy, Clone)]
pub struct HiddenNode {
    pub id: u64, // ID of node. Global for all hidden nodes
    pub bias: f64,
    pub activation: Activation, // Activation function of node
}

impl HiddenNode {
    pub fn new(id: u64) -> HiddenNode {
        HiddenNode {
            id: id,
            bias: 0.0,
            activation: Activation::ReLU,
        }
    }
}

#[derive(Copy, Clone)]
pub struct OutputNode {
    pub id: u64, // ID of node. Global for all output nodes
    pub bias: f64,
    pub activation: Activation, // Activation function of node
}

impl OutputNode {
    pub fn new(id: u64) -> OutputNode {
        OutputNode {
            id: id,
            bias: 0.0,
            activation: Activation::Softmax,
        }
    }
}

pub trait Node {
    fn get_ref(&self) -> NodeRef;
    fn get_activation(&self) -> Option<Activation>;
    fn get_bias(&self) -> Option<f64>;
    fn crossover(&self, other: &Self) -> Self;
}

impl Node for InputNode {
    fn get_ref(&self) -> NodeRef {
        return NodeRef::Input(self.id);
    }

    fn get_activation(&self) -> Option<Activation> {
        None
    }

    fn get_bias(&self) -> Option<f64> {
        None
    }

    fn crossover(&self, other: &Self) -> Self {
        assert_eq!(self.id, other.id);

        InputNode { id: self.id }
    }
}

impl Node for HiddenNode {
    fn get_ref(&self) -> NodeRef {
        return NodeRef::Hidden(self.id);
    }

    fn get_activation(&self) -> Option<Activation> {
        Some(self.activation)
    }

    fn get_bias(&self) -> Option<f64> {
        Some(self.bias)
    }

    fn crossover(&self, other: &Self) -> Self {
        assert_eq!(self.id, other.id);

        HiddenNode {
            id: self.id,
            bias: (self.bias + other.bias) / 2.0,
            activation: if rand::thread_rng().gen::<bool>() {
                self.activation
            } else {
                other.activation
            },
        }
    }
}

impl Node for OutputNode {
    fn get_ref(&self) -> NodeRef {
        return NodeRef::Output(self.id);
    }

    fn get_activation(&self) -> Option<Activation> {
        Some(self.activation)
    }

    fn get_bias(&self) -> Option<f64> {
        Some(self.bias)
    }

    fn crossover(&self, other: &Self) -> Self {
        assert_eq!(self.id, other.id);

        OutputNode {
            id: self.id,
            bias: (self.bias + other.bias) / 2.0,
            activation: if rand::thread_rng().gen::<bool>() {
                self.activation
            } else {
                other.activation
            },
        }
    }
}

/// NodeRef refers to node type (Input, Hidden, Output) and ID
/// The ID is separate for the three types, to allow for increase of both input and output nodes during evolution
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeRef {
    Input(u64),
    Hidden(u64),
    Output(u64),
}

impl fmt::Display for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeRef::Input(id) => write!(f, "i{}", id),
            NodeRef::Hidden(id) => write!(f, "h{}", id),
            NodeRef::Output(id) => write!(f, "o{}", id),
        }
    }
}

impl NodeRef {
    pub fn get_id(&self) -> u64 {
        match self {
            NodeRef::Input(id) => *id,
            NodeRef::Hidden(id) => *id,
            NodeRef::Output(id) => *id,
        }
    }
}

#[derive(Copy, Clone, Debug, Display, PartialEq)]
pub enum Activation {
    None,
    ReLU,
    Sigmoid,
    Softmax,
}

impl Activation {
    pub fn activate(&self, x: f64) -> f64 {
        match self {
            Activation::None => x,
            Activation::ReLU => {
                if x > 0.0 {
                    x
                } else {
                    0.0
                }
            }
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Softmax => {
                let v = x.exp();

                if v.is_infinite() {
                    10000.0
                } else {
                    v
                }
            },
        }
    }
}
