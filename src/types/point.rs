use super::Vect3;
use core::ops::*;
use std::{f32, fmt};

/// A point in ℝ³.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point3 {
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
