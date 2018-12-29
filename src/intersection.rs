use super::ray::Ray;
use nalgebra::{ Vector3 };
type float3 = Vector3<f32>;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Intersection {
    pub t: f32,
    pub normal: float3,
}

impl Intersection {
    pub fn new(t: f32, normal: float3) -> Intersection {
        Intersection {
            t,
            normal,
        }
    }
}
