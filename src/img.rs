use std::{convert::TryInto};
use image::{ImageFormat, DynamicImage, io::Reader, imageops::FilterType::Triangle, GenericImageView};
use std::{fs::File, io::BufReader};

pub struct Img {
  pub width: u32,
  pub height: u32,
  pub data: Vec<u8>,
  pub name: String
}

impl Img {
  // Creates new img
  pub fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = height * width * 4;
    let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
    Img {
      width,
      height,
      data: buffer,
      name
    }
  }
}

#[derive(Debug)]
pub enum ImgDataErrors {
  DiffFormats,
}

// Finds image from path
pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
  let image_format: ImageFormat = image_reader.format().unwrap();
  let image: DynamicImage = image_reader.decode().unwrap();
  (image, image_format)
}

// Gets the smallest image dimensions
fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_2.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

// Converts both images to have the same size
pub fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}
