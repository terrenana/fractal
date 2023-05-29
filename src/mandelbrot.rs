use num::complex::Complex;

use crate::*;

trait Fractal {
    fn step(z: Complex<f64>, c: Complex<f64>) -> Complex<f64>;
    fn iter(c: Complex<f64>) -> u32;
}

pub struct Mandelbrot;

impl Fractal for Mandelbrot {
    fn step(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z.powu(2) + c
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

pub struct BurningShip;

impl Fractal for BurningShip {
    fn step(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        Complex::new(z.re.abs(), z.im.abs()).powu(2) + c
    }
    fn iter(c: Complex<f64>) -> u32 {
        let mut result = Complex::new(0.0, 0.0);
        let mut n = 0;

        loop {
            result = BurningShip::step(result, c);
            n += 1;

            if (n >= MAX_ITERATIONS) || (result.norm() > 2.0) {
                break;
            }
        }

        n
    }
}

pub fn render_frame<T: Fractal>(
    zoom: i32,
    zoom_point: (f64, f64),
    colors: &Vec<[u8; 3]>,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let factor = WIDTH as f64 / HEIGHT as f64;
    let half_width = WIDTH as f64 / 2.0;
    let half_height = HEIGHT as f64 / 2.0;
    let deletion = f64::powf(DELETION_FACTOR, zoom as f64);

    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let a =
            zoom_point.0 + ((x as f64 - WIDTH as f64 / 2.0) / half_width) * 2.0 * factor * deletion;
        let b = zoom_point.1
            + ((y as f64 - HEIGHT as f64 / 2.0) / half_height) * 2.0 * factor * deletion;
        let color = to_rgb(<T>::iter(Complex::new(a, b)), colors);

        *pixel = image::Rgb(color);
    }

    imgbuf
}

pub fn to_rgb(it: u32, colors: &Vec<[u8; 3]>) -> [u8; 3] {
    if it >= MAX_ITERATIONS {
        return [0, 0, 0];
    }

    let i = it as usize % colors.len();

    colors[i]
}
