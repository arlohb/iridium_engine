use std::ops;

/// Represents a point, vector, or normal in 3D space.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
    /// A tiny value used when checking for equality.
    const EPSILON: f32 = 0.000001;

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn as_bytes<const N: usize>(&self) -> [u8; N] {
        let mut bytes = [0u8; N];

        [
            self.x.to_le_bytes(),
            self.y.to_le_bytes(),
            self.z.to_le_bytes(),
        ]
            .concat()
            .into_iter()
            .enumerate()
            .for_each(|(i, b)| bytes[i] = b);
        
        bytes
    }

    /// The length of a vector.
    pub fn length(&self) -> f32 {
        ((self.x.powi(2)) + (self.y.powi(2)) + (self.z.powi(2))).sqrt()
    }

    /// The dot product between self and v.
    pub fn dot(&self, v: Vec3) -> f32 {
        (self.x * v.x) 
        + (self.y * v.y)
        + (self.z * v.z)
    }

    /// Change this vector to have a length of 1
    pub fn normalize(&self) -> Vec3 {
        let length_squared = self.dot(*self);

        if length_squared > 0. {
            let inverse_length = 1. / length_squared.sqrt();
            return Vec3 {
                x: self.x * inverse_length,
                y: self.y * inverse_length,
                z: self.z * inverse_length,
            };
        }

        *self
    }

    /// Gets the fractional part of each component.
    pub fn fract(&self) -> Vec3 {
        Vec3 {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract(),
        }
    }

    /// Gets the absolute value of each component
    pub fn abs(&self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        (self.x - other.x).abs() < Vec3::EPSILON
        && (self.y - other.y).abs() < Vec3::EPSILON
        && (self.z - other.z).abs() < Vec3::EPSILON
    }
}
