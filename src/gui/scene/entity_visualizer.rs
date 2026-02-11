// Entity visualizer with scale-specific rendering
// From GUI_IMPLEMENTATION_ROADMAP.md: "Different renderers per scale level: Quantum, Cellular, Stellar, Galactic"

use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::gui::camera::Camera2D;
use crate::gui::scene::lod::LODLevel;
use crate::types::Density;
use nalgebra_glm::Vec3;

/// Scale level for entity visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleLevel {
    Quantum,   // -35 to -15: Probability clouds, wave functions
    Atomic,    // -15 to -8:  Electron orbitals, atomic nuclei
    Molecular, // -8  to -6:  Bond networks, molecular structures
    Cellular,  // -6  to -4:  Organic shapes, membranes
    Organism,  // -4  to -2:  Life forms, animals, humans
    Planetary, // -2  to +8:  Spheres with terrain, atmosphere
    Stellar,   // +8  to +15: Glowing spheres with corona effects
    Galactic,  // +15 to +26: Spiral arms, dark matter halos
}

impl ScaleLevel {
    /// Get scale level from logarithmic camera zoom
    pub fn from_zoom(zoom: f32) -> Self {
        let log_zoom = zoom.log10();

        match log_zoom {
            l if l < -15.0 => ScaleLevel::Quantum,
            l if l < -8.0 => ScaleLevel::Atomic,
            l if l < -6.0 => ScaleLevel::Molecular,
            l if l < -4.0 => ScaleLevel::Cellular,
            l if l < -2.0 => ScaleLevel::Organism,
            l if l < 8.0 => ScaleLevel::Planetary,
            l if l < 15.0 => ScaleLevel::Stellar,
            _ => ScaleLevel::Galactic,
        }
    }

    /// Get scale range for this level
    pub fn range(&self) -> (f32, f32) {
        match self {
            ScaleLevel::Quantum => (-35.0, -15.0),
            ScaleLevel::Atomic => (-15.0, -8.0),
            ScaleLevel::Molecular => (-8.0, -6.0),
            ScaleLevel::Cellular => (-6.0, -4.0),
            ScaleLevel::Organism => (-4.0, -2.0),
            ScaleLevel::Planetary => (-2.0, 8.0),
            ScaleLevel::Stellar => (8.0, 15.0),
            ScaleLevel::Galactic => (15.0, 26.0),
        }
    }

    /// Get scale level name
    pub fn name(&self) -> &str {
        match self {
            ScaleLevel::Quantum => "Quantum",
            ScaleLevel::Atomic => "Atomic",
            ScaleLevel::Molecular => "Molecular",
            ScaleLevel::Cellular => "Cellular",
            ScaleLevel::Organism => "Organism",
            ScaleLevel::Planetary => "Planetary",
            ScaleLevel::Stellar => "Stellar",
            ScaleLevel::Galactic => "Galactic",
        }
    }
}

/// Entity visualization style
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisualizationStyle {
    /// Probability cloud with wave function visualization
    ProbabilityCloud,
    /// Glowing particle with orbital rings
    GlowingParticle,
    /// Molecular structure with bonds
    MolecularStructure,
    /// Organic shape with membrane
    OrganicShape,
    /// Detailed organism with features
    DetailedOrganism,
    /// Sphere with terrain and atmosphere
    PlanetarySphere,
    /// Glowing sphere with corona
    StellarSphere,
    /// Spiral galaxy with arms
    SpiralGalaxy,
}

/// Entity render data for visualization
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityRenderData {
    /// Position in logarithmic space
    pub position: [f32; 3],
    /// Scale in logarithmic space
    pub scale: f32,
    /// Color (rgba)
    pub color: [f32; 4],
    /// Density level (1-8)
    pub density: f32,
    /// Visualization style
    pub style: u32,
    /// LOD level (0-5)
    pub lod: u32,
    /// Is focused entity
    pub focused: u32,
    /// Consciousness level (0-1)
    pub consciousness: f32,
}

/// Entity visualizer with scale-specific rendering
pub struct EntityVisualizer {
    /// Current scale level
    current_scale: ScaleLevel,
    /// Visualization style mappings per entity type
    style_mappings: Vec<(EntityType, VisualizationStyle)>,
}

impl EntityVisualizer {
    /// Create a new entity visualizer
    pub fn new() -> Self {
        let mut visualizer = Self {
            current_scale: ScaleLevel::Organism,
            style_mappings: Vec::new(),
        };

        // Initialize default style mappings
        visualizer.initialize_style_mappings();

        visualizer
    }

    /// Initialize default visualization style mappings
    fn initialize_style_mappings(&mut self) {
        // Entity type to visualization style mappings
        self.style_mappings = vec![
            (EntityType::Individual, VisualizationStyle::DetailedOrganism),
            (EntityType::SolarLogos, VisualizationStyle::StellarSphere),
            (EntityType::GalacticLogos, VisualizationStyle::SpiralGalaxy),
        ];
    }

    /// Update current scale level from camera
    pub fn update_scale(&mut self, camera: &Camera2D) {
        self.current_scale = ScaleLevel::from_zoom(camera.zoom);
    }

    /// Get current scale level
    pub fn current_scale(&self) -> ScaleLevel {
        self.current_scale
    }

    /// Get visualization style for entity at current scale
    pub fn get_visualization_style(
        &self,
        entity_type: EntityType,
        scale: ScaleLevel,
    ) -> VisualizationStyle {
        // Override style based on scale level
        match scale {
            ScaleLevel::Quantum => VisualizationStyle::ProbabilityCloud,
            ScaleLevel::Atomic => VisualizationStyle::GlowingParticle,
            ScaleLevel::Molecular => VisualizationStyle::MolecularStructure,
            ScaleLevel::Cellular => VisualizationStyle::OrganicShape,
            ScaleLevel::Organism => self.get_style_for_entity_type(entity_type),
            ScaleLevel::Planetary => VisualizationStyle::PlanetarySphere,
            ScaleLevel::Stellar => VisualizationStyle::StellarSphere,
            ScaleLevel::Galactic => VisualizationStyle::SpiralGalaxy,
        }
    }

    /// Get visualization style for entity type
    fn get_style_for_entity_type(&self, entity_type: EntityType) -> VisualizationStyle {
        for (et, style) in &self.style_mappings {
            if *et == entity_type {
                return *style;
            }
        }

        // Default style
        VisualizationStyle::DetailedOrganism
    }

    /// Convert entity to render data
    pub fn entity_to_render_data(
        &self,
        entity_id: &EntityId,
        entity_type: EntityType,
        density: Density,
        position: Vec3,
        scale: f32,
        lod: LODLevel,
        focused: bool,
        consciousness: f32,
    ) -> EntityRenderData {
        let color = self.get_entity_color(density, consciousness, focused);
        let style = self.get_visualization_style(entity_type, self.current_scale);

        EntityRenderData {
            position: [
                position.x.log10().max(-35.0).min(26.0),
                position.y.log10().max(-35.0).min(26.0),
                position.z.log10().max(-35.0).min(26.0),
            ],
            scale: scale.log10().max(-35.0).min(26.0),
            color,
            density: density as u32 as f32,
            style: style as u32,
            lod: lod as u32,
            focused: if focused { 1 } else { 0 },
            consciousness,
        }
    }

    /// Get entity color based on density and consciousness
    fn get_entity_color(&self, density: Density, consciousness: f32, focused: bool) -> [f32; 4] {
        // Base color by density
        let base_color = match density {
            Density::First => [1.0, 0.27, 0.27, 1.0],   // Red
            Density::Second => [1.0, 0.53, 0.27, 1.0],  // Orange
            Density::Third => [1.0, 0.8, 0.27, 1.0],    // Yellow
            Density::Fourth => [0.27, 1.0, 0.27, 1.0],  // Green
            Density::Fifth => [0.27, 1.0, 1.0, 1.0],    // Cyan
            Density::Sixth => [0.27, 0.27, 1.0, 1.0],   // Blue
            Density::Seventh => [0.53, 0.27, 1.0, 1.0], // Violet
            Density::Eighth => [1.0, 1.0, 1.0, 1.0],    // White
        };

        // Apply consciousness-based alpha
        let alpha = consciousness * 0.5 + 0.5;

        // Apply focus glow
        let mut color = base_color;
        if focused {
            // Add white glow for focused entities
            color[0] = (color[0] + 0.3_f32).min(1.0_f32);
            color[1] = (color[1] + 0.3_f32).min(1.0_f32);
            color[2] = (color[2] + 0.3_f32).min(1.0_f32);
        }

        [color[0], color[1], color[2], alpha]
    }

    /// Get visualization style as string
    pub fn style_name(style: VisualizationStyle) -> &'static str {
        match style {
            VisualizationStyle::ProbabilityCloud => "Probability Cloud",
            VisualizationStyle::GlowingParticle => "Glowing Particle",
            VisualizationStyle::MolecularStructure => "Molecular Structure",
            VisualizationStyle::OrganicShape => "Organic Shape",
            VisualizationStyle::DetailedOrganism => "Detailed Organism",
            VisualizationStyle::PlanetarySphere => "Planetary Sphere",
            VisualizationStyle::StellarSphere => "Stellar Sphere",
            VisualizationStyle::SpiralGalaxy => "Spiral Galaxy",
        }
    }

    /// Check if entity should be visible at current scale
    pub fn is_entity_visible(&self, entity_scale: f32) -> bool {
        let (min_scale, max_scale) = self.current_scale.range();
        let log_entity_scale = entity_scale.log10();

        log_entity_scale >= min_scale && log_entity_scale <= max_scale
    }

    /// Get LOD level for entity based on distance from camera
    pub fn get_lod_for_entity(
        &self,
        entity_position: Vec3,
        camera_position: Vec3,
        camera_zoom: f32,
    ) -> LODLevel {
        let distance = (entity_position - camera_position).magnitude();
        let log_distance = distance.log10().abs();

        // LOD based on distance in logarithmic space
        match log_distance {
            d if d < 1.0 => LODLevel::Level0,
            d if d < 2.0 => LODLevel::Level1,
            d if d < 4.0 => LODLevel::Level2,
            d if d < 8.0 => LODLevel::Level3,
            d if d < 16.0 => LODLevel::Level4,
            _ => LODLevel::Level5,
        }
    }

    /// Set visualization style for entity type
    pub fn set_style_for_entity_type(
        &mut self,
        entity_type: EntityType,
        style: VisualizationStyle,
    ) {
        // Remove existing mapping for this entity type
        self.style_mappings.retain(|(et, _)| *et != entity_type);

        // Add new mapping
        self.style_mappings.push((entity_type, style));
    }
}

impl Default for EntityVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_level_from_zoom() {
        assert_eq!(ScaleLevel::from_zoom(1e-20), ScaleLevel::Quantum);
        assert_eq!(ScaleLevel::from_zoom(1e-10), ScaleLevel::Atomic);
        assert_eq!(ScaleLevel::from_zoom(1e-7), ScaleLevel::Molecular);
        assert_eq!(ScaleLevel::from_zoom(1e-5), ScaleLevel::Cellular);
        assert_eq!(ScaleLevel::from_zoom(1e-3), ScaleLevel::Organism);
        assert_eq!(ScaleLevel::from_zoom(1.0), ScaleLevel::Planetary);
        assert_eq!(ScaleLevel::from_zoom(1e10), ScaleLevel::Stellar);
        assert_eq!(ScaleLevel::from_zoom(1e20), ScaleLevel::Galactic);
    }

    #[test]
    fn test_scale_level_range() {
        let quantum_range = ScaleLevel::Quantum.range();
        assert_eq!(quantum_range, (-35.0, -15.0));

        let galactic_range = ScaleLevel::Galactic.range();
        assert_eq!(galactic_range, (15.0, 26.0));
    }

    #[test]
    fn test_scale_level_name() {
        assert_eq!(ScaleLevel::Quantum.name(), "Quantum");
        assert_eq!(ScaleLevel::Stellar.name(), "Stellar");
    }

    #[test]
    fn test_visualizer_creation() {
        let visualizer = EntityVisualizer::new();
        assert_eq!(visualizer.current_scale(), ScaleLevel::Organism);
        assert!(!visualizer.style_mappings.is_empty());
    }

    #[test]
    fn test_visualization_style_at_scale() {
        let visualizer = EntityVisualizer::new();

        let quantum_style =
            visualizer.get_visualization_style(EntityType::Individual, ScaleLevel::Quantum);
        assert_eq!(quantum_style, VisualizationStyle::ProbabilityCloud);

        let stellar_style =
            visualizer.get_visualization_style(EntityType::Individual, ScaleLevel::Stellar);
        assert_eq!(stellar_style, VisualizationStyle::StellarSphere);
    }

    #[test]
    fn test_entity_visibility() {
        let mut visualizer = EntityVisualizer::new();
        visualizer.current_scale = ScaleLevel::Planetary;

        // Entity at planetary scale should be visible
        assert!(visualizer.is_entity_visible(1.0));

        // Entity at quantum scale should not be visible
        assert!(!visualizer.is_entity_visible(1e-30));
    }

    #[test]
    fn test_entity_color() {
        let visualizer = EntityVisualizer::new();

        let red_color = visualizer.get_entity_color(Density::First, 0.5, false);
        assert!(red_color[0] > 0.9); // High red component

        let white_color = visualizer.get_entity_color(Density::Eighth, 1.0, false);
        assert!(white_color[0] > 0.9 && white_color[1] > 0.9 && white_color[2] > 0.9);
    }

    #[test]
    fn test_focused_entity_glow() {
        let visualizer = EntityVisualizer::new();

        let normal_color = visualizer.get_entity_color(Density::Second, 0.5, false);
        let focused_color = visualizer.get_entity_color(Density::Second, 0.5, true);

        // Focused entity should be brighter (using Second density to avoid saturation at 1.0)
        assert!(focused_color[0] >= normal_color[0]);
        assert!(focused_color[1] >= normal_color[1]);
        assert!(focused_color[2] >= normal_color[2]);
    }

    #[test]
    fn test_lod_calculation() {
        let visualizer = EntityVisualizer::new();
        let camera_pos = Vec3::new(0.0, 0.0, 0.0);

        // Close entity should have high LOD
        let close_lod = visualizer.get_lod_for_entity(Vec3::new(1.0, 0.0, 0.0), camera_pos, 1.0);
        assert_eq!(close_lod, LODLevel::Level0);

        // Distant entity should have low LOD
        let distant_lod = visualizer.get_lod_for_entity(Vec3::new(1e20, 0.0, 0.0), camera_pos, 1.0);
        assert_eq!(distant_lod, LODLevel::Level5);
    }

    #[test]
    fn test_style_mapping() {
        let mut visualizer = EntityVisualizer::new();

        visualizer
            .set_style_for_entity_type(EntityType::Individual, VisualizationStyle::GlowingParticle);

        let style =
            visualizer.get_visualization_style(EntityType::Individual, ScaleLevel::Organism);
        assert_eq!(style, VisualizationStyle::GlowingParticle);
    }

    #[test]
    fn test_style_name() {
        assert_eq!(
            EntityVisualizer::style_name(VisualizationStyle::ProbabilityCloud),
            "Probability Cloud"
        );
        assert_eq!(
            EntityVisualizer::style_name(VisualizationStyle::SpiralGalaxy),
            "Spiral Galaxy"
        );
    }
}
