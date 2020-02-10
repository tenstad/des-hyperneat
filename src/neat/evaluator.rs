extern crate libc;
use crate::neat::nodes::Activation;
use std::mem;

pub struct Evaluator {
    values: Vec<f64>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    actions: Vec<FastAction>,
}

#[derive(Debug)]
pub enum FastAction {
    Link(usize, usize, f64),            // from, to, weight
    Activation(usize, f64, Activation), // node, bias, activation
}

impl Evaluator {
    pub fn create(
        length: usize,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        actions: Vec<FastAction>,
    ) -> Evaluator {
        Evaluator {
            values: vec![0.0; length],
            inputs,
            outputs,
            actions,
        }
    }

    /// Evaluate network, takes input node values, returns output node values
    pub fn evaluate(&mut self, inputs: &Vec<f64>) -> Vec<f64> {
        /*for i in 0..self.values.len() {
            self.values[i] = 0.0;
        }*/

        // Same as loop above, but this unsafe implementation is faster
        unsafe {
            libc::memset(
                self.values.as_mut_ptr() as _,
                0,
                self.values.len() * mem::size_of::<f64>(),
            );
        }

        // Copy inputs into values
        for (i, index) in self.inputs.iter().enumerate() {
            self.values[i] = inputs[*index];
        }

        // Do forward pass
        for action in self.actions.iter() {
            match action {
                FastAction::Link(from, to, weight) => {
                    self.values[*to] += self.values[*from] * weight;
                }
                FastAction::Activation(node, bias, activation) => {
                    self.values[*node] = activation.activate(self.values[*node] + bias)
                }
            }
        }

        return self.outputs.iter().map(|o| self.values[*o]).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator() {
        let mut actions = Evaluator::create(
            10,
            vec![0, 2, 1],
            vec![7, 8, 9, 6],
            vec![
                FastAction::Link(0, 3, 0.5),
                FastAction::Link(1, 3, 2.0),
                FastAction::Activation(3, 0.5, Activation::None),
                FastAction::Link(3, 7, 2.0),
                FastAction::Activation(7, 1.0, Activation::None),
            ],
        );

        assert_eq!(
            actions.evaluate(&vec![1.0, 2.0, 3.0, 4.0]),
            vec![15.0, 0.0, 0.0, 0.0]
        );
    }
}
