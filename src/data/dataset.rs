use crate::data::accuracy;
use crate::data::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
pub struct Dataset {
    pub dimensions: Dimensions,
    pub size: usize,
    pub is_classification: bool,
    pub one_hot_output: bool,
    pub inputs: Vec<Vec<f64>>,
    pub targets: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub struct Dimensions {
    pub inputs: usize,
    pub outputs: usize,
}

impl Dataset {
    pub fn load(filename: &String) -> Result<Dataset, Error> {
        let input = File::open(filename)?;
        let mut buffered = BufReader::new(input);

        let mut inputs: Vec<Vec<f64>> = Vec::new();
        let mut outputs: Vec<Vec<f64>> = Vec::new();

        let mut read_state: bool = false;

        let mut line: String = String::new();
        buffered.read_line(&mut line)?;
        line.retain(|c| !c.is_whitespace());
        let is_classification = line == "true";

        line.clear();
        buffered.read_line(&mut line)?;
        line.retain(|c| !c.is_whitespace());
        let one_hot_encoded = line == "true";

        buffered.read_line(&mut line)?;

        for line in buffered.lines() {
            let mut line: String = line?;
            line.retain(|c| !c.is_whitespace());

            if line.len() == 0 {
                if !read_state {
                    read_state = true;
                    continue;
                } else {
                    break;
                }
            }

            let line: Vec<f64> = line
                .split(|c| c == ',')
                .map(|val| val.parse().unwrap())
                .collect();

            if !read_state {
                inputs.push(line);
            } else {
                outputs.push(line);
            }
        }

        assert_ne!(inputs.len(), 0);
        assert_eq!(inputs.len(), outputs.len());

        Ok(Dataset {
            is_classification: is_classification,
            size: inputs.len(),
            one_hot_output: one_hot_encoded,
            dimensions: Dimensions {
                inputs: inputs[0].len(),
                outputs: outputs[0].len(),
            },
            inputs: inputs,
            targets: outputs,
        })
    }

    pub fn mse(&self, outputs: impl Iterator<Item = Vec<f64>>) -> f64 {
        error::mse(&self.targets, outputs, self.is_classification && self.is_classification)
    }

    pub fn acc(&self, outputs: impl Iterator<Item = Vec<f64>>) -> f64 {
        if !self.is_classification {
            0.0
        } else if self.one_hot_output {
            accuracy::one_hot_accuracy(&self.targets, outputs)
        } else {
            accuracy::rounded_accuracy(&self.targets, outputs)
        }
    }
}
