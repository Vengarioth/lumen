use crate::intersection::*;
use crate::ray::*;

pub struct Scene {
    elements: Vec<Box<Intersectable + Sync>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            elements: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Box<Intersectable + Sync>) {
        self.elements.push(item);
    }

    pub fn cast(&self, ray: &Ray) -> Option<Intersection> {
        let mut t = 999.0;
        let mut r = None;
        for element in self.elements.iter() {
            if let Some(intersection) = element.intersect(ray) {
                if intersection.t < t {
                    t = intersection.t;
                    r = Some(intersection);
                }
            }
        }

        r
    }
}
