use std::fmt::Formatter;
use std::ops;

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

impl ops::Add<Pixel> for Pixel {
    type Output = Pixel;

    fn add(self, rhs: Pixel) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign<Pixel> for Pixel {
    fn add_assign(&mut self, rhs: Pixel) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Mul<f64> for Pixel {
    type Output = Pixel;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::Mul<Pixel> for f64 {
    type Output = Pixel;

    fn mul(self, rhs: Pixel) -> Self::Output {
        rhs * self
    }
}

impl ops::DivAssign<i32> for Pixel {
    fn div_assign(&mut self, rhs: i32) {
        let scale = 1.0 / (rhs as f64);
        self.r *= scale;
        self.g *= scale;
        self.b *= scale;
    }
}
