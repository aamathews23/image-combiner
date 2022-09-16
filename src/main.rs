mod args;
mod img;
use args::Args;
use image::GenericImageView;
use img::{Img, ImgDataErrors, find_image_from_path, standardise_size};

fn main() -> Result<(), ImgDataErrors> {
  let args = Args::new();
  let (image_1, image_format_1) = find_image_from_path(args.image_1);
  let (image_2, image_format_2) = find_image_from_path(args.image_2);

  if image_format_1 != image_format_2 {
    return Err(ImgDataErrors::DiffFormats);
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = Img::new(image_1.width(), image_1.height(), args.output);
  Ok(())
}
