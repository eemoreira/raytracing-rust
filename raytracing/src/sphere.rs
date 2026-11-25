use crate::Interval;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::{Hit, Hittable};
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: Vec3::new(x, y, z),
            radius,
            material,
        }
    }
    pub fn center(&self) -> Vec3 {
        self.center
    }
    pub fn r(&self) -> f64 {
        self.radius
    }
    pub fn normal(&self, p: Vec3) -> Vec3 {
        let outward_normal = p - self.center;
        //outward_normal.unit_vector() -> p - self.center has magnitude = self.radius
        outward_normal / self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit> {
        // ((C - ori()) - dir * t) dot ((C - ori()) - dir() * t) - r^2 = 0
        // dot(C - ori(), C - ori()) - 2 * t * dot(C - ori(), dir()) + t^2 * dot(dir(), dir()) - r^2 = 0
        // a = dot(dir(), dir())
        // b = -2 * dot(C - ori(), dir())
        // c = dot(C - ori(), C - ori()) - r^2
        // divide geral por 2

        let oc = self.center() - ray.origin();
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = Vec3::dot(ray.direction(), oc) * -1.0;
        let c = Vec3::dot(oc, oc) - self.r() * self.r();

        let delta = b * b - a * c;
        if delta < 0.0 {
            return None;
        }

        let d_sqrt = delta.sqrt();
        [(-b - d_sqrt) / a, (-b + d_sqrt) / a]
            .into_iter()
            .find(|&x| range.contains(x))
            .map(|t| {
                let normal = self.normal(ray.at(t));
                let front_face = normal.dot(ray.direction()) < 0.0;
                Hit::new(ray.at(t), t, normal, front_face, Arc::clone(&self.material))
            })
    }
}
