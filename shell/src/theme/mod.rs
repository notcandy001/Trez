use color_engine::{Color, Theme};

pub struct ThemeManager {
    current: Theme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current: Self::default_theme(),
        }
    }

    pub fn current(&self) -> &Theme {
        &self.current
    }

    pub fn update(&mut self, theme: Theme) {
        self.current = theme;
    }

    fn default_theme() -> Theme {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_manager_initializes_with_default() {
        let manager = ThemeManager::new();
        assert_eq!(manager.current().is_dark, true);
    }

    #[test]
    fn theme_manager_updates_theme() {
        let mut manager = ThemeManager::new();
        let mut new_theme = ThemeManager::default_theme();
        new_theme.is_dark = false;
        
        manager.update(new_theme.clone());
        assert_eq!(manager.current().is_dark, false);
    }
}
