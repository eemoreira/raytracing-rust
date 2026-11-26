use crate::Hit;
use crate::Ray;
use crate::Vec3;
use crate::materials::Material;
use crate::math::rand;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let random_unit = rand::random_vec3();
        let normal = hit.normal() * (if hit.front_face() { 1.0 } else { -1.0 });
        if (normal - random_unit).zero() {
            Some((self.albedo, Ray::new(hit.p(), normal)))
        } else {
            Some((self.albedo, Ray::new(hit.p(), normal + random_unit)))
        }
    }
}
