mod common;
mod core;
mod math;

fn ray_color(ray: core::Ray, world: &dyn core::Primitive, depth: i32) -> math::Vec3 {
    let hit_record = core::HitRecord {
        p: math::Vec3{x: 0.0, y: 0.0, z: 0.0},
        n: math::Vec3{x: 0.0, y: 0.0, z: 0.0},
        mat: core::materials::Material {
            emmisive: false,
            albedo: common::Color {
                r: 0.0,
                g: 0.0, 
                b: 0.0
            }
        },
        t: 0.0,
        front_face: false,
        emmisive: false
    };

    if depth <= 0 {
        return math::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    if world.hit(ray, 0.001, math::C_INFINITY, hit_record){}

    return math::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 
}

fn main() {
    let r = core::Ray {
        origin: math::Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        },
        direction: math::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        time: 0.0,
    };

    let depth: i32 = 200;

    let mut primitives: Vec<Box<dyn core::Primitive>> = Vec::new();

    let boxed_sphere: Box<core::Sphere> = Box::new(core::Sphere::new(
        math::Vec3 {
            x: 5.0,
            y: 8.0,
            z: 45.0,
        },
        1.0,
    ));

    primitives.push(boxed_sphere);

    ray_color(r, &*primitives[0], depth);

    let out_image: common::PPM = common::PPM::new(1280, 720, 256);
    //out_image.write_color();
    out_image.write_out();
    println!("Hello, world!");
}
