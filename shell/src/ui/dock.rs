use super::{layout::Rect, widget::Container, Widget};
use crate::renderer::canvas::Canvas;
use color_engine::Theme;

/// The minimal dock for launching or tracking applications.
pub struct Dock {
    pub bounds: Rect,
    container: Container,
}

impl Dock {
    pub fn new() -> Self {
        Self {
            bounds: Rect::new(0.0, 0.0, 400.0, 48.0),
            container: Container::new(),
        }
    }
}

impl Widget for Dock {
    fn size_hint(&self) -> (f32, f32) {
        (self.bounds.width, self.bounds.height)
    }

    fn layout(&mut self, width: f32, height: f32) {
        self.bounds.width = width;
        self.bounds.height = height;
        self.container.bounds = self.bounds;
        self.container.layout(width, height);
    }

    fn draw(&self, canvas: &mut Canvas, theme: &Theme) {
        let bg = [
            theme.surface.r,
            theme.surface.g,
            theme.surface.b,
            230,
        ];
        
        let outline = [
            theme.border.r,
            theme.border.g,
            theme.border.b,
            255,
        ];

        // Draw an elegant floating pill shape for the dock
        // Instead of excessive shadows, we use a simple border
        canvas.draw_rounded_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            self.bounds.height,
            24.0, // Floating pill radius
            bg,
        );

        canvas.draw_rounded_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            self.bounds.height,
            24.0,
            outline,
        );
        
        self.container.draw(canvas, theme);
    }
}
