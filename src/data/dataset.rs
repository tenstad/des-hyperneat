use crate::data::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
pub struct Dataset {
    pub dimensions: Dimensions,
    is_classification: bool,
    one_hot_output: bool,
    pub inputs: Vec<Vec<f64>>,
    pub targets: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub struct Dimensions {
    pub inputs: u64,
    pub outputs: u64,
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
            one_hot_output: one_hot_encoded,
            dimensions: Dimensions {
                inputs: inputs[0].len() as u64,
                outputs: outputs[0].len() as u64,
            },
            inputs: inputs,
            targets: outputs,
        })
    }

    pub fn mse(&self, outputs: &Vec<Vec<f64>>) -> f64 {
        self.targets
            .iter()
            .zip(outputs.iter())
            .map(|(t, o)| error::mse(t, o))
            .sum::<f64>()
            / self.targets.len() as f64
    }

    pub fn acc(&self, outputs: &Vec<Vec<f64>>) -> f64 {
        if !self.is_classification {
            return 0.0;
        }

        if self.one_hot_output {
            self.targets
                .iter()
                .zip(outputs.iter())
                .map(|(t, o)| if argmax(t) == argmax(o) { 1.0 } else { 0.0 })
                .sum::<f64>()
                / self.targets.len() as f64
        } else {
            self.targets
                .iter()
                .zip(outputs.iter())
                .map(|(t, o)| {
                    t.iter()
                        .zip(o.iter())
                        .map(|(t, o)| if t.round() == o.round() { 1.0 } else { 0.0 })
                        .sum::<f64>()
                })
                .sum::<f64>()
                / self.targets.len() as f64
        }
    }
}

pub fn argmax(vec: &Vec<f64>) -> usize {
    let mut max_i: usize = 0;
    let mut max_v: f64 = -100000.0;

    for (i, &v) in vec.iter().enumerate() {
        if v > max_v {
            max_i = i;
            max_v = v;
        }
    }

    return max_i;
}
