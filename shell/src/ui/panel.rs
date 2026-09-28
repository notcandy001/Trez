use super::{layout::Rect, widget::Container, Widget};
use crate::renderer::canvas::Canvas;
use color_engine::Theme;

/// The primary status panel for the shell.
pub struct Panel {
    pub bounds: Rect,
    container: Container,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            bounds: Rect::new(0.0, 0.0, 0.0, 32.0),
            container: Container::new(),
        }
    }
}

impl Widget for Panel {
    fn size_hint(&self) -> (f32, f32) {
        (self.bounds.width, self.bounds.height)
    }

    fn layout(&mut self, width: f32, height: f32) {
        self.bounds.width = width;
        // Panel sticks to the top, typically full width.
        self.container.bounds = self.bounds;
        self.container.layout(width, height);
    }

    fn draw(&self, canvas: &mut Canvas, theme: &Theme) {
        // Semantic color mapping for the panel
        // Use surface_elevated for a subtle, spatial presence without heavy drop shadows
        let bg = [
            theme.surface_elevated.r,
            theme.surface_elevated.g,
            theme.surface_elevated.b,
            240, // slight transparency, not excessive blur
        ];
        
        let border = [
            theme.border.r,
            theme.border.g,
            theme.border.b,
            255,
        ];

        // Draw panel background
        canvas.draw_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            self.bounds.height,
            bg,
        );

        // Draw an elegant, minimal bottom border for structural separation
        canvas.draw_rect(
            self.bounds.x,
            self.bounds.y + self.bounds.height - 1.0,
            self.bounds.width,
            1.0,
            border,
        );

        // Draw children
        self.container.draw(canvas, theme);
    }
}
