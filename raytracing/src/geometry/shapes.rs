use crate::{Material, Triangle, TriangleError, Vec3, World, geometry::Hittable};
use std::f64::consts::PI;
use std::sync::Arc;

pub fn make_box(min: Vec3, max: Vec3, material: Arc<dyn Material>) -> Result<World, TriangleError> {
    let (x0, y0, z0) = (min.x(), min.y(), min.z());
    let (x1, y1, z1) = (max.x(), max.y(), max.z());
    let v = |x, y, z| Vec3::new(x, y, z);

    // Each face is listed counter-clockwise as seen from OUTSIDE the box,
    // so (b - a) × (c - a) points outward.
    let faces = [
        [v(x0, y0, z1), v(x1, y0, z1), v(x1, y1, z1), v(x0, y1, z1)], // front  (+z)
        [v(x1, y0, z0), v(x0, y0, z0), v(x0, y1, z0), v(x1, y1, z0)], // back   (-z)
        [v(x1, y0, z1), v(x1, y0, z0), v(x1, y1, z0), v(x1, y1, z1)], // right  (+x)
        [v(x0, y0, z0), v(x0, y0, z1), v(x0, y1, z1), v(x0, y1, z0)], // left   (-x)
        [v(x0, y1, z1), v(x1, y1, z1), v(x1, y1, z0), v(x0, y1, z0)], // top    (+y)
        [v(x0, y0, z0), v(x1, y0, z0), v(x1, y0, z1), v(x0, y0, z1)], // bottom (-y)
    ];

    let mut tris: Vec<Box<dyn Hittable>> = Vec::new();
    for [p0, p1, p2, p3] in faces {
        // split the quad along the p0–p2 diagonal
        tris.push(Box::new(Triangle::new(p0, p1, p2, Arc::clone(&material))?));
        tris.push(Box::new(Triangle::new(p0, p2, p3, Arc::clone(&material))?));
    }
    Ok(World::new(tris))
}

// A cube balanced on one corner. `spin` turns it around the vertical axis.
pub fn make_diamond_cube(
    center: Vec3,
    size: f64,
    spin: f64,
    material: Arc<dyn Material>,
) -> Result<World, TriangleError> {
    let h = size / 2.0;
    let tilt = -(1.0 / 2f64.sqrt()).atan(); // ≈ -35.26°

    // local corner -> rotated -> moved to `center`
    let v = |x: f64, y: f64, z: f64| {
        let p = Vec3::new(x, y, z).rotate_z(PI / 4.0); // 45° around z
        let p = p.rotate_x(tilt); // tip the (+,+,+) corner straight up
        center + p.rotate_y(spin)
    };

    // same faces / winding as make_box, with the cube centered on the origin
    let faces = [
        [v(-h, -h, h), v(h, -h, h), v(h, h, h), v(-h, h, h)], // +z
        [v(h, -h, -h), v(-h, -h, -h), v(-h, h, -h), v(h, h, -h)], // -z
        [v(h, -h, h), v(h, -h, -h), v(h, h, -h), v(h, h, h)], // +x
        [v(-h, -h, -h), v(-h, -h, h), v(-h, h, h), v(-h, h, -h)], // -x
        [v(-h, h, h), v(h, h, h), v(h, h, -h), v(-h, h, -h)], // +y
        [v(-h, -h, -h), v(h, -h, -h), v(h, -h, h), v(-h, -h, h)], // -y
    ];

    let mut tris: Vec<Box<dyn Hittable>> = Vec::new();
    for [p0, p1, p2, p3] in faces {
        tris.push(Box::new(Triangle::new(p0, p1, p2, Arc::clone(&material))?));
        tris.push(Box::new(Triangle::new(p0, p2, p3, Arc::clone(&material))?));
    }
    Ok(World::new(tris))
}
