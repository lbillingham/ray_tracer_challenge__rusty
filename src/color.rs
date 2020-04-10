use crate::f64_helpers::EPS;
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

#[cfg(test)]
mod tests_for_color {
    use crate::color::color;
    use crate::f64_helpers::EPS;

    #[test]
    fn it_has_red_green_blue_components() {
        let c = color(-0.5, 0.4, 1.7);
        assert_abs_diff_eq!(c.red, -0.5, epsilon = EPS);
        assert_abs_diff_eq!(c.green, 0.4, epsilon = EPS);
        assert_abs_diff_eq!(c.blue, 1.7, epsilon = EPS);
    }

    #[test]
    fn it_can_be_added_to_another_color() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let expected = color(1.6, 0.7, 1.0);

        let got = c1 + c2;
        assert!(got == expected);
    }

    #[test]
    fn it_can_be_subtracted_from_another_color() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let expected = color(0.2, 0.5, 0.5);

        let got = c1 - c2;
        assert!(got == expected);
    }

    #[test]
    fn it_can_be_left_multiplied_by_a_scalar_f64() {
        let c = color(0.2, 0.3, 0.4);
        let expected = color(0.4, 0.6, 0.8);

        let got = 2.0 * c;
        assert!(got == expected);
    }

    #[test]
    fn it_can_be_right_multiplied_by_a_scalar_f64() {
        let c = color(0.2, 0.3, 0.4);
        let expected = color(0.4, 0.6, 0.8);

        let got = c * 2.0;
        assert!(got == expected);
    }

    #[test]
    fn it_can_be_multiplied_by_another_color() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        let expected = color(0.9, 0.2, 0.04);

        assert!(c1 * c2 == expected);
        assert!(c2 * c1 == expected);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        abs_diff_eq!(self.red, other.red, epsilon = EPS)
            && abs_diff_eq!(self.green, other.green, epsilon = EPS)
            && abs_diff_eq!(self.blue, other.blue, epsilon = EPS)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        color(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        color(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        color(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        color(self.red * scalar, self.green * scalar, self.blue * scalar)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, col: Color) -> Color {
        color(col.red * self, col.green * self, col.blue * self)
    }
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color {
        red: red,
        green: green,
        blue: blue,
    }
}
