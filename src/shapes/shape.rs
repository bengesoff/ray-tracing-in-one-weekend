use crate::geometry::ray;

pub trait Shape {
    fn intersects(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<f64>;
}
