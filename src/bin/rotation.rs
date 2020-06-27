use image::error::ImageResult;
use image::ImageBuffer;
use rotation::geometry::{get_rotation, magnitude};
use rotation::get_fft_spectrum;
use std::env;
use std::path::Path;

fn estimate_rotation<P: AsRef<Path>>(input: P) -> ImageResult<f32> {
    let img = image::open(input)?;
    let (spectrum, size) = get_fft_spectrum(img);
    Ok(get_rotation(&spectrum, size, size, 10000))
}

fn save_spectrum<P: AsRef<Path>>(input: P, output: P) -> ImageResult<()> {
    let img = image::open(input)?;
    let (spectrum, size) = get_fft_spectrum(img);

    let mut buf = ImageBuffer::new(size as u32, size as u32);

    let values: Vec<f32> = spectrum.iter().map(|c| magnitude(&c).log2()).collect();
    let max = values.iter().fold(0.0, |a: f32, &b| a.max(b));

    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let rx = ((x as usize) + (size / 2)) % size;
        let ry = ((y as usize) + (size / 2)) % size;

        let value = ((values[rx * size + ry] / max) * 255.0) as u8;

        *pixel = image::Rgb([value, value, value]);
    }

    buf.save(output)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "--output" && args.len() == 4 {
        save_spectrum(&args[3], &args[2]).unwrap();
    } else {
        let rotation = estimate_rotation(&args[1]).unwrap();
        println!("{}", rotation.to_degrees());
    }
}
