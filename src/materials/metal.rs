use crate::colour::Colour;
use crate::geometry::point3::Point3;
use crate::geometry::ray::Ray;
use crate::materials::material::{random_in_unit_sphere, Material};
use crate::surface_interaction::SurfaceInteraction;

pub struct MetalMaterial {
    albedo: Colour,
    roughness: f64,
}

impl MetalMaterial {
    pub fn new(colour: Colour, roughness: f64) -> Self {
        Self {
            albedo: colour,
            roughness,
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, interaction: &SurfaceInteraction) -> Option<(Ray, Colour)> {
        let reflected = ray_in.direction.reflect(interaction.n.to_vector());
        let perturbation = self.roughness * (random_in_unit_sphere() - Point3::ORIGIN);
        let scattered = Ray::new(interaction.p, reflected + perturbation);
        if scattered.direction.dot(interaction.n.to_vector()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
