use crate::conf;
use crate::data::accuracy;
use crate::data::dataset::Dataset;
use crate::data::dataset::Dimensions;
use crate::data::error;
use crate::generic_neat::environment::Environment;
use crate::network::evaluate;
use std::fmt;
use std::io::Error;
use std::thread;

pub struct DatasetEnvironment {
    name: String,
    dataset: Dataset,
}

impl DatasetEnvironment {
    pub fn init(filename: &String) -> Result<DatasetEnvironment, Error> {
        Ok(DatasetEnvironment {
            name: filename.clone(),
            dataset: Dataset::load(filename)?,
        })
    }
}

impl Environment<evaluate::Evaluator> for DatasetEnvironment {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_dimensions(&self) -> &Dimensions {
        return &self.dataset.dimensions;
    }

    fn fitness(&self, network: &mut evaluate::Evaluator) -> f64 {
        if conf::GENERAL.threads > 1 {
            let handles: Vec<thread::JoinHandle<f64>> = self
                .dataset
                .batches
                .iter()
                .cloned()
                .map(|(inputs, targets)| {
                    let mut evaluator = network.clone();
                    thread::spawn(move || -> f64 {
                        error::mse(
                            &targets,
                            inputs.iter().map(|input| evaluator.evaluate(input)),
                        ) * inputs.len() as f64
                    })
                })
                .collect();

            let mut errors = vec![];
            for handle in handles {
                errors.push(handle.join().unwrap());
            }

            1.0 - errors.iter().sum::<f64>() / self.dataset.size as f64
        } else {
            1.0 - self.dataset.mse(
                self.dataset
                    .inputs
                    .iter()
                    .map(|input| network.evaluate(input)),
            )
        }
    }

    fn accuracy(&self, network: &mut evaluate::Evaluator) -> f64 {
        if !self.dataset.is_classification {
            return 0.0;
        }

        if conf::GENERAL.threads > 1 {
            let handles: Vec<thread::JoinHandle<f64>> = self
                .dataset
                .batches
                .iter()
                .cloned()
                .map(|(inputs, targets)| {
                    let one_hot_output = self.dataset.one_hot_output;
                    let mut evaluator = network.clone();
                    thread::spawn(move || -> f64 {
                        let outputs = inputs.iter().map(|input| evaluator.evaluate(input));
                        if one_hot_output {
                            accuracy::one_hot_accuracy(&targets, outputs) * inputs.len() as f64
                        } else {
                            accuracy::rounded_accuracy(&targets, outputs) * inputs.len() as f64
                        }
                    })
                })
                .collect();

            let mut accuracies = vec![];
            for handle in handles {
                accuracies.push(handle.join().unwrap());
            }

            accuracies.iter().sum::<f64>() / self.dataset.size as f64
        } else {
            self.dataset.acc(
                self.dataset
                    .inputs
                    .iter()
                    .map(|input| network.evaluate(input)),
            )
        }
    }
}

impl fmt::Display for DatasetEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.name);
    }
}
