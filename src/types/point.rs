use super::Vect3;
use auto_ops::*;
use std::{f32, fmt};

/// A point in ℝ³.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point3(pub f32, pub f32, pub f32);

impl Point3 {
    /// Return the origin point.
    pub fn zero() -> Point3 {
        Point3::default()
    }

    /// The x coordinate of the point.
    pub fn x(&self) -> f32 {
        self.0
    }

    /// The y coordinate of the point.
    pub fn y(&self) -> f32 {
        self.1
    }

    /// The z coordinate of the point.
    pub fn z(&self) -> f32 {
        self.2
    }

    /// Generate a random point in the unit ball.
    #[cfg(test)]
    pub fn sample<R: rand::Rng>(rng: &mut R) -> Point3 {
        use rand_distr::{Distribution, UnitBall};
        let point = UnitBall.sample(rng);
        Point3(point[0], point[1], point[2])
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x(), self.y(), self.z())
    }
}

impl_op_ex!(+= |a: &mut Point3, b: &Vect3| { *a = *a + b; });
impl_op_ex!(+|a: &Point3, b: &Vect3| -> Point3 {
    Point3 (
        a.x() + b.x(), // x
        a.y() + b.y(), // y
        a.z() + b.z(), // z
    )
});

impl_op_ex!(-= |a: &mut Point3, b: &Vect3| { *a = *a - b; });
impl_op_ex!(-|a: &Point3, b: &Vect3| -> Point3 { a + (-b) });

impl_op_ex!(-|a: &Point3, b: &Point3| -> Vect3 {
    Vect3(
        a.x() - b.x(), // x
        a.y() - b.y(), // y
        a.z() - b.z(), // z
    )
});

#[cfg(test)]
impl approx::AbsDiffEq for Point3 {
    type Epsilon = f32;

    fn default_epsilon() -> f32 {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Point3, epsilon: f32) -> bool {
        (self - other).norm() <= epsilon
    }
}

#[cfg(test)]
impl approx::UlpsEq for Point3 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }

    fn ulps_eq(
        &self,
        other: &Point3,
        epsilon: <Point3 as approx::AbsDiffEq>::Epsilon,
        max_ulps: u32,
    ) -> bool {
        f32::ulps_eq(&self.x(), &other.x(), epsilon.clone(), max_ulps)
            && f32::ulps_eq(&self.y(), &other.y(), epsilon.clone(), max_ulps)
            && f32::ulps_eq(&self.z(), &other.z(), epsilon.clone(), max_ulps)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_point3_ops() {
        let zero = Point3::zero();
        let point = Point3(1.0, 1.0, 1.0);
        let vect = Vect3(1.0, 1.0, 1.0);

        assert_eq!(zero - point, -vect);
        assert_eq!(point - zero, vect);
        assert_eq!(zero + vect, point);
        assert_eq!(point - vect, zero);
    }

    #[test]
    fn test_point3_assign_ops() {
        let orig = Point3::zero();
        let mut point = orig;
        let vect = Vect3(1.0, 1.0, 1.0);

        point += vect;
        assert_eq!(point, orig + vect);
        point -= vect;
        assert_eq!(point, orig);
    }
}
