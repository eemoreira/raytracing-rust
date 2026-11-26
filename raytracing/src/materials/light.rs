use crate::Hit;
use crate::Ray;
use crate::Vec3;
use crate::materials::Material;

pub struct DiffuseLight {
    albedo: Vec3,
}

impl DiffuseLight {
    pub fn new(albedo: Vec3) -> DiffuseLight {
        DiffuseLight { albedo }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<(Vec3, Ray)> {
        None
    }
    fn emit(&self) -> Vec3 {
        self.albedo
    }
}
