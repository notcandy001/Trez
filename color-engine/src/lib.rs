pub mod cluster;
pub mod color;
pub mod image;
pub mod palette;
pub mod theme;

use crate::cluster::kmeans;
use crate::color::OklabColor;
use crate::image::ImageContext;
use crate::palette::PaletteGenerator;
pub use crate::theme::{Color, Theme};
use anyhow::Result;
use std::path::Path;

pub struct EngineOptions {
    pub max_image_dimension: u32,
    pub cluster_count: usize,
    pub max_iterations: usize,
    pub prefer_dark: Option<bool>,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            max_image_dimension: 128,
            cluster_count: 8,
            max_iterations: 20,
            prefer_dark: None,
        }
    }
}

pub struct Engine {
    options: EngineOptions,
}

impl Engine {
    pub fn new(options: EngineOptions) -> Self {
        Self { options }
    }

    pub fn extract_theme_from_path<P: AsRef<Path>>(&self, path: P) -> Result<Theme> {
        let img = ImageContext::load_and_resize(path, self.options.max_image_dimension)?;
        self.extract_theme_from_context(&img)
    }

    pub fn extract_theme_from_context(&self, ctx: &ImageContext) -> Result<Theme> {
        // Normalize and convert to Oklab
        let oklab_colors: Vec<OklabColor> = ctx
            .pixels
            .iter()
            .map(|&(r, g, b)| OklabColor::from_rgb(r, g, b))
            .collect();

        // Cluster colors
        let clusters = kmeans(
            &oklab_colors,
            self.options.cluster_count,
            self.options.max_iterations,
        );

        // Generate palette
        let mut generator = PaletteGenerator::new();
        generator.prefer_dark = self.options.prefer_dark;
        
        Ok(generator.generate(&clusters))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::image::{ImageBuffer, Rgb};

    fn create_test_image(color: [u8; 3]) -> ::image::DynamicImage {
        let img = ImageBuffer::from_fn(64, 64, |_, _| Rgb(color));
        ::image::DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn deterministic_extraction() {
        // Red image
        let img = create_test_image([255, 0, 0]);
        let ctx = ImageContext::from_dynamic_image(&img, 64).unwrap();
        
        let engine = Engine::new(EngineOptions::default());
        
        let theme1 = engine.extract_theme_from_context(&ctx).unwrap();
        let theme2 = engine.extract_theme_from_context(&ctx).unwrap();
        
        assert_eq!(theme1, theme2, "Theme extraction must be deterministic");
    }

    #[test]
    fn basic_dark_theme_extraction() {
        // Very dark image
        let img = create_test_image([10, 10, 10]);
        let ctx = ImageContext::from_dynamic_image(&img, 64).unwrap();
        
        let engine = Engine::new(EngineOptions::default());
        let theme = engine.extract_theme_from_context(&ctx).unwrap();
        
        assert!(theme.is_dark, "Dark image should yield dark theme");
    }
    
    #[test]
    fn basic_light_theme_extraction() {
        // Very bright image
        let img = create_test_image([240, 240, 240]);
        let ctx = ImageContext::from_dynamic_image(&img, 64).unwrap();
        
        let engine = Engine::new(EngineOptions::default());
        let theme = engine.extract_theme_from_context(&ctx).unwrap();
        
        assert!(!theme.is_dark, "Light image should yield light theme");
    }
}
