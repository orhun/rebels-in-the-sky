use image::{ImageBuffer, Rgba};
use ratatui::text::Line;

use crate::{store::ASSETS_DIR, types::AppResult, ui::utils::img_to_lines};

pub type Gif = Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>;
pub type FrameLines = Vec<Line<'static>>;
pub type GifLines = Vec<FrameLines>;

pub trait PrintableGif: Sized {
    fn open(filename: String) -> AppResult<Self>;
    fn to_lines(&self) -> GifLines;
}

impl PrintableGif for Gif {
    fn open(filename: String) -> AppResult<Gif> {
        let mut decoder = gif::DecodeOptions::new();
        // Configure the decoder such that it will expand the image to RGBA.
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let file = ASSETS_DIR
            .get_file(filename.clone())
            .ok_or(format!("Unable to open file {}", filename))?
            .contents();
        let mut decoder = decoder.read_info(file)?;
        let mut gif: Gif = vec![];
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            let img = ImageBuffer::from_raw(
                frame.width as u32,
                frame.height as u32,
                frame.buffer.to_vec(),
            )
            .ok_or(format!("Unable to decode file {} into gif", filename))?;
            gif.push(img);
        }
        Ok(gif)
    }

    fn to_lines(&self) -> GifLines {
        self.iter().map(|img| img_to_lines(img)).collect()
    }
}
