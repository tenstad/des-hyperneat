use crate::data::dataset::Dimensions;
use crate::generic_neat::evaluate::Environment;
use crate::neat::dataset_environment::DatasetEnvironment as NeatDatasetEnvironment;
use crate::network::execute;

pub struct DatasetEnvironment {
    environment: NeatDatasetEnvironment,
}

impl Default for DatasetEnvironment {
    fn default() -> DatasetEnvironment {
        DatasetEnvironment {
            environment: NeatDatasetEnvironment::default(),
        }
    }
}

impl Environment<execute::Executor> for DatasetEnvironment {
    fn get_dimensions(&self) -> &Dimensions {
        return &Dimensions {
            inputs: 4,
            outputs: 2,
        };
    }

    fn fitness(&self, executor: &mut execute::Executor) -> f64 {
        self.environment.fitness(executor)
    }
}
