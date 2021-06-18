use crate::hittable::{Hittable, WithHittableTrait};
use crate::onb::Onb;
use crate::vec3::{Point3, Vec3};

use sdl2::rect::Point;
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f32;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    pub uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: Onb::new_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f32 {
        let cosine = direction.unit().dot(&self.uvw.w);
        return if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        };
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_from_vec(&Vec3::random_cosine_direction())
    }
}

pub struct HittablePdf<'a> {
    pub object: &'a WithHittableTrait,
    pub origin: Point3,
}

impl<'a> HittablePdf<'a> {
    pub fn new(object: &'a WithHittableTrait, origin: Point3) -> Self {
        Self { object, origin }
    }
}

impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: &Vec3) -> f32 {
        self.object.pdf_value(&self.origin, &direction)
    }

    fn generate(&self) -> Vec3 {
        self.object.random(&self.origin)
    }
}
