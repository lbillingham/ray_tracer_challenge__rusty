use std::cmp::PartialEq;
use std::f64;
use std::ops::Add;
use std::ops::Div;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

const EPS: f64 = f64::EPSILON;

#[cfg(test)]
mod tests_for_point {
    use super::*;

    #[test]
    fn it_can_be_accessed_by_components() {
        let a = point(4.3, -4.2, 3.1);
        assert_abs_diff_eq!(a.x, 4.3, epsilon = EPS);
        assert_abs_diff_eq!(a.y, -4.2, epsilon = EPS);
        assert_abs_diff_eq!(a.z, 3.1, epsilon = EPS);
    }

    #[test]
    fn it_supports_comparison_equality() {
        let a = point(4.3, -4.2, 3.1);
        let b = point(4.3, -4.2, 3.1);
        assert!(a == b);
    }

    #[test]
    fn it_supports_comparison_inequality() {
        let a = point(4.3, -4.2, 3.1);
        let b = point(1.0, 1.0, 1.0);
        assert!(a != b);
    }

    #[test]
    fn it_supports_addition_by_vector() {
        let p1 = point(-2.0, 3.0, 1.0);
        let v2 = vector(3.0, -2.0, 5.0);
        let expected = point(1., 1., 6.);
        assert_eq!(p1 + v2, expected);
        assert!((p1 + v2).is_point());
    }

    #[test]
    fn it_supports_subtraction_by_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let expected = vector(-2., -4., -6.);
        assert_eq!(p1 - p2, expected);
        assert!((p1 - p2).is_vector());
    }

    #[test]
    fn it_supports_subtraction_by_vector() {
        let p1 = point(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let expected = point(-2., -4., -6.);
        assert_eq!(p1 - v2, expected);
        assert!((p1 - v2).is_point());
    }

    #[test]
    fn it_can_be_negated() {
        let a = point(1.0, -2.0, 3.0);
        let expected = point(-1., 2., -3.);
        assert_eq!(-a, expected);
        assert!((-a).is_point())
    }

    #[test]
    fn it_can_be_right_multipled_by_a_scalar_f64() {
        let a = point(1.0, -2.0, 3.0);
        let expected = point(3.5, -7.0, 10.5);
        assert!(a * 3.5 == expected);
    }

    #[test]
    fn it_can_be_right_multipled_by_a_fractional_scalar_f64() {
        let a = point(1.0, -2.0, 3.0);
        let expected = point(0.5, -1., 1.5);
        assert!(a * 0.5 == expected);
    }

    #[test]
    fn it_can_be_left_multipled_by_a_scalar_f64() {
        let a = point(1.0, -2.0, 3.0);
        let expected = point(3.5, -7.0, 10.5);
        assert!(3.5 * a == expected);
    }

    #[test]
    fn it_can_be_divided_by_a_scalar_f64() {
        let a = point(1.0, -2.0, 3.0);
        let expected = point(0.5, -1.0, 1.5);
        assert!(a / 2.0 == expected);
    }

    #[test]
    fn it_can_be_indexed_like_an_array() {
        let a = point(0.0, -10.0, 100.0);
        assert!(a[0] == 0.0);
        assert!(a[1] == -10.0);
        assert!(a[2] == 100.0);
    }

    #[test]
    fn it_is_a_point_and_not_a_vector() {
        let a = point(4.3, -4.2, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }
}

#[cfg(test)]
mod tests_for_vector {
    use super::*;

    #[test]
    fn it_can_be_accessed_by_components() {
        let a = vector(4.3, -4.2, 3.1);
        assert_abs_diff_eq!(a.x, 4.3, epsilon = EPS);
        assert_abs_diff_eq!(a.y, -4.2, epsilon = EPS);
        assert_abs_diff_eq!(a.z, 3.1, epsilon = EPS);
    }

    #[test]
    fn it_supports_comparison_equality() {
        let a = vector(4.3, -4.2, 3.1);
        let b = vector(4.3, -4.2, 3.1);
        assert!(a == b);
    }

    #[test]
    fn it_supports_comparison_inequality() {
        let a = vector(4.3, -4.2, 3.1);
        let b = vector(1.0, 1.0, 1.0);
        assert!(a != b);
    }

    #[test]
    fn it_is_a_vector_and_not_a_point() {
        let a = vector(4.3, -4.2, 3.1);
        assert!(a.is_vector());
        assert!(!a.is_point());
    }

    #[test]
    fn it_supports_addition_by_point() {
        let v1 = vector(3.0, -2.0, 5.0);
        let p2 = point(-2.0, 3.0, 1.0);
        let expected = point(1., 1., 6.);
        assert_eq!(v1 + p2, expected);
        assert!((v1 + p2).is_point());
    }

    #[test]
    fn it_supports_addition_by_vector() {
        let v1 = vector(3.0, -2.0, 5.0);
        let v2 = vector(-2.0, 3.0, 1.0);
        let expected = vector(1., 1., 6.);
        assert_eq!(v1 + v2, expected);
        assert_eq!(v2 + v1, expected);
        assert!((v2 + v1).is_vector());
    }

    #[test]
    fn it_supports_subtraction_by_vector() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let expected = vector(-2., -4., -6.);
        assert_eq!(v1 - v2, expected);
        assert!((v1 - v2).is_vector());
    }

    #[test]
    fn it_can_be_subtracted_from_the_null_vector() {
        let v0 = vector(0.0, 0.0, 0.0);
        let v2 = vector(1.0, -2.0, 3.0);
        let expected = vector(-1., 2., -3.);
        assert_eq!(v0 - v2, expected);
        assert!((v0 - v2).is_vector());
    }

    #[test]
    fn it_can_be_negated() {
        let a = vector(1.0, -2.0, 3.0);
        let expected = vector(-1., 2., -3.);
        assert_eq!(-a, expected);
        assert!((-a).is_vector())
    }

    #[test]
    fn it_can_be_right_multipled_by_a_scalar_f64() {
        let a = vector(1.0, -2.0, 3.0);
        let expected = vector(3.5, -7.0, 10.5);
        assert!(a * 3.5 == expected);
    }

    #[test]
    fn it_can_be_right_multipled_by_a_fractional_scalar_f64() {
        let a = vector(1.0, -2.0, 3.0);
        let expected = vector(0.5, -1., 1.5);
        assert!(a * 0.5 == expected);
    }

    #[test]
    fn it_can_be_left_multipled_by_a_scalar_f64() {
        let a = vector(1.0, -2.0, 3.0);
        let expected = vector(3.5, -7.0, 10.5);
        assert!(3.5 * a == expected);
    }

    #[test]
    fn it_can_be_divided_by_a_scalar_f64() {
        let a = vector(1.0, -2.0, 3.0);
        let expected = vector(0.5, -1.0, 1.5);
        assert!(a / 2.0 == expected);
    }

    #[test]
    fn its_magnitude_can_be_computed_by_function() {
        assert!(magnitude(vector(1., 0., 0.)) == 1.);
        assert!(magnitude(vector(0., 1., 0.)) == 1.);
        assert!(magnitude(vector(0., 0., 1.)) == 1.);
        assert!(magnitude(vector(1., 2., 3.)) == f64::sqrt(14.0));
        assert!(magnitude(vector(-1., -2., -3.)) == f64::sqrt(14.0));
    }

    #[test]
    fn it_has_a_magnitude_method() {
        assert!(vector(1., 0., 0.).magnitude() == 1.);
        assert!(vector(0., 1., 0.).magnitude() == 1.);
        assert!(vector(0., 0., 1.).magnitude() == 1.);
        assert!(vector(1., 2., 3.).magnitude() == f64::sqrt(14.0));
        assert!(vector(-1., -2., -3.).magnitude() == f64::sqrt(14.0));
    }

    #[test]
    fn it_has_a_normalize_method() {
        assert!(vector(4., 0., 0.).normalize() == vector(1.0, 0.0, 0.0));
        let len_expect = f64::sqrt(14.0);
        assert!(
            vector(1., 2., 3.).normalize()
                == vector(1.0 / len_expect, 2.0 / len_expect, 3.0 / len_expect)
        )
    }

    #[test]
    fn it_can_be_normalized_by_function() {
        assert!(normalize(vector(4., 0., 0.)) == vector(1., 0., 0.));
        let len_expect = f64::sqrt(14.0);
        assert!(
            normalize(vector(1., 2., 3.))
                == vector(1.0 / len_expect, 2.0 / len_expect, 3.0 / len_expect)
        );
    }

    #[test]
    fn it_can_be_indexed_like_an_array() {
        let a = vector(0.0, -10.0, 100.0);
        assert!(a[0] == 0.0);
        assert!(a[1] == -10.0);
        assert!(a[2] == 100.0);
    }

    #[test]
    fn it_can_be_dot_producted() {
        let v1 = vector(1., 2., 3.);
        let v2 = vector(2., 3., 4.);
        assert!(v1.dot(v2) == 20.);
        assert!(v2.dot(v1) == 20.);
    }

    #[test]
    fn can_dot_product_2_vectors_via_function() {
        let v1 = vector(1., 2., 3.);
        let v2 = vector(2., 3., 4.);
        assert!(dot(v1, v2) == 20.);
        assert!(dot(v2, v1) == 20.);
    }

    #[test]
    fn it_can_be_cross_producted() {
        let v1 = vector(1., 2., 3.);
        let v2 = vector(2., 3., 4.);
        assert!(v1.cross(v2) == vector(-1., 2., -1.));
        assert!(v2.cross(v1) == vector(1., -2., 1.));
    }

    #[test]
    fn can_cross_product_2_vectors_via_function() {
        let v1 = vector(1., 2., 3.);
        let v2 = vector(2., 3., 4.);
        assert!(cross(v1, v2) == vector(-1., 2., -1.));
        assert!(cross(v2, v1) == vector(1., -2., 1.));
    }
}

// #[derive(Debug, Copy, Clone)]
// pub struct Tuple {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
//     pub w: f64,
// }

// impl Tuple {
//     pub fn is_point(self) -> bool {
//         abs_diff_eq!(self.w, 1.0)
//     }

//     pub fn is_vector(self) -> bool {
//         abs_diff_eq!(self.w, 0.0)
//     }

//     pub fn magnitude(self) -> f64 {
//         let w = self.w;
//         match w {
//             w if w == 0.0 => f64::sqrt(self.dot(self)),
//             _ => panic!(
//                 "cannot take magnitude of a point, w should == 0 but here w={}",
//                 w
//             ),
//         }
//     }

//     pub fn normalize(self) -> Tuple {
//         let w = self.w;
//         match w {
//             w if w == 0.0 => self / self.magnitude(),
//             _ => panic!(
//                 "cannot take normalize a point, w should == 0 but here w={}",
//                 w
//             ),
//         }
//     }

//     pub fn dot(self: Tuple, other: Tuple) -> f64 {
//         let w = self.w + other.w;
//         match w {
//             w if w == 0.0 => self.x * other.x + self.y * other.y + self.z * other.z,
//             _ => panic!(
//                 "cannot take dot product points, w should == 0 for self and other but here w={}",
//                 w
//             ),
//         }
//     }

//     pub fn cross(self: Tuple, other: Tuple) -> Tuple {
//         let w = self.w + other.w;
//         match w {
//             w if w == 0.0 => vector(
//                 self.y * other.z - self.z * other.y,
//                 self.z * other.x - self.x * other.z,
//                 self.x * other.y - self.y * other.x,
//             ),
//             _ => panic!(
//                 "cannot take dot product points, w should == 0 for self and other but here w={}",
//                 w
//             ),
//         }
//     }
// }

// impl PartialEq for Tuple {
//     fn eq(&self, other: &Tuple) -> bool {
//         self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
//     }
// }
// Should not derive Eq as our f64's could be NaN-y and NaN != NaN

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn is_point(self) -> bool {
        true
    }

    pub fn is_vector(self) -> bool {
        false
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
// Should not derive Eq as our f64's could be NaN-y and NaN != NaN

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(
                "Unknown value {} for index found: must be in 0, 1, 2",
                index
            ),
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, vec: Vector) -> Point {
        point(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        vector(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, vec: Vector) -> Point {
        point(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, scalar: f64) -> Point {
        point(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, pointy: Point) -> Point {
        pointy * self
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, vec: Vector) -> Vector {
        vec * self
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        -1.0 * self
    }
}

impl Div<f64> for Point {
    type Output = Point;
    fn div(self, scalar: f64) -> Point {
        let inverted_scalar = 1.0 / scalar;
        point(
            self.x * inverted_scalar,
            self.y * inverted_scalar,
            self.z * inverted_scalar,
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn is_point(self) -> bool {
        false
    }

    pub fn is_vector(self) -> bool {
        true
    }

    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    pub fn normalize(self) -> Vector {
        self / self.magnitude()
    }

    pub fn dot(self: Vector, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self: Vector, other: Vector) -> Vector {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
// Should not derive Eq as our f64's could be NaN-y and NaN != NaN

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(
                "Unknown value {} for index found: must be in 0, 1, 2",
                index
            ),
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, vec: Vector) -> Vector {
        vector(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, pointy: Point) -> Point {
        point(self.x + pointy.x, self.y + pointy.y, self.z + pointy.z)
    }
}

impl Sub<Point> for Vector {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        vector(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        vector(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, scalar: f64) -> Vector {
        vector(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        -1.0 * self
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, scalar: f64) -> Vector {
        let inverted_scalar = 1.0 / scalar;
        vector(
            self.x * inverted_scalar,
            self.y * inverted_scalar,
            self.z * inverted_scalar,
        )
    }
}

// trait InOORange {
//     fn in_oo_range(self, begin: Self, end: Self) -> bool;
// }

// impl InOORange for f64 {
//     fn in_oo_range(self, begin: f64, end: f64) -> bool {
//         self >= begin && self <= end
//     }
// }

// impl Add for Tuple {
//     type Output = Tuple;
//     fn add(self, other: Tuple) -> Tuple {
//         let w = self.w + other.w;
//         match w {
//             w if w.in_oo_range(0.0, 1.0) => tuple(
//                 self.x + other.x,
//                 self.y + other.y,
//                 self.z + other.z,
//                 self.w + other.w,
//             ),
//             _ => panic!("cannot add point to point, 0<= w <=1 but here w={}", w),
//         }
//     }
// }

// impl Sub for Tuple {
//     type Output = Tuple;
//     fn sub(self, other: Tuple) -> Tuple {
//         let w = self.w - other.w;
//         match w {
//             w if w.in_oo_range(0.0, 1.0) => tuple(
//                 self.x - other.x,
//                 self.y - other.y,
//                 self.z - other.z,
//                 self.w - other.w,
//             ),
//             _ => panic!(
//                 "cannot subtract point from vector, 0<= w <=1 but here w={}",
//                 w
//             ),
//         }
//     }
// }

// impl Neg for Tuple {
//     type Output = Tuple;
//     fn neg(self) -> Tuple {
//         -1.0 * self
//     }
// }

// impl Mul<f64> for Tuple {
//     type Output = Tuple;
//     fn mul(self, scalar: f64) -> Tuple {
//         tuple(
//             self.x * scalar,
//             self.y * scalar,
//             self.z * scalar,
//             self.w * scalar,
//         )
//     }
// }

// impl Mul<Tuple> for f64 {
//     type Output = Tuple;
//     fn mul(self, tup: Tuple) -> Tuple {
//         tup * self
//     }
// }

// impl Div<f64> for Tuple {
//     type Output = Tuple;
//     fn div(self, scalar: f64) -> Tuple {
//         let inverted_scalar = 1.0 / scalar;
//         tuple(
//             self.x * inverted_scalar,
//             self.y * inverted_scalar,
//             self.z * inverted_scalar,
//             self.w * inverted_scalar,
//         )
//     }
// }

// impl Index<usize> for Tuple {
//     type Output = f64;

//     fn index(&self, index: usize) -> &f64 {
//         match index {
//             0 => &self.x,
//             1 => &self.y,
//             2 => &self.z,
//             3 => &self.w,
//             _ => panic!(
//                 "Unknown value {} for index found: must be in 0, 1, 2, 3",
//                 index
//             ),
//         }
//     }
// }

// pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
//     Tuple {
//         x: x,
//         y: y,
//         z: z,
//         w: w,
//     }
// }

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point { x: x, y: y, z: z }
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector { x: x, y: y, z: z }
}

pub fn magnitude(v: Vector) -> f64 {
    v.magnitude()
}

pub fn normalize(v: Vector) -> Vector {
    v.normalize()
}

pub fn dot(v1: Vector, v2: Vector) -> f64 {
    v1.dot(v2)
}

pub fn cross(v1: Vector, v2: Vector) -> Vector {
    v1.cross(v2)
}
