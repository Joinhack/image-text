use crate::error::Error;
use image::Rgba;
use imageproc::drawing;
use rusttype::{Font, Scale};
use std::{fs::File, io::Read, path::Path};

pub struct PngTextBuilder {
    text: String,
    point: (f32, f32),
    color: [u8; 4],
    font_size: (f32, f32),
    png_path: String,
    font_path: String,
}

impl PngTextBuilder {
    pub fn new() -> PngTextBuilder {
        Self {
            text: String::new(),
            point: (0.0, 0.0),
            font_size: (45.0, 45.0),
            png_path: String::new(),
            font_path: String::new(),
            color: [255, 255, 255, 255],
        }
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = text.into();
        self
    }

    pub fn point(&mut self, point: (f32, f32)) -> &mut Self {
        self.point = point;
        self
    }

    pub fn font_size(&mut self, font_size: (f32, f32)) -> &mut Self {
        self.font_size = font_size;
        self
    }

    pub fn png_path(&mut self, png_path: &str) -> &mut Self {
        self.png_path = png_path.into();
        self
    }

    pub fn color(&mut self, color: [u8; 4]) -> &mut Self {
        self.color = color;
        self
    }

    pub fn font_path(&mut self, font_path: &str) -> &mut Self {
        self.font_path = font_path.into();
        self
    }

    fn load_file(path: &str) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        let mut file = File::open(path).map_err(|_| Error::ReadFileError)?;
        let _ = file
            .read_to_end(&mut buf)
            .map_err(|_| Error::ReadFileError)?;
        Ok(buf)
    }

    pub fn build(&mut self) -> Result<PngText, Error> {
        let font_buf = Self::load_file(&self.font_path)?;
        let png = Self::load_file(&self.png_path)?;
        let font = Font::try_from_vec(font_buf).ok_or(Error::FontError)?;
        Ok(PngText {
            text: self.text.clone(),
            point: self.point.clone(),
            font_size: Scale {
                x: self.font_size.0,
                y: self.font_size.1,
            },
            font,
            color: Rgba::from(self.color),
            png,
        })
    }
}

pub struct PngText {
    text: String,
    point: (f32, f32),
    font_size: Scale,
    color: Rgba<u8>,
    png: Vec<u8>,
    font: Font<'static>,
}

impl PngText {
    pub fn save(&self, outpath: impl AsRef<Path>) -> Result<(), Error> {
        let mut img = image::load_from_memory(&self.png).map_err(|_| Error::InvalidImage)?;
        let scale = self.font_size;
        let x = self.point.0 as _;
        let y = self.point.1 as _;
        drawing::draw_text_mut(&mut img, self.color, x, y, scale, &self.font, &self.text);
        img.save(outpath).map_err(|_| Error::ImageWriteError)?;
        Ok(())
    }
}
