use core::ops::*;
use std::{f32, fmt};

/// A vector in ℝ³.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Vect3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vect3 {
    /// Return the zero-length vector.
    pub fn zero() -> Vect3 {
        Vect3::default()
    }

    /// Return the dot product of two vectors.
    pub fn dot(self, other: Vect3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Return the cross product of two vectors.
    pub fn cross(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    /// Return the norm of the vector.
    pub fn norm(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Return a normalized copy of the vector.
    pub fn normalize(self) -> Vect3 {
        self / self.norm()
    }
}

impl fmt::Display for Vect3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

impl Neg for Vect3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vect3::zero() - self
    }
}

impl Add for Vect3 {
    type Output = Self;

    fn add(self, other: Vect3) -> Self::Output {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vect3 {
    fn add_assign(&mut self, other: Vect3) {
        *self = *self + other
    }
}

impl Sub for Vect3 {
    type Output = Self;

    fn sub(self, other: Vect3) -> Self::Output {
        Vect3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vect3 {
    fn sub_assign(&mut self, other: Vect3) {
        *self = *self - other
    }
}

impl Mul<f32> for Vect3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Vect3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f32> for Vect3 {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other
    }
}

impl Mul<Vect3> for f32 {
    type Output = Vect3;

    fn mul(self, other: Vect3) -> Self::Output {
        other * self
    }
}

impl Div<f32> for Vect3 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: f32) -> Self::Output {
        self * other.recip()
    }
}

impl DivAssign<f32> for Vect3 {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other
    }
}

/// A point in ℝ³.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    /// Return the origin point.
    pub fn zero() -> Point3 {
        Point3::default()
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

impl Add<Vect3> for Point3 {
    type Output = Self;

    fn add(self, other: Vect3) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<Vect3> for Point3 {
    fn add_assign(&mut self, other: Vect3) {
        *self = *self + other
    }
}

impl Sub<Vect3> for Point3 {
    type Output = Self;

    fn sub(self, other: Vect3) -> Self::Output {
        self + (-other)
    }
}

impl SubAssign<Vect3> for Point3 {
    fn sub_assign(&mut self, other: Vect3) {
        *self = *self - other
    }
}

impl Sub for Point3 {
    type Output = Vect3;

    fn sub(self, other: Point3) -> Self::Output {
        Vect3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_vect3_ops() {
        let zero = Vect3::zero();
        let vect1 = Vect3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let vect2 = Vect3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(vect1 - vect1, zero);
        assert_eq!(vect1 + vect1, vect2);
        assert_eq!(2.0 * vect1, vect2);
        assert_eq!(vect1 * 2.0, vect2);
        assert_eq!(vect2 / 2.0, vect1);
        assert_eq!(vect1 - vect2, -vect1);
    }

    #[test]
    fn test_vect3_assign_ops() {
        let orig = Vect3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let mut vect = orig;

        vect += orig;
        assert_eq!(vect, orig + orig);
        vect -= orig;
        assert_eq!(vect, orig);
        vect *= 2.0;
        assert_eq!(vect, orig + orig);
        vect /= 2.0;
        assert_eq!(vect, orig);
    }

    #[test]
    fn test_vect3_norm() {
        let zero = Vect3::zero();
        let vect = Vect3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert_eq!(zero.norm(), 0.0);
        assert_eq!(vect.norm(), 3.0_f32.sqrt());
        use approx::assert_ulps_eq;
        assert_ulps_eq!(vect.normalize().norm(), 1.0);
    }

    #[test]
    fn test_vect3_dot() {
        let zero = Vect3::zero();
        let vect = Vect3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert_eq!(zero.dot(vect), 0.0);
        assert_eq!(vect.dot(zero), 0.0);
        assert_eq!(zero.dot(zero), 0.0);
        assert_eq!(vect.dot(vect), 3.0);
        assert_eq!(vect.dot(2.0 * vect), 6.0);
        assert_eq!((2.0 * vect).dot(vect), 6.0);
    }

    #[test]
    fn test_vect3_cross() {
        let vect1 = Vect3 {
            x: 1.0,
            ..Vect3::zero()
        };
        let vect2 = Vect3 {
            y: 1.0,
            ..Vect3::zero()
        };
        let vect3 = Vect3 {
            z: 1.0,
            ..Vect3::zero()
        };

        assert_eq!(vect1.cross(vect2), vect3);
        assert_eq!(vect2.cross(vect3), vect1);
        assert_eq!(vect3.cross(vect1), vect2);
        assert_eq!(vect2.cross(vect1), -vect3);
        assert_eq!(vect3.cross(vect2), -vect1);
        assert_eq!(vect1.cross(vect3), -vect2);
    }

    #[test]
    fn test_point3_ops() {
        let zero = Point3::zero();
        let point = Point3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let vect = Vect3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert_eq!(zero - point, -vect);
        assert_eq!(point - zero, vect);
        assert_eq!(zero + vect, point);
        assert_eq!(point - vect, zero);
    }

    #[test]
    fn test_point3_assign_ops() {
        let orig = Point3::zero();
        let mut point = orig;
        let vect = Vect3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        point += vect;
        assert_eq!(point, orig + vect);
        point -= vect;
        assert_eq!(point, orig);
    }
}
