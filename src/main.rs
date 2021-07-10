extern crate slice_of_array;

use ::slice_of_array::prelude::*;
use std::time::Instant;
use std::{process::Command, vec};

mod geo;
mod mat;

use geo::sphere::Sphere;
use geo::vec3::Vec3;
use mat::light::Light;
use mat::material::Material;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static WIDTH: usize = 1024;
static HEIGHT: usize = 768;
static NUM_PIXELS: usize = WIDTH * HEIGHT;

static GAMMA: f64 = 2.2;

fn pixel(v: &Vec3) -> [u8; 3] {
    let p = v.clamped() * 255.0;
    [p.x as u8, p.y as u8, p.z as u8]
}

struct Hit<'a> {
    point: Vec3,
    normal: Vec3,
    material: &'a Material,
}

fn scene_intersect<'a>(orig: Vec3, dir: Vec3, scene: &'a Vec<Sphere>) -> Option<Hit<'a>> {
    let mut min_dist = f64::MAX;
    let mut hit: Option<Hit> = None;

    for sphere in scene {
        match sphere.ray_intersect(orig, dir) {
            Some(sphere_dist) if sphere_dist < min_dist => {
                min_dist = sphere_dist;
                let point = orig + dir * sphere_dist;
                let normal = (point - sphere.center).normalized();
                hit = Some(Hit {
                    point,
                    normal,
                    material: &sphere.material,
                })
            }
            _ => (),
        }
    }
    hit
}

fn cast_ray(orig: Vec3, dir: Vec3, scene: &Vec<Sphere>, lights: &Vec<Light>) -> Vec3 {
    let hit = scene_intersect(orig, dir, scene);
    match hit {
        Some(hit) => {
            let mut diffuse_intensity = 0.0;
            let mut specular_intensity = 0.0;
            let normal = hit.normal;
            let material = hit.material;
            let point = hit.point;
            for light in lights {
                let light_dir = (light.position - point).normalized();
                let light_dist = (light.position - point).length();

                let shadow_orig = if light_dir.dot(normal) < 0.0 {
                    point - normal * 0.001
                } else {
                    point + normal * 0.001
                };
                if let Some(hit2) = scene_intersect(shadow_orig, light_dir, scene) {
                    if (hit2.point - shadow_orig).length() < light_dist {
                        continue;
                    }
                }

                diffuse_intensity += light.intensity * f64::max(0.0, light_dir.dot(normal));
                specular_intensity += f64::powf(
                    f64::max(0.0, light_dir.reflect(normal).dot(dir)),
                    material.specular_exponent,
                ) * light.intensity;
            }
            let ambient = Vec3::new(0.1, 0.1, 0.1);
            ambient * 0.025
                + material.diffuse * diffuse_intensity * material.albedo.x
                + Vec3::new(1.0, 1.0, 1.0) * specular_intensity * material.albedo.y
        }
        None => Vec3::new(0.2, 0.7, 0.8),
    }
}

fn raytrace(framebuffer: &mut Vec<Vec3>) {
    let world_z = -30.0;
    let mut scene = vec![];
    for depth in 0..1 {
        for i in 0..12 {
            let angle = 30.0_f64.to_radians() * i as f64;
            let material = match (i + depth) % 2 {
                0 => Material::INDIGO,
                _ => Material::RED_RUBBER,
            };
            let z = world_z - (depth as f64 * 10.0);
            let center = Vec3::new(4.0 * angle.cos(), 4.0 * angle.sin(), z);
            scene.push(Sphere::new(center, 1.0, material));
            scene.push(Sphere::new(
                Vec3::new(center.x * 0.7, center.y * 0.7, z),
                0.25,
                material,
            ));
        }
    }
    // scene.push(Sphere::new(
    //     Vec3::new(-1.0, 1.0, world_z),
    //     1.0,
    //     Material::IVORY,
    // ));

    let lights = vec![
        Light::new(Vec3::new(0.0, 0.0, world_z), 0.75),
        Light::new(Vec3::new(0.0, 40.0, 0.0), 1.15),
    ];
    let fov = 20.0_f64.to_radians();
    let tan_fov = (fov / 2.0).tan();
    let fwidth = WIDTH as f64;
    let fheight = HEIGHT as f64;

    let start = Instant::now();

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2.0 * (i as f64 + 0.5) / fwidth - 1.0) * tan_fov * fwidth / fheight;
            let y = -(2.0 * (j as f64 + 0.5) / fheight - 1.0) * tan_fov;
            let dir = Vec3::new(x, y, -1.0).normalized();
            let color_value = cast_ray(Vec3::origin(), dir, &scene, &lights);
            let gamma_corrected = Vec3::new(
                color_value.x.powf(1.0 / GAMMA),
                color_value.y.powf(1.0 / GAMMA),
                color_value.z.powf(1.0 / GAMMA),
            );
            framebuffer[i + j * WIDTH] = gamma_corrected;
        }
    }

    let duration = start.elapsed();
    println!(
        "Time elapsed in raytrace() is: {:?} seconds",
        duration.as_secs_f64()
    );
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
    irfanview.arg(path).arg("/one");
    irfanview.spawn().expect("Failed to spawn irfanview");
}
