use crate::Hit;
use crate::Ray;
use crate::Vec3;

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
