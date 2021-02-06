use crate::{HitResult, Hittable, Ray};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn from(list: Vec<Box<dyn Hittable>>) -> Self {
        Self { list }
    }

    pub fn clear(&mut self) {
        self.list.clear()
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut hit_result: Option<HitResult> = None;
        let mut closest_so_far = t_max;
        for hittable in &self.list {
            if let Some(result) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = result.t();
                hit_result = Some(result);
            }
        }

        hit_result
    }
}
