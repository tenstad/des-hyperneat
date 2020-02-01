use crate::data::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
pub struct Dataset {
    pub dimensions: Dimensions,
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
        let buffered = BufReader::new(input);

        let mut inputs: Vec<Vec<f64>> = Vec::new();
        let mut outputs: Vec<Vec<f64>> = Vec::new();

        let mut read_state: bool = false;

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
}
