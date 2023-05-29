use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    thread,
    time::Instant,
};

use gif::Encoder;
use raster::Color;

#[allow(unused_imports)]
use crate::mandelbrot::{BurningShip, Mandelbrot};

mod mandelbrot;

pub const MAX_ITERATIONS: u32 = 500;
pub const WIDTH: usize = 2100;
pub const HEIGHT: usize = 1500;
pub const START_ZOOM: i32 = -4;
pub const END_ZOOM: i32 = 500;
pub const DELETION_FACTOR: f64 = 0.93;
pub const FRAME_TIME: u16 = 2;
pub const ZOOM_POINT: (f64, f64) = (-0.77568377, 0.13646737);

fn main() -> std::io::Result<()> {
    let mut name = "";
    // println!("Render name: ");
    //io::stdin().read_line(&mut name)?;
    name = "mp13";
    let colors = load_colors("colors.txt");

    let range: Vec<i32> = (START_ZOOM..END_ZOOM).collect();

    fs::create_dir_all("render/").unwrap();

    let output = File::create(format!("render/{}.gif", name)).unwrap();
    let mut gif = Encoder::new(output, WIDTH as u16, HEIGHT as u16, &[]).unwrap();
    gif.set_repeat(gif::Repeat::Infinite).unwrap();

    let start = Instant::now();

    let mut threadpool = Vec::new();

    for x in range {
        let c = colors.clone();

        threadpool.push(thread::spawn(move || {
            println!("Rendering {}", x);
            let imgbuf = mandelbrot::render_frame::<Mandelbrot>(x, ZOOM_POINT, &c);
            println!("Rendered {}", x);

            let mut frame =
                gif::Frame::from_rgb_speed(WIDTH as u16, HEIGHT as u16, &imgbuf.into_vec(), 28);
            frame.delay = FRAME_TIME;
            println!("Loaded frame {x}");

            frame
        }));
    }
    for thread in threadpool {
        let frame = thread.join().unwrap();
        gif.write_frame(&frame).unwrap();
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

fn load_colors(path: &str) -> Vec<[u8; 3]> {
    let mut v = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let c = Color::hex(&line.unwrap()).unwrap();

        v.push([c.r, c.g, c.b])
    }

    let mut b = v.clone();
    b.reverse();

    v.append(&mut b);

    v
}
