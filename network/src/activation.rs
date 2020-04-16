use rand::Rng;
use std::fmt;
use std::fmt::Display;
use std::str;

#[derive(Copy, Clone, Debug, Display, PartialEq)]
pub enum Activation {
    None,
    ReLU,
    Sigmoid,
    Softmax,
    Normal,
    Sine,
    Square,
    Exp,
}

#[derive(Clone)]
pub struct Activations {
    activations: Vec<Activation>,
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
            }
            Activation::Normal => 0.3989422804 * (-0.5 * x.powi(2)).exp(),
            Activation::Sine => x.sin(),
            Activation::Square => x * x,
            Activation::Exp => x.exp(),
        }
    }
}

#[allow(dead_code)]
impl Activations {
    pub fn iter(&self) -> impl Iterator<Item = &Activation> {
        self.activations.iter()
    }

    pub fn random(&self) -> Activation {
        *self
            .activations
            .iter()
            .skip(rand::thread_rng().gen_range(0, self.activations.len()))
            .next()
            .expect("list of activation functions cannot be empty")
    }
}

#[derive(Debug, Clone)]
pub struct ParseActivationError;

impl fmt::Display for ParseActivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse activation function")
    }
}

impl str::FromStr for Activation {
    type Err = ParseActivationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Activation::None),
            "ReLU" => Ok(Activation::ReLU),
            "Sigmoid" => Ok(Activation::Sigmoid),
            "Softmax" => Ok(Activation::Softmax),
            "Normal" => Ok(Activation::Normal),
            "Sine" => Ok(Activation::Sine),
            "Square" => Ok(Activation::Square),
            "Exp" => Ok(Activation::Exp),
            _ => Err(ParseActivationError {}),
        }
    }
}

impl str::FromStr for Activations {
    type Err = ParseActivationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Activations {
            activations: s
                .trim()
                .split_whitespace()
                .map(|word| Activation::from_str(word).unwrap())
                .collect::<Vec<Activation>>(),
        })
    }
}
