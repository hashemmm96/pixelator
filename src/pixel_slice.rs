extern crate image;
use image::{Rgb, RgbImage};

enum Color {
    Red,
    Green,
    Blue,
}

pub struct PixelSlice {
    slice: Vec<Rgb<u8>>,
    x_start: u32,
    x_end: u32,
    y_start: u32,
    y_end: u32,
}

impl PixelSlice {
    fn get_red_color(&self) -> u8 {
        let pixel_count = self.slice.len() as u32;
        let red = self
            .slice
            .iter()
            .map(|&p| get_pixel_color(p, Color::Red) as u32)
            .sum::<u32>()
            / pixel_count;
        red as u8
    }

    fn get_green_color(&self) -> u8 {
        let pixel_count = self.slice.len() as u32;
        let green = self
            .slice
            .iter()
            .map(|&p| get_pixel_color(p, Color::Green) as u32)
            .sum::<u32>()
            / pixel_count;
        green as u8
    }

    fn get_blue_color(&self) -> u8 {
        let pixel_count = self.slice.len() as u32;
        let blue = self
            .slice
            .iter()
            .map(|&p| get_pixel_color(p, Color::Blue) as u32)
            .sum::<u32>()
            / pixel_count;
        blue as u8
    }

    fn create_new_pixel(&self) -> Rgb<u8> {
        let red = self.get_red_color();
        let green = self.get_green_color();
        let blue = self.get_blue_color();
        Rgb([red, green, blue])
    }

    pub fn put_pixels(&self, image: &mut RgbImage) {
        let new_pixel = self.create_new_pixel();
        for x in self.x_start..self.x_end {
            for y in self.y_start..self.y_end {
                image.put_pixel(x, y, new_pixel);
            }
        }
    }
}

fn get_pixel_color(pixel: Rgb<u8>, color: Color) -> u8 {
    match color {
        Color::Red => pixel.0[0],
        Color::Green => pixel.0[1],
        Color::Blue => pixel.0[2],
    }
}

fn create_pixel_slice(image: &RgbImage, x_start: u32, y_start: u32, pixel_size: u32) -> PixelSlice {
    let x_end = x_start + pixel_size;
    let y_end = y_start + pixel_size;
    let mut slice = Vec::new();
    for x in x_start..x_end {
        for y in y_start..y_end {
            slice.push(*image.get_pixel(x, y));
        }
    }
    PixelSlice {
        slice,
        x_start,
        x_end,
        y_start,
        y_end,
    }
}

fn create_corner_slice(image: &RgbImage, pixel_size: u32) -> PixelSlice {
    let (width, height) = image.dimensions();
    let x_start = width - pixel_size + 1;
    let y_start = height - pixel_size + 1;
    let mut slice = Vec::new();
    for x in x_start..width {
        for y in y_start..height {
            slice.push(*image.get_pixel(x, y));
        }
    }
    PixelSlice {
        slice,
        x_start,
        x_end: width,
        y_start,
        y_end: height,
    }
}

pub fn create_pixel_slices(image: &RgbImage, pixel_size: u32) -> Vec<PixelSlice> {
    let (width, height) = image.dimensions();
    let mut pixel_slices = Vec::new();
    let mut x_start = 0;
    let mut y_start = 0;

    while x_start < width - pixel_size + 1 {
        while y_start < height - pixel_size + 1 {
            let pixel_slice = create_pixel_slice(image, x_start, y_start, pixel_size);
            pixel_slices.push(pixel_slice);
            y_start += pixel_size;
        }
        y_start = 0;
        x_start += pixel_size;
    }
    let corner_slice = create_corner_slice(image, pixel_size);
    pixel_slices.push(corner_slice);
    pixel_slices
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_new_pixel() {
        use super::*;
        let pixel: Rgb<u8> = Rgb([100, 100, 100]);
        let mut slice = Vec::new();
        for _ in 0..10 {
            slice.push(pixel);
        }
        let pixel_slice = PixelSlice {
            slice,
            x_start: 0,
            x_end: 0,
            y_start: 0,
            y_end: 0,
        };
        assert_eq!(pixel_slice.create_new_pixel(), pixel);
    }

    #[test]
    fn get_pixel_color() {
        use super::{Color, Rgb};
        let (r, g, b) = (10, 20, 30);
        let pixel = Rgb([r, g, b]);
        assert_eq!(super::get_pixel_color(pixel, Color::Red), r);
        assert_eq!(super::get_pixel_color(pixel, Color::Green), g);
        assert_eq!(super::get_pixel_color(pixel, Color::Blue), b);
    }
}
