//! UI Module - User Interface Components
//!
//! Phase 6: Interactive Inspection
//!
//! Provides all UI components for the cosmological simulation:
//! - Entity Inspector (enhanced with complete cosmological data)
//! - Entity Tooltips (hover information)
//! - Dashboard panels (collective, spectrum, emergence, structure, time)
//! - Theme and animation systems
//! - EGUI integration

pub mod animations;
pub mod egui_integration;
pub mod entity_tooltip;
pub mod font_config;
pub mod loading_screen;
pub mod panels;
pub mod theme;
pub mod tutorial;

pub use animations::{
    Animation, AnimationConfig, AnimationManager, EasingFunction, PanelAnimation,
};
pub use egui_integration::EguiIntegration;
pub use entity_tooltip::EntityTooltip;
pub use font_config::{FontConfig, FontFamily, FontManager, FontSize, FontWeight};
pub use loading_screen::{LoadingScreen, LoadingScreenConfig, LoadingStage};
pub use panels::{EntityInspector, EntityInspectorEnhanced};
pub use theme::{ColorPalette, Theme, ThemeManager};
pub use tutorial::{Tutorial, TutorialCategory, TutorialManager, TutorialStep};
