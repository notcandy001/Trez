use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub is_dark: bool,
    
    // Backgrounds
    pub background: Color,
    pub background_secondary: Color,
    
    // Surfaces
    pub surface: Color,
    pub surface_elevated: Color,

    // Foregrounds (text/icons)
    pub foreground: Color,
    pub foreground_secondary: Color,
    pub foreground_muted: Color,

    // Accents
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_active: Color,

    // Status
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // UI elements
    pub border: Color,
    pub selection: Color,
    pub workspace_active: Color,
    pub workspace_inactive: Color,
}
