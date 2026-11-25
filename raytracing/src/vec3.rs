use std::ops;

#[derive(Clone, Debug, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn new_eq(t: f64) -> Vec3 {
        Vec3 { x: t, y: t, z: t }
    }
    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }
    pub fn close_to(self, x: f64) -> bool {
        self.x.abs() < x && self.y.abs() < x && self.z.abs() < x
    }
    pub fn zero(self) -> bool {
        self.close_to(1e-9)
    }
    pub fn reflect_across_normal(self, axis: Vec3) -> Vec3 {
        // axis is assumed to have unit length
        let proj = self.dot(axis);
        self - axis * (proj * 2.0)
    }
    pub fn rotate_around(self, axis: Vec3) -> Vec3 {
        // axis is assumed to have unit lenght
        let proj = self.dot(axis);
        self - axis * (proj * 2.0)
    }
    pub fn rotate_x(self, a: f64) -> Vec3 {
        let (s, c) = a.sin_cos();
        Vec3::new(
            self.x(),
            self.y() * c - self.z() * s,
            self.y() * s + self.z() * c,
        )
    }
    pub fn rotate_y(self, a: f64) -> Vec3 {
        let (s, c) = a.sin_cos();
        Vec3::new(
            self.x() * c + self.z() * s,
            self.y(),
            -self.x() * s + self.z() * c,
        )
    }
    pub fn rotate_z(self, a: f64) -> Vec3 {
        let (s, c) = a.sin_cos();
        Vec3::new(
            self.x() * c - self.y() * s,
            self.x() * s + self.y() * c,
            self.z(),
        )
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<T: Iterator<Item = Vec3>>(iter: T) -> Vec3 {
        iter.fold(Vec3::new_eq(0.0), |acc, v| acc + v)
    }
}
