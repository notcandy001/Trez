use std::time::Duration;
use super::{layout::Rect, widget::Container, Widget};
use crate::renderer::{animation::{Animator, Easing}, canvas::Canvas};
use color_engine::Theme;

/// A notification popup widget with smooth slide-in animations.
pub struct NotificationToast {
    pub bounds: Rect,
    container: Container,
    animator: Animator,
}

impl NotificationToast {
    pub fn new() -> Self {
        let mut animator = Animator::new(
            Duration::from_millis(600),
            Easing::EaseOutExpo,
            -100.0, // Start offset (off-screen above)
            0.0,    // End offset (resting position)
        );
        animator.start();

        Self {
            bounds: Rect::new(0.0, 0.0, 320.0, 80.0),
            container: Container::new(),
            animator,
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
        // Evaluate animation state
        let y_offset = self.animator.value();

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

        // Draw with animated Y offset
        canvas.draw_rounded_rect(
            self.bounds.x,
            self.bounds.y + y_offset,
            self.bounds.width,
            self.bounds.height,
            12.0,
            bg,
        );

        canvas.draw_rounded_rect(
            self.bounds.x,
            self.bounds.y + y_offset,
            self.bounds.width,
            self.bounds.height,
            12.0,
            outline,
        );
        
        // Children would also need the offset applied, but for this MVP, 
        // the container handles its own drawing state.
        self.container.draw(canvas, theme);
    }
}
