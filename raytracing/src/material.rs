use crate::Hit;
use crate::Ray;
use crate::Vec3;
use crate::rand;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
    fn emit(&self) -> Vec3 {
        Vec3::new_eq(0.0)
    }
}

impl std::fmt::Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", "<material>")
    }
}

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
