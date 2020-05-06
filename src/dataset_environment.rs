use data::dataset::Dataset;
use evolution::environment::{Environment, EnvironmentDescription, Stats};
use network::execute::Executor;
use std::fmt::{Display, Formatter, Result};

pub struct DatasetEnvironment {
    dataset: Dataset,
    description: EnvironmentDescription,
}

pub struct DatasetStats {
    accuracy: f64,
}

impl Display for DatasetStats {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Accuracy: {}", self.accuracy)
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
    fn accuracy(&self, predictions: &Vec<Vec<f64>>) -> f64 {
        if !self.dataset.is_classification {
            return 0.0;
        }

        self.dataset.acc(&predictions)
    }

    fn fitness(&self, predictions: &Vec<Vec<f64>>) -> f64 {
        if self.dataset.is_classification && self.dataset.one_hot_output {
            (3.0 - self.dataset.crossentropy(&predictions)).max(0.0)
        } else {
            1.0 - self.dataset.mse(&predictions)
        }
    }
}

impl Environment for DatasetEnvironment {
    type Phenotype = Executor;
    type Stats = DatasetStats;

    fn description(&self) -> EnvironmentDescription {
        self.description.clone()
    }

    fn evaluate(&self, executor: &mut Executor) -> (f64, DatasetStats) {
        let predictions = self
            .dataset
            .inputs
            .iter()
            .map(|input| executor.execute(input))
            .collect::<Vec<Vec<_>>>();

        (
            self.fitness(&predictions),
            DatasetStats {
                accuracy: self.accuracy(&predictions),
            },
        )
    }
}
