use rustfft::num_complex::Complex;
use std::f32::consts::PI;

pub fn magnitude(c: &Complex<f32>) -> f32 {
    (c.re * c.re + c.im * c.im).sqrt()
}

/// Given a Fourier spectrum, estimate the rotation of the original image
///
/// Note that this isn't very principled, but it seems to kind of work.
pub fn get_rotation(input: &[Complex<f32>], w: usize, h: usize, resolution: usize) -> f32 {
    let mut values: Vec<(f32, usize)> = vec![(0.0, 0); resolution];
    let ignored = w / 100;
    let half = h / 2;

    for x in 0..(w / 2) {
        for y in 0..h {
            let ry = if y > half {
                (h - y) as f32
            } else {
                -(y as f32)
            };

            if x > ignored || ry.abs() as usize > ignored {
                let value: f32 = (((x as f32) / (ry as f32)).atan() / PI) + 0.5;
                let i = (value * (resolution as f32)) as usize;
                let j = if i == resolution { i - 1 } else { i };
                let (sum, count) = values[j];

                // Note that the input is transposed.
                values[j] = (sum + magnitude(&input[x * h + y]), count + 1);
            }
        }
    }

    let avgs = values
        .iter()
        .map(|(sum, count)| {
            if *count == 0 {
                0.0
            } else {
                sum / *count as f32
            }
        })
        .collect::<Vec<f32>>();

    let center = get_center(&avgs);

    (((center + 1) as f32 / resolution as f32) - 0.5) * PI
}

fn get_center(vs: &[f32]) -> usize {
    let mut min = f32::INFINITY;
    let mut result: usize = 0;
    let mut current: f32;
    let len = vs.len();
    let h = len / 2;
    let mut i: usize = h / 2;

    while i < (h / 2) + h {
        current = 0.0;
        for x in 1..h {
            let j = if i >= x { i - x } else { len + i - x };
            let k = if i + x < len { i + x } else { x + i - len };

            current += (vs[j] - vs[k]).powf(2.0);

            if current > min {
                break;
            }
        }
        if current < min {
            min = current;
            result = i;
        }

        i += 1;
    }

    result
}
