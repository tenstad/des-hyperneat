pub fn mse(target: &Vec<f64>, output: &Vec<f64>) -> f64 {
    target
        .iter()
        .zip(output.iter())
        .map(|(t, o)| f64::powf(t - o, 2.0))
        .sum::<f64>()
        / target.len() as f64
}
