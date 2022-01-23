use crate::core;
use crate::math;
use crate::common;

pub struct Material {
    pub emmisive: bool,
    pub albedo: common::Color
}

impl Material {
    pub fn new() -> Material {
        return Material {
            emmisive: false,
            albedo: common::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0
            }
        }
    }
    
    pub fn scatter(&self, r_in: &core::Ray, rec: &core::HitRecord, attenuation: &math::Vec3, scattered: &core::Ray) -> bool {
        return false;
    }


    fn fresnel_schlick(ir: f64, cosTheta: f64) -> f64 {
        let r0: f64 = ((1.0 - ir) / (1.0 + ir)).powf(2.0);
        return r0 + (1.0 - r0) * (1.0 - cosTheta).powf(5.0);
    }
}