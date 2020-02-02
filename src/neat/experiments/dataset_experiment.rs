use crate::data::dataset::Dataset;
use crate::data::dataset::Dimensions;
use crate::neat::environment::Environment;
use crate::neat::genome::Genome;
use std::io::Error;

#[derive(Debug)]
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

    fn evaluate_binary(&self, genome: &Genome) -> f64 {
        let outputs: Vec<Vec<f64>> = self
            .dataset
            .inputs
            .iter()
            .map(|input| genome.evaluate_n(input, self.dataset.dimensions.outputs))
            .collect();

        let outputs: Vec<Vec<f64>> = outputs
            .iter()
            .map(|vec| {
                vec.iter()
                    .map(|v| if *v > 0.5 { 1.0 } else { 0.0 })
                    .collect()
            })
            .collect();

        self.dataset.mse(&outputs)
    }
}
