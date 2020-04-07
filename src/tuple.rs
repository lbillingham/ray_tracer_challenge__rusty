use std::cmp::PartialEq;
use std::f64;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = f64::EPSILON;

    #[test]
    fn it_can_be_accessed_by_components() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_abs_diff_eq!(a.x, 4.3, epsilon = EPS);
        assert_abs_diff_eq!(a.y, -4.2, epsilon = EPS);
        assert_abs_diff_eq!(a.z, 3.1, epsilon = EPS);
        assert_abs_diff_eq!(a.w, 1.0, epsilon = EPS);
    }

    #[test]
    fn it_supports_comparison_equality() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        let b = tuple(4.3, -4.2, 3.1, 1.0);
        assert!(a == b);
    }

    #[test]
    fn it_supports_comparison_inequality() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        let b = tuple(1.0, 1.0, 1.0, 0.0);
        assert!(a != b);
    }

    #[test]
    fn it_can_be_instantiated_via_point() {
        let apoint = point(4.0, -4.0, 3.0);
        let ref_tuple = tuple(4.0, -4.0, 3.0, 1.0);
        assert!(apoint.is_point());
        assert!(apoint == ref_tuple);
    }

    #[test]
    fn it_can_be_instantiated_via_vector_point() {
        let avec = vector(4.0, -4.0, 3.0);
        let ref_tuple = tuple(4.0, -4.0, 3.0, 0.0);
        assert!(avec.is_vector());
        assert!(avec == ref_tuple);
    }
    #[test]
    fn given_w_is_1_it_is_a_point_and_not_a_vector() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn given_w_is_0_it_is_a_vector_and_not_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert!(a.is_vector());
        assert!(!a.is_point());
    }

    #[test]
    fn can_add_a_vector_to_a_point() {
        let v1 = vector(3.0, -2.0, 5.0);
        let p2 = point(-2.0, 3.0, 1.0);
        let expected = tuple(1., 1., 6., 1.);
        assert_eq!(v1 + p2, expected);
        assert_eq!(p2 + v1, expected);
        assert!((p2 + v1).is_point());
    }

    #[test]
    fn can_add_2_vectors() {
        let v1 = vector(3.0, -2.0, 5.0);
        let v2 = vector(-2.0, 3.0, 1.0);
        let expected = tuple(1., 1., 6., 0.);
        assert_eq!(v1 + v2, expected);
        assert_eq!(v2 + v1, expected);
        assert!((v2 + v1).is_vector());
    }

    #[test]
    #[should_panic]
    #[allow(unused)]
    fn cannot_add_2_points_it_panics() {
        let p1 = point(3.0, -2.0, 5.0);
        let p2 = point(-2.0, 3.0, 1.0);
        p1 + p2;
    }

    #[test]
    fn can_subtract_2_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let expected = vector(-2., -4., -6.);
        assert_eq!(p1 - p2, expected);
        assert!((p1 - p2).is_vector());
    }

    #[test]
    fn can_subtract_a_vector_from_a_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let expected = point(-2., -4., -6.);
        assert_eq!(p1 - v2, expected);
        assert!((p1 - v2).is_point());
    }

    #[test]
    fn can_subtract_2_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let expected = vector(-2., -4., -6.);
        assert_eq!(v1 - v2, expected);
        assert!((v1 - v2).is_vector());
    }

    #[test]
    #[should_panic]
    #[allow(unused)]
    fn cannot_subtract_a_point_from_a_vector_it_panics() {
        let v1 = vector(3.0, -2.0, 5.0);
        let p2 = point(-2.0, 3.0, 1.0);
        v1 - p2;
    }

    #[test]
    fn can_subtract_a_vector_from_0_vector() {
        let v0 = vector(0.0, 0.0, 0.0);
        let v2 = vector(1.0, -2.0, 3.0);
        let expected = vector(-1., 2., -3.);
        assert_eq!(v0 - v2, expected);
        assert!((v0 - v2).is_vector());
    }

    #[test]
    fn it_can_be_negated() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let expected = tuple(-1., 2., -3., 4.0);
        assert_eq!(-a, expected);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(self) -> bool {
        abs_diff_eq!(self.w, 1.0)
    }

    pub fn is_vector(self) -> bool {
        abs_diff_eq!(self.w, 0.0)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
// Should not derive Eq as our f64's could be NaN-y and NaN != NaN

trait InOORange {
    fn in_oo_range(self, begin: Self, end: Self) -> bool;
}

impl InOORange for f64 {
    fn in_oo_range(self, begin: f64, end: f64) -> bool {
        self >= begin && self <= end
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        let w = self.w + other.w;
        match w {
            w if w.in_oo_range(0.0, 1.0) => tuple(
                self.x + other.x,
                self.y + other.y,
                self.z + other.z,
                self.w + other.w,
            ),
            _ => panic!("cannot add point to point, 0<= w <=1 but here w={}", w),
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        let w = self.w - other.w;
        match w {
            w if w.in_oo_range(0.0, 1.0) => tuple(
                self.x - other.x,
                self.y - other.y,
                self.z - other.z,
                self.w - other.w,
            ),
            _ => panic!(
                "cannot subtract point from vector, 0<= w <=1 but here w={}",
                w
            ),
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        tuple(-self.x, -self.y, -self.z, -self.w)
    }
}

fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: w,
    }
}

fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 1.0,
    }
}

fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 0.0,
    }
}
