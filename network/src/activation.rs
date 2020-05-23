use rand::Rng;
use serde::Serialize;
use std::{
    fmt::{self, Display},
    str,
};

#[derive(Copy, Clone, Debug, Display, PartialEq, Serialize)]
pub enum Activation {
    None,
    Linear,
    Step,
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
    Gaussian,
    OffsetGaussian,
    Sine,
    Cos,
    Square,
    Abs,
    Exp,
}

#[derive(Clone, new, Serialize)]
pub struct Activations {
    activations: Vec<Activation>,
}

impl Activation {
    pub fn activate(&self, x: f64) -> f64 {
        match self {
            Activation::None => x,
            Activation::Linear => x.min(1.0).max(-1.0),
            Activation::Step => ((x > 0.0) as u8) as f64,
            Activation::ReLU => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
            Activation::Softmax => x.exp(),
            Activation::Gaussian => (-(2.5 * x).powi(2)).exp(),
            Activation::OffsetGaussian => 2.0 * (-(2.5 * x).powi(2)).exp() - 1.0,
            Activation::Sine => (2.0 * x).sin(),
            Activation::Cos => (2.0 * x).cos(),
            Activation::Square => x * x,
            Activation::Abs => x.abs(),
            Activation::Exp => x.min(1.0).exp(),
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
            "Linear" => Ok(Activation::Linear),
            "Step" => Ok(Activation::Step),
            "ReLU" => Ok(Activation::ReLU),
            "Sigmoid" => Ok(Activation::Sigmoid),
            "Tanh" => Ok(Activation::Tanh),
            "Softmax" => Ok(Activation::Softmax),
            "Gaussian" => Ok(Activation::Gaussian),
            "OffsetGaussian" => Ok(Activation::OffsetGaussian),
            "Sine" => Ok(Activation::Sine),
            "Cos" => Ok(Activation::Cos),
            "Square" => Ok(Activation::Square),
            "Abs" => Ok(Activation::Abs),
            "Exp" => Ok(Activation::Exp),
            _ => Err(ParseActivationError {}),
        }
    }
}

impl str::FromStr for Activations {
    type Err = ParseActivationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "All" {
            Ok(Activations::new(vec![
                Activation::None,
                Activation::Linear,
                Activation::Step,
                Activation::ReLU,
                Activation::Sigmoid,
                Activation::Tanh,
                Activation::Softmax,
                Activation::Gaussian,
                Activation::OffsetGaussian,
                Activation::Sine,
                Activation::Cos,
                Activation::Square,
                Activation::Abs,
                Activation::Exp,
            ]))
        } else {
            Ok(Activations::new(
                s.trim()
                    .split_whitespace()
                    .map(|word| Activation::from_str(word).unwrap())
                    .collect::<Vec<Activation>>(),
            ))
        }
    }
}
