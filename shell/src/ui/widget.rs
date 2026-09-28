use super::{layout::Rect, Widget};
use crate::renderer::canvas::Canvas;
use color_engine::Theme;

pub struct Container {
    pub bounds: Rect,
    pub children: Vec<Box<dyn Widget>>,
    pub background: Option<[u8; 4]>,
    pub border_radius: f32,
}

impl Container {
    pub fn new() -> Self {
        Self {
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            children: Vec::new(),
            background: None,
            border_radius: 0.0,
        }
    }

    pub fn with_background(mut self, color: [u8; 4]) -> Self {
        self.background = Some(color);
        self
    }
}

impl Widget for Container {
    fn size_hint(&self) -> (f32, f32) {
        (self.bounds.width, self.bounds.height)
    }

    fn layout(&mut self, width: f32, height: f32) {
        self.bounds.width = width;
        self.bounds.height = height;
        // Simple stacking layout for now
        for child in &mut self.children {
            child.layout(width, height);
        }
    }

    fn draw(&self, canvas: &mut Canvas, theme: &Theme) {
        if let Some(bg) = self.background {
            if self.border_radius > 0.0 {
                canvas.draw_rounded_rect(
                    self.bounds.x,
                    self.bounds.y,
                    self.bounds.width,
                    self.bounds.height,
                    self.border_radius,
                    bg,
                );
            } else {
                canvas.draw_rect(
                    self.bounds.x,
                    self.bounds.y,
                    self.bounds.width,
                    self.bounds.height,
                    bg,
                );
            }
        }

        for child in &self.children {
            child.draw(canvas, theme);
        }
    }
}
