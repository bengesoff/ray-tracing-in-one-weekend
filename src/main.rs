mod pixel;

use crate::pixel::Pixel;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..image_width {
            let p = Pixel {
                r: (i as f64) / (image_width as f64 - 1.0),
                g: (j as f64) / (image_height as f64 - 1.0),
                b: 0.25,
            };

            println!("{}", p)
        }
    }
    eprintln!("Done.");
}
