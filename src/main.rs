extern crate image;
extern crate structopt;

use image::io::Reader as ImageReader;
use image::{Rgb, RgbImage};
use structopt::StructOpt;

use std::path::PathBuf;
use std::process::exit;

enum Color {
    Red,
    Green,
    Blue,
}

fn create_new_pixel(pixels: Vec<&Rgb<u8>>) -> Rgb<u8> {
    let pixel_count = pixels.len() as u32;
    let red: u32 = pixels
        .iter()
        .map(|p| get_pixel_color(p, Color::Red) as u32)
        .sum::<u32>()
        / pixel_count;
    let green: u32 = pixels
        .iter()
        .map(|p| get_pixel_color(p, Color::Green) as u32)
        .sum::<u32>()
        / pixel_count;
    let blue: u32 = pixels
        .iter()
        .map(|p| get_pixel_color(p, Color::Blue) as u32)
        .sum::<u32>()
        / pixel_count;
    Rgb([red as u8, green as u8, blue as u8])
}

/* Take out rows of pixel_size, then loop over one row to create a pixel_slice of all rows, repeat
 * until looped over whole row. Repeat until all rows are pixelated.
 */
fn create_pixel_grid(image_buf: RgbImage, pixel_size: u32) -> RgbImage {
    let (width, height) = image_buf.dimensions();

    let mut pixel_slice = Vec::new();
    for x in 0..pixel_size {
        for y in 0..pixel_size {
            pixel_slice.push(image_buf.get_pixel(x, y));
        }
    }
    let new_pixel = create_new_pixel(pixel_slice);

    let mut new_image = RgbImage::new(width, height);
    for x in 0..pixel_size {
        for y in 0..pixel_size {
            new_image.put_pixel(x, y, new_pixel);
        }
    }
    new_image
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    /// Image file.
    image: PathBuf,

    #[structopt(default_value = "10", short)]
    /// The size of every pixel in the new image.
    pixel_size: u32,
}

fn main() {
    let args = Cli::from_args();
    let image_name = args.image.to_str().unwrap();
    if !args.image.exists() {
        println!("Image file \"{}\" does not exist", image_name);
        exit(1);
    }

    let image_extension = match args.image.extension() {
        Some(s) => s.to_str().unwrap(),
        None => "",
    };

    let image_basename = args.image.file_stem().unwrap();
    let image = match ImageReader::open(image_name).expect("").decode() {
        Ok(img) => img,
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };

    let new_image = create_pixel_grid(image.to_rgb8(), args.pixel_size);
    let mut new_image_name = String::new();
    new_image_name.push_str(image_basename.to_str().unwrap());
    new_image_name.push_str("-pixelated.");
    new_image_name.push_str(image_extension);
    new_image.save(new_image_name).expect("");
}

mod tests {
    #[test]
    fn test_create_new_pixel() {
        use super::*;
        let pixel: Rgb<u8> = Rgb([100, 100, 100]);
        let mut pixel_vec = Vec::new();
        for _ in 0..10 {
            pixel_vec.push(&pixel);
        }
        assert_eq!(create_new_pixel(pixel_vec), pixel);
    }
}

fn get_pixel_color(pixel: &Rgb<u8>, color: Color) -> u8 {
    match color {
        Color::Red => pixel.0[0],
        Color::Green => pixel.0[1],
        Color::Blue => pixel.0[2],
    }
}
