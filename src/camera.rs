use nalgebra::{ Vector3, normalize };
use super::ray::Ray;

type float3 = Vector3<f32>;

pub struct Camera {
    origin: float3,
    lower_left_corner: float3,
    horizontal: float3,
    vertical: float3,
}

impl Camera {

    pub fn from_fov(origin: float3, look_at: float3, up: float3, vfov: f32, aspect_ratio: f32) -> Camera {
        let t = vfov * (std::f32::consts::PI / 180.0);
        let hh = (t / 2.0).tan();
        let hw = aspect_ratio * hh;

        let w = normalize(&(origin - look_at));
        let u = normalize(&up.cross(&w));
        let v = w.cross(&u);

        let lower_left_corner = origin - (hw * u) - (hh * v) - w;
        let horizontal = 2.0 * hw * u;
        let vertical = 2.0 * hh * v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn new(origin: float3, lower_left_corner: float3, horizontal: float3, vertical: float3) -> Camera {
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
