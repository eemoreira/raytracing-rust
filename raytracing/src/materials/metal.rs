use crate::Hit;
use crate::Ray;
use crate::Vec3;
use crate::materials::Material;
use crate::math::rand;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let normal = hit.normal() * (if hit.front_face() { 1.0 } else { -1.0 });
        let dir = ray.direction().reflect_across_normal(normal).unit_vector()
            + (rand::random_vec3() * self.fuzz);

        if normal.dot(dir) < 0.0 {
            None
        } else {
            Some((self.albedo, Ray::new(hit.p(), dir)))
        }
    }
}
