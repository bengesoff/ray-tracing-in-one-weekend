fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r: f64 = (i as f64) / (image_width as f64 - 1.0);
            let g: f64 = (j as f64) / (image_height as f64 - 1.0);
            let b: f64 = 0.25;

            let ir = (r * 255.0) as u8;
            let ig = (g * 255.0) as u8;
            let ib = (b * 255.0) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
