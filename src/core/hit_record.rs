use crate::math;
use crate::core;

pub struct HitRecord {
    p: math::Vec3,
    n: math::Vec3,
    mat: core::Material,
    t: f32,
    front_face: bool,
    emmisive: bool
}