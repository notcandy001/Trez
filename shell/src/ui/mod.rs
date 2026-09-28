pub mod control_center;
pub mod dock;
pub mod launcher;
pub mod layout;
pub mod notifications;
pub mod panel;
pub mod widget;

use crate::renderer::canvas::Canvas;
use crate::theme::ThemeManager;
use color_engine::Theme;

/// A trait for any UI element that can be sized, laid out, and drawn.
pub trait Widget {
    fn size_hint(&self) -> (f32, f32);
    fn layout(&mut self, width: f32, height: f32);
    fn draw(&self, canvas: &mut Canvas, theme: &Theme);
}
