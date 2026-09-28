use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, imageops::FilterType};
use std::path::Path;

pub struct ImageContext {
    pub pixels: Vec<(u8, u8, u8)>,
    pub width: u32,
    pub height: u32,
}

impl ImageContext {
    /// Loads an image from a path and resizes it for fast color analysis.
    pub fn load_and_resize<P: AsRef<Path>>(path: P, max_dimension: u32) -> Result<Self> {
        let img = image::open(path.as_ref())
            .with_context(|| format!("failed to open image: {}", path.as_ref().display()))?;
        
        Self::from_dynamic_image(&img, max_dimension)
    }

    pub fn from_dynamic_image(img: &DynamicImage, max_dimension: u32) -> Result<Self> {
        let (width, height) = img.dimensions();
        
        let resized = if width > max_dimension || height > max_dimension {
            img.resize(max_dimension, max_dimension, FilterType::Triangle)
        } else {
            img.clone()
        };

        let rgb = resized.to_rgb8();
        let (new_width, new_height) = rgb.dimensions();
        
        let pixels = rgb.pixels().map(|p| (p[0], p[1], p[2])).collect();

        Ok(Self {
            pixels,
            width: new_width,
            height: new_height,
        })
    }
}
