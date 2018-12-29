#[macro_use]
extern crate approx;
extern crate nalgebra;
extern crate png;
extern crate rand;
extern crate indicatif;
extern crate rayon;

mod ray;
mod buffer;
mod camera;
mod primitives;
mod intersection;
mod scene;
mod rect_2d;
mod processing;

use indicatif::ProgressBar;
use self::buffer::Buffer;
use self::camera::Camera;
use self::ray::Ray;
use self::scene::*;
use self::primitives::*;

use nalgebra::{ Vector3, normalize, magnitude };
type float3 = Vector3<f32>;

fn random_in_unit_sphere() -> float3 {
    let mut p = 2.0 * float3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - float3::new(1.0, 1.0, 1.0);
    while magnitude(&p) >= 1.0 {
        p = 2.0 * float3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - float3::new(1.0, 1.0, 1.0);
    }
    normalize(&p)
}

fn trace(scene: &Scene, ray: &Ray, depth: u32) -> float3 {
    if let Some(intersection) = scene.cast(&ray) {
        if depth > 4 {
            return float3::new(0.5, 0.5, 0.5);
        }

        let mut color = float3::new(0.0, 0.0, 0.0);
        for _i in 0..4 {
            let p = ray.get_at(intersection.t);
            let target = p + intersection.normal + random_in_unit_sphere();
            color += 0.5 * trace(scene, &Ray::new(p, target - p), depth + 1);
        }

        color / 4.0
    } else {
        let unit_direction = normalize(&ray.direction);
        let t = 0.5*(unit_direction.y + 1.0);
        (1.0 - t) * float3::new(1.0, 1.0, 1.0) + t*float3::new(0.5, 0.7, 1.0)
    }
}

fn normal(scene: &Scene, ray: &Ray) -> float3 {
    if let Some(intersection) = scene.cast(&ray) {
        0.5*(intersection.normal + float3::new(1.0, 1.0, 1.0))
    } else {
        float3::new(0.0, 0.0, 0.0)
    }
}

fn main() {
    let width = 256;
    let height = 144;
    let samples = 1;
    let subsamples = 4;
    let correct_gamma = true;
    let mut buffer = Buffer::new(width, height);
    let mut scene = Scene::new();
    let camera = Camera::from_fov(
        float3::new(-2.0, 2.0, 1.0),
        float3::new(0.0, 0.0, -1.0),
        float3::new(0.0, 1.0, 0.0),
        90.0,
        width as f32 / height as f32
    );

    scene.add(Box::new(Plane::new(float3::new(0.0, 0.0, 0.0), float3::new(0.0, 1.0, 0.0))));

    for _x in 0..5 {
        for _y in 0..5 {
            let x = (10.0 * rand::random::<f32>()) - 5.0;
            let y = (10.0 * rand::random::<f32>()) - 5.0;
            scene.add(Box::new(Sphere::new(float3::new(x, 0.5, y), 0.5)));
        }
    }

    let bar = ProgressBar::new((width * height * samples) as u64);
    for _ in 0..samples {
        for y in 0..height {
            for x in 0..width {
                let mut color = float3::new(0.0, 0.0, 0.0);
                for _ in 0..subsamples {
                    let u = (x as f32 + rand::random::<f32>()) / width as f32;
                    let v = (y as f32 + rand::random::<f32>()) / height as f32;

                    let ray = camera.get_ray(u, v);
                    // color += normal(&scene, &ray);
                    color += trace(&scene, &ray, 0);
                }

                color /= subsamples as f32;

                let (r, g, b, _) = buffer.get_pixel(x, y);
                let old_color = float3::new(r as f32 / 255.9, g as f32 / 255.9, b as f32 / 255.9);
                color += old_color;
                color /= 2.0;

                let r = (color.x * 255.9) as u8;
                let g = (color.y * 255.9) as u8;
                let b = (color.z * 255.9) as u8;

                buffer.set_pixel(x, y, r, g, b, 255);
                bar.inc(1);
            }
            buffer.save_to_file("./image.png");
        }
    }

    bar.finish();

    if correct_gamma {
        for x in 0..width {
            for y in 0..height {
                let (r, g, b, _) = buffer.get_pixel(x, y);
                let mut color = float3::new(r as f32 / 255.9, g as f32 / 255.9, b as f32 / 255.9);

                color.x = color.x.sqrt();
                color.y = color.y.sqrt();
                color.z = color.z.sqrt();

                let r = (color.x * 255.9) as u8;
                let g = (color.y * 255.9) as u8;
                let b = (color.z * 255.9) as u8;

                buffer.set_pixel(x, y, r, g, b, 255);
            }
        }
    }

    buffer.save_to_file("./image.png");
}
