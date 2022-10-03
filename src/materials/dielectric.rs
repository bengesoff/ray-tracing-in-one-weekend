use crate::colour::Colour;
use crate::geometry::normal3::Normal3;
use crate::geometry::ray::Ray;
use crate::geometry::vector3::Vector3;
use crate::materials::material::Material;
use crate::surface_interaction::SurfaceInteraction;

pub struct DielectricMaterial {
    index_of_refraction: f64,
}

impl DielectricMaterial {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, interaction: &SurfaceInteraction) -> Option<(Ray, Colour)> {
        let refraction_ratio = if interaction.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let refracted = refract(
            ray_in.direction.normalised(),
            interaction.n,
            refraction_ratio,
        );
        let scattered = Ray::new(interaction.p, refracted);
        Some((
            scattered,
            Colour {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        ))
    }
}

fn refract(direction_in: Vector3, normal: Normal3, refraction_ratio: f64) -> Vector3 {
    let cos_theta = f64::min((-direction_in).dot(normal.to_vector()), 1.0);
    let out_ray_perpendicular = refraction_ratio * (direction_in + cos_theta * normal.to_vector());
    let out_ray_parallel =
        -(1.0 - out_ray_perpendicular.quadrance()).abs().sqrt() * normal.to_vector();
    out_ray_perpendicular + out_ray_parallel
}
