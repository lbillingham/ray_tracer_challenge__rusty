use crate::color::{Color, BLACK};
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests_for_canvas {
    use crate::canvas::*;
    use crate::color::RED;

    #[test]
    fn it_can_be_instatiated_with_a_width_and_height() {
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
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
    fn can_use_pixel_at_method_like_in_buck() {
        let c = canvas(10, 20);
        assert_eq!(c.pixel_at(2, 3), BLACK);
    }

    #[test]
    fn it_can_have_its_pixels_set_by_index() {
        let mut c = canvas(10, 20);
        c[2][3] = RED;
        assert_eq!(c[2][3], RED);
        assert_eq!(c.pixel_at(2, 3), RED);
    }

    #[test]
    fn it_can_have_its_pixels_set_by_function_like_in_buck() {
        let mut c = canvas(10, 20);
        write_pixel(&mut c, 2, 3, RED);
        assert_eq!(c[2][3], RED);
        assert_eq!(pixel_at(c, 2, 3), RED);
    }

    #[test]
    fn it_can_have_its_pixels_set_by_method_like_in_buck() {
        let mut c = canvas(10, 20);
        c.write_pixel(2, 3, RED);
        assert_eq!(c[2][3], RED);
        assert_eq!(c.pixel_at(2, 3), RED);
    }

    #[test]
    fn it_can_write_the_header_of_a_ppm_file() {
        let c = canvas(5, 3);
        let ppm = c.to_ppm();
        let ppm_header = ppm.lines().take(3).collect::<Vec<&str>>().join("\n");
        assert_eq!(ppm_header, "P3\n5 3\n255");
    }
}

const MAX_COLOR: u8 = 255;
const NUM_COLOR_VALS: u16 = MAX_COLOR as u16 + 1;

#[derive(Debug, Clone)]
struct PpmRowBytes(Vec<u8>);

impl Index<usize> for PpmRowBytes {
    type Output = u8;
    fn index(&self, index: usize) -> &u8 {
        &self.0[index]
    }
}

impl IndexMut<usize> for PpmRowBytes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl PpmRowBytes {
    pub fn new(size: usize) -> Self {
        Self(vec![0 as u8; size])
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    fn _ppm_header(&self) -> String {
        format!(
            "P3\n{width} {height}\n{max_color}\n",
            width = self.width,
            height = self.height,
            max_color = MAX_COLOR
        )
    }

    fn _ppm_pixels(&self) -> PpmRowBytes {
        const PIXELS_BYTES: usize = 3;
        let mut bytes = PpmRowBytes::new(self.width * self.height * PIXELS_BYTES);
        let mut i_rgb = 0;
        for row in &self.pixels {
            for pixel in row {
                bytes[i_rgb] = clamp_to_byte(pixel.red);
                bytes[i_rgb + 1] = clamp_to_byte(pixel.green);
                bytes[i_rgb + 2] = clamp_to_byte(pixel.blue);
                i_rgb += PIXELS_BYTES;
            }
        }
        bytes
    }

    pub fn to_ppm(&self) -> String {
        let mut s = self._ppm_header();
        let px_bytes = self._ppm_pixels();
        let px_string = format!("{:#?}", px_bytes);
        s.push_str(&px_string);
        s
    }

    pub fn pixel_at(self, column_index: usize, row_index: usize) -> Color {
        self.pixels[column_index][row_index]
    }

    pub fn write_pixel(&mut self, column_index: usize, row_index: usize, color: Color) {
        self.pixels[column_index][row_index] = color;
    }
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
        width,
        height,
        pixels: vec![vec![BLACK; height]; width],
    }
}

pub fn pixel_at(canvas: Canvas, column_index: usize, row_index: usize) -> Color {
    canvas.pixel_at(column_index, row_index)
}

pub fn write_pixel(canvas: &mut Canvas, column_index: usize, row_index: usize, color: Color) {
    canvas.write_pixel(column_index, row_index, color)
}

fn clamp_to_byte(color_component: f64) -> u8 {
    match color_component {
        x if x <= 0.0 => 0,
        x if x >= 1.0 => MAX_COLOR,
        _ => (color_component * (NUM_COLOR_VALS) as f64) as u8,
    }
}
