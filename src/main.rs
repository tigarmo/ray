#![feature(clamp)]

extern crate slice_of_array;

use ::slice_of_array::prelude::*;

mod geo;

use geo::vec3::Vec3;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn pixel(v: &Vec3) -> [u8; 3] {
    let p = v.clamped() * 255.0;
    [p.x as u8, p.y as u8, p.z as u8]
}

fn main() {
    let width = 1024;
    let height = 768;
    let num_pixels = width * height;
    let mut framebuffer: Vec<Vec3> = vec![Default::default(); num_pixels];
    let mut pixels: Vec<[u8; 3]> = vec![[255, 0, 255]; num_pixels];

    for j in 0..height {
        for i in 0..width {
            framebuffer[i + j * width] = Vec3 {
                x: j as f64 / height as f64,
                y: i as f64 / width as f64,
                z: 0.0,
            };
        }
    }

    for (v, p) in framebuffer.iter().zip(pixels.iter_mut()) {
        *p = pixel(v);
    }

    let path = Path::new("F:/image.ppm");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let header = format!("P6\n{} {}\n255\n", width, height);
    file.write_all(header.as_bytes()).unwrap();
    file.write_all(pixels.flat()).unwrap();
}
