use std::{cmp::Ordering, ops::Range, sync::Arc};

use rand::Rng;

use crate::{Aabb, HitResult, Hittable, HittableList, Ray};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounds: Aabb,
}

impl BvhNode {
    pub fn new(
        rng: &mut dyn rand::RngCore,
        mut list: HittableList,
        time_range: Range<f64>,
    ) -> Self {
        Self::from_list(rng, list.list_mut(), time_range)
    }

    fn from_list(
        rng: &mut dyn rand::RngCore,
        list: &mut [Arc<dyn Hittable>],
        time_range: Range<f64>,
    ) -> Self {
        let axis = rng.gen_range(0..=2);

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match list.len() {
            1 => (list[0].clone(), list[0].clone()),
            2 => {
                if Self::box_compare(&list[0], &list[1], axis, time_range.clone()) == Ordering::Less
                {
                    (list[0].clone(), list[1].clone())
                } else {
                    (list[1].clone(), list[0].clone())
                }
            }
            _ => {
                list.sort_by(|a, b| Self::box_compare(a, b, axis, time_range.clone()));
                let mid = list.len() / 2;
                (
                    Arc::new(BvhNode::from_list(
                        rng,
                        &mut list[0..mid],
                        time_range.clone(),
                    )),
                    Arc::new(BvhNode::from_list(
                        rng,
                        &mut list[mid..],
                        time_range.clone(),
                    )),
                )
            }
        };

        const MESSAGE: &'static str = "No bounding box constructing BvhNode - note: need to implement support for items without a bounding box";
        let box_left = left.bounding_box(time_range.clone()).expect(MESSAGE);
        let box_right = right.bounding_box(time_range.clone()).expect(MESSAGE);

        Self {
            left,
            right,
            bounds: Aabb::surround(&box_left, &box_right),
        }
    }

    fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis: usize,
        time_range: Range<f64>,
    ) -> Ordering {
        const MESSAGE: &'static str = "No bounding box comparing hittables - note: need to implement support for items without a bounding box";
        let box_a = a.bounding_box(time_range.clone()).expect(MESSAGE);
        let box_b = b.bounding_box(time_range).expect(MESSAGE);
        box_a.min()[axis]
            .partial_cmp(&box_b.min()[axis])
            .unwrap_or(Ordering::Greater)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        if !self.bounds.hit(ray, t_min, t_max) {
            return None;
        }

        match self.left.hit(ray, t_min, t_max) {
            Some(left_hit) => self
                .right
                .hit(ray, t_min, left_hit.t())
                .or_else(|| Some(left_hit)),
            None => self.right.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, _time_range: Range<f64>) -> Option<Aabb> {
        Some(self.bounds.clone())
    }
}
