use super::shape::Hittable;
use crate::geometry::ray::Ray;
use crate::geometry::{normal3, point3};
use crate::surface_interaction::SurfaceInteraction;

pub struct Sphere {
    pub centre: point3::Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(SurfaceInteraction, f64)> {
        let o_c = r.origin - self.centre;
        let a = r.direction.quadrance();
        let half_b = r.direction.dot(o_c);
        let c = o_c.quadrance() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (-half_b - discriminant_sqrt) / a;
        // check root lies in range, then try other root if not
        if root < t_min || root > t_max {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = r.get_point(root);
        Some((
            SurfaceInteraction::new(
                hit_point,
                r,
                normal3::Normal3::from_vector((hit_point - self.centre) / self.radius),
            ),
            root,
        ))
    }
}
