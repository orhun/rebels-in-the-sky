use super::color_map::ColorMap;
use crate::store::ASSETS_DIR;
use image::error::{ParameterError, ParameterErrorKind};
use image::io::Reader as ImageReader;
use image::{ImageBuffer, ImageError, ImageResult, Rgba, RgbaImage};
use std::error::Error;
use std::io::Cursor;

pub trait ExtraImageUtils {
    fn copy_non_trasparent_from(
        &mut self,
        other: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        x: u32,
        y: u32,
    ) -> ImageResult<()>;
    fn apply_color_map(&mut self, color_map: ColorMap) -> &ImageBuffer<Rgba<u8>, Vec<u8>>;
}

impl ExtraImageUtils for ImageBuffer<Rgba<u8>, Vec<u8>> {
    /// Copies all non-transparent the pixels from another image into this image.
    ///
    /// The other image is copied with the top-left corner of the
    /// other image placed at (x, y).
    ///
    /// In order to copy only a piece of the other image, use [`GenericImageView::view`].
    ///
    /// You can use [`FlatSamples`] to source pixels from an arbitrary regular raster of channel
    /// values, for example from a foreign interface or a fixed image.
    ///
    /// # Returns
    /// Returns an error if the image is too large to be copied at the given position
    ///
    /// [`GenericImageView::view`]: trait.GenericImageView.html#method.view
    /// [`FlatSamples`]: flat/struct.FlatSamples.html
    fn copy_non_trasparent_from(
        &mut self,
        other: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        x: u32,
        y: u32,
    ) -> ImageResult<()> {
        // Do bounds checking here so we can use the non-bounds-checking
        // functions to copy pixels.
        if self.width() < other.width() + x || self.height() < other.height() + y {
            return Err(ImageError::Parameter(ParameterError::from_kind(
                ParameterErrorKind::DimensionMismatch,
            )));
        }

        for k in 0..other.height() {
            for i in 0..other.width() {
                let p = other.get_pixel(i, k);
                if p[3] > 0 {
                    self.put_pixel(i + x, k + y, *p);
                }
            }
        }
        Ok(())
    }
    fn apply_color_map(&mut self, color_map: ColorMap) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        for k in 0..self.height() {
            for i in 0..self.width() {
                let p = self.get_pixel(i, k);
                if p[3] > 0 {
                    let mapped_pixel = match p {
                        _ if p[0] == 255 && p[1] == 0 && p[2] == 0 => {
                            let [r, g, b] = color_map.red.0;
                            Rgba([r, g, b, p[3]])
                        }
                        _ if p[0] == 0 && p[1] == 255 && p[2] == 0 => {
                            let [r, g, b] = color_map.green.0;
                            Rgba([r, g, b, p[3]])
                        }
                        _ if p[0] == 0 && p[1] == 0 && p[2] == 255 => {
                            let [r, g, b] = color_map.blue.0;
                            Rgba([r, g, b, p[3]])
                        }
                        _ => *p,
                    };
                    self.put_pixel(i, k, mapped_pixel);
                }
            }
        }
        self
    }
}

pub fn read_image(path: &str) -> Result<RgbaImage, Box<dyn Error>> {
    let file = ASSETS_DIR.get_file(path);
    if file.is_none() {
        return Err(format!("File {} not found", path).into());
    }
    let img = ImageReader::new(Cursor::new(file.unwrap().contents()))
        .with_guessed_format()?
        .decode()?
        .into_rgba8();
    Ok(img)
}
