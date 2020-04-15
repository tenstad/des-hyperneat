use crate::eshyperneat::search;
use crate::hyperneat::img;
use crate::network::execute;
use image::ImageBuffer;
use image::Rgb;
pub fn plot_weights(
    executor: execute::Executor,
    x: f64,
    y: f64,
    input_scale: f64,
    size: usize,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut executor = executor;

    let connection_indexes = search::find_connections(x, y, &mut executor, false)
        .iter()
        .map(|target| {
            let ix = size as f64 / 2.0 + (target.node.0 / input_scale * size as f64 / 2.0).round();
            let iy = size as f64 / 2.0 - (target.node.1 / input_scale * size as f64 / 2.0).round();
            (iy * 3.0 * (size as f64) + ix * 3.0) as usize
        })
        .collect::<Vec<usize>>();

    let mut image = img::plot_weights(executor, x, y, input_scale, size).into_raw();

    for i in connection_indexes {
        image[i] = 255;
        image[i + 1] = 0;
        image[i + 2] = 0;
    }

    ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(size as u32, size as u32, image).unwrap()
}
