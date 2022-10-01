use crate::geometry::ray;
use crate::geometry::ray::Ray;
use crate::surface_interaction::SurfaceInteraction;

pub trait Hittable {
    fn intersect(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, f64)>;
}

pub struct Shapes(Vec<Box<dyn Hittable>>);

impl Shapes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, shape: Box<dyn Hittable>) {
        self.0.push(shape);
    }
}

impl Hittable for Shapes {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, f64)> {
        let mut closest_t = t_max;
        let mut hit: Option<SurfaceInteraction> = None;

        for shape in self.0.iter() {
            if let Some((interaction, t)) = shape.intersect(r, t_min, closest_t) {
                closest_t = t;
                hit = Some(interaction);
            }
        }

        hit.map(|h| (h, closest_t))
    }
}
