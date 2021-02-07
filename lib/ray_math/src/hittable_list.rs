use std::{ops::Range, sync::Arc};

use crate::{Aabb, HitResult, Hittable, Ray};

pub struct HittableList {
    list: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn from(list: Vec<Arc<dyn Hittable>>) -> Self {
        Self { list }
    }

    pub fn clear(&mut self) {
        self.list.clear()
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.list.push(hittable)
    }

    pub fn list(&self) -> &[Arc<dyn Hittable>] {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut [Arc<dyn Hittable>] {
        &mut self.list
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

    fn bounding_box(&self, time_range: Range<f64>) -> Option<Aabb> {
        if self.list.is_empty() {
            return None;
        }

        let mut result: Option<Aabb> = None;

        for hittable in &self.list {
            match hittable.bounding_box(time_range.clone()) {
                Some(aabb) => {
                    result = result
                        .map(|res| Aabb::surround(&res, &aabb))
                        .or_else(|| Some(aabb));
                }
                None => return None,
            }
        }

        result
    }
}
