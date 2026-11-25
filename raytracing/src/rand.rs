use crate::Vec3;
use rand;

pub fn random() -> f64 {
    rand::random_range(-1.0..=1.0)
}

pub fn random_from(l: f64, r: f64) -> Vec3 {
    Vec3::new(
        rand::random_range(l..=r),
        rand::random_range(l..=r),
        rand::random_range(l..=r),
    )
}

pub fn random_square() -> Vec3 {
    Vec3::new(random() / 2.0, random() / 2.0, 0.0)
}

pub fn random_vec3() -> Vec3 {
    loop {
        let vec = Vec3::new(random(), random(), random());
        let len = vec.length_squared();
        if len > 1e-160 && len <= 1.0 {
            return vec / len.sqrt();
        }
    }
}

//pub fn random_vec3_on_hemisphere(normal: Vec3) -> Vec3 {
//    let vec = random_vec3();
//    if normal.dot(vec) > 0.0 {
//        vec
//    } else {
//        vec * -1.0
//    }
//}
