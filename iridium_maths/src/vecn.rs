use std::ops;

/// Represents a Vector with N dimensions.
#[derive(Clone, Copy)]
pub struct VecN<const N: usize> {
    /// The components of the vector.
    data: [f32; N],
}

impl<const N: usize> std::fmt::Debug for VecN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VecN<{}>({:?})", N, self.data)
    }
}

impl VecN<2> {
    //// Gets the x component of the vector.
    /// 
    /// No point returning a reference as its just a f32.
    pub fn x(&self) -> f32 { self.data[0] }

    /// Gets the x component of the vector as a mutable reference.
    pub fn x_mut(&mut self) -> &mut f32 { self.data.get_mut(0).unwrap() }

    /// Gets the y component of the vector.
    /// 
    /// No point returning a reference as its just a f32.
    pub fn y(&self) -> f32 { self.data[1] }

    /// Gets the y component of the vector as a mutable reference.
    pub fn y_mut(&mut self) -> &mut f32 { self.data.get_mut(1).unwrap() }
}

impl VecN<3> {
    /// Gets the x component of the vector.
    /// 
    /// No point returning a reference as its just a f32.
    pub fn x(&self) -> f32 { self.data[0] }

    /// Gets the x component of the vector as a mutable reference.
    pub fn x_mut(&mut self) -> &mut f32 { self.data.get_mut(0).unwrap() }

    /// Gets the y component of the vector.
    /// 
    /// No point returning a reference as its just a f32.
    pub fn y(&self) -> f32 { self.data[1] }

    /// Gets the y component of the vector as a mutable reference.
    pub fn y_mut(&mut self) -> &mut f32 { self.data.get_mut(1).unwrap() }

    /// Gets the z component of the vector.
    /// 
    /// No point returning a reference as its just a f32.
    pub fn z(&self) -> f32 { self.data[2] }

    /// Gets the z component of the vector as a mutable reference.
    pub fn z_mut(&mut self) -> &mut f32 { self.data.get_mut(2).unwrap() }
}

impl<const N: usize> std::fmt::Display for VecN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<const N: usize> VecN<N> {
    /// A tiny value used when checking for equality.
    const EPSILON: f32 = 0.000001;

    /// Creates a new `VecN` with the given values.
    pub fn new(data: [f32; N]) -> Self {
        VecN { data }
    }

    /// Converts to bytes with the given size.
    pub fn as_bytes<const L: usize>(&self) -> [u8; L] {
        let mut bytes = [0u8; L];

        self.data.iter()
            .flat_map(|value| value.to_le_bytes())
            .enumerate()
            .for_each(|(index, b)| bytes[index] = b);
        
        bytes
    }

    /// Makes a new `VecN` from a string.
    /// 
    /// The string should be in the form `[x, y, z, ...]`.
    /// 
    /// Leading commas are valid.
    pub fn from_string(mut str: &str) -> Option<Self> {
        if !str.starts_with('[') {
            return None;
        }
        str = str.trim_start_matches('[');

        if !str.ends_with(']') {
            return None;
        }
        str = str.trim_end_matches(']');

        str = str.trim();

        if str.ends_with(',') {
            str = str.trim_end_matches(',');
        }

        let string: String = str.chars().filter(|c|
            *c != ' '
        ).collect();

        if string.split(',').count() != N {
            return None;
        }

        let mut data = [0.; N];

        for (index, value) in string.split(',').enumerate() {
            data[index] = value.parse::<f32>().ok()?;
        }

        Some(VecN::new(data))
    }

    /// The length of a vector.
    pub fn length(&self) -> f32 {
        self.data.iter()
            .map(|x| x.powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// The dot product between self and v.
    pub fn dot(&self, v: VecN<N>) -> f32 {
        self.data.iter()
            .enumerate()
            .map(|(i, x)| x * v.data[i])
            .sum::<f32>()
    }

    /// Change this vector to have a length of 1
    pub fn normalize(&self) -> VecN<N> {
        let length_squared = self.dot(*self);

        if length_squared > 0. {
            let inverse_length = 1. / length_squared.sqrt();

            let mut values = self.data;

            for v in &mut values {
                *v *= inverse_length;
            }

            VecN::new(values)
        } else {
            *self
        }
    }

    /// Gets the fractional part of each component.
    pub fn fract(&self) -> VecN<N> {
        let mut values = self.data;

        for v in &mut values {
            *v = v.fract();
        }

        VecN::new(values)
    }

    /// Gets the absolute value of each component
    pub fn abs(&self) -> VecN<N> {
        let mut values = self.data;

        for v in &mut values {
            *v = v.abs();
        }

        VecN::new(values)
    }
}

impl<const N: usize> ops::Add for VecN<N> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        for (i, v) in self.data.iter_mut().enumerate() {
            *v += rhs.data[i];
        }

        self
    }
}

impl<const N: usize> ops::AddAssign for VecN<N> {
    fn add_assign(&mut self, rhs: Self) {
        for (i, v) in self.data.iter_mut().enumerate() {
            *v += rhs.data[i];
        }
    }
}

impl<const N: usize> ops::Sub for VecN<N> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        for (i, v) in self.data.iter_mut().enumerate() {
            *v -= rhs.data[i];
        }

        self
    }
}

impl<const N: usize> ops::SubAssign for VecN<N> {
    fn sub_assign(&mut self, rhs: Self) {
        for (i, v) in self.data.iter_mut().enumerate() {
            *v -= rhs.data[i];
        }
    }
}

impl<const N: usize> ops::Neg for VecN<N> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for v in &mut self.data {
            *v = -*v;
        }

        self
    }
}

impl ops::Mul<VecN<3>> for VecN<3> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        VecN::new([
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        ])
    }
}

impl ops::MulAssign<VecN<3>> for VecN<3> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = VecN::new([
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        ])
    }
}

impl<const N: usize> ops::Mul<f32> for VecN<N> {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self {
        for v in self.data.iter_mut() {
            *v *= rhs;
        }

        self
    }
}

impl<const N: usize> ops::MulAssign<f32> for VecN<N> {
    fn mul_assign(&mut self, rhs: f32) {
        for v in self.data.iter_mut() {
            *v *= rhs;
        }
    }
}

impl<const N: usize> ops::Div<f32> for VecN<N> {
    type Output = Self;
    fn div(mut self, rhs: f32) -> Self {
        for v in self.data.iter_mut() {
            *v /= rhs;
        }

        self
    }
}

impl<const N: usize> ops::DivAssign<f32> for VecN<N> {
    fn div_assign(&mut self, rhs: f32) {
        for v in self.data.iter_mut() {
            *v /= rhs;
        }
    }
}

impl<const N: usize> PartialEq for VecN<N> {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter()
            .enumerate()
            .all(|(i, v)|
                (v - other.data[i]).abs() < VecN::<N>::EPSILON
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let v = VecN::new([1., 2., 3.]);

        assert_eq!(format!("{:?}", v), "VecN<3>([1.0, 2.0, 3.0])");
    }

    #[test]
    fn test_new() {
        let v = VecN::new([1., 2., 3.]);

        assert_eq!(v.data, [1., 2., 3.]);
    }

    #[test]
    fn test_as_bytes() {
        let v = VecN::new([1., 2., 3.]);

        let bytes = [1f32.to_le_bytes(), 2f32.to_le_bytes(), 3f32.to_le_bytes()]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(v.as_bytes::<12>().to_vec(), bytes);
    }

    #[test]
    fn test_length() {
        let v = VecN::new([1., 2., 3.]);

        assert_eq!(v.length(), (1f32.powi(2) + 2f32.powi(2) + 3f32.powi(2)).sqrt());
    }

    #[test]
    fn test_dot() {
        let v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([4., 5., 6.]);

        assert_eq!(v1.dot(v2), 32.);
    }

    #[test]
    fn test_normalize() {
        let v = VecN::new([1., 2., 3.]);
        let v_len = v.length();
        let v_norm = v.normalize();

        assert!((v_norm.length() - 1.).abs() < VecN::<3>::EPSILON);
        assert_eq!(v_norm, v / v_len);
    }

    #[test]
    fn test_fract() {
        let v = VecN::new([1.1, 2.2, 3.3]);

        assert_eq!(v.fract(), VecN::new([0.1, 0.2, 0.3]));
    }

    #[test]
    fn test_abs() {
        let v = VecN::new([-1., 2., -3.]);

        assert_eq!(v.abs(), VecN::new([1., 2., 3.]));
    }

    #[test]
    fn test_add() {
        let v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([4., 5., 6.]);

        assert_eq!(v1 + v2, VecN::new([5., 7., 9.]));
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([4., 5., 6.]);

        v1 += v2;

        assert_eq!(v1, VecN::new([5., 7., 9.]));
    }

    #[test]
    fn test_sub() {
        let v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([4., 5., 6.]);

        assert_eq!(v1 - v2, VecN::new([-3., -3., -3.]));
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([4., 5., 6.]);

        v1 -= v2;

        assert_eq!(v1, VecN::new([-3., -3., -3.]));
    }

    #[test]
    fn test_neg() {
        let v = VecN::new([1., -2., 3.]);

        assert_eq!(-v, VecN::new([-1., 2., -3.]));
    }

    #[test]
    fn test_mul_vecn() {
        let v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([4., 5., 6.]);

        assert_eq!(v1 * v2, VecN::new([-3., 6., -3.]));
    }

    #[test]
    fn test_mul_assign_vecn() {
        let mut v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([4., 5., 6.]);

        v1 *= v2;

        assert_eq!(v1, VecN::new([-3., 6., -3.]));
    }

    #[test]
    fn test_mul_f32() {
        let v = VecN::new([1., 2., 3.]);

        assert_eq!(v * 2., VecN::new([2., 4., 6.]));
    }

    #[test]
    fn test_mul_assign_f32() {
        let mut v = VecN::new([1., 2., 3.]);

        v *= 2.;

        assert_eq!(v, VecN::new([2., 4., 6.]));
    }

    #[test]
    fn test_div_f32() {
        let v = VecN::new([1., 2., 3.]);

        assert_eq!(v / 2., VecN::new([0.5, 1., 1.5]));
    }

    #[test]
    fn test_div_assign_f32() {
        let mut v = VecN::new([1., 2., 3.]);

        v /= 2.;

        assert_eq!(v, VecN::new([0.5, 1., 1.5]));
    }

    #[test]
    fn test_eq() {
        let v1 = VecN::new([1., 2., 3.]);
        let v2 = VecN::new([1., 2., 3.]);

        assert!(v1 == v2);
    }
}
