use crate::constants::*;
use crate::renderer::Fractal;
use num::complex::Complex;

pub struct Mandelbrot;

impl Fractal for Mandelbrot {
    fn step(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z.powu(3) + c
    }
    fn iter(c: Complex<f64>) -> u32 {
        let mut result = Complex::new(0.0, 0.0);
        let mut n = 0;

        loop {
            result = Mandelbrot::step(result, c);
            n += 1;

            if (n >= MAX_ITERATIONS) || (result.norm() > 2.0) {
                break;
            }
        }

        n
    }
}
