use data::dataset::Dataset;
use evolution::environment::{Environment, EnvironmentDescription};
use network::execute;

pub struct DatasetEnvironment {
    dataset: Dataset,
    description: EnvironmentDescription,
}

impl DatasetEnvironment {
    fn accuracy(&self, executor: &mut execute::Executor) -> f64 {
        if !self.dataset.is_classification {
            return 0.0;
        }

        self.dataset.acc(
            self.dataset
                .inputs
                .iter()
                .map(|input| executor.execute(input)),
        )
    }
}

impl Default for DatasetEnvironment {
    fn default() -> DatasetEnvironment {
        let dataset = Dataset::load();

        let description =
            EnvironmentDescription::new(dataset.dimensions.inputs, dataset.dimensions.outputs);

        DatasetEnvironment {
            dataset: dataset,
            description: description,
        }
    }
}

impl Environment<execute::Executor> for DatasetEnvironment {
    fn description(&self) -> EnvironmentDescription {
        self.description.clone()
    }

    fn fitness(&self, executor: &mut execute::Executor) -> f64 {
        1.0 - self.dataset.mse(
            self.dataset
                .inputs
                .iter()
                .map(|input| executor.execute(input)),
        )
    }
}
