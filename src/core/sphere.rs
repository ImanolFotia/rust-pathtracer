
use crate::math;
use crate::core;

pub struct Sphere {
    position: math::Vec3,
    radius: f32
}

impl Sphere {

    pub fn new(p: math::Vec3, r: f32) -> Sphere {
        return Sphere{position: p, radius: r};
    }

    pub fn get_radius(&self) -> f32 {
        return self.radius;
    }
    
    pub fn get_position(&self) -> &math::Vec3 {
        return &self.position;
    }
}

impl core::Primitive for Sphere {
    fn hit(&self, _ray: core::Ray, t_min: f32, t_max: f32/*, record: core::HitRecord*/) -> bool {
        println!("Called Sphere::hit");
        return true;
    }
}