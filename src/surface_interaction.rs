use crate::geometry::normal3::Normal3;
use crate::geometry::point3::Point3;
use crate::geometry::ray::Ray;

pub struct SurfaceInteraction {
    pub p: Point3,
    pub n: Normal3,
    pub front_face: bool,
}

impl SurfaceInteraction {
    pub fn new(p: Point3, r: &Ray, n: Normal3) -> Self {
        let mut s = Self {
            p,
            n,
            front_face: false,
        };
        s.set_normal(r, n);
        s
    }

    pub fn set_normal(&mut self, r: &Ray, outward_normal: Normal3) {
        self.front_face = r.direction.dot(outward_normal.to_vector()) < 0.0;
        self.n = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
