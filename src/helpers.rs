use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use raster::Color;

use crate::constants::*;

pub fn to_rgb(it: u32, colors: &Vec<[u8; 3]>) -> [u8; 3] {
    if !(it < MAX_ITERATIONS) {
        return [0, 0, 0];
    }

    let i = it as usize % colors.len();

    return colors[i];
}

pub fn load_colors(path: &str) -> Vec<[u8; 3]> {
    let mut v = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let c = Color::hex(&line.unwrap()).unwrap();

        v.push([c.r, c.g, c.b])
    }

    let mut b = v.clone();
    b.reverse();

    v.append(&mut b);

    v
}
