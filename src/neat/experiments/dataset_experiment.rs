use crate::data::dataset::Dataset;
use crate::data::dataset::Dimensions;
use crate::neat::environment::Environment;
use crate::neat::genome::Genome;
use std::fmt;
use std::io::Error;

pub struct DatasetExperiment {
    name: String,
    dataset: Dataset,
}

impl DatasetExperiment {
    pub fn init(filename: &String) -> Result<DatasetExperiment, Error> {
        Ok(DatasetExperiment {
            name: filename.clone(),
            dataset: Dataset::load(filename)?,
        })
    }
}

impl Environment for DatasetExperiment {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_dimensions(&self) -> &Dimensions {
        return &self.dataset.dimensions;
    }

    fn evaluate(&self, genome: &Genome) -> f64 {
        let outputs: Vec<Vec<f64>> = self
            .dataset
            .inputs
            .iter()
            .map(|input| genome.evaluate_n(input, self.dataset.dimensions.outputs))
            .collect();

        self.dataset.mse(&outputs)
    }

    fn evaluate_accuracy(&self, genome: &Genome) -> f64 {
        let outputs: Vec<Vec<f64>> = self
            .dataset
            .inputs
            .iter()
            .map(|input| genome.evaluate_n(input, self.dataset.dimensions.outputs))
            .collect();

        self.dataset.acc(&outputs)
    }
}

impl fmt::Display for DatasetExperiment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.name)
    }
}
