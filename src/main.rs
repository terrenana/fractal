use std::{
    fs::{self, File},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use constants::*;
use fractals::*;
use gif::Encoder;
use renderer::render_frame;

mod constants;
mod fractals;
mod helpers;
mod renderer;

fn main() -> std::io::Result<()> {
    let colors = helpers::load_colors("colors.txt");

    let mut threadpool = Vec::new();

    let range: Vec<i32> = (START_ZOOM..END_ZOOM).collect();

    fs::create_dir_all(format!("render/{}", SET_NAME))?;

    let output = File::create(format!("render/{}/fractal.gif", SET_NAME)).unwrap();
    let mut gif = Encoder::new(output, WIDTH as u16, HEIGHT as u16, &[]).unwrap();
    gif.set_repeat(gif::Repeat::Infinite).unwrap();

    let start = SystemTime::now();

    for x in range {
        let c = colors.clone();

        threadpool.push(thread::spawn(move || {
            println!("Rendering {}", x);
            let imgbuf = render_frame::<Mandelbrot>(x, DEFAULT_ZOOM_POINT, &c);
            println!("Rendered {}", x);

            imgbuf
                .save(format!(
                    "render/{}/frame-{:04}.png",
                    SET_NAME,
                    x + -1 * START_ZOOM
                ))
                .unwrap();
            let mut frame =
                gif::Frame::from_rgb_speed(WIDTH as u16, HEIGHT as u16, &imgbuf.into_vec(), 28);
            frame.delay = FRAME_TIME;
            println!("Loaded frame {x}");

            return frame;
        }));
    }
    for thread in threadpool {
        let frame = thread.join().unwrap();
        gif.write_frame(&frame).unwrap();
    }
    let end = SystemTime::now();

    let time = end.duration_since(UNIX_EPOCH).unwrap().as_secs()
        - start.duration_since(UNIX_EPOCH).unwrap().as_secs();

    println!("Elapsed: {time}");
    Ok(())
}
