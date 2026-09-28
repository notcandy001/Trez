use crate::cluster::Cluster;
use crate::color::OklabColor;
use crate::theme::{Color, Theme};

pub struct PaletteGenerator {
    pub prefer_dark: Option<bool>,
}

impl PaletteGenerator {
    pub fn new() -> Self {
        Self { prefer_dark: None }
    }

    pub fn generate(&self, clusters: &[Cluster]) -> Theme {
        if clusters.is_empty() {
            return self.default_theme(true);
        }

        // Determine if wallpaper is predominantly light or dark
        let avg_l = clusters.iter()
            .map(|c| c.centroid.l * c.weight as f32)
            .sum::<f32>() / clusters.iter().map(|c| c.weight).sum::<usize>() as f32;
        
        let is_dark = self.prefer_dark.unwrap_or(avg_l < 0.6);

        // Find primary background (dominant color, but ensuring it fits light/dark)
        let bg_cluster = clusters.iter().find(|c| {
            if is_dark { c.centroid.l < 0.4 } else { c.centroid.l > 0.8 }
        }).unwrap_or(&clusters[0]);

        let mut bg = bg_cluster.centroid;
        // Enforce background luminance constraints
        if is_dark {
            bg.l = bg.l.min(0.25);
        } else {
            bg.l = bg.l.max(0.9);
        }

        // Find best accent (colorful, high chroma)
        let mut best_accent = clusters[0].centroid;
        let mut best_score = -1.0;
        
        for c in clusters {
            let chroma = (c.centroid.a * c.centroid.a + c.centroid.b * c.centroid.b).sqrt();
            // Score = chroma + weight bonus
            let score = chroma + (c.weight as f32 / clusters[0].weight as f32) * 0.1;
            
            // Ensure contrast with background
            let contrast = (c.centroid.l - bg.l).abs();
            if contrast > 0.3 && score > best_score {
                best_score = score;
                best_accent = c.centroid;
            }
        }
        
        // If we couldn't find a good accent, make one up by modifying background hue
        if best_score < 0.05 {
            best_accent = bg;
            best_accent.a += 0.1;
            best_accent.b += 0.1;
            if is_dark { best_accent.l = 0.7; } else { best_accent.l = 0.3; }
        }

        let bg_rgb = bg.to_rgb();
        let accent_rgb = best_accent.to_rgb();

        // Calculate foreground based on background
        let fg_l = if is_dark { 0.95 } else { 0.1 };
        let fg = OklabColor::new(fg_l, bg.a * 0.1, bg.b * 0.1); // subtle tint
        let fg_rgb = fg.to_rgb();

        Theme {
            is_dark,
            background: Color::new(bg_rgb.0, bg_rgb.1, bg_rgb.2),
            background_secondary: Self::shift_l(&bg, if is_dark { 0.05 } else { -0.05 }),
            surface: Self::shift_l(&bg, if is_dark { 0.08 } else { -0.08 }),
            surface_elevated: Self::shift_l(&bg, if is_dark { 0.12 } else { -0.12 }),
            
            foreground: Color::new(fg_rgb.0, fg_rgb.1, fg_rgb.2),
            foreground_secondary: Self::shift_l(&fg, if is_dark { -0.2 } else { 0.2 }),
            foreground_muted: Self::shift_l(&fg, if is_dark { -0.4 } else { 0.4 }),
            
            accent: Color::new(accent_rgb.0, accent_rgb.1, accent_rgb.2),
            accent_hover: Self::shift_l(&best_accent, if is_dark { 0.1 } else { -0.1 }),
            accent_active: Self::shift_l(&best_accent, if is_dark { -0.1 } else { 0.1 }),
            
            // Semantic colors (standardized but tinted by accent slightly if desired)
            success: Color::new(46, 204, 113),
            warning: Color::new(241, 196, 15),
            error: Color::new(231, 76, 60),
            info: Color::new(52, 152, 219),
            
            border: Self::shift_l(&bg, if is_dark { 0.15 } else { -0.15 }),
            selection: Self::shift_l(&best_accent, if is_dark { -0.2 } else { 0.2 }),
            workspace_active: Color::new(accent_rgb.0, accent_rgb.1, accent_rgb.2),
            workspace_inactive: Self::shift_l(&bg, if is_dark { 0.2 } else { -0.2 }),
        }
    }

    fn shift_l(color: &OklabColor, dl: f32) -> Color {
        let mut shifted = *color;
        shifted.l = (shifted.l + dl).clamp(0.0, 1.0);
        let rgb = shifted.to_rgb();
        Color::new(rgb.0, rgb.1, rgb.2)
    }

    fn default_theme(&self, is_dark: bool) -> Theme {
        if is_dark {
            Theme {
                is_dark: true,
                background: Color::new(20, 20, 20),
                background_secondary: Color::new(30, 30, 30),
                surface: Color::new(40, 40, 40),
                surface_elevated: Color::new(50, 50, 50),
                foreground: Color::new(240, 240, 240),
                foreground_secondary: Color::new(200, 200, 200),
                foreground_muted: Color::new(150, 150, 150),
                accent: Color::new(52, 152, 219),
                accent_hover: Color::new(41, 128, 185),
                accent_active: Color::new(31, 97, 141),
                success: Color::new(46, 204, 113),
                warning: Color::new(241, 196, 15),
                error: Color::new(231, 76, 60),
                info: Color::new(52, 152, 219),
                border: Color::new(60, 60, 60),
                selection: Color::new(41, 128, 185),
                workspace_active: Color::new(52, 152, 219),
                workspace_inactive: Color::new(100, 100, 100),
            }
        } else {
            Theme {
                is_dark: false,
                background: Color::new(250, 250, 250),
                background_secondary: Color::new(240, 240, 240),
                surface: Color::new(230, 230, 230),
                surface_elevated: Color::new(220, 220, 220),
                foreground: Color::new(20, 20, 20),
                foreground_secondary: Color::new(60, 60, 60),
                foreground_muted: Color::new(100, 100, 100),
                accent: Color::new(52, 152, 219),
                accent_hover: Color::new(41, 128, 185),
                accent_active: Color::new(31, 97, 141),
                success: Color::new(46, 204, 113),
                warning: Color::new(241, 196, 15),
                error: Color::new(231, 76, 60),
                info: Color::new(52, 152, 219),
                border: Color::new(200, 200, 200),
                selection: Color::new(174, 214, 241),
                workspace_active: Color::new(52, 152, 219),
                workspace_inactive: Color::new(180, 180, 180),
            }
        }
    }
}
