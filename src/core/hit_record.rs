use crate::math;
use crate::core;

#[allow(dead_code)]
pub struct HitRecord {
    pub p: math::Vec3,
    pub n: math::Vec3,
    pub mat: core::Material,
    pub t: f32,
    pub front_face: bool,
    pub emmisive: bool
}