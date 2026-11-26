mod geometry;
mod materials;
mod math;
mod render;

pub use geometry::{Hit, Hittable, Sphere, Triangle, TriangleError};
pub use materials::{DiffuseLight, Lambertian, Metal, Portal, Material};
pub use math::{Interval, Ray, Vec3, rand};
pub use render::{Camera, Rgb, World};
