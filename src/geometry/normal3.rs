use super::vector3::Vector3;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Normal3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Normal3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn to_vector(self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn from_vector(v: Vector3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn quadrance(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.quadrance().sqrt()
    }

    pub fn normalised(self) -> Self {
        self / self.length()
    }
}

impl ops::Add<Normal3> for Normal3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Normal3> for Normal3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Normal3> for Normal3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Normal3> for Normal3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<f64> for Normal3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Normal3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Normal3> for f64 {
    type Output = Normal3;

    fn mul(self, rhs: Normal3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Normal3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let divisor = 1.0 / rhs;
        self * divisor
    }
}

impl ops::DivAssign<f64> for Normal3 {
    fn div_assign(&mut self, rhs: f64) {
        let divisor = 1.0 / rhs;
        self.x *= divisor;
        self.y *= divisor;
        self.z *= divisor;
    }
}

impl ops::Neg for Normal3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vectors() {
        let v1 = Normal3::new(1.0, 2.0, 3.0);
        let v2 = Normal3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Normal3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn vector_length() {
        let v1 = Normal3::new(3.0, 4.0, 0.0);
        let v2 = Normal3::new(0.0, -4.0, 3.0);
        assert_eq!(v1.length(), 5.0);
        assert_eq!(v2.length(), 5.0)
    }
}
