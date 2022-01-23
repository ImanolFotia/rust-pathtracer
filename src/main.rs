mod common;
mod math; 
mod core;

fn ray_color(ray: core::Ray, world: &dyn core::Primitive) {
    println!("Ray origin: {}, {}, {}", ray.origin.x, ray.origin.y, ray.origin.z);
    println!("Ray direction: {}, {}, {}", ray.direction.x, ray.direction.y, ray.direction.z);

    println!("World: {}", world.hit(ray, 1.0, 1000.0));

}

fn main() {
    let r = core::Ray {
        origin: math::Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0
        },
        direction: math::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0
        },
        time: 0.0
    };

    let mut primitives: Vec<Box<dyn core::Primitive>> = Vec::new();

    let boxed_sphere: Box<core::Sphere> = Box::new(core::Sphere::new(math::Vec3{x: 5.0, y: 8.0, z: 45.0}, 1.0));

    primitives.push(boxed_sphere);

    ray_color(r, &*primitives[0]);

    let out_image: common::PPM = common::PPM::new(1280, 720, 256);
    //out_image.write_color();
    out_image.write_out();
    println!("Hello, world!");
}
