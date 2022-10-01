use crate::geometry::ray;
use crate::surface_interaction::SurfaceInteraction;

pub trait Shape {
    fn intersect(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<SurfaceInteraction>;
}
