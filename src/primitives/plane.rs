use nalgebra::{ Vector3, dot, normalize };
use crate::intersection::*;
use crate::ray::*;
type float3 = Vector3<f32>;

pub struct Plane {
    position: float3,
    up: float3,
}

impl Plane {
    pub fn new(position: float3, up: float3) -> Plane {
        Plane {
            position,
            up: normalize(&up),
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let d = dot(&self.up, &ray.direction);
        if d < -0.0001 {
            let t = dot(&(self.position - ray.origin), &self.up) / d;
            Some(Intersection::new(t, self.up))
        } else {
            None
        }
    }
}
