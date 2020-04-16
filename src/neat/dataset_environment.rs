use crate::conf;
use crate::data::dataset::Dataset;
use crate::data::dataset::Dimensions;
use crate::generic_neat::evaluate::Environment;
use network::execute;
use std::fmt;

pub struct DatasetEnvironment {
    name: String,
    dataset: Dataset,
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
        DatasetEnvironment {
            name: conf::GENERAL.dataset.clone(),
            dataset: Dataset::load(&conf::GENERAL.dataset)
                .ok()
                .expect("unable to load dataset"),
        }
    }
}

impl Environment<execute::Executor> for DatasetEnvironment {
    fn get_dimensions(&self) -> &Dimensions {
        return &self.dataset.dimensions;
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

impl fmt::Display for DatasetEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.name);
    }
}
