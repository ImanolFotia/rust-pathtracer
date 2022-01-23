use crate::math;

pub struct Ray {
    pub origin: math::Vec3,
    pub direction: math::Vec3,
    pub time: f32
}