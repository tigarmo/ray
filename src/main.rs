#![feature(clamp)]

extern crate slice_of_array;

use ::slice_of_array::prelude::*;
use std::process::Command;

mod geo;

use geo::sphere::Sphere;
use geo::vec3::Vec3;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static WIDTH: usize = 1024;
static HEIGHT: usize = 768;
static NUM_PIXELS: usize = WIDTH * HEIGHT;

fn pixel(v: &Vec3) -> [u8; 3] {
    let p = v.clamped() * 255.0;
    [p.x as u8, p.y as u8, p.z as u8]
}

fn cast_ray(orig: Vec3, dir: Vec3, sphere: &Sphere) -> Vec3 {
    match sphere.ray_intersect(orig, dir) {
        Some(_) => Vec3::new(0.4, 0.4, 0.3),
        None => Vec3::new(0.2, 0.7, 0.8),
    }
}

fn raytrace(framebuffer: &mut Vec<Vec3>) {
    let sphere = Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0);
    //let fov = std::f64::consts::PI / 2.0;
    let fov = 60.0_f64.to_radians();
    let tan_fov = (fov / 2.0).tan();
    let fwidth = WIDTH as f64;
    let fheight = HEIGHT as f64;

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2.0 * (i as f64 + 0.5) / fwidth - 1.0) * tan_fov * fwidth / fheight;
            let y = -(2.0 * (j as f64 + 0.5) / fheight - 1.0) * tan_fov;
            let dir = Vec3::new(x, y, -1.0).normalized();
            framebuffer[i + j * WIDTH] = cast_ray(Vec3::origin(), dir, &sphere);
        }
    }
}

fn main() {
    let mut framebuffer: Vec<Vec3> = vec![Default::default(); NUM_PIXELS];
    let mut pixels: Vec<[u8; 3]> = vec![[255, 0, 255]; NUM_PIXELS];

    raytrace(&mut framebuffer);
    for (v, p) in framebuffer.iter().zip(pixels.iter_mut()) {
        *p = pixel(v);
    }

    let path = Path::new(r"F:/image.ppm").canonicalize().unwrap();
    let display = path.display();

    {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let header = format!("P6\n{} {}\n255\n", WIDTH, HEIGHT);
        file.write_all(header.as_bytes()).unwrap();
        file.write_all(pixels.flat()).unwrap();
    }

    let irfanview_path = r"C:\Program Files\IrfanView\i_view64.exe";
    let mut irfanview = Command::new(irfanview_path);
    irfanview.arg(path);
    irfanview.spawn().expect("Failed to spawn irfanview");
}
