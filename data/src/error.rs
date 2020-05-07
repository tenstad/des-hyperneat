pub fn normalize(list: &Vec<f64>) -> Vec<f64> {
    let sum = list.iter().sum::<f64>();

    if sum != 0.0 {
        list.iter().map(|x| x / sum).collect::<Vec<_>>()
    } else {
        list.clone()
    }
}

pub fn mse(targets: &[std::vec::Vec<f64>], predictions: &[std::vec::Vec<f64>], norm: bool) -> f64 {
    targets
        .iter()
        .zip(predictions)
        .map(|(t, p)| mse_single(t, &p, norm))
        .sum::<f64>()
        / targets.len() as f64
}

pub fn mse_single(target: &Vec<f64>, prediction: &Vec<f64>, norm: bool) -> f64 {
    let prediction = if norm {
        normalize(prediction)
    } else {
        prediction.clone()
    };

    target
        .iter()
        .zip(prediction.iter())
        .map(|(t, p)| f64::powi(t - p, 2))
        .sum::<f64>()
        / target.len() as f64
}

pub fn crossentropy(
    targets: &[std::vec::Vec<f64>],
    predictions: &[std::vec::Vec<f64>],
    norm: bool,
) -> f64 {
    targets
        .iter()
        .zip(predictions)
        .map(|(t, p)| crossentropy_single(t, &p, norm))
        .sum::<f64>()
        / targets.len() as f64
}

pub fn crossentropy_single(target: &Vec<f64>, prediction: &Vec<f64>, norm: bool) -> f64 {
    let prediction = if norm {
        normalize(prediction)
    } else {
        prediction.clone()
    };

    let e = 0.0000001;
    let (mi, ma) = (e, 1.0 - e);
    let prediction = prediction
        .iter()
        .map(|x| x.min(ma).max(mi))
        .collect::<Vec<_>>();

    let prediction = normalize(&prediction);

    -target
        .iter()
        .zip(prediction.iter())
        .map(|(t, p)| t * p.ln())
        .sum::<f64>()
}
