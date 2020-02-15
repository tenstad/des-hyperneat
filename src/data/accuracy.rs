pub fn one_hot_accuracy(targets: &Vec<Vec<f64>>, outputs: impl Iterator<Item = Vec<f64>>) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .map(|(t, o)| if argmax(t) == argmax(&o) { 1.0 } else { 0.0 })
        .sum::<f64>()
        / targets.len() as f64
}

pub fn rounded_accuracy(targets: &Vec<Vec<f64>>, outputs: impl Iterator<Item = Vec<f64>>) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .map(|(t, o)| {
            t.iter()
                .zip(o.iter())
                .map(|(t, o)| if t.round() == o.round() { 1.0 } else { 0.0 })
                .sum::<f64>()
        })
        .sum::<f64>()
        / targets.len() as f64
}

pub fn argmax(vec: &Vec<f64>) -> usize {
    let mut max_i: usize = 0;
    let mut max_v: f64 = -100000.0;

    for (i, &v) in vec.iter().enumerate() {
        if v > max_v {
            max_i = i;
            max_v = v;
        }
    }

    return max_i;
}
