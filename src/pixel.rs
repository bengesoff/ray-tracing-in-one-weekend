use std::fmt::Formatter;

pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }
}
