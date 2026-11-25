use crate::Interval;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use std::sync::Arc;

#[derive(Debug)]
pub struct Hit {
    p: Vec3,
    t: f64,
    normal: Vec3,
    front_face: bool,
    material: Arc<dyn Material>,
}

impl Hit {
    pub fn new(
        p: Vec3,
        t: f64,
        normal: Vec3,
        front_face: bool,
        material: Arc<dyn Material>,
    ) -> Hit {
        Hit {
            p,
            t,
            normal,
            front_face,
            material,
        }
    }
    pub fn p(&self) -> Vec3 {
        self.p
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn material(&self) -> Arc<dyn Material> {
        Arc::clone(&self.material)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit>;
}
