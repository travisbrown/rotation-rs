pub mod fft;
pub mod geometry;
mod web;

use crate::fft::transform;
use image::{DynamicImage, GenericImageView, Pixel};
use rustfft::num_complex::Complex;
use std::cmp::max;

/// Return the spectrum for an image
///
/// Note that the result is transposed.
//#[wasm_bindgen]
pub fn get_fft_spectrum(img: DynamicImage) -> (Vec<Complex<f32>>, usize) {
    let dim = img.dimensions();
    let w = dim.0 as usize;
    let h = dim.1 as usize;

    let size = max(w, h).next_power_of_two();

    let mut input: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); size * size];

    let margin_w = (size - w) / 2;
    let margin_h = (size - h) / 2;

    for y in 0..h {
        for x in 0..w {
            let (r, g, b, _) = img.get_pixel(x as u32, y as u32).channels4();
            let value = (((r as f32) / 255.0) + ((b as f32) / 255.0) + ((g as f32) / 255.0)) / 3.0;

            input[(margin_h + y) * size + margin_w + x] = Complex::new(value, 0.0);
        }
    }

    (transform(&mut input, size, false, false), size)
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn estimate_image_rotation(url: String) -> Result<JsValue, JsValue> {
    let img = web::load_image(url).await?;
    let (spectrum, size) = get_fft_spectrum(img);

    Ok(JsValue::from_f64(
        (geometry::get_rotation(&spectrum, size, size, 10000) as f64).to_degrees(),
    ))
}
