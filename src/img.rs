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

  // Set the image data
  pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImgDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImgDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

#[derive(Debug)]
pub enum ImgDataErrors {
  DiffFormats,
  BufferTooSmall,
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

// Combine the images
pub fn combine_image(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

// Alternate the pixels, adding 4 of each pixel for each image at a time
fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }
  combined_data
}

// Grab chunk of pixels
fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val: u8 = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds")
    };
    rgba.push(val);
  }
  rgba
}
