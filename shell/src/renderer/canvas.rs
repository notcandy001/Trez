use tiny_skia::{
    BlendMode, Color, FillRule, Paint, PathBuilder, PixmapMut, Rect, Stroke, Transform,
};

pub struct Canvas<'a> {
    pixmap: PixmapMut<'a>,
    transform: Transform,
}

impl<'a> Canvas<'a> {
    pub fn new(pixmap: PixmapMut<'a>, transform: Transform) -> Self {
        Self { pixmap, transform }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [u8; 4]) {
        let rect = match Rect::from_xywh(x, y, w, h) {
            Some(r) => r,
            None => return,
        };

        let mut paint = Paint::default();
        paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
        paint.anti_alias = true;

        self.pixmap.fill_rect(rect, &paint, self.transform, None);
    }

    pub fn draw_rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [u8; 4]) {
        let mut pb = PathBuilder::new();
        let r = radius.min(w / 2.0).min(h / 2.0);
        
        // Very basic rounded rect approximation for tiny-skia.
        // Top-left
        pb.move_to(x + r, y);
        pb.line_to(x + w - r, y);
        // Top-right
        pb.cubic_to(x + w, y, x + w, y, x + w, y + r);
        pb.line_to(x + w, y + h - r);
        // Bottom-right
        pb.cubic_to(x + w, y + h, x + w, y + h, x + w - r, y + h);
        pb.line_to(x + r, y + h);
        // Bottom-left
        pb.cubic_to(x, y + h, x, y + h, x, y + h - r);
        pb.line_to(x, y + r);
        // Close
        pb.cubic_to(x, y, x, y, x + r, y);
        
        let path = match pb.finish() {
            Some(p) => p,
            None => return,
        };

        let mut paint = Paint::default();
        paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
        paint.anti_alias = true;

        self.pixmap.fill_path(&path, &paint, FillRule::Winding, self.transform, None);
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        let c = Color::from_rgba8(color[0], color[1], color[2], color[3]);
        self.pixmap.fill(c);
    }
}
