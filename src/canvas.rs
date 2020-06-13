use crate::color::{Color, BLACK};
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests_for_canvas {
    use crate::canvas::*;
    use crate::color::{color, RED};

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
        let ppm = c._ppm_header();
        let ppm_header = ppm.lines().take(3).collect::<Vec<&str>>().join("\n");
        assert_eq!(ppm_header, "P3\n5 3\n255");
    }

    #[test]
    fn it_can_write_the_pixels_of_a_ppm_file() {
        let mut canv = canvas(5, 3);
        let red = color(1.5, 0., 0.);
        let green = color(0., 0.5, 0.);
        let blue = color(-0.5, 0., 1.);
        canv[0][0] = red;
        canv.write_pixel(2, 1, green);
        write_pixel(&mut canv, 4, 2, blue);
        let ppm_body = canv._ppm_body();
        let expected = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
                        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
                        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";
        assert_eq!(expected, ppm_body);
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

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    fn _pixels_bytes(&self) -> usize {
        3
    }

    fn _buffer_size(&self) -> usize {
        self.width * self.height * self._pixels_bytes()
    }

    fn _ppm_header(&self) -> String {
        format!(
            "P3\n{width} {height}\n{max_color}\n",
            width = self.width,
            height = self.height,
            max_color = MAX_COLOR
        )
    }

    fn _ppm_pixels(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; self._buffer_size()];
        let mut i_rgb = 0;
        for row in &self.pixels {
            for pixel in row {
                bytes[i_rgb] = clamp_to_byte(pixel.red);
                bytes[i_rgb + 1] = clamp_to_byte(pixel.green);
                bytes[i_rgb + 2] = clamp_to_byte(pixel.blue);
                i_rgb += self._pixels_bytes();
            }
        }
        bytes
    }

    fn _ppm_body(&self) -> String {
        let mut s = String::new();
        let px_bytes = self._ppm_pixels();
        for (i, u) in px_bytes.iter().enumerate() {
            let f = format!("{digit}", digit = u);
            s.push_str(&f);
            let sep = self._pixel_output_seperator(i);
            s.push_str(sep);
        }
        s
    }

    pub fn to_ppm(&self) -> String {
        let mut s = self._ppm_header();
        s.push_str(&self._ppm_body());
        s
    }

    fn _pixel_output_seperator(&self, idx: usize) -> &str {
        let n_row = self._pixels_bytes() * self.width;
        match idx {
            idx if ((idx + 1) >= self._buffer_size()) => "",
            idx if (idx + 1) % n_row == 0 => "\n",
            _ => " ",
        }
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
