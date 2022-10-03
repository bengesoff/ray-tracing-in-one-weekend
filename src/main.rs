mod camera;
mod colour;
mod geometry;
mod materials;
mod shapes;
mod surface_interaction;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::geometry::point3;
use crate::geometry::ray::Ray;
use crate::materials::dielectric::DielectricMaterial;
use crate::materials::lambertian::LambertianMaterial;
use crate::materials::metal::MetalMaterial;
use crate::shapes::shape::{Hittable, Shapes};
use crate::shapes::sphere::Sphere;
use std::rc::Rc;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50 as u8;

    // World
    let mut world = Shapes::new();
    // ground
    world.add(Box::new(Sphere {
        centre: point3::Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: Rc::new(LambertianMaterial::new(Colour {
            r: 0.8,
            g: 0.8,
            b: 0.0,
        })),
    }));
    // centre
    world.add(Box::new(Sphere {
        centre: point3::Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Rc::new(DielectricMaterial::new(1.5)),
    }));
    // left
    world.add(Box::new(Sphere {
        centre: point3::Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Rc::new(DielectricMaterial::new(1.5)),
    }));
    world.add(Box::new(Sphere {
        centre: point3::Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Rc::new(MetalMaterial::new(
            Colour {
                r: 0.8,
                g: 0.6,
                b: 0.2,
            },
            1.0,
        )),
    }));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..image_width {
            let mut p = Colour {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand::random::<f64>()) / (image_height as f64 - 1.0);

                let r = camera.get_ray(u, v);

                p += ray_colour(&r, &world, max_depth);
            }

            p /= samples_per_pixel;
            p.gamma_correct();

            println!("{}", p);
        }
    }
    eprintln!("Done.");
}

fn ray_colour<T: Hittable>(r: &Ray, world: &T, depth: u8) -> Colour {
    // limit recursion
    if depth == 0 {
        return Colour {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    // ignore ray hits very close to ray origin by using t_min of 0.001 instead of 0. Fixes "shadow
    // acne" problem
    if let Some((interaction, _)) = world.intersect(r, 0.001, f64::INFINITY) {
        if let Some((scattered_ray, attenuation)) = interaction.material.scatter(r, &interaction) {
            attenuation * ray_colour(&scattered_ray, world, depth - 1)
        } else {
            Colour {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }
        }
    } else {
        let unit_direction = r.direction.normalised();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)
            * Colour {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            }
            + t * Colour {
                r: 0.5,
                g: 0.7,
                b: 1.0,
            }
    }
}
