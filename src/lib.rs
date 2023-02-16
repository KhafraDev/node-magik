#![deny(clippy::all)]

use image::{imageops, DynamicImage, EncodableLayout, GenericImageView, ImageOutputFormat};
use napi::{
  bindgen_prelude::{AbortSignal, AsyncTask, Uint8Array, Uint8ClampedArray},
  Env, Error, Task,
};
use seamcarving;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn magik_sync(buffer: Uint8Array) -> Result<Uint8ClampedArray, Error> {
  let img = match image::load_from_memory(buffer.as_bytes()) {
    Ok(buffer) => buffer,
    Err(err) => return Err(Error::from_reason(err.to_string())),
  };

  let (width, height) = img.dimensions();
  let rescaled = seamcarving::resize(&img, width / 2, height / 2);
  let resized = imageops::resize(&rescaled, width, height, imageops::Gaussian);

  let mut v: Vec<u8> = Vec::new();

  return match DynamicImage::ImageRgba8(resized).write_to(&mut v, ImageOutputFormat::Png) {
    Ok(_) => Ok(Uint8ClampedArray::new(v)),
    Err(err) => Err(Error::from_reason(err.to_string())),
  }
}

pub struct MagikTask {
  buffer: Uint8Array,
}

#[napi]
impl Task for MagikTask {
  type Output = Uint8ClampedArray;
  type JsValue = Uint8ClampedArray;

  fn compute(&mut self) -> Result<Self::Output, Error> {
    magik_sync(self.buffer.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue, Error> {
    Ok(output)
  }
}

#[napi]
pub struct Magik {
  pub(crate) buffer: Uint8Array,
}

#[napi]
impl Magik {
  #[napi(constructor)]
  pub fn new(input: Uint8Array) -> Magik {
    Self { buffer: input }
  }

  #[napi]
  pub fn magikify(&mut self, signal: Option<AbortSignal>) -> AsyncTask<MagikTask> {
    AsyncTask::with_optional_signal(
      MagikTask {
        buffer: self.buffer.clone(),
      },
      signal,
    )
  }
}
