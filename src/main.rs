mod camera;
mod geometry;
mod pixel;
mod shapes;
mod surface_interaction;

use crate::pixel::Pixel;

use crate::camera::Camera;
use crate::geometry::point3;
use crate::geometry::ray;
use crate::shapes::shape::{Hittable, Shapes};
use crate::shapes::sphere::Sphere;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = Shapes::new();
    world.add(Box::new(Sphere {
        centre: point3::Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        centre: point3::Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
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
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_height as f64 - 1.0);

            let r = camera.get_ray(u, v);

            let p = ray_colour(&r, &world);
            println!("{}", p);
        }
    }
    eprintln!("Done.");
}

fn ray_colour<T: Hittable>(r: &ray::Ray, world: &T) -> Pixel {
    if let Some((interaction, _)) = world.intersect(r, 0.0, f64::INFINITY) {
        let n = interaction.n;
        0.5 * Pixel {
            r: n.x + 1.0,
            g: n.y + 1.0,
            b: n.z + 1.0,
        }
    } else {
        let unit_direction = r.direction.normalised();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)
            * Pixel {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            }
            + t * Pixel {
                r: 0.5,
                g: 0.7,
                b: 1.0,
            }
    }
}
