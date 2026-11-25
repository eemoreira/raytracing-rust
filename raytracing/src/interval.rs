#[derive(Debug, Copy, Clone)]
pub struct Interval {
    l: f64,
    r: f64,
}

impl Interval {
    pub fn empty() -> Interval {
        Interval {
            l: f64::INFINITY,
            r: f64::INFINITY,
        }
    }
    pub fn inf() -> Interval {
        Interval {
            l: -f64::INFINITY,
            r: f64::INFINITY,
        }
    }
    pub fn new(l: f64, r: f64) -> Interval {
        Interval { l, r }
    }
    pub fn l(self) -> f64 {
        self.l
    }
    pub fn r(self) -> f64 {
        self.r
    }
    pub fn contains(&self, x: f64) -> bool {
        x >= self.l && x <= self.r
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.l {
            self.l
        } else if x > self.r {
            self.r
        } else {
            x
        }
    }
}
