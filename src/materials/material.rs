use crate::colour::Colour;
use crate::geometry::point3::Point3;
use crate::geometry::ray::Ray;
use crate::geometry::vector3::Vector3;
use crate::surface_interaction::SurfaceInteraction;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, interaction: &SurfaceInteraction) -> Option<(Ray, Colour)>;
}

pub fn random_unit_vector() -> Vector3 {
    (random_in_unit_sphere() - Point3::ORIGIN).normalised()
}

pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let p = Point3 {
            x: -1.0 + 2.0 * rand::random::<f64>(),
            y: -1.0 + 2.0 * rand::random::<f64>(),
            z: -1.0 + 2.0 * rand::random::<f64>(),
        };
        if (p - Point3::ORIGIN).quadrance() <= 1.0 {
            return p;
        }
    }
}
