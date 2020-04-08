use crate::eshyperneat::search::find_connections;
use crate::network::execute;
use image::ImageBuffer;
use image::Rgb;

pub fn plot_weights(
    executor: execute::Executor,
    x: f64,
    y: f64,
    input_scale: f64,
    size: usize,
    fname: &'static str,
) {
    let mut image: Vec<f64> = vec![0.0; size * size * 3];
    let mut executor = executor;

    for i in 0..size {
        for j in 0..size {
            let v = executor.execute(&vec![
                x,
                y,
                (i as f64 / (size as f64) * 2.0 - 1.0) * input_scale,
                ((size-j) as f64 / (size as f64) * 2.0 - 1.0) * input_scale,
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

    let mut image: Vec<u8> = image
        .iter()
        .map(|v| (255.0 * (v - mi) / delta).floor() as u8)
        .collect();

    for connection in find_connections(x, y, &mut executor) {
        println!("{} {}", connection.x, connection.y);

        let ix = size as f64 / 2.0 + (connection.x / input_scale * size as f64 / 2.0).round();
        let iy = size as f64 / 2.0 - (connection.y / input_scale * size as f64 / 2.0).round();
        let i = (iy * 3.0 * (size as f64) + ix * 3.0) as usize;

        image[i] = 255;
        image[i + 1] = 0;
        image[i + 2] = 0;
    }

    ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(size as u32, size as u32, image)
        .unwrap()
        .save(fname)
        .ok();
}
