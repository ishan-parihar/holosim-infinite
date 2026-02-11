//! Advanced Entity Visualization
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Week 5:
//! "Advanced entity visualization: 3D rendering, density-based colors, polarity overlay, archetype activation glow"
//!
//! This module provides comprehensive entity visualization with:
//! - 3D geometry (spheres, cubes, organic shapes)
//! - Density-based color coding (8 densities)
//! - Polarity overlay (STO/STS)
//! - Archetype activation visualization (22 archetypes)
//! - Consciousness level visualization
//! - Focus-based glow effects

use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::gui::scene::entity_visualizer::ScaleLevel;
use crate::types::{Density, Polarity};
use nalgebra_glm::Vec3;

/// Entity visualization data for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityVisualizationData {
    /// Position in logarithmic space
    pub position: [f32; 3],
    /// Scale in logarithmic space
    pub scale: f32,
    /// Base color (rgba)
    pub color: [f32; 4],
    /// Polarity overlay color (rgba)
    pub polarity_color: [f32; 4],
    /// Archetype activation glow (rgba)
    pub archetype_glow: [f32; 4],
    /// Density level (1-8)
    pub density: f32,
    /// Polarity value (-1 to +1)
    pub polarity: f32,
    /// Consciousness level (0-1)
    pub consciousness: f32,
    /// Archetype activations (22 archetypes, normalized 0-1)
    pub archetype_activations: [f32; 22],
    /// Visualization style
    pub style: u32,
    /// Is focused entity
    pub focused: u32,
    /// Geometry type (sphere, cube, organic)
    pub geometry: u32,
}

/// Geometry type for entity rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryType {
    /// Sphere (for Logoi, planets, stars)
    Sphere,
    /// Cube (for crystalline structures)
    Cube,
    /// Organic shape (for biological entities)
    Organic,
    /// Torus (for vortex structures)
    Torus,
    /// Icosahedron (for energy structures)
    Icosahedron,
    /// Custom mesh (for complex structures)
    Custom,
}

impl GeometryType {
    pub fn as_u32(&self) -> u32 {
        match self {
            GeometryType::Sphere => 0,
            GeometryType::Cube => 1,
            GeometryType::Organic => 2,
            GeometryType::Torus => 3,
            GeometryType::Icosahedron => 4,
            GeometryType::Custom => 5,
        }
    }

    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => GeometryType::Sphere,
            1 => GeometryType::Cube,
            2 => GeometryType::Organic,
            3 => GeometryType::Torus,
            4 => GeometryType::Icosahedron,
            _ => GeometryType::Custom,
        }
    }
}

/// Visualization style for entities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl VisualizationStyle {
    pub fn as_u32(&self) -> u32 {
        match self {
            VisualizationStyle::ProbabilityCloud => 0,
            VisualizationStyle::GlowingParticle => 1,
            VisualizationStyle::MolecularStructure => 2,
            VisualizationStyle::OrganicShape => 3,
            VisualizationStyle::DetailedOrganism => 4,
            VisualizationStyle::PlanetarySphere => 5,
            VisualizationStyle::StellarSphere => 6,
            VisualizationStyle::SpiralGalaxy => 7,
        }
    }

    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => VisualizationStyle::ProbabilityCloud,
            1 => VisualizationStyle::GlowingParticle,
            2 => VisualizationStyle::MolecularStructure,
            3 => VisualizationStyle::OrganicShape,
            4 => VisualizationStyle::DetailedOrganism,
            5 => VisualizationStyle::PlanetarySphere,
            6 => VisualizationStyle::StellarSphere,
            7 => VisualizationStyle::SpiralGalaxy,
            _ => VisualizationStyle::DetailedOrganism,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
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
}

/// Advanced entity visualizer with comprehensive property visualization
pub struct EntityVisualizer {
    /// Current scale level
    current_scale: ScaleLevel,
    /// Visualization style mappings per entity type
    pub style_mappings: Vec<(EntityType, VisualizationStyle)>,
    /// Geometry type mappings per entity type
    pub geometry_mappings: Vec<(EntityType, GeometryType)>,
}

impl EntityVisualizer {
    /// Create a new entity visualizer
    pub fn new() -> Self {
        let mut visualizer = Self {
            current_scale: ScaleLevel::Organism,
            style_mappings: Vec::new(),
            geometry_mappings: Vec::new(),
        };

        // Initialize default mappings
        visualizer.initialize_default_mappings();

        visualizer
    }

    /// Initialize default visualization and geometry mappings
    fn initialize_default_mappings(&mut self) {
        // Entity type to visualization style mappings
        self.style_mappings = vec![
            (EntityType::Individual, VisualizationStyle::DetailedOrganism),
            (EntityType::SolarLogos, VisualizationStyle::StellarSphere),
            (EntityType::GalacticLogos, VisualizationStyle::SpiralGalaxy),
            (EntityType::Collective, VisualizationStyle::OrganicShape),
            (
                EntityType::Environmental,
                VisualizationStyle::PlanetarySphere,
            ),
        ];

        // Entity type to geometry type mappings
        self.geometry_mappings = vec![
            (EntityType::Individual, GeometryType::Organic),
            (EntityType::SolarLogos, GeometryType::Sphere),
            (EntityType::GalacticLogos, GeometryType::Sphere),
            (EntityType::Collective, GeometryType::Organic),
            (EntityType::Environmental, GeometryType::Sphere),
        ];
    }

    /// Update current scale level
    pub fn update_scale(&mut self, scale: ScaleLevel) {
        self.current_scale = scale;
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
        VisualizationStyle::DetailedOrganism
    }

    /// Get geometry type for entity
    pub fn get_geometry_type(&self, entity_type: EntityType) -> GeometryType {
        for (et, geometry) in &self.geometry_mappings {
            if *et == entity_type {
                return *geometry;
            }
        }
        GeometryType::Organic
    }

    /// Get entity color based on density
    pub fn get_density_color(density: Density) -> [f32; 4] {
        match density {
            Density::First => [1.0_f32, 0.27_f32, 0.27_f32, 1.0_f32], // Red
            Density::Second => [1.0_f32, 0.53_f32, 0.27_f32, 1.0_f32], // Orange
            Density::Third => [1.0_f32, 0.8_f32, 0.27_f32, 1.0_f32],  // Yellow
            Density::Fourth => [0.27_f32, 1.0_f32, 0.27_f32, 1.0_f32], // Green
            Density::Fifth => [0.27_f32, 1.0_f32, 1.0_f32, 1.0_f32],  // Cyan
            Density::Sixth => [0.27_f32, 0.27_f32, 1.0_f32, 1.0_f32], // Blue
            Density::Seventh => [0.53_f32, 0.27_f32, 1.0_f32, 1.0_f32], // Violet
            Density::Eighth => [1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32],  // White
        }
    }

    /// Get polarity overlay color
    pub fn get_polarity_color(polarity: Polarity) -> [f32; 4] {
        match polarity {
            Polarity::STO | Polarity::ServiceToOthers => [0.0_f32, 0.0_f32, 0.3_f32, 0.5_f32], // Blue tint
            Polarity::STS | Polarity::ServiceToSelf => [0.3_f32, 0.0_f32, 0.0_f32, 0.5_f32], // Red tint
            Polarity::Neutral => [0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32], // No overlay
            Polarity::SinkholeOfIndifference => [0.3_f32, 0.3_f32, 0.0_f32, 0.5_f32], // Yellow tint
        }
    }

    /// Get archetype glow color based on archetype number
    pub fn get_archetype_glow_color(archetype_num: usize) -> [f32; 4] {
        // Archetype colors based on Law of One
        match archetype_num {
            1 => [0.5_f32, 0.0_f32, 0.5_f32, 0.8_f32], // Matrix - Purple
            2 => [0.8_f32, 0.0_f32, 0.2_f32, 0.8_f32], // Potentiator - Red
            3 => [0.2_f32, 0.8_f32, 0.0_f32, 0.8_f32], // Catalyst - Green
            4 => [0.0_f32, 0.5_f32, 0.8_f32, 0.8_f32], // Experience - Blue
            5 => [0.8_f32, 0.5_f32, 0.0_f32, 0.8_f32], // Significator - Orange
            6 => [0.0_f32, 0.8_f32, 0.8_f32, 0.8_f32], // Transformation - Cyan
            7 => [0.8_f32, 0.0_f32, 0.8_f32, 0.8_f32], // Path - Magenta
            8 => [0.5_f32, 0.5_f32, 0.5_f32, 0.8_f32], // Potentiator of Mind - Gray
            9 => [0.9_f32, 0.9_f32, 0.0_f32, 0.8_f32], // Catalyst of Mind - Yellow
            10 => [0.7_f32, 0.3_f32, 0.7_f32, 0.8_f32], // Experience of Mind - Purple-pink
            11 => [0.3_f32, 0.7_f32, 0.3_f32, 0.8_f32], // Significator of Mind - Green
            12 => [0.5_f32, 0.0_f32, 0.5_f32, 0.8_f32], // Transformation of Mind - Purple
            13 => [0.8_f32, 0.3_f32, 0.0_f32, 0.8_f32], // Path of Mind - Orange
            14 => [0.0_f32, 0.5_f32, 0.5_f32, 0.8_f32], // Potentiator of Body - Teal
            15 => [0.9_f32, 0.6_f32, 0.0_f32, 0.8_f32], // Catalyst of Body - Gold
            16 => [0.0_f32, 0.6_f32, 0.4_f32, 0.8_f32], // Experience of Body - Green-blue
            17 => [0.6_f32, 0.0_f32, 0.6_f32, 0.8_f32], // Significator of Body - Purple
            18 => [0.4_f32, 0.4_f32, 0.8_f32, 0.8_f32], // Transformation of Body - Blue
            19 => [0.7_f32, 0.7_f32, 0.0_f32, 0.8_f32], // Path of Body - Yellow
            20 => [0.3_f32, 0.3_f32, 0.7_f32, 0.8_f32], // Potentiator of Spirit - Indigo
            21 => [0.6_f32, 0.4_f32, 0.6_f32, 0.8_f32], // Catalyst of Spirit - Purple
            22 => [1.0_f32, 0.84_f32, 0.0_f32, 0.9_f32], // Choice - Gold
            _ => [0.5_f32, 0.5_f32, 0.5_f32, 0.3_f32], // Default - Gray
        }
    }

    /// Get dominant archetype activation
    pub fn get_dominant_archetype(activations: &[f32; 22]) -> Option<usize> {
        let mut max_activation = 0.0_f32;
        let mut dominant = None;

        for (i, &activation) in activations.iter().enumerate() {
            if activation > max_activation {
                max_activation = activation;
                dominant = Some(i);
            }
        }

        dominant
    }

    /// Convert entity to visualization data
    pub fn entity_to_visualization_data(
        &self,
        entity_id: &EntityId,
        entity_type: EntityType,
        density: Density,
        polarity: Polarity,
        consciousness: f32,
        archetype_activations: [f32; 22],
        position: Vec3,
        scale: f32,
        focused: bool,
    ) -> EntityVisualizationData {
        let base_color = Self::get_density_color(density);
        let polarity_color = Self::get_polarity_color(polarity);

        // Get archetype glow from dominant archetype
        let dominant_archetype = Self::get_dominant_archetype(&archetype_activations);
        let archetype_glow = if let Some(arch_num) = dominant_archetype {
            let glow_color = Self::get_archetype_glow_color(arch_num);
            let intensity = archetype_activations[arch_num];
            [
                glow_color[0] * intensity,
                glow_color[1] * intensity,
                glow_color[2] * intensity,
                glow_color[3] * intensity,
            ]
        } else {
            [0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32]
        };

        let visualization_style = self.get_visualization_style(entity_type, self.current_scale);
        let geometry_type = self.get_geometry_type(entity_type);

        // Apply focus glow to base color
        let mut final_color = base_color;
        if focused {
            final_color[0] = (final_color[0] + 0.3_f32).min(1.0_f32);
            final_color[1] = (final_color[1] + 0.3_f32).min(1.0_f32);
            final_color[2] = (final_color[2] + 0.3_f32).min(1.0_f32);
        }

        // Apply consciousness-based alpha
        final_color[3] = consciousness * 0.5_f32 + 0.5_f32;

        EntityVisualizationData {
            position: [
                position.x.log10().max(-35.0_f32).min(26.0_f32),
                position.y.log10().max(-35.0_f32).min(26.0_f32),
                position.z.log10().max(-35.0_f32).min(26.0_f32),
            ],
            scale: scale.log10().max(-35.0_f32).min(26.0_f32),
            color: final_color,
            polarity_color,
            archetype_glow,
            density: density.as_u8() as f32,
            polarity: match polarity {
                Polarity::STO | Polarity::ServiceToOthers => 1.0_f32,
                Polarity::STS | Polarity::ServiceToSelf => -1.0_f32,
                Polarity::Neutral => 0.0_f32,
                Polarity::SinkholeOfIndifference => 0.0_f32,
            },
            consciousness,
            archetype_activations,
            style: visualization_style.as_u32(),
            focused: if focused { 1 } else { 0 },
            geometry: geometry_type.as_u32(),
        }
    }

    /// Set visualization style for entity type
    pub fn set_style_for_entity_type(
        &mut self,
        entity_type: EntityType,
        style: VisualizationStyle,
    ) {
        self.style_mappings.retain(|(et, _)| *et != entity_type);
        self.style_mappings.push((entity_type, style));
    }

    /// Set geometry type for entity type
    pub fn set_geometry_for_entity_type(
        &mut self,
        entity_type: EntityType,
        geometry: GeometryType,
    ) {
        self.geometry_mappings.retain(|(et, _)| *et != entity_type);
        self.geometry_mappings.push((entity_type, geometry));
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
    fn test_focused_entity_glow() {
        let visualizer = EntityVisualizer::new();

        let entity_id = EntityId::new("test-entity".to_string());
        let archetype_activations = [0.0_f32; 22];
        let position = Vec3::new(1.0, 0.0, 0.0);

        let normal_data = visualizer.entity_to_visualization_data(
            &entity_id,
            EntityType::Individual,
            Density::Second,
            Polarity::Neutral,
            0.7,
            archetype_activations,
            position,
            1.0,
            false,
        );

        let focused_data = visualizer.entity_to_visualization_data(
            &entity_id,
            EntityType::Individual,
            Density::Second,
            Polarity::Neutral,
            0.7,
            archetype_activations,
            position,
            1.0,
            true,
        );

        // Focused entity should be brighter (or equal if saturated)
        assert!(focused_data.color[0] >= normal_data.color[0]);
        assert!(focused_data.color[1] >= normal_data.color[1]);
        assert!(focused_data.color[2] >= normal_data.color[2]);
        assert_eq!(focused_data.focused, 1);
    }

    #[test]
    fn test_density_colors() {
        let red = EntityVisualizer::get_density_color(Density::First);
        assert!(red[0] > 0.9_f32 && red[1] < 0.5_f32 && red[2] < 0.5_f32);

        let white = EntityVisualizer::get_density_color(Density::Eighth);
        assert!(white[0] > 0.9_f32 && white[1] > 0.9_f32 && white[2] > 0.9_f32);
    }

    #[test]
    fn test_polarity_colors() {
        let sto = EntityVisualizer::get_polarity_color(Polarity::STO);
        assert!(sto[2] > 0.2_f32); // Blue component

        let sts = EntityVisualizer::get_polarity_color(Polarity::STS);
        assert!(sts[0] > 0.2_f32); // Red component
    }

    #[test]
    fn test_archetype_glow_colors() {
        let matrix = EntityVisualizer::get_archetype_glow_color(1);
        assert!(matrix[0] > 0.4_f32 && matrix[2] > 0.4_f32); // Purple

        let choice = EntityVisualizer::get_archetype_glow_color(22);
        assert!(choice[0] > 0.9_f32 && choice[1] > 0.7_f32); // Gold
    }

    #[test]
    fn test_dominant_archetype() {
        let mut activations = [0.0_f32; 22];
        activations[5] = 0.8_f32;
        activations[10] = 0.3_f32;
        activations[21] = 0.5_f32;

        let dominant = EntityVisualizer::get_dominant_archetype(&activations);
        assert_eq!(dominant, Some(5));
    }

    #[test]
    fn test_visualization_style_at_scale() {
        let visualizer = EntityVisualizer::new();

        let quantum_style =
            visualizer.get_visualization_style(EntityType::Individual, ScaleLevel::Quantum);
        assert_eq!(quantum_style, VisualizationStyle::ProbabilityCloud);

        let stellar_style =
            visualizer.get_visualization_style(EntityType::SolarLogos, ScaleLevel::Stellar);
        assert_eq!(stellar_style, VisualizationStyle::StellarSphere);
    }

    #[test]
    fn test_geometry_type() {
        let visualizer = EntityVisualizer::new();

        let individual_geom = visualizer.get_geometry_type(EntityType::Individual);
        assert_eq!(individual_geom, GeometryType::Organic);

        let solar_geom = visualizer.get_geometry_type(EntityType::SolarLogos);
        assert_eq!(solar_geom, GeometryType::Sphere);
    }

    #[test]
    fn test_entity_to_visualization_data() {
        let visualizer = EntityVisualizer::new();

        let entity_id = EntityId::new("test-entity".to_string());
        let archetype_activations = [0.0_f32; 22];
        let position = Vec3::new(1.0, 0.0, 0.0);

        let viz_data = visualizer.entity_to_visualization_data(
            &entity_id,
            EntityType::Individual,
            Density::Fourth,
            Polarity::STO,
            0.7,
            archetype_activations,
            position,
            1.0,
            false,
        );

        assert_eq!(viz_data.density, 4.0_f32);
        assert_eq!(viz_data.polarity, 1.0_f32);
        assert_eq!(viz_data.consciousness, 0.7);
        assert_eq!(viz_data.focused, 0);
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
    fn test_geometry_mapping() {
        let mut visualizer = EntityVisualizer::new();

        visualizer.set_geometry_for_entity_type(EntityType::Individual, GeometryType::Sphere);

        let geometry = visualizer.get_geometry_type(EntityType::Individual);
        assert_eq!(geometry, GeometryType::Sphere);
    }

    #[test]
    fn test_consciousness_alpha() {
        let visualizer = EntityVisualizer::new();

        let entity_id = EntityId::new("test-entity".to_string());
        let archetype_activations = [0.0_f32; 22];
        let position = Vec3::new(1.0, 0.0, 0.0);

        let low_consciousness = visualizer.entity_to_visualization_data(
            &entity_id,
            EntityType::Individual,
            Density::Fourth,
            Polarity::Neutral,
            0.0,
            archetype_activations,
            position,
            1.0,
            false,
        );

        let high_consciousness = visualizer.entity_to_visualization_data(
            &entity_id,
            EntityType::Individual,
            Density::Fourth,
            Polarity::Neutral,
            1.0,
            archetype_activations,
            position,
            1.0,
            false,
        );

        // Higher consciousness = higher alpha
        assert!(high_consciousness.color[3] > low_consciousness.color[3]);
    }
}
