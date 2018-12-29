use nalgebra::{ Vector3 };

type float3 = Vector3<f32>;

pub struct Ray {
    pub origin: float3,
    pub direction: float3,
}

impl Ray {
    pub fn new(origin: float3, direction: float3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn get_at(&self, t: f32) -> float3 {
        self.origin + (t * self.direction)
    }
}
