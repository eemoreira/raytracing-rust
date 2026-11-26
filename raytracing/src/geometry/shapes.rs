use crate::{
    Hit, Interval, Ray, Triangle, TriangleError, Vec3, geometry::Hittable,
    materials::Material,
};
use std::f64::consts::PI;
use std::sync::Arc;

pub struct Cube {
    triangles: Vec<Triangle>,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Arc<dyn Material>) -> Result<Cube, TriangleError> {
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

        let tris = faces
            .into_iter()
            .flat_map(|face| {
                let [p0, p1, p2, p3] = face;
                [
                    Triangle::new(p0, p1, p2, Arc::clone(&material)),
                    Triangle::new(p0, p2, p3, Arc::clone(&material)),
                ]
            })
            .collect::<Result<Vec<Triangle>, TriangleError>>()?;

        Ok(Cube { triangles: tris })
    }
    pub fn triangles<'a>(&'a self) -> &'a Vec<Triangle> {
        &self.triangles
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit> {
        self.triangles()
            .into_iter()
            .filter(|triangle| triangle.normal().dot(ray.direction()) < 0.0)
            .find_map(|triangle| triangle.hit(ray, range))
    }
}

pub struct Diamond {
    triangles: Vec<Triangle>,
}

impl Diamond {
    pub fn new(
        center: Vec3,
        size: f64,
        spin: f64,
        material: Arc<dyn Material>,
    ) -> Result<Diamond, TriangleError> {
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

        let triangles = faces
            .into_iter()
            .flat_map(|triangle| {
                let [p0, p1, p2, p3] = triangle;
                [
                    Triangle::new(p0, p1, p2, Arc::clone(&material)),
                    Triangle::new(p0, p2, p3, Arc::clone(&material)),
                ]
            })
            .collect::<Result<Vec<Triangle>, TriangleError>>()?;
        Ok(Diamond { triangles })
    }
    pub fn triangles<'a>(&'a self) -> &'a Vec<Triangle> {
        &self.triangles
    }
}

impl Hittable for Diamond {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit> {
        self.triangles()
            .into_iter()
            .filter(|triangle| triangle.normal().dot(ray.direction()) < 0.0)
            .find_map(|triangle| triangle.hit(ray, range))
    }
}
