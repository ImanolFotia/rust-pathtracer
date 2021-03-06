pub mod ray;
pub mod primitive;
pub mod hit_record;
pub mod materials;
pub mod sphere;

pub use ray::Ray as Ray;
pub use primitive::Primitive as Primitive;
pub use hit_record::HitRecord as HitRecord;
pub use materials::Material as Material;
pub use sphere::Sphere as Sphere;