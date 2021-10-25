use auto_ops::*;
use std::{f32, fmt};

/// A vector in ℝ³.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vect3 {
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

impl_op_ex!(-|a: &Vect3| -> Vect3 { Vect3::zero() - a });

impl_op_ex!(+= |a: &mut Vect3, b: &Vect3| { *a = *a + b; });
impl_op_ex!(+|a: &Vect3, b: &Vect3| -> Vect3 {
    Vect3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(-= |a: &mut Vect3, b: &Vect3| { *a = *a - b; });
impl_op_ex!(-|a: &Vect3, b: &Vect3| -> Vect3 {
    Vect3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(*= |a: &mut Vect3, b: &f32| { *a = *a * b; });
impl_op_ex_commutative!(*|a: &Vect3, b: &f32| -> Vect3 {
    Vect3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op_ex!(/= |a: &mut Vect3, b: &f32| { *a = *a / b; });
impl_op_ex!(/|a: &Vect3, b: &f32| -> Vect3 { a * b.recip()});

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
}
