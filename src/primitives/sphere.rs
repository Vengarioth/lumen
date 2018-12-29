use nalgebra::{ Vector3, dot, normalize };
use crate::intersection::*;
use crate::ray::*;
type float3 = Vector3<f32>;

pub struct Sphere {
    position: float3,
    radius: f32,
}

impl Sphere {
    pub fn new(position: float3, radius: f32) -> Sphere {
        Sphere {
            position,
            radius,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.position;
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let d = (b*b) - (a*c);

        if d > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;
            if temp < std::f32::INFINITY && temp > 0.0001 {
                let t = temp;
                let p = ray.get_at(t);
                let normal = normalize(&((p - self.position) / self.radius));
                return Some(Intersection::new(t, normal));
            }

            let temp = (-b + (b*b - a*c).sqrt()) / a;
            if temp < std::f32::INFINITY && temp > 0.0001 {
                let t = temp;
                let p = ray.get_at(t);
                let normal = normalize(&((p - self.position) / self.radius));
                return Some(Intersection::new(t, normal));
            }
        }

        None
    }
}
