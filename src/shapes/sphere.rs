use super::shape::Shape;
use crate::geometry::point3;
use crate::geometry::ray::Ray;

pub struct Sphere {
    pub centre: point3::Point3,
    pub radius: f64,
}

impl Shape for Sphere {
    fn intersects(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
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

        Some(root)
    }
}
