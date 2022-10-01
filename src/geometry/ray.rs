use super::point3::Point3;
use super::vector3::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub t_max: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin,
            direction,
            t_max: f64::INFINITY,
        }
    }

    pub fn get_point(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_along_ray() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = ray.get_point(4.0);
        assert_eq!(point, Point3::new(4.0, 4.0, 4.0));
    }
}
