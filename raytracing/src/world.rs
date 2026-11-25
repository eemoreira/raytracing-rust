use crate::Interval;
use crate::Ray;
use crate::{Hit, Hittable};

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(scene: Vec<Box<dyn Hittable>>) -> World {
        World { objects: scene }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit> {
        let mut closest: Option<Hit> = None;
        for obj in self.objects.iter() {
            let mx = closest.as_ref().map_or(range.r(), |h| h.t());
            if let Some(h) = obj.hit(ray, Interval::new(range.l(), mx)) {
                closest = Some(h);
            }
        }
        closest
    }
}
