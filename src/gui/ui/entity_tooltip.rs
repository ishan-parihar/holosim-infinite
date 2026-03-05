//! Entity Tooltip - Hover Information Display
//!
//! Phase 6: Interactive Inspection
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md Phase 6:
//! "Hover tooltips - show brief info when hovering over entities"
//!
//! This lightweight tooltip shows:
//! - Entity ID and type
//! - Top archetype activation with name
//! - Current density
//! - Polarity orientation
//! - Consciousness level

use crate::entity_layer7::layer7::SubSubLogos;
use crate::evolution_density_octave::density_octave::Density;
use egui::{Color32, Context, RichText};

/// Archetype names for tooltip display
const ARCHETYPE_NAMES_SHORT: [&str; 22] = [
    "Magician",
    "High Priestess",
    "Empress",
    "Emperor",
    "Hierophant",
    "Lovers",
    "Chariot",
    "Strength",
    "Hermit",
    "Wheel of Fortune",
    "Justice",
    "Hanged Man",
    "Death",
    "Temperance",
    "Devil",
    "Tower",
    "Star",
    "Moon",
    "Sun",
    "Judgement",
    "World",
    "Choice",
];

/// Entity tooltip component
///
/// Shows brief entity information when hovering over an entity.
/// Positioned near the mouse cursor with a small delay to avoid flicker.
#[derive(Debug, Clone)]
pub struct EntityTooltip {
    /// Show tooltip
    pub show_tooltip: bool,
    /// Delay before showing tooltip (seconds)
    pub delay_seconds: f32,
    /// Time when hover started
    hover_start_time: Option<std::time::Instant>,
    /// Maximum width of tooltip
    pub max_width: f32,
}

impl EntityTooltip {
    pub fn new() -> Self {
        EntityTooltip {
            show_tooltip: true,
            delay_seconds: 0.3, // 300ms delay
            hover_start_time: None,
            max_width: 300.0,
        }
    }

    /// Reset hover timer
    pub fn reset_hover(&mut self) {
        self.hover_start_time = None;
    }

    /// Start hover timer
    pub fn start_hover(&mut self) {
        self.hover_start_time = Some(std::time::Instant::now());
    }

    /// Check if tooltip should be shown
    pub fn should_show(&self) -> bool {
        if !self.show_tooltip {
            return false;
        }

        if let Some(start) = self.hover_start_time {
            let elapsed = start.elapsed().as_secs_f32();
            elapsed >= self.delay_seconds
        } else {
            false
        }
    }

    /// Show the tooltip at the given screen position
    pub fn show(&self, ctx: &Context, entity: &SubSubLogos, mouse_pos: egui::Pos2) {
        if !self.should_show() {
            return;
        }

        // Find top archetype
        let (top_archetype_idx, top_activation) = entity
            .archetype_activations
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap_or((0, &0.0));

        let top_archetype_name = ARCHETYPE_NAMES_SHORT[top_archetype_idx];

        // Polarity text
        let polarity_bias = entity.polarization.polarity_bias();
        let polarity_text = if polarity_bias > 0.1 {
            format!("STO (+{:.2})", polarity_bias)
        } else if polarity_bias < -0.1 {
            format!("STS ({:.2})", polarity_bias)
        } else {
            "Neutral".to_string()
        };

        let polarity_color = if polarity_bias > 0.0 {
            Color32::LIGHT_GREEN
        } else if polarity_bias < 0.0 {
            Color32::LIGHT_RED
        } else {
            Color32::GRAY
        };

        // Position tooltip near cursor
        let tooltip_pos = egui::Pos2::new(mouse_pos.x + 15.0, mouse_pos.y + 15.0);

        egui::Area::new(egui::Id::new("entity_tooltip"))
            .pivot(egui::Align2::LEFT_TOP)
            .fixed_pos(tooltip_pos)
            .order(egui::Order::Tooltip)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(32, 32, 32, 230))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .rounding(4.0)
                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Entity type
                            ui.label(
                                RichText::new(entity.entity_type.display_name())
                                    .color(Color32::LIGHT_BLUE)
                                    .size(14.0),
                            );
                            ui.add_space(5.0);

                            // Top archetype
                            ui.horizontal(|ui| {
                                ui.label("Top Archetype:");
                                ui.label(
                                    RichText::new(format!(
                                        "#{}: {}",
                                        top_archetype_idx + 1,
                                        top_archetype_name
                                    ))
                                    .color(Color32::GOLD),
                                );
                            });
                            ui.label(format!("Activation: {:.3}", top_activation));
                            ui.add_space(5.0);

                            // Density
                            ui.label(format!(
                                "Density: {}",
                                format_density(entity.current_density)
                            ));
                            ui.add_space(5.0);

                            // Polarity
                            ui.horizontal(|ui| {
                                ui.label("Polarity:");
                                ui.label(RichText::new(polarity_text).color(polarity_color));
                            });
                            ui.add_space(5.0);

                            // Consciousness
                            ui.label(format!("Consciousness: {:.4}", entity.consciousness_level));
                        });
                    });
            });
    }
}

impl Default for EntityTooltip {
    fn default() -> Self {
        Self::new()
    }
}

fn format_density(density: Density) -> String {
    match density {
        Density::First(sublevel) => format!("1st ({:?})", sublevel),
        Density::Second(sublevel) => format!("2nd ({:?})", sublevel),
        Density::Third => "3rd".to_string(),
        Density::Fourth => "4th".to_string(),
        Density::Fifth => "5th".to_string(),
        Density::Sixth => "6th".to_string(),
        Density::Seventh => "7th".to_string(),
        Density::Eighth => "8th".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_creation() {
        let tooltip = EntityTooltip::new();
        assert!(tooltip.show_tooltip);
        assert_eq!(tooltip.delay_seconds, 0.3);
        assert!(tooltip.hover_start_time.is_none());
    }

    #[test]
    fn test_tooltip_default() {
        let tooltip = EntityTooltip::default();
        assert!(tooltip.show_tooltip);
    }

    #[test]
    fn test_tooltip_hover() {
        let mut tooltip = EntityTooltip::new();

        // Initially should not show
        assert!(!tooltip.should_show());

        // Start hover
        tooltip.start_hover();
        // Still should not show immediately
        assert!(!tooltip.should_show());

        // Reset
        tooltip.reset_hover();
        assert!(!tooltip.should_show());
    }

    #[test]
    fn test_archetype_names_short() {
        assert_eq!(ARCHETYPE_NAMES_SHORT.len(), 22);
        assert_eq!(ARCHETYPE_NAMES_SHORT[0], "Magician");
        assert_eq!(ARCHETYPE_NAMES_SHORT[21], "Choice");
    }
}
