mod geometry;
mod pixel;

use crate::pixel::Pixel;

use crate::geometry::point3;
use crate::geometry::ray;
use crate::geometry::vector3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = point3::Point3::ORIGIN;
    let horizontal = vector3::Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = vector3::Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - (horizontal / 2.0)
        - (vertical / 2.0)
        - vector3::Vector3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..image_width {
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_height as f64 - 1.0);

            let r = ray::Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let p = ray_colour(&r);
            println!("{}", p);
        }
    }
    eprintln!("Done.");
}

fn ray_colour(r: &ray::Ray) -> Pixel {
    let centre = point3::Point3::new(0.0, 0.0, -1.0);
    if let Some(t) = hits_sphere(&centre, 0.5, r) {
        let n = (r.get_point(t) - centre).normalised();
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

fn hits_sphere(centre: &point3::Point3, radius: f64, r: &ray::Ray) -> Option<f64> {
    let o_c = r.origin - *centre;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * r.direction.dot(o_c);
    let c = o_c.dot(o_c) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    } else {
        None
    }
}
