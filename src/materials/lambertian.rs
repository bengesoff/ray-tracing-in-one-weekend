use crate::colour::Colour;
use crate::geometry::point3::Point3;
use crate::geometry::ray::Ray;
use crate::geometry::vector3::Vector3;
use crate::materials::material::Material;
use crate::surface_interaction::SurfaceInteraction;

pub struct LambertianMaterial {
    albedo: Colour,
}

impl LambertianMaterial {
    pub fn new(colour: Colour) -> Self {
        Self { albedo: colour }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _: &Ray, interaction: &SurfaceInteraction) -> Option<(Ray, Colour)> {
        let mut scatter_direction = interaction.n.to_vector() + random_unit_vector();

        // check that the random vector isn't the opposite of the normal vector otherwise it could
        // cause infinities and NaNs
        if scatter_direction.almost_zero() {
            scatter_direction = interaction.n.to_vector();
        }

        let scattered = Ray::new(interaction.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

fn random_unit_vector() -> Vector3 {
    (random_in_unit_sphere() - Point3::ORIGIN).normalised()
}

fn random_in_unit_sphere() -> Point3 {
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
