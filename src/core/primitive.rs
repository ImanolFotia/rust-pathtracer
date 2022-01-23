use crate::core;

pub trait Primitive {
    fn hit(&self, _ray: core::Ray, t_min: f32, t_max: f32/*, record: core::HitRecord*/) -> bool;
} 
