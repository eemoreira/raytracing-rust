use raytracing::{
    Camera, DiffuseLight, Hittable, Lambertian, Material, Metal, Portal, Sphere, Triangle,
    TriangleError, Vec3, World,
};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

fn make_box(min: Vec3, max: Vec3, material: Arc<dyn Material>) -> Result<World, TriangleError> {
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
fn make_diamond_cube(
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _width = 1920;
    let height = 1200;

    let fov = 120.0;
    let angle = fov * PI / 180.0;

    let camera = Camera::new(16.0 / 10.0, height, 2.0 * ((angle / 2.0).tan()));

    let lambert_gray: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let lambert_red: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.9, 0.0, 0.0)));
    //let lambert_yellow: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let fuzzy_metal: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.05));
    let metal: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.4, 0.4, 0.4), 0.0));
    let metal_yellow: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.4, 0.4, 0.0), 0.3));
    //let metal_red: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.6, 0.0, 0.0), 0.9));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Vec3::new_eq(0.5)));

    let portal_enter_center = Vec3::new(-0.9, 0.9, -1.4);
    let portal_exit_center = Vec3::new(0.3, 0.0, -0.8);
    let portal_albedo = Vec3::new(0.3, 0.4, 0.7);
    let portal_radius = 0.37;
    let portal: Arc<dyn Material> = Arc::new(Portal::new(
        portal_albedo,
        portal_enter_center,
        portal_exit_center,
    ));

    let portal2_albedo = Vec3::new(0.7, 0.3, 0.4);
    let portal2_enter_center = Vec3::new(0.0, 0.9, -1.8);
    let portal2: Arc<dyn Material> = Arc::new(Portal::new(
        portal2_albedo,
        portal2_enter_center,
        portal_exit_center,
    ));

    let cube = make_box(
        Vec3::new(0.3, -0.5, -1.4),
        Vec3::new(0.8, -0.1, -0.9),
        Arc::clone(&metal_yellow),
    )?;
    let diamond = make_diamond_cube(
        Vec3::new(0.0, 0.5, -1.0),
        0.5,
        0.5,
        Arc::clone(&lambert_red),
    )?;

    let world = World::new(vec![
        Box::new(cube),
        Box::new(diamond),
        Box::new(Sphere::new(0.0, 0.0, 2.0, 1.9, Arc::clone(&light))),
        //Box::new(Sphere::new(1.0, 3.0, 2.0, 1.9, Arc::clone(&light))),
        //Box::new(Sphere::new(2.0, 0.9, -0.5, 0.5, Arc::clone(&light))),
        //Box::new(Sphere::new(-2.0, 0.9, -0.5, 0.5, Arc::clone(&light))),
        //Box::new(Sphere::new(
        //    portal_enter_center.x(),
        //    portal_enter_center.y(),
        //    portal_enter_center.z(),
        //    portal_radius,
        //    Arc::clone(&portal),
        //)),
        //Box::new(Sphere::new(
        //    portal2_enter_center.x(),
        //    portal2_enter_center.y(),
        //    portal2_enter_center.z(),
        //    portal_radius,
        //    Arc::clone(&portal2),
        //)),
        //Box::new(Sphere::new(
        //    portal_enter_center.x(),
        //    portal_enter_center.y(),
        //    portal_enter_center.z(),
        //    0.1,
        //    Arc::clone(&metal_yellow),
        //)),
        //Box::new(Sphere::new(
        //    portal_exit_center.x(),
        //    portal_exit_center.y(),
        //    portal_exit_center.z(),
        //    portal_radius,
        //    Arc::clone(&lambert_red),
        //)),
        Box::new(Sphere::new(-0.95, 0.0, -2.0, 0.5, Arc::clone(&metal))),
        Box::new(Sphere::new(0.95, 0.0, -2.0, 0.5, Arc::clone(&lambert_red))),
        Box::new(Sphere::new(0.0, 0.0, -2.0, 0.5, Arc::clone(&metal_yellow))),
        Box::new(Sphere::new(1.4, 1.4, -2.0, 0.3, Arc::clone(&fuzzy_metal))),
        Box::new(Sphere::new(
            0.0,
            -100.5,
            -1.0,
            100.0,
            Arc::clone(&lambert_gray),
        )),
    ]);

    camera.render(20, world)
}
