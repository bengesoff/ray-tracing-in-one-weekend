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
