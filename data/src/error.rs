pub fn normalize(list: &Vec<f64>) -> Vec<f64> {
    let sum = list.iter().sum::<f64>();

    if sum != 0.0 {
        list.iter().map(|x| x / sum).collect::<Vec<_>>()
    } else {
        list.clone()
    }
}

pub fn mse(targets: &Vec<Vec<f64>>, outputs: &Vec<Vec<f64>>, norm: bool) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .map(|(t, o)| mse_single(t, &o, norm))
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
        .map(|(t, o)| f64::powi(t - o, 2))
        .sum::<f64>()
        / target.len() as f64
}

pub fn crossentropy(targets: &Vec<Vec<f64>>, outputs: &Vec<Vec<f64>>, norm: bool) -> f64 {
    targets
        .iter()
        .zip(outputs)
        .map(|(t, o)| crossentropy_single(t, &o, norm))
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
        .map(|(t, pred)| t * pred.ln())
        .sum::<f64>()
}
