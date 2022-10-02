use crate::colour::Colour;
use crate::geometry::ray::Ray;
use crate::materials::material::{random_unit_vector, Material};
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
