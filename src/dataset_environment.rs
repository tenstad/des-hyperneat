use data::{accuracy, conf::DatasetConfig, dataset::Dataset, error};
use evolution::{
    environment::{Environment, EnvironmentDescription},
    stats::Stats,
};
use network::execute::Executor;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

pub struct DatasetEnvironment {
    dataset: Dataset,
    description: EnvironmentDescription,
}

#[derive(Serialize)]
pub struct DatasetStats {
    validation_fitness: f64,
    training_accuracy: f64,
    validation_accuracy: f64,
}

impl Display for DatasetStats {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Val fitness: {} \t Tr acc: {} \t Val acc: {}",
            self.validation_fitness, self.training_accuracy, self.validation_accuracy
        )
    }
}

impl Stats for DatasetStats {}

impl Default for DatasetEnvironment {
    fn default() -> DatasetEnvironment {
        let dataset = Dataset::load();
        let description =
            EnvironmentDescription::new(dataset.dimensions.inputs, dataset.dimensions.outputs);

        DatasetEnvironment {
            dataset,
            description,
        }
    }
}

impl DatasetEnvironment {
    fn accuracy(&self, targets: &Vec<Vec<f64>>, predictions: &Vec<Vec<f64>>) -> f64 {
        if !self.dataset.is_classification {
            0.0
        } else if self.dataset.one_hot_output {
            accuracy::one_hot_accuracy(targets, predictions)
        } else {
            accuracy::binary_accuracy(targets, predictions)
        }
    }

    fn fitness(&self, targets: &Vec<Vec<f64>>, predictions: &Vec<Vec<f64>>) -> f64 {
        let norm = self.dataset.is_classification && self.dataset.one_hot_output;

        if self.dataset.is_classification && self.dataset.one_hot_output {
            (-error::crossentropy(targets, predictions, norm)).exp()
        } else {
            let e = 1.0 - error::mse(targets, predictions, norm);
            if e.is_finite() {
                e
            } else {
                0.0
            }
        }
    }
}

impl Environment for DatasetEnvironment {
    type Config = DatasetConfig;
    type Phenotype = Executor;
    type Stats = DatasetStats;

    fn description(&self) -> EnvironmentDescription {
        self.description.clone()
    }

    fn evaluate(&self, executor: &mut Executor) -> (f64, DatasetStats) {
        let tr_pred = self
            .dataset
            .training_inputs
            .iter()
            .map(|input| executor.execute(input))
            .collect::<Vec<Vec<_>>>();
        let val_pred = self
            .dataset
            .validation_inputs
            .iter()
            .map(|input| executor.execute(input))
            .collect::<Vec<Vec<_>>>();

        let training_fitness = self.fitness(&self.dataset.training_targets, &tr_pred);
        let validation_fitness = self.fitness(&self.dataset.validation_targets, &val_pred);
        let training_accuracy = self.accuracy(&self.dataset.training_targets, &tr_pred);
        let validation_accuracy = self.accuracy(&self.dataset.validation_targets, &val_pred);

        (
            training_fitness,
            DatasetStats {
                validation_fitness,
                training_accuracy,
                validation_accuracy,
            },
        )
    }
}
