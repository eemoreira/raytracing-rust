use crate::Hit;
use crate::Hittable;
use crate::Interval;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    material: Arc<dyn Material>,
}

#[derive(Debug)]
pub enum TriangleError {
    Degenerate,
}

impl fmt::Display for TriangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriangleError::Degenerate => write!(f, "triangle points are collinear"),
        }
    }
}

impl std::error::Error for TriangleError {}

// ax + by + cz + k = 0
// dot(n, p) + k = 0
// to be in the triangle's plane, dot(n, p) = dot(n, (b - a))
impl Triangle {
    pub fn new(
        a: Vec3,
        b: Vec3,
        c: Vec3,
        material: Arc<dyn Material>,
    ) -> Result<Triangle, TriangleError> {
        let normal = (b - a).cross(c - a);
        if normal.zero() {
            Err(TriangleError::Degenerate)
        } else {
            Ok(Triangle {
                a,
                b,
                c,
                normal: (b - a).cross(c - a).unit_vector(),
                material,
            })
        }
    }
    fn inside(&self, p: Vec3) -> bool {
        // p is assumed to be on the same plane as the triangle
        let sign_a_b = Vec3::dot(self.normal, (self.b - self.a).cross(p - self.a));
        let sign_b_c = Vec3::dot(self.normal, (self.c - self.b).cross(p - self.b));
        let sign_c_a = Vec3::dot(self.normal, (self.a - self.c).cross(p - self.c));

        if sign_a_b <= 0.0 && sign_b_c <= 0.0 && sign_c_a <= 0.0 {
            true
        } else if sign_a_b >= 0.0 && sign_b_c >= 0.0 && sign_c_a >= 0.0 {
            true
        } else {
            false
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit> {
        // dot(n, ray.ori + ray.dir * t) == dot(n, a)
        // dot(n, ray.ori) + t * dot(n, ray.dir) == dot(n, a)
        // t = (dot(n, a) - dot(n, ray.ori)) / dot(n, ray.dir)
        let n_dot_ray_dir = self.normal.dot(ray.direction());
        if n_dot_ray_dir.abs() < 1e-9 {
            None
        } else {
            let t = (self.normal.dot(self.a) - self.normal.dot(ray.origin())) / n_dot_ray_dir;

            let p = ray.at(t);
            if range.contains(t) && self.inside(p) {
                Some(Hit::new(
                    p,
                    t,
                    self.normal,
                    n_dot_ray_dir < 0.0,
                    Arc::clone(&self.material),
                ))
            } else {
                None
            }
        }
    }
}
