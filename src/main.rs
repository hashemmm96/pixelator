extern crate image;
extern crate structopt;

use image::io::Reader as ImageReader;
use image::RgbImage;
use structopt::StructOpt;

use std::path::PathBuf;
use std::process::exit;

mod pixel_slice;
use pixel_slice::*;

/* Take out pixel_size rows from image, then loop over one row to create a pixel_slice of all rows, repeat
 * until looped over whole row. Repeat until all rows are pixelated.
 */
fn pixelate(image: &RgbImage, pixel_size: u32) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut new_image = RgbImage::new(width, height);
    let pixel_slices = create_pixel_slices(image, pixel_size);

    for pixel_slice in pixel_slices {
        pixel_slice.put_pixels(&mut new_image);
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

    let new_image = pixelate(&image.to_rgb8(), args.pixel_size);
    let mut new_image_name = String::new();
    new_image_name.push_str(image_basename.to_str().unwrap());
    new_image_name.push_str("-pixelated.");
    new_image_name.push_str(image_extension);
    new_image.save(new_image_name).expect("");
}
