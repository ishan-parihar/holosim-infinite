//! 7-Layer Architecture Visualization
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 4 (Weeks 13-14):
//! "Visualize all 7 layers simultaneously, show 'transcend and include' dynamically,
//! display layer interactions, color-code by layer"

use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::types::Density;

// ============================================================================
// Layer Colors
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayerColors {
    pub violet: [f32; 4],
    pub indigo: [f32; 4],
    pub blue: [f32; 4],
    pub green: [f32; 4],
    pub yellow: [f32; 4],
    pub orange: [f32; 4],
    pub red: [f32; 4],
    pub layer_7: [f32; 4],
}

impl Default for LayerColors {
    fn default() -> Self {
        LayerColors {
            violet: [0.54, 0.17, 0.89, 0.8],
            indigo: [0.29, 0.0, 0.51, 0.8],
            blue: [0.0, 0.0, 1.0, 0.8],
            green: [0.0, 1.0, 0.0, 0.8],
            yellow: [1.0, 1.0, 0.0, 0.8],
            orange: [1.0, 0.65, 0.0, 0.8],
            red: [1.0, 0.0, 0.0, 0.8],
            layer_7: [1.0, 1.0, 1.0, 0.9],
        }
    }
}

impl LayerColors {
    pub fn get_layer_color(&self, layer_index: usize) -> [f32; 4] {
        match layer_index {
            0 => self.violet,
            1 => self.indigo,
            2 => self.blue,
            3 => self.green,
            4 => self.yellow,
            5 => self.orange,
            6 => self.red,
            7 => self.layer_7,
            _ => self.layer_7,
        }
    }

    pub fn get_layer_name(&self, layer_index: usize) -> &'static str {
        match layer_index {
            0 => "Violet (Unity/Source)",
            1 => "Indigo (Gateway/Awareness)",
            2 => "Blue (Creative Principle)",
            3 => "Green (Field of Potential)",
            4 => "Yellow (Dimensions/Veil)",
            5 => "Orange (Galactic Logoi)",
            6 => "Red (Solar Logoi)",
            7 => "Layer 7 (Individual Entities)",
            _ => "Unknown Layer",
        }
    }
}

// ============================================================================
// Layer View
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct LayerView {
    pub layer_index: usize,
    pub name: String,
    pub color: [f32; 4],
    pub opacity: f32,
    pub scale: f64,
    pub active: bool,
    pub resolution: f32,
    pub strength: f32,
    pub nesting_depth: usize,
}

impl LayerView {
    pub fn new(
        layer_index: usize,
        name: String,
        color: [f32; 4],
        opacity: f32,
        scale: f64,
        resolution: f32,
        strength: f32,
    ) -> Self {
        let nesting_depth = layer_index + 1;
        LayerView {
            layer_index,
            name,
            color,
            opacity: opacity.clamp(0.0_f32, 1.0_f32),
            scale: scale.max(0.1),
            resolution: resolution.clamp(0.0_f32, 1.0_f32),
            strength: strength.clamp(0.0_f32, 1.0_f32),
            active: true,
            nesting_depth,
        }
    }

    pub fn with_defaults(layer_index: usize, colors: &LayerColors) -> Self {
        let color = colors.get_layer_color(layer_index);
        let name = colors.get_layer_name(layer_index).to_string();
        let scale = 1.0_f64 + (layer_index as f64) * 0.2_f64;
        let opacity = 0.8_f32 - (layer_index as f32) * 0.05_f32;
        let resolution = 1.0_f32 - (layer_index as f32) * 0.1_f32;
        let strength = 1.0_f32 - (layer_index as f32) * 0.1_f32;

        LayerView::new(
            layer_index,
            name,
            color,
            opacity,
            scale,
            resolution,
            strength,
        )
    }

    pub fn visualization_size(&self) -> f64 {
        self.scale * (self.resolution as f64)
    }

    pub fn visualization_intensity(&self) -> f32 {
        self.opacity * self.strength
    }

    pub fn is_nested_in(&self, other_layer: &LayerView) -> bool {
        self.layer_index > other_layer.layer_index
    }

    pub fn included_layer_indices(&self, all_layers: &[LayerView]) -> Vec<usize> {
        all_layers
            .iter()
            .filter(|l| l.layer_index < self.layer_index)
            .map(|l| l.layer_index)
            .collect()
    }
}

// ============================================================================
// Layer Interaction
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct LayerInteraction {
    pub from_layer: usize,
    pub to_layer: usize,
    pub strength: f64,
    pub interaction_type: LayerInteractionType,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerInteractionType {
    TranscendInclude,
    AttractorInfluence,
    InformationFlow,
    EnergyTransfer,
    ResonanceCoupling,
    ArchetypeActivation,
}

impl LayerInteraction {
    pub fn new(
        from_layer: usize,
        to_layer: usize,
        strength: f64,
        interaction_type: LayerInteractionType,
    ) -> Self {
        LayerInteraction {
            from_layer,
            to_layer,
            strength: strength.clamp(0.0, 1.0),
            interaction_type,
            active: true,
        }
    }

    pub fn is_hierarchical(&self) -> bool {
        self.from_layer < self.to_layer
    }

    pub fn layer_distance(&self) -> u32 {
        ((self.from_layer as i32 - self.to_layer as i32).unsigned_abs()) as u32
    }
}

// ============================================================================
// Layer Visualization Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LayerVisualizationData {
    pub layer_index: u32,
    pub position: [f32; 3],
    pub scale: f32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub strength: f32,
    pub resolution: f32,
    pub nesting_depth: u32,
    pub active: u32,
    pub geometry: u32,
}

// ============================================================================
// Layer Visualization
// ============================================================================

#[derive(Debug, Clone)]
pub struct LayerVisualization {
    pub violet_layer: LayerView,
    pub indigo_layer: LayerView,
    pub blue_layer: LayerView,
    pub green_layer: LayerView,
    pub yellow_layer: LayerView,
    pub orange_layer: LayerView,
    pub red_layer: LayerView,
    pub layer_7: LayerView,
    pub colors: LayerColors,
    pub interactions: Vec<LayerInteraction>,
    pub active_layer: Option<usize>,
    pub show_all_layers: bool,
}

impl Default for LayerVisualization {
    fn default() -> Self {
        let colors = LayerColors::default();
        LayerVisualization {
            violet_layer: LayerView::with_defaults(0, &colors),
            indigo_layer: LayerView::with_defaults(1, &colors),
            blue_layer: LayerView::with_defaults(2, &colors),
            green_layer: LayerView::with_defaults(3, &colors),
            yellow_layer: LayerView::with_defaults(4, &colors),
            orange_layer: LayerView::with_defaults(5, &colors),
            red_layer: LayerView::with_defaults(6, &colors),
            layer_7: LayerView::with_defaults(7, &colors),
            colors,
            interactions: Vec::new(),
            active_layer: None,
            show_all_layers: true,
        }
    }
}

impl LayerVisualization {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_colors(colors: LayerColors) -> Self {
        let mut viz = Self::default();
        viz.colors = colors;
        viz.update_layer_colors();
        viz
    }

    pub fn update_layer_colors(&mut self) {
        self.violet_layer.color = self.colors.violet;
        self.indigo_layer.color = self.colors.indigo;
        self.blue_layer.color = self.colors.blue;
        self.green_layer.color = self.colors.green;
        self.yellow_layer.color = self.colors.yellow;
        self.orange_layer.color = self.colors.orange;
        self.red_layer.color = self.colors.red;
        self.layer_7.color = self.colors.layer_7;
    }

    pub fn get_layer(&self, layer_index: usize) -> Option<&LayerView> {
        match layer_index {
            0 => Some(&self.violet_layer),
            1 => Some(&self.indigo_layer),
            2 => Some(&self.blue_layer),
            3 => Some(&self.green_layer),
            4 => Some(&self.yellow_layer),
            5 => Some(&self.orange_layer),
            6 => Some(&self.red_layer),
            7 => Some(&self.layer_7),
            _ => None,
        }
    }

    pub fn all_layers(&self) -> Vec<&LayerView> {
        vec![
            &self.violet_layer,
            &self.indigo_layer,
            &self.blue_layer,
            &self.green_layer,
            &self.yellow_layer,
            &self.orange_layer,
            &self.red_layer,
            &self.layer_7,
        ]
    }

    pub fn active_layers(&self) -> Vec<&LayerView> {
        self.all_layers().into_iter().filter(|l| l.active).collect()
    }

    pub fn visible_layers(&self) -> Vec<&LayerView> {
        if self.show_all_layers {
            self.active_layers()
        } else if let Some(active_idx) = self.active_layer {
            self.all_layers()
                .into_iter()
                .filter(|l| l.layer_index <= active_idx)
                .collect()
        } else {
            self.active_layers()
        }
    }

    pub fn add_interaction(&mut self, interaction: LayerInteraction) {
        self.interactions.push(interaction);
    }

    pub fn get_layer_interactions(&self, layer_index: usize) -> Vec<&LayerInteraction> {
        self.interactions
            .iter()
            .filter(|i| i.from_layer == layer_index || i.to_layer == layer_index)
            .collect()
    }

    pub fn set_active_layer(&mut self, layer_index: Option<usize>) {
        self.active_layer = layer_index;
    }

    pub fn toggle_show_all_layers(&mut self) {
        self.show_all_layers = !self.show_all_layers;
    }

    pub fn generate_default_interactions(&mut self) {
        self.interactions.clear();

        for higher in 1..=7 {
            for lower in 0..higher {
                let interaction = LayerInteraction::new(
                    lower,
                    higher,
                    0.8 - ((higher - lower) as f64) * 0.1,
                    LayerInteractionType::TranscendInclude,
                );
                self.interactions.push(interaction);
            }
        }

        self.interactions.push(LayerInteraction::new(
            1,
            2,
            0.7,
            LayerInteractionType::AttractorInfluence,
        ));
        self.interactions.push(LayerInteraction::new(
            2,
            3,
            0.7,
            LayerInteractionType::AttractorInfluence,
        ));
        self.interactions.push(LayerInteraction::new(
            3,
            4,
            0.7,
            LayerInteractionType::AttractorInfluence,
        ));
        self.interactions.push(LayerInteraction::new(
            4,
            5,
            0.7,
            LayerInteractionType::AttractorInfluence,
        ));
        self.interactions.push(LayerInteraction::new(
            5,
            6,
            0.7,
            LayerInteractionType::AttractorInfluence,
        ));
        self.interactions.push(LayerInteraction::new(
            6,
            7,
            0.7,
            LayerInteractionType::AttractorInfluence,
        ));
    }

    pub fn get_render_data(&self) -> Vec<LayerVisualizationData> {
        let mut render_data = Vec::new();

        for layer in self.visible_layers() {
            let data = LayerVisualizationData {
                layer_index: layer.layer_index as u32,
                position: [0.0f32; 3],
                scale: layer.scale as f32,
                color: layer.color,
                opacity: layer.opacity,
                strength: layer.strength,
                resolution: layer.resolution,
                nesting_depth: layer.nesting_depth as u32,
                active: if layer.active { 1 } else { 0 },
                geometry: if layer.layer_index == 7 { 1 } else { 0 },
            };
            render_data.push(data);
        }

        render_data
    }
}

// ============================================================================
// Entity Layer Mapping
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct EntityLayerMapping {
    pub entity_id: EntityId,
    pub entity_type: EntityType,
    pub density: Density,
    pub layer_activations: [f32; 8],
    pub layer_resolutions: [f32; 8],
    pub layer_spectrum_access: [f32; 8],
}

impl EntityLayerMapping {
    pub fn new(entity_id: EntityId, entity_type: EntityType, density: Density) -> Self {
        let layer_activations = Self::default_layer_activations(density);
        let layer_resolutions = Self::default_layer_resolutions();
        let layer_spectrum_access = Self::default_layer_spectrum_access(density);

        EntityLayerMapping {
            entity_id,
            entity_type,
            density,
            layer_activations,
            layer_resolutions,
            layer_spectrum_access,
        }
    }

    fn default_layer_activations(density: Density) -> [f32; 8] {
        let density_idx = density as usize - 1;
        let mut activations = [0.0_f32; 8];

        for i in 0..8 {
            activations[i] = if i <= density_idx {
                0.5_f32 + ((i as f32) / 7.0) * 0.5_f32
            } else {
                0.2_f32
            };
        }

        activations
    }

    fn default_layer_resolutions() -> [f32; 8] {
        let mut resolutions = [0.0_f32; 8];

        for i in 0..8 {
            resolutions[i] = 0.3_f32 + ((i as f32) / 7.0) * 0.7_f32;
        }

        resolutions
    }

    fn default_layer_spectrum_access(density: Density) -> [f32; 8] {
        let density_idx = density as usize - 1;
        let mut access = [0.0_f32; 8];

        for i in 0..8 {
            access[i] = if i <= density_idx + 2 {
                0.4_f32 + ((i as f32) / 7.0) * 0.6_f32
            } else {
                0.1_f32
            };
        }

        access
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_colors_default() {
        let colors = LayerColors::default();
        assert_eq!(colors.violet[3], 0.8);
    }

    #[test]
    fn test_layer_visualization_default() {
        let viz = LayerVisualization::new();
        assert_eq!(viz.violet_layer.layer_index, 0);
        assert!(viz.show_all_layers);
    }

    #[test]
    fn test_entity_layer_mapping_creation() {
        let mapping = EntityLayerMapping::new(
            EntityId::new("entity_1".to_string()),
            EntityType::Individual,
            Density::Third,
        );
        assert_eq!(mapping.entity_id, EntityId::new("entity_1".to_string()));
        assert_eq!(mapping.density, Density::Third);
    }
}
