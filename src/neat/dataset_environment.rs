use crate::data::dataset::Dataset;
use crate::data::dataset::Dimensions;
use crate::generic_neat::environment::Environment;
use crate::network::evaluate;
use std::fmt;
use std::io::Error;

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
        let outputs: Vec<Vec<f64>> = self
            .dataset
            .inputs
            .iter()
            .map(|input| network.evaluate(input))
            .collect();

        1.0 - self.dataset.mse(&outputs)
    }

    fn accuracy(&self, network: &mut evaluate::Evaluator) -> f64 {
        let outputs: Vec<Vec<f64>> = self
            .dataset
            .inputs
            .iter()
            .map(|input| network.evaluate(input))
            .collect();

        self.dataset.acc(&outputs)
    }
}

impl fmt::Display for DatasetEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.name);
    }
}
