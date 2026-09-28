use super::{layout::Rect, widget::Container, Widget};
use crate::renderer::canvas::Canvas;
use color_engine::Theme;

/// A notification popup widget.
pub struct NotificationToast {
    pub bounds: Rect,
    container: Container,
}

impl NotificationToast {
    pub fn new() -> Self {
        Self {
            bounds: Rect::new(0.0, 0.0, 320.0, 80.0),
            container: Container::new(),
        }
    }
}

impl Widget for NotificationToast {
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
            theme.surface_elevated.r,
            theme.surface_elevated.g,
            theme.surface_elevated.b,
            245,
        ];
        
        let outline = [
            theme.border.r,
            theme.border.g,
            theme.border.b,
            255,
        ];

        canvas.draw_rounded_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            self.bounds.height,
            12.0,
            bg,
        );

        canvas.draw_rounded_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            self.bounds.height,
            12.0,
            outline,
        );
        
        self.container.draw(canvas, theme);
    }
}
