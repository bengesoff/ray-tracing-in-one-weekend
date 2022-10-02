use crate::colour::Colour;
use crate::geometry::ray::Ray;
use crate::surface_interaction::SurfaceInteraction;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, interaction: &SurfaceInteraction) -> Option<(Ray, Colour)>;
}
