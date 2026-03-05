//! Font Configuration Panel
//!
//! Provides font selection, sizing, and customization options
//! for the UI typography.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Font selection and sizing"

/// Font family options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontFamily {
    /// System default font
    #[default]
    System,
    /// Sans-serif font
    SansSerif,
    /// Serif font
    Serif,
    /// Monospace font for code/data
    Monospace,
    /// Custom font 1
    Custom1,
    /// Custom font 2
    Custom2,
}

impl FontFamily {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            FontFamily::System => "System Default",
            FontFamily::SansSerif => "Sans Serif",
            FontFamily::Serif => "Serif",
            FontFamily::Monospace => "Monospace",
            FontFamily::Custom1 => "Custom 1",
            FontFamily::Custom2 => "Custom 2",
        }
    }

    /// Get all font families
    pub fn all() -> &'static [FontFamily] {
        &[
            FontFamily::System,
            FontFamily::SansSerif,
            FontFamily::Serif,
            FontFamily::Monospace,
            FontFamily::Custom1,
            FontFamily::Custom2,
        ]
    }
}

/// Font size preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum FontSize {
    /// Extra small
    XSmall,
    /// Small
    Small,
    /// Normal (default)
    #[default]
    Normal,
    /// Large
    Large,
    /// Extra large
    XLarge,
    /// Extra extra large
    XXLarge,
}


impl FontSize {
    /// Get size in points
    pub fn size_points(&self) -> f32 {
        match self {
            FontSize::XSmall => 10.0,
            FontSize::Small => 12.0,
            FontSize::Normal => 14.0,
            FontSize::Large => 16.0,
            FontSize::XLarge => 18.0,
            FontSize::XXLarge => 20.0,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            FontSize::XSmall => "Extra Small",
            FontSize::Small => "Small",
            FontSize::Normal => "Normal",
            FontSize::Large => "Large",
            FontSize::XLarge => "Extra Large",
            FontSize::XXLarge => "XX Large",
        }
    }

    /// Get all sizes
    pub fn all() -> &'static [FontSize] {
        &[
            FontSize::XSmall,
            FontSize::Small,
            FontSize::Normal,
            FontSize::Large,
            FontSize::XLarge,
            FontSize::XXLarge,
        ]
    }
}

/// Font weight options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontWeight {
    /// Light weight
    Light,
    /// Normal weight (default)
    #[default]
    Normal,
    /// Medium weight
    Medium,
    /// Bold
    Bold,
}

impl FontWeight {
    /// Get numeric weight value
    pub fn weight_value(&self) -> u16 {
        match self {
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::Bold => 700,
        }
    }
}

/// Complete font configuration
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// Main UI font family
    pub main_family: FontFamily,
    /// Monospace font family (for code/data)
    pub monospace_family: FontFamily,
    /// Font size
    pub size: FontSize,
    /// Font weight
    pub weight: FontWeight,
    /// Custom scale factor (multiplies size)
    pub scale_factor: f32,
    /// Line height multiplier
    pub line_height: f32,
    /// Letter spacing in pixels
    pub letter_spacing: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            main_family: FontFamily::System,
            monospace_family: FontFamily::Monospace,
            size: FontSize::Normal,
            weight: FontWeight::Normal,
            scale_factor: 1.0,
            line_height: 1.4,
            letter_spacing: 0.0,
        }
    }
}

impl FontConfig {
    /// Create a new font config with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Get effective font size in points
    pub fn effective_size(&self) -> f32 {
        self.size.size_points() * self.scale_factor
    }

    /// Set main font family
    pub fn with_main_family(mut self, family: FontFamily) -> Self {
        self.main_family = family;
        self
    }

    /// Set monospace font family
    pub fn with_monospace_family(mut self, family: FontFamily) -> Self {
        self.monospace_family = family;
        self
    }

    /// Set font size
    pub fn with_size(mut self, size: FontSize) -> Self {
        self.size = size;
        self
    }

    /// Set font weight
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Set scale factor
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale_factor = scale.clamp(0.5, 3.0);
        self
    }

    /// Set line height
    pub fn with_line_height(mut self, height: f32) -> Self {
        self.line_height = height.clamp(1.0, 3.0);
        self
    }

    /// Set letter spacing
    pub fn with_letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing.clamp(-2.0, 5.0);
        self
    }

    /// Increase font size by one step
    pub fn increase_size(&mut self) {
        self.size = match self.size {
            FontSize::XSmall => FontSize::Small,
            FontSize::Small => FontSize::Normal,
            FontSize::Normal => FontSize::Large,
            FontSize::Large => FontSize::XLarge,
            FontSize::XLarge => FontSize::XXLarge,
            FontSize::XXLarge => FontSize::XXLarge,
        };
    }

    /// Decrease font size by one step
    pub fn decrease_size(&mut self) {
        self.size = match self.size {
            FontSize::XSmall => FontSize::XSmall,
            FontSize::Small => FontSize::XSmall,
            FontSize::Normal => FontSize::Small,
            FontSize::Large => FontSize::Normal,
            FontSize::XLarge => FontSize::Large,
            FontSize::XXLarge => FontSize::XLarge,
        };
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Font configuration manager
pub struct FontManager {
    /// Current configuration
    pub config: FontConfig,
    /// Available custom fonts
    custom_fonts: Vec<String>,
    /// Change callbacks
    change_callbacks: Vec<Box<dyn Fn(&FontConfig) + Send>>,
}

impl Default for FontManager {
    fn default() -> Self {
        Self {
            config: FontConfig::new(),
            custom_fonts: Vec::new(),
            change_callbacks: Vec::new(),
        }
    }
}

impl FontManager {
    /// Create a new font manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Update configuration
    pub fn set_config(&mut self, config: FontConfig) {
        self.config = config;

        // Notify callbacks
        for callback in &self.change_callbacks {
            callback(&self.config);
        }
    }

    /// Register a change callback
    pub fn on_change<F>(&mut self, callback: F)
    where
        F: Fn(&FontConfig) + Send + 'static,
    {
        self.change_callbacks.push(Box::new(callback));
    }

    /// Get available custom font names
    pub fn get_custom_fonts(&self) -> &[String] {
        &self.custom_fonts
    }

    /// Add a custom font
    pub fn add_custom_font(&mut self, name: impl Into<String>) {
        self.custom_fonts.push(name.into());
    }

    /// Export configuration
    pub fn export(&self) -> FontConfigExport {
        FontConfigExport {
            main_family: format!("{:?}", self.config.main_family),
            monospace_family: format!("{:?}", self.config.monospace_family),
            size: format!("{:?}", self.config.size),
            weight: format!("{:?}", self.config.weight),
            scale_factor: self.config.scale_factor,
            line_height: self.config.line_height,
            letter_spacing: self.config.letter_spacing,
        }
    }
}

/// Export format for font configuration
#[derive(Debug, Clone)]
pub struct FontConfigExport {
    pub main_family: String,
    pub monospace_family: String,
    pub size: String,
    pub weight: String,
    pub scale_factor: f32,
    pub line_height: f32,
    pub letter_spacing: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_family_display() {
        assert_eq!(FontFamily::System.display_name(), "System Default");
        assert_eq!(FontFamily::Monospace.display_name(), "Monospace");
    }

    #[test]
    fn test_font_size_points() {
        assert_eq!(FontSize::XSmall.size_points(), 10.0);
        assert_eq!(FontSize::Normal.size_points(), 14.0);
        assert_eq!(FontSize::XXLarge.size_points(), 20.0);
    }

    #[test]
    fn test_font_weight_value() {
        assert_eq!(FontWeight::Light.weight_value(), 300);
        assert_eq!(FontWeight::Normal.weight_value(), 400);
        assert_eq!(FontWeight::Bold.weight_value(), 700);
    }

    #[test]
    fn test_font_config_builder() {
        let config = FontConfig::new()
            .with_main_family(FontFamily::SansSerif)
            .with_size(FontSize::Large)
            .with_scale(1.2)
            .with_line_height(1.5);

        assert_eq!(config.main_family, FontFamily::SansSerif);
        assert_eq!(config.size, FontSize::Large);
        assert_eq!(config.scale_factor, 1.2);
        assert_eq!(config.line_height, 1.5);
    }

    #[test]
    fn test_font_config_effective_size() {
        let config = FontConfig::new()
            .with_size(FontSize::Normal)
            .with_scale(1.5);

        assert_eq!(config.effective_size(), 21.0); // 14 * 1.5
    }

    #[test]
    fn test_font_config_size_steps() {
        let mut config = FontConfig::new();

        config.increase_size();
        assert_eq!(config.size, FontSize::Large);

        config.increase_size();
        assert_eq!(config.size, FontSize::XLarge);

        config.decrease_size();
        assert_eq!(config.size, FontSize::Large);
    }

    #[test]
    fn test_font_config_reset() {
        let mut config = FontConfig::new()
            .with_size(FontSize::XXLarge)
            .with_scale(2.0);

        assert_eq!(config.size, FontSize::XXLarge);

        config.reset();

        assert_eq!(config.size, FontSize::Normal);
        assert_eq!(config.scale_factor, 1.0);
    }

    #[test]
    fn test_font_manager() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let mut manager = FontManager::new();

        let new_config = FontConfig::new().with_size(FontSize::Large);

        let callback_called: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let callback_called_clone = callback_called.clone();

        manager.on_change(move |_config| {
            callback_called_clone.store(true, Ordering::SeqCst);
        });

        manager.set_config(new_config);

        assert_eq!(manager.config.size, FontSize::Large);
        assert!(callback_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_font_manager_custom_fonts() {
        let mut manager = FontManager::new();

        assert!(manager.get_custom_fonts().is_empty());

        manager.add_custom_font("Roboto".to_string());
        manager.add_custom_font("Open Sans".to_string());

        assert_eq!(manager.get_custom_fonts().len(), 2);
    }
}
