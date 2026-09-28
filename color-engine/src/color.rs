use palette::{IntoColor, Oklab, Srgb};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OklabColor {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl OklabColor {
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Self { l, a, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0).into_linear();
        let oklab: Oklab = srgb.into_color();
        Self {
            l: oklab.l,
            a: oklab.a,
            b: oklab.b,
        }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let oklab = Oklab::new(self.l, self.a, self.b);
        let srgb: Srgb = oklab.into_color();
        (
            (srgb.red.max(0.0).min(1.0) * 255.0).round() as u8,
            (srgb.green.max(0.0).min(1.0) * 255.0).round() as u8,
            (srgb.blue.max(0.0).min(1.0) * 255.0).round() as u8,
        )
    }

    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dl = self.l - other.l;
        let da = self.a - other.a;
        let db = self.b - other.b;
        dl * dl + da * da + db * db
    }

    pub fn distance(&self, other: &Self) -> f32 {
        self.distance_squared(other).sqrt()
    }
}
