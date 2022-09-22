#![deny(clippy::all)]

use napi::{Error, bindgen_prelude::{Uint8Array, Uint8ClampedArray}};
use image::{GenericImageView, imageops, ImageOutputFormat, DynamicImage, EncodableLayout};
use seamcarving;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn magik (buffer: Uint8Array) -> Result<Uint8ClampedArray, Error> {
  let img = match image::load_from_memory(buffer.as_bytes()) {
    Ok(buffer) => buffer,
    Err(err) => return Err(Error::from_reason(err.to_string())),
  };

  let (width, height) = img.dimensions();
  let rescaled = seamcarving::resize(&img, width / 2, height / 2);
  let resized = imageops::resize(&rescaled, width, height, imageops::Gaussian);

  let mut v: Vec<u8> = Vec::new();

  match DynamicImage::ImageRgba8(resized).write_to(&mut v, ImageOutputFormat::Png) {
    Ok(_) => return Ok(Uint8ClampedArray::new(v)),
    Err(err) => return Err(Error::from_reason(err.to_string())),
  }
}