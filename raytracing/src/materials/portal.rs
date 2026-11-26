use crate::Hit;
use crate::Ray;
use crate::Vec3;
use crate::materials::Material;

pub struct Portal {
    albedo: Vec3,
    enter_center: Vec3,
    exit_center: Vec3,
}

impl Portal {
    pub fn new(albedo: Vec3, enter_center: Vec3, exit_center: Vec3) -> Portal {
        Portal {
            albedo,
            enter_center,
            exit_center,
        }
    }
    pub fn get_exit_point(&self, p: Vec3) -> Vec3 {
        self.exit_center + (p - self.enter_center)
    }
    pub fn get_exit_ray(&self, ray: &Ray, hit: &Hit) -> Option<Ray> {
        let d = ray.direction().unit_vector();
        let o = hit.p() - self.enter_center;
        let o_exit = o.reflect_across_normal(d);
        if (o - o_exit).close_to(0.05) {
            None
        } else {
            Some(Ray::new(self.exit_center + o_exit, ray.direction()))
        }
    }
}

impl Material for Portal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        if let Some(nxt_ray) = self.get_exit_ray(ray, hit) {
            Some((self.albedo, nxt_ray))
        } else {
            None
        }
    }
}
