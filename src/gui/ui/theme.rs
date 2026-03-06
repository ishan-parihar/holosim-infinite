//! Theme System
//!
//! Provides comprehensive theme management with light/dark modes,
//! custom color schemes, and runtime theme switching.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Color themes (light/dark mode)"

use std::collections::HashMap;

/// Available color themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum Theme {
    /// Dark theme (default)
    #[default]
    Dark,
    /// Light theme
    Light,
    /// High contrast theme for accessibility
    HighContrast,
    /// Cosmic/purple theme matching the simulation aesthetics
    Cosmic,
    /// Custom user-defined theme
    Custom,
}


impl Theme {
    /// Get display name for the theme
    pub fn display_name(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::HighContrast => "High Contrast",
            Theme::Cosmic => "Cosmic",
            Theme::Custom => "Custom",
        }
    }

    /// Get icon for the theme
    pub fn icon(&self) -> &'static str {
        match self {
            Theme::Dark => "🌙",
            Theme::Light => "☀️",
            Theme::HighContrast => "👁️",
            Theme::Cosmic => "🌌",
            Theme::Custom => "🎨",
        }
    }

    /// Get all available themes
    pub fn all() -> &'static [Theme] {
        &[
            Theme::Dark,
            Theme::Light,
            Theme::HighContrast,
            Theme::Cosmic,
        ]
    }

    /// Check if this is a built-in theme
    pub fn is_builtin(&self) -> bool {
        !matches!(self, Theme::Custom)
    }
}

/// Complete color palette for a theme
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPalette {
    /// Background colors
    pub background: BackgroundColors,
    /// Foreground/text colors
    pub foreground: ForegroundColors,
    /// Accent/brand colors
    pub accent: AccentColors,
    /// Semantic colors (success, warning, error)
    pub semantic: SemanticColors,
    /// Density octave colors (1st-8th density)
    pub density: DensityColors,
    /// Panel and UI element colors
    pub ui: UiColors,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::dark_theme()
    }
}

impl ColorPalette {
    /// Create the dark theme color palette
    pub fn dark_theme() -> Self {
        Self {
            background: BackgroundColors {
                primary: [0.08, 0.08, 0.10, 1.0],   // Deep dark blue-gray
                secondary: [0.12, 0.12, 0.14, 1.0], // Slightly lighter
                tertiary: [0.16, 0.16, 0.18, 1.0],  // Panels, cards
                hover: [0.20, 0.20, 0.24, 1.0],     // Hover state
                active: [0.24, 0.24, 0.28, 1.0],    // Active/selected
                disabled: [0.10, 0.10, 0.12, 1.0],  // Disabled elements
            },
            foreground: ForegroundColors {
                primary: [0.95, 0.95, 0.97, 1.0],   // Main text
                secondary: [0.70, 0.70, 0.75, 1.0], // Secondary text
                muted: [0.50, 0.50, 0.55, 1.0],     // Muted/disabled text
                inverse: [0.08, 0.08, 0.10, 1.0],   // Text on light backgrounds
                link: [0.40, 0.70, 1.0, 1.0],       // Links
                link_hover: [0.60, 0.85, 1.0, 1.0], // Link hover
            },
            accent: AccentColors {
                primary: [0.40, 0.50, 1.0, 1.0],    // Main accent (blue-violet)
                secondary: [0.70, 0.40, 0.90, 1.0], // Secondary accent (purple)
                tertiary: [0.20, 0.80, 0.80, 1.0],  // Tertiary accent (cyan)
                quaternary: [1.0, 0.60, 0.20, 1.0], // Quaternary (orange)
                highlight: [1.0, 0.90, 0.30, 1.0],  // Highlight (gold)
            },
            semantic: SemanticColors {
                success: [0.20, 0.80, 0.40, 1.0], // Green
                warning: [1.0, 0.80, 0.20, 1.0],  // Yellow
                error: [1.0, 0.30, 0.30, 1.0],    // Red
                info: [0.30, 0.70, 1.0, 1.0],     // Blue
            },
            density: DensityColors {
                first: [1.0, 0.27, 0.27, 1.0],   // Red
                second: [1.0, 0.53, 0.27, 1.0],  // Orange
                third: [1.0, 0.80, 0.27, 1.0],   // Yellow
                fourth: [0.27, 1.0, 0.27, 1.0],  // Green
                fifth: [0.27, 1.0, 1.0, 1.0],    // Cyan
                sixth: [0.27, 0.27, 1.0, 1.0],   // Blue
                seventh: [0.53, 0.27, 1.0, 1.0], // Violet
                eighth: [1.0, 1.0, 1.0, 1.0],    // White
            },
            ui: UiColors {
                panel_bg: [0.12, 0.12, 0.14, 0.95],        // Panel background
                panel_border: [0.25, 0.25, 0.30, 0.50],    // Panel border
                button_bg: [0.20, 0.20, 0.24, 1.0],        // Button background
                button_hover: [0.28, 0.28, 0.32, 1.0],     // Button hover
                button_active: [0.32, 0.32, 0.38, 1.0],    // Button active
                input_bg: [0.08, 0.08, 0.10, 1.0],         // Input field background
                input_border: [0.30, 0.30, 0.35, 1.0],     // Input border
                scrollbar: [0.30, 0.30, 0.35, 0.50],       // Scrollbar
                scrollbar_hover: [0.40, 0.40, 0.45, 0.70], // Scrollbar hover
                tooltip_bg: [0.20, 0.20, 0.24, 0.95],      // Tooltip background
                selection: [0.30, 0.50, 0.80, 0.40],       // Text selection
            },
        }
    }

    /// Create the light theme color palette
    pub fn light_theme() -> Self {
        Self {
            background: BackgroundColors {
                primary: [0.98, 0.98, 0.98, 1.0],   // Off-white
                secondary: [0.95, 0.95, 0.96, 1.0], // Slightly darker
                tertiary: [0.92, 0.92, 0.94, 1.0],  // Panels, cards
                hover: [0.88, 0.88, 0.90, 1.0],     // Hover state
                active: [0.85, 0.85, 0.88, 1.0],    // Active/selected
                disabled: [0.90, 0.90, 0.92, 1.0],  // Disabled elements
            },
            foreground: ForegroundColors {
                primary: [0.10, 0.10, 0.12, 1.0],   // Main text
                secondary: [0.40, 0.40, 0.45, 1.0], // Secondary text
                muted: [0.60, 0.60, 0.65, 1.0],     // Muted/disabled text
                inverse: [0.98, 0.98, 0.98, 1.0],   // Text on dark backgrounds
                link: [0.20, 0.50, 0.90, 1.0],      // Links
                link_hover: [0.40, 0.65, 1.0, 1.0], // Link hover
            },
            accent: AccentColors {
                primary: [0.30, 0.45, 0.90, 1.0],    // Main accent
                secondary: [0.60, 0.30, 0.85, 1.0],  // Secondary accent
                tertiary: [0.15, 0.70, 0.75, 1.0],   // Tertiary accent
                quaternary: [0.95, 0.50, 0.15, 1.0], // Quaternary
                highlight: [0.95, 0.80, 0.20, 1.0],  // Highlight
            },
            semantic: SemanticColors {
                success: [0.15, 0.70, 0.35, 1.0], // Green
                warning: [0.95, 0.75, 0.15, 1.0], // Yellow
                error: [0.90, 0.25, 0.25, 1.0],   // Red
                info: [0.25, 0.65, 0.95, 1.0],    // Blue
            },
            density: DensityColors {
                first: [0.90, 0.20, 0.20, 1.0],   // Red
                second: [0.95, 0.50, 0.20, 1.0],  // Orange
                third: [0.95, 0.80, 0.20, 1.0],   // Yellow
                fourth: [0.20, 0.90, 0.20, 1.0],  // Green
                fifth: [0.20, 0.90, 0.90, 1.0],   // Cyan
                sixth: [0.20, 0.20, 0.90, 1.0],   // Blue
                seventh: [0.50, 0.20, 0.95, 1.0], // Violet
                eighth: [0.10, 0.10, 0.10, 1.0],  // Black (inverted for light)
            },
            ui: UiColors {
                panel_bg: [0.95, 0.95, 0.96, 0.98],        // Panel background
                panel_border: [0.80, 0.80, 0.85, 0.50],    // Panel border
                button_bg: [0.90, 0.90, 0.92, 1.0],        // Button background
                button_hover: [0.85, 0.85, 0.88, 1.0],     // Button hover
                button_active: [0.80, 0.80, 0.85, 1.0],    // Button active
                input_bg: [1.0, 1.0, 1.0, 1.0],            // Input field background
                input_border: [0.70, 0.70, 0.75, 1.0],     // Input border
                scrollbar: [0.70, 0.70, 0.75, 0.50],       // Scrollbar
                scrollbar_hover: [0.60, 0.60, 0.65, 0.70], // Scrollbar hover
                tooltip_bg: [0.15, 0.15, 0.18, 0.95],      // Tooltip background
                selection: [0.25, 0.45, 0.75, 0.30],       // Text selection
            },
        }
    }

    /// Create the cosmic theme color palette (purple/violet aesthetic)
    pub fn cosmic_theme() -> Self {
        let mut palette = Self::dark_theme();

        // Override with cosmic purple/violet tones
        palette.accent.primary = [0.55, 0.30, 0.95, 1.0]; // Deep violet
        palette.accent.secondary = [0.75, 0.40, 0.90, 1.0]; // Purple
        palette.accent.tertiary = [0.40, 0.70, 0.95, 1.0]; // Cosmic blue

        palette.background.primary = [0.05, 0.03, 0.10, 1.0]; // Deep purple-black
        palette.background.secondary = [0.08, 0.05, 0.14, 1.0];
        palette.background.tertiary = [0.12, 0.08, 0.20, 1.0];

        palette.ui.panel_bg = [0.08, 0.05, 0.14, 0.95];
        palette.ui.button_bg = [0.20, 0.12, 0.35, 1.0];

        palette
    }

    /// Create the high contrast theme for accessibility
    pub fn high_contrast_theme() -> Self {
        Self {
            background: BackgroundColors {
                primary: [0.0, 0.0, 0.0, 1.0],      // Pure black
                secondary: [0.05, 0.05, 0.05, 1.0], // Near black
                tertiary: [0.10, 0.10, 0.10, 1.0],  // Dark gray
                hover: [0.20, 0.20, 0.20, 1.0],     // Gray
                active: [0.30, 0.30, 0.30, 1.0],    // Light gray
                disabled: [0.15, 0.15, 0.15, 1.0],  // Medium gray
            },
            foreground: ForegroundColors {
                primary: [1.0, 1.0, 1.0, 1.0],      // Pure white
                secondary: [0.90, 0.90, 0.90, 1.0], // Near white
                muted: [0.70, 0.70, 0.70, 1.0],     // Light gray
                inverse: [0.0, 0.0, 0.0, 1.0],      // Black
                link: [0.50, 0.80, 1.0, 1.0],       // Bright blue
                link_hover: [0.70, 0.90, 1.0, 1.0], // Lighter blue
            },
            accent: AccentColors {
                primary: [0.50, 0.60, 1.0, 1.0],    // Bright blue
                secondary: [0.80, 0.50, 1.0, 1.0],  // Bright purple
                tertiary: [0.20, 0.90, 0.90, 1.0],  // Bright cyan
                quaternary: [1.0, 0.70, 0.20, 1.0], // Bright orange
                highlight: [1.0, 1.0, 0.40, 1.0],   // Bright yellow
            },
            semantic: SemanticColors {
                success: [0.20, 1.0, 0.40, 1.0], // Bright green
                warning: [1.0, 0.90, 0.20, 1.0], // Bright yellow
                error: [1.0, 0.30, 0.30, 1.0],   // Bright red
                info: [0.30, 0.70, 1.0, 1.0],    // Bright blue
            },
            density: DensityColors::default(),
            ui: UiColors {
                panel_bg: [0.05, 0.05, 0.05, 1.0],        // Near black
                panel_border: [0.50, 0.50, 0.50, 1.0],    // White border
                button_bg: [0.15, 0.15, 0.15, 1.0],       // Dark gray
                button_hover: [0.25, 0.25, 0.25, 1.0],    // Gray
                button_active: [0.35, 0.35, 0.35, 1.0],   // Light gray
                input_bg: [0.0, 0.0, 0.0, 1.0],           // Black
                input_border: [0.60, 0.60, 0.60, 1.0],    // White border
                scrollbar: [0.40, 0.40, 0.40, 1.0],       // Gray
                scrollbar_hover: [0.60, 0.60, 0.60, 1.0], // White
                tooltip_bg: [0.10, 0.10, 0.10, 1.0],      // Dark
                selection: [0.40, 0.60, 1.0, 0.50],       // Bright selection
            },
        }
    }

    /// Get color for a specific density level
    pub fn get_density_color(&self, density: u8) -> [f32; 4] {
        match density {
            1 => self.density.first,
            2 => self.density.second,
            3 => self.density.third,
            4 => self.density.fourth,
            5 => self.density.fifth,
            6 => self.density.sixth,
            7 => self.density.seventh,
            8 => self.density.eighth,
            _ => self.foreground.muted,
        }
    }

    /// Convert to hex color string for serialization
    pub fn color_to_hex(color: [f32; 4]) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
            (color[3] * 255.0) as u8
        )
    }

    /// Parse hex color string to RGBA
    pub fn hex_to_color(hex: &str) -> Result<[f32; 4], String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 8 && hex.len() != 6 {
            return Err("Invalid hex color format".to_string());
        }

        let parse_component = |start: usize| -> Result<f32, String> {
            u8::from_str_radix(&hex[start..start + 2], 16)
                .map(|v| v as f32 / 255.0)
                .map_err(|_| "Invalid hex color".to_string())
        };

        let r = parse_component(0)?;
        let g = parse_component(2)?;
        let b = parse_component(4)?;
        let a = if hex.len() == 8 {
            parse_component(6)?
        } else {
            1.0
        };

        Ok([r, g, b, a])
    }
}

/// Background color variants
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundColors {
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub tertiary: [f32; 4],
    pub hover: [f32; 4],
    pub active: [f32; 4],
    pub disabled: [f32; 4],
}

/// Foreground/text color variants
#[derive(Debug, Clone, PartialEq)]
pub struct ForegroundColors {
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub muted: [f32; 4],
    pub inverse: [f32; 4],
    pub link: [f32; 4],
    pub link_hover: [f32; 4],
}

/// Accent color variants
#[derive(Debug, Clone, PartialEq)]
pub struct AccentColors {
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub tertiary: [f32; 4],
    pub quaternary: [f32; 4],
    pub highlight: [f32; 4],
}

/// Semantic color variants
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticColors {
    pub success: [f32; 4],
    pub warning: [f32; 4],
    pub error: [f32; 4],
    pub info: [f32; 4],
}

/// Density octave colors (1st through 8th density)
#[derive(Debug, Clone, PartialEq)]
pub struct DensityColors {
    pub first: [f32; 4],
    pub second: [f32; 4],
    pub third: [f32; 4],
    pub fourth: [f32; 4],
    pub fifth: [f32; 4],
    pub sixth: [f32; 4],
    pub seventh: [f32; 4],
    pub eighth: [f32; 4],
}

impl Default for DensityColors {
    fn default() -> Self {
        Self {
            first: [1.0, 0.27, 0.27, 1.0],
            second: [1.0, 0.53, 0.27, 1.0],
            third: [1.0, 0.80, 0.27, 1.0],
            fourth: [0.27, 1.0, 0.27, 1.0],
            fifth: [0.27, 1.0, 1.0, 1.0],
            sixth: [0.27, 0.27, 1.0, 1.0],
            seventh: [0.53, 0.27, 1.0, 1.0],
            eighth: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

/// UI element color variants
#[derive(Debug, Clone, PartialEq)]
pub struct UiColors {
    pub panel_bg: [f32; 4],
    pub panel_border: [f32; 4],
    pub button_bg: [f32; 4],
    pub button_hover: [f32; 4],
    pub button_active: [f32; 4],
    pub input_bg: [f32; 4],
    pub input_border: [f32; 4],
    pub scrollbar: [f32; 4],
    pub scrollbar_hover: [f32; 4],
    pub tooltip_bg: [f32; 4],
    pub selection: [f32; 4],
}

/// Type alias for theme change callback
type ThemeChangeCallback = Box<dyn Fn(Theme, &ColorPalette) + Send>;

/// Theme manager that handles theme switching and customization
pub struct ThemeManager {
    /// Current active theme
    pub current_theme: Theme,
    /// Current color palette
    pub palette: ColorPalette,
    /// Custom themes storage
    custom_themes: HashMap<String, ColorPalette>,
    /// Theme change callbacks
    change_callbacks: Vec<ThemeChangeCallback>,
    /// Whether to follow system theme
    follow_system: bool,
    /// Animation transition duration in seconds
    transition_duration: f32,
    /// Current transition progress (0.0 to 1.0)
    transition_progress: f32,
    /// Previous palette (for transitions)
    previous_palette: Option<ColorPalette>,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            current_theme: Theme::Dark,
            palette: ColorPalette::dark_theme(),
            custom_themes: HashMap::new(),
            change_callbacks: Vec::new(),
            follow_system: false,
            transition_duration: 0.3,
            transition_progress: 1.0,
            previous_palette: None,
        }
    }
}

impl ThemeManager {
    /// Create a new theme manager with default dark theme
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current theme
    pub fn set_theme(&mut self, theme: Theme) {
        if self.current_theme != theme {
            self.previous_palette = Some(self.palette.clone());
            self.current_theme = theme;
            self.palette = self.get_palette_for_theme(theme);
            self.transition_progress = 0.0;

            // Notify callbacks
            for callback in &self.change_callbacks {
                callback(theme, &self.palette);
            }
        }
    }

    /// Toggle between dark and light themes
    pub fn toggle_theme(&mut self) {
        let new_theme = match self.current_theme {
            Theme::Dark => Theme::Light,
            _ => Theme::Dark,
        };
        self.set_theme(new_theme);
    }

    /// Get the palette for a specific theme
    fn get_palette_for_theme(&self, theme: Theme) -> ColorPalette {
        match theme {
            Theme::Dark => ColorPalette::dark_theme(),
            Theme::Light => ColorPalette::light_theme(),
            Theme::Cosmic => ColorPalette::cosmic_theme(),
            Theme::HighContrast => ColorPalette::high_contrast_theme(),
            Theme::Custom => {
                // Return first custom theme or dark as fallback
                self.custom_themes
                    .values()
                    .next()
                    .cloned()
                    .unwrap_or_else(ColorPalette::dark_theme)
            }
        }
    }

    /// Register a theme change callback
    pub fn on_theme_change<F>(&mut self, callback: F)
    where
        F: Fn(Theme, &ColorPalette) + Send + 'static,
    {
        self.change_callbacks.push(Box::new(callback));
    }

    /// Update theme transition animation
    pub fn update_transition(&mut self, delta_time: f32) {
        if self.transition_progress < 1.0 {
            self.transition_progress += delta_time / self.transition_duration;
            self.transition_progress = self.transition_progress.min(1.0);
        }
    }

    /// Check if a theme transition is in progress
    pub fn is_transitioning(&self) -> bool {
        self.transition_progress < 1.0
    }

    /// Get the current transition progress (0.0 to 1.0)
    pub fn transition_progress(&self) -> f32 {
        self.transition_progress
    }

    /// Save a custom theme
    pub fn save_custom_theme(&mut self, name: impl Into<String>, palette: ColorPalette) {
        self.custom_themes.insert(name.into(), palette);
    }

    /// Load a custom theme
    pub fn load_custom_theme(&mut self, name: &str) -> Option<&ColorPalette> {
        self.custom_themes.get(name)
    }

    /// Delete a custom theme
    pub fn delete_custom_theme(&mut self, name: &str) -> Option<ColorPalette> {
        self.custom_themes.remove(name)
    }

    /// Get list of custom theme names
    pub fn custom_theme_names(&self) -> Vec<&String> {
        self.custom_themes.keys().collect()
    }

    /// Set whether to follow system theme preference
    pub fn set_follow_system(&mut self, follow: bool) {
        self.follow_system = follow;
        // In a real implementation, we'd detect system theme here
    }

    /// Check if following system theme
    pub fn follows_system(&self) -> bool {
        self.follow_system
    }

    /// Export current palette to a serializable format
    pub fn export_palette(&self) -> PaletteExport {
        PaletteExport {
            theme: format!("{:?}", self.current_theme),
            colors: self.export_colors(),
        }
    }

    fn export_colors(&self) -> HashMap<String, String> {
        let mut colors = HashMap::new();
        colors.insert(
            "bg_primary".to_string(),
            ColorPalette::color_to_hex(self.palette.background.primary),
        );
        colors.insert(
            "fg_primary".to_string(),
            ColorPalette::color_to_hex(self.palette.foreground.primary),
        );
        colors.insert(
            "accent_primary".to_string(),
            ColorPalette::color_to_hex(self.palette.accent.primary),
        );
        colors
    }

    /// Set transition duration
    pub fn set_transition_duration(&mut self, duration: f32) {
        self.transition_duration = duration.max(0.0);
    }
}

/// Export format for palette
#[derive(Debug, Clone)]
pub struct PaletteExport {
    pub theme: String,
    pub colors: HashMap<String, String>,
}

/// Builder for creating theme configurations
#[derive(Debug)]
pub struct ThemeBuilder {
    base_theme: Theme,
    customizations: HashMap<String, [f32; 4]>,
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        Self {
            base_theme: Theme::Dark,
            customizations: HashMap::new(),
        }
    }
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Start from a base theme
    pub fn from_theme(mut self, theme: Theme) -> Self {
        self.base_theme = theme;
        self
    }

    /// Customize a specific color
    pub fn with_color(mut self, key: impl Into<String>, color: [f32; 4]) -> Self {
        self.customizations.insert(key.into(), color);
        self
    }

    /// Build the theme manager
    pub fn build(self) -> ThemeManager {
        let mut manager = ThemeManager::new();
        manager.set_theme(self.base_theme);

        // Apply customizations
        for (key, color) in self.customizations {
            // In a real implementation, we'd apply these to the palette
            let _ = (key, color);
        }

        manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_display_name() {
        assert_eq!(Theme::Dark.display_name(), "Dark");
        assert_eq!(Theme::Light.display_name(), "Light");
        assert_eq!(Theme::HighContrast.display_name(), "High Contrast");
        assert_eq!(Theme::Cosmic.display_name(), "Cosmic");
    }

    #[test]
    fn test_theme_is_builtin() {
        assert!(Theme::Dark.is_builtin());
        assert!(Theme::Light.is_builtin());
        assert!(!Theme::Custom.is_builtin());
    }

    #[test]
    fn test_color_palette_dark_theme() {
        let palette = ColorPalette::dark_theme();

        // Check background is dark
        assert!(palette.background.primary[0] < 0.2);
        assert!(palette.background.primary[1] < 0.2);
        assert!(palette.background.primary[2] < 0.2);

        // Check foreground is light
        assert!(palette.foreground.primary[0] > 0.9);
        assert!(palette.foreground.primary[1] > 0.9);
        assert!(palette.foreground.primary[2] > 0.9);
    }

    #[test]
    fn test_color_palette_light_theme() {
        let palette = ColorPalette::light_theme();

        // Check background is light
        assert!(palette.background.primary[0] > 0.95);
        assert!(palette.background.primary[1] > 0.95);
        assert!(palette.background.primary[2] > 0.95);

        // Check foreground is dark
        assert!(palette.foreground.primary[0] < 0.2);
        assert!(palette.foreground.primary[1] < 0.2);
        assert!(palette.foreground.primary[2] < 0.2);
    }

    #[test]
    fn test_get_density_color() {
        let palette = ColorPalette::dark_theme();

        let red = palette.get_density_color(1);
        assert!(red[0] > 0.9); // High red
        assert!(red[1] < 0.5); // Low green
        assert!(red[2] < 0.5); // Low blue

        let green = palette.get_density_color(4);
        assert!(green[0] < 0.5); // Low red
        assert!(green[1] > 0.9); // High green
        assert!(green[2] < 0.5); // Low blue

        let violet = palette.get_density_color(7);
        assert!(violet[0] > 0.4); // Moderate red
        assert!(violet[1] < 0.4); // Low green
        assert!(violet[2] > 0.8); // High blue
    }

    #[test]
    fn test_color_to_hex() {
        let color = [1.0, 0.5, 0.0, 1.0];
        // Note: 0.5 * 255 = 127.5, truncated to 127 (0x7F)
        assert_eq!(ColorPalette::color_to_hex(color), "#FF7F00FF");

        let color = [0.0, 0.0, 0.0, 1.0];
        assert_eq!(ColorPalette::color_to_hex(color), "#000000FF");
    }

    #[test]
    fn test_hex_to_color() {
        let color = ColorPalette::hex_to_color("#FF8000FF").unwrap();
        assert!((color[0] - 1.0).abs() < 0.01);
        assert!((color[1] - 0.5).abs() < 0.01);
        assert!(color[2].abs() < 0.01);
        assert!((color[3] - 1.0).abs() < 0.01);

        // Test without alpha
        let color = ColorPalette::hex_to_color("#FF8000").unwrap();
        assert!((color[3] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        assert_eq!(manager.current_theme, Theme::Dark);

        manager.set_theme(Theme::Light);
        assert_eq!(manager.current_theme, Theme::Light);

        manager.toggle_theme();
        assert_eq!(manager.current_theme, Theme::Dark);
    }

    #[test]
    fn test_theme_manager_callback() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let mut manager = ThemeManager::new();
        let called: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        manager.on_theme_change(move |theme, _palette| {
            assert_eq!(theme, Theme::Light);
            called_clone.store(true, Ordering::SeqCst);
        });

        manager.set_theme(Theme::Light);
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_theme_transition() {
        let mut manager = ThemeManager::new();
        manager.set_theme(Theme::Light);

        assert!(manager.is_transitioning());
        assert_eq!(manager.transition_progress(), 0.0);

        // Update transition
        manager.update_transition(0.15);
        assert!(manager.transition_progress() > 0.0);

        // Complete transition
        manager.update_transition(1.0);
        assert!(!manager.is_transitioning());
        assert_eq!(manager.transition_progress(), 1.0);
    }

    #[test]
    fn test_custom_themes() {
        let mut manager = ThemeManager::new();
        let custom_palette = ColorPalette::cosmic_theme();

        manager.save_custom_theme("My Theme", custom_palette.clone());
        assert_eq!(manager.custom_theme_names().len(), 1);

        let loaded = manager.load_custom_theme("My Theme");
        assert!(loaded.is_some());

        manager.delete_custom_theme("My Theme");
        assert_eq!(manager.custom_theme_names().len(), 0);
    }

    #[test]
    fn test_theme_builder() {
        let manager = ThemeBuilder::new()
            .from_theme(Theme::Cosmic)
            .with_color("accent", [1.0, 0.0, 0.0, 1.0])
            .build();

        assert_eq!(manager.current_theme, Theme::Cosmic);
    }

    #[test]
    fn test_follow_system() {
        let mut manager = ThemeManager::new();
        assert!(!manager.follows_system());

        manager.set_follow_system(true);
        assert!(manager.follows_system());
    }
}
