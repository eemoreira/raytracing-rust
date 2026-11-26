mod hit;
pub mod shapes;
mod sphere;
mod triangle;

pub use hit::{Hit, Hittable};
pub use sphere::Sphere;
pub use triangle::{Triangle, TriangleError};
