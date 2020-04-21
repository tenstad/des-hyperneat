use image::ImageBuffer;
use image::Rgb;
use network::execute;

pub fn plot_weights(
    executor: &mut execute::Executor,
    x: f64,
    y: f64,
    input_scale: f64,
    size: usize,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image: Vec<f64> = vec![0.0; size * size * 3];

    for i in 0..size {
        for j in 0..size {
            let v = executor.execute(&vec![
                x,
                y,
                (i as f64 / (size as f64) * 2.0 - 1.0) * input_scale,
                ((size - j) as f64 / (size as f64) * 2.0 - 1.0) * input_scale,
            ])[0];

            let ii = j * size * 3 + i * 3;
            image[ii] = v;
            image[ii + 1] = v;
            image[ii + 2] = v;
        }
    }

    let mi = image
        .iter()
        .min_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();
    let ma = image
        .iter()
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();
    let delta = ma - mi;

    let image: Vec<u8> = image
        .iter()
        .map(|v| (255.0 * (v - mi) / delta).floor() as u8)
        .collect();

    ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(size as u32, size as u32, image).unwrap()
}
