extern crate libc;
use crate::network::activation::Activation;
use std::mem;

#[derive(Clone)]
pub struct Evaluator {
    values: Vec<f64>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    actions: Vec<Action>,
}

#[derive(Clone, Debug)]
pub enum Action {
    Link(usize, usize, f64),            // from, to, weight
    Activation(usize, f64, Activation), // node, bias, activation
}

impl Evaluator {
    pub fn create(
        length: usize,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        actions: Vec<Action>,
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

        // Clear network
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
                Action::Link(from, to, weight) => {
                    self.values[*to] += self.values[*from] * weight;
                }
                Action::Activation(node, bias, activation) => {
                    self.values[*node] = activation.activate(self.values[*node] + bias)
                }
            }
        }

        // Collect output
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
                Action::Link(0, 3, 0.5),
                Action::Link(1, 3, 2.0),
                Action::Activation(3, 0.5, Activation::None),
                Action::Link(3, 7, 2.0),
                Action::Activation(7, 1.0, Activation::None),
            ],
        );

        assert_eq!(
            actions.evaluate(&vec![1.0, 2.0, 3.0, 4.0]),
            vec![15.0, 0.0, 0.0, 0.0]
        );

        // Data from last evaluation should not impact next evaluation
        assert_eq!(
            actions.evaluate(&vec![1.0, 2.0, 3.0, 4.0]),
            vec![15.0, 0.0, 0.0, 0.0]
        );
    }
}
