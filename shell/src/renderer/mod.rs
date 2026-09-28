pub mod animation;
pub mod canvas;
pub mod text;

use anyhow::Result;
use tiny_skia::{Pixmap, PixmapMut, Transform};

pub struct Renderer {
    width: u32,
    height: u32,
    scale: f32,
}

impl Renderer {
    pub fn new(width: u32, height: u32, scale: f32) -> Self {
        Self { width, height, scale }
    }

    pub fn resize(&mut self, width: u32, height: u32, scale: f32) {
        self.width = width;
        self.height = height;
        self.scale = scale;
    }

    /// Provides a canvas (PixmapMut) to draw on.
    /// This abstracts the base tiny-skia Pixmap setup and applies the global scale.
    pub fn create_canvas(&self) -> Option<Pixmap> {
        Pixmap::new(self.width, self.height)
    }

    pub fn base_transform(&self) -> Transform {
        Transform::from_scale(self.scale, self.scale)
    }
}
