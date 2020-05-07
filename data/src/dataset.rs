use crate::conf::DATA;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

#[derive(Debug)]
pub struct Dataset {
    pub dimensions: Dimensions,
    pub is_classification: bool,
    pub one_hot_output: bool,

    pub training_inputs: Vec<Vec<f64>>,
    pub training_targets: Vec<Vec<f64>>,
    pub validation_inputs: Vec<Vec<f64>>,
    pub validation_targets: Vec<Vec<f64>>,
    pub test_inputs: Vec<Vec<f64>>,
    pub test_targets: Vec<Vec<f64>>,

    pub total_count: usize,
    pub training_count: usize,
    pub validation_count: usize,
    pub test_count: usize,
}

#[derive(Debug)]
pub struct Dimensions {
    pub inputs: usize,
    pub outputs: usize,
}

impl Dataset {
    pub fn load() -> Dataset {
        Self::load_specific(&DATA.dataset)
            .ok()
            .expect("unable to load dataset")
    }

    pub fn load_specific<P: AsRef<Path>>(path: P) -> Result<Dataset, Error> {
        let file = File::open(path)?;
        let mut buffered = BufReader::new(file);

        let mut inputs: Vec<Vec<f64>> = Vec::new();
        let mut targets: Vec<Vec<f64>> = Vec::new();

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
                targets.push(line);
            }
        }

        assert_ne!(inputs.len(), 0);
        assert_eq!(inputs.len(), targets.len());

        inputs.shuffle(&mut StdRng::seed_from_u64(DATA.seed));
        targets.shuffle(&mut StdRng::seed_from_u64(DATA.seed));

        let total_count = inputs.len();
        let validation_count = (total_count as f64 * DATA.validation_fraction).round() as usize;
        let test_count = (total_count as f64 * DATA.test_fraction).round() as usize;
        let training_count = total_count - validation_count - test_count;

        let i = training_count + validation_count;
        let test_inputs = inputs[i..].iter().cloned().collect::<Vec<_>>();
        let test_targets = targets[i..].iter().cloned().collect::<Vec<_>>();
        inputs.truncate(i);
        targets.truncate(i);

        let i = training_count;
        let validation_inputs = inputs[i..].iter().cloned().collect::<Vec<_>>();
        let validation_targets = targets[i..].iter().cloned().collect::<Vec<_>>();
        inputs.truncate(i);
        targets.truncate(i);

        Ok(Dataset {
            dimensions: Dimensions {
                inputs: inputs[0].len(),
                outputs: targets[0].len(),
            },
            is_classification: is_classification,
            one_hot_output: one_hot_encoded,

            training_inputs: inputs,
            training_targets: targets,
            validation_inputs,
            validation_targets,
            test_inputs,
            test_targets,

            total_count,
            training_count,
            validation_count,
            test_count,
        })
    }
}
