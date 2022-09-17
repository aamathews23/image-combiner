mod args;
mod img;
use args::Args;
use image::GenericImageView;
use img::{Img, ImgDataErrors, find_image_from_path, standardise_size, combine_image};

fn main() -> Result<(), ImgDataErrors> {
  // Grab images from storage
  let args = Args::new();
  let (image_1, image_format_1) = find_image_from_path(args.image_1);
  let (image_2, image_format_2) = find_image_from_path(args.image_2);

  // Check image formats
  if image_format_1 != image_format_2 {
    return Err(ImgDataErrors::DiffFormats);
  }

  // Combine images
  let (image_1, image_2) = standardise_size(image_1, image_2);
  let mut output = Img::new(image_1.width(), image_1.height(), args.output);
  let combined_data = combine_image(image_1, image_2);
  output.set_data(combined_data)?;

  // Save image
  image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image_format_1).unwrap();
  Ok(())
}
