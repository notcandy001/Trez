use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use tiny_skia::{Color, PixmapMut};

pub struct TextRenderer {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }

    pub fn draw_text(
        &mut self,
        pixmap: &mut PixmapMut,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        color: [u8; 4],
    ) {
        let metrics = Metrics::new(font_size, font_size);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        buffer.set_size(None, None);
        buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.font_system, true);

        let _text_color = Color::from_rgba8(color[0], color[1], color[2], color[3]);

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((x, y), 1.0);
                
                let image = self.swash_cache.get_image(
                    &mut self.font_system,
                    physical_glyph.cache_key,
                );

                if let Some(image) = image {
                    let _rx = physical_glyph.x as f32 + image.placement.left as f32;
                    let _ry = physical_glyph.y as f32 - image.placement.top as f32;

                    // Iterate over pixels of the glyph image and blend into pixmap.
                    // (Omitted for brevity in this foundational block. In a real shell, 
                    // this would iterate `image.data` and blend alpha onto `pixmap`.)
                }
            }
        }
    }
}
