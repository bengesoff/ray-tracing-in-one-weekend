use crate::colour::Colour;
use crate::geometry::ray::Ray;
use crate::materials::material::Material;
use crate::surface_interaction::SurfaceInteraction;

pub struct MetalMaterial {
    albedo: Colour,
}

impl MetalMaterial {
    pub fn new(colour: Colour) -> Self {
        Self { albedo: colour }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, interaction: &SurfaceInteraction) -> Option<(Ray, Colour)> {
        let reflected = ray_in.direction.reflect(interaction.n.to_vector());
        let scattered = Ray::new(interaction.p, reflected);
        if scattered.direction.dot(interaction.n.to_vector()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
