use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;
use transpose::transpose;

/// Returns a 2-D FFT for a square image
///
/// Note that we don't assume that the size is a power of two, but the image must be a square. Also
/// note that the final transposition is optional, and may be unnecessary, depending on what
/// subsequent processing needs to happen.
pub fn transform(
    input: &mut [Complex<f32>],
    size: usize,
    inverse: bool,
    final_transpose: bool,
) -> Vec<Complex<f32>> {
    let mut planner = FFTplanner::<f32>::new(inverse);
    let fft = planner.plan_fft(size);
    let len = size * size;

    let mut output: Vec<Complex<f32>> = vec![Zero::zero(); len];
    fft.process_multi(input, &mut output);

    let mut transposed: Vec<Complex<f32>> = vec![Zero::zero(); len];
    transpose(&output, &mut transposed, size, size);

    fft.process_multi(&mut transposed, &mut output);

    if final_transpose {
        transpose(&output, &mut transposed, size, size);

        transposed
    } else {
        output
    }
}
