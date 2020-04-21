use crate::neat::dataset_environment::DatasetEnvironment as NeatDatasetEnvironment;
use evolution::environment::Environment;
use evolution::environment::EnvironmentDescription;
use network::execute;

pub struct DatasetEnvironment {
    environment: NeatDatasetEnvironment,
    description: EnvironmentDescription,
}

impl Default for DatasetEnvironment {
    fn default() -> DatasetEnvironment {
        DatasetEnvironment {
            environment: NeatDatasetEnvironment::default(),
            description: EnvironmentDescription {
                inputs: 4,
                outputs: 2,
            },
        }
    }
}

impl Environment<execute::Executor> for DatasetEnvironment {
    fn description(&self) -> EnvironmentDescription {
        self.description.clone()
    }

    fn fitness(&self, executor: &mut execute::Executor) -> f64 {
        self.environment.fitness(executor)
    }
}
