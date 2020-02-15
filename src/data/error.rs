use crate::conf;
use std::iter::Iterator;

pub fn mse(targets: &Vec<Vec<f64>>, outputs: impl Iterator<Item = Vec<f64>>) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .map(|(t, o)| mse_single(t, &o))
        .sum::<f64>()
        / targets.len() as f64
}

pub fn mse_single(target: &Vec<f64>, output: &Vec<f64>) -> f64 {
    // Normalize
    let sum: f64 = if conf::NEAT.normalize_output {
        let sum = output.iter().sum();
        if sum == 0.0 {
            1.0
        } else {
            sum
        }
    } else {
        1.0
    };

    target
        .iter()
        .zip(output.iter())
        .map(|(t, o)| f64::powf(t - o / sum, 2.0))
        .sum::<f64>()
        / target.len() as f64
}
