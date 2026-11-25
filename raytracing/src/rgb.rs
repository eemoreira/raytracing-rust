use crate::Interval;
use crate::Vec3;
use std::fmt;
use std::io::Write;

#[derive(Clone, Debug, Copy)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

fn linear_to_gamma(x: f64) -> f64 {
    if x > 0.0 {
        x.sqrt()
    } else {
        x
    }
}

impl Rgb {
    pub fn to_rgb(v: Vec3) -> Rgb {
        let interval = Interval::new(0.0, 0.999);
        Rgb {
            r: (256.0 * interval.clamp(linear_to_gamma(v.x()))) as u8,
            g: (256.0 * interval.clamp(linear_to_gamma(v.y()))) as u8,
            b: (256.0 * interval.clamp(linear_to_gamma(v.z()))) as u8,
        }
    }
    pub fn write(self, out: &mut impl Write) -> std::io::Result<()> {
        writeln!(out, "{}", self)
    }
    pub fn r(self) -> u8 {
        self.r
    }
    pub fn g(self) -> u8 {
        self.g
    }
    pub fn b(self) -> u8 {
        self.b
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
