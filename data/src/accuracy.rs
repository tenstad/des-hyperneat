use std::f64;

pub fn one_hot_accuracy(targets: &[std::vec::Vec<f64>], outputs: &[std::vec::Vec<f64>]) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .filter(|(t, o)| argmax(t) == argmax(&o))
        .count() as f64
        / targets.len() as f64
}

pub fn rounded_accuracy(targets: &[std::vec::Vec<f64>], outputs: &[std::vec::Vec<f64>]) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .map(|(t, o)| {
            t.iter()
                .zip(o.iter())
                .filter(|(t, o)| t.round() == o.round())
                .count() as f64
                / t.len() as f64
        })
        .sum::<f64>()
        / targets.len() as f64
}

pub fn binary_accuracy(targets: &[std::vec::Vec<f64>], outputs: &[std::vec::Vec<f64>]) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .map(|(t, o)| {
            t.iter()
                .zip(o.iter())
                .filter(|(t, o)| (**t > 0.0) == (**o > 0.0))
                .count() as f64
                / t.len() as f64
        })
        .sum::<f64>()
        / targets.len() as f64
}

pub fn argmax(vec: &Vec<f64>) -> usize {
    vec.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap()
}
