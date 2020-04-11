use crate::color::{Color, BLACK, RED};
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests_for_canvas {
    use crate::canvas::{canvas, pixel_at, write_pixel, BLACK, RED};

    #[test]
    fn it_can_be_instatiated_with_a_width_and_height() {
        let c = canvas(10, 20);
        assert!(c.width == 10);
        assert!(c.height == 20);
        for column in c.pixels {
            for pixel in column {
                assert_eq!(pixel, BLACK);
            }
        }
    }

    #[test]
    fn it_can_have_its_pixels_read_by_index() {
        let c = canvas(10, 20);
        assert_eq!(c[2][3], BLACK);
    }

    #[test]
    fn can_use_pixel_at_function_like_in_buck() {
        let c = canvas(10, 20);
        assert_eq!(pixel_at(c, 2, 3), BLACK);
    }

    #[test]
    fn it_can_have_its_pixels_set_by_index() {
        let mut c = canvas(10, 20);
        c[2][3] = RED;
        assert_eq!(c[2][3], RED);
        assert_eq!(pixel_at(c, 3, 2), BLACK);
    }

    #[test]
    fn it_can_have_its_pixels_set_by_function_like_in_buck() {
        let mut c = canvas(10, 20);
        write_pixel(&mut c, 2, 3, RED);
        assert_eq!(c[2][3], RED);
        // assert_eq!(pixel_at(c, 2, 3), RED);
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Index<usize> for Canvas {
    type Output = Vec<Color>;
    fn index(&self, column_index: usize) -> &Vec<Color> {
        &self.pixels[column_index]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, column_index: usize) -> &mut Self::Output {
        &mut self.pixels[column_index]
    }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        width: width,
        height: height,
        pixels: vec![vec![BLACK; width]; height],
    }
}

pub fn pixel_at(canvas: Canvas, column_index: usize, row_index: usize) -> Color {
    canvas[row_index][column_index]
}

pub fn write_pixel(canvas: &mut Canvas, row_index: usize, column_index: usize, color: Color) {
    canvas[row_index][column_index] = color
    // canvas[column_index][row_index] = color
}
