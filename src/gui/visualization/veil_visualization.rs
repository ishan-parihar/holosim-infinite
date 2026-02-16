//! Veil Visualization Module
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 4 (Weeks 15-16):
//! "Spectrum and Veil Visualization - Veil at v=1 clearly visible,
//! density transitions animated, entity spectrum access shown"

use crate::entity_layer7::layer7::EntityId;
use crate::types::Density;

pub const VEIL_RATIO: f64 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VeilData {
    pub veil_ratio: f64,
    pub thickness: f64,
    pub opacity: f64,
    pub color: [f32; 4],
    pub visible: bool,
    pub transparency: f64,
}

impl Default for VeilData {
    fn default() -> Self {
        VeilData {
            veil_ratio: VEIL_RATIO,
            thickness: 0.05,
            opacity: 0.7,
            color: [0.8, 0.8, 0.8, 0.7],
            visible: true,
            transparency: 0.3,
        }
    }
}

impl VeilData {
    pub fn new(opacity: f64, transparency: f64) -> Self {
        VeilData {
            veil_ratio: VEIL_RATIO,
            thickness: 0.05,
            opacity: opacity.clamp(0.0, 1.0),
            color: [0.8, 0.8, 0.8, opacity.clamp(0.0, 1.0) as f32],
            visible: true,
            transparency: transparency.clamp(0.0, 1.0),
        }
    }

    pub fn is_at_veil(ratio: f64) -> bool {
        (ratio - VEIL_RATIO).abs() < 0.1
    }

    pub fn distance_from_veil(ratio: f64) -> f64 {
        (ratio - VEIL_RATIO).abs()
    }

    pub fn get_side(ratio: f64) -> VeilSide {
        if ratio < VEIL_RATIO - 0.1 {
            VeilSide::TimeSpace
        } else if ratio > VEIL_RATIO + 0.1 {
            VeilSide::SpaceTime
        } else {
            VeilSide::AtVeil
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VeilSide {
    TimeSpace,
    AtVeil,
    SpaceTime,
}

impl VeilSide {
    pub fn name(&self) -> &'static str {
        match self {
            VeilSide::TimeSpace => "Time/Space (Metaphysical)",
            VeilSide::AtVeil => "At the Veil (v=1.0)",
            VeilSide::SpaceTime => "Space/Time (Physical)",
        }
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            VeilSide::TimeSpace => [0.2, 0.4, 0.8, 0.6],
            VeilSide::AtVeil => [0.8, 0.8, 0.8, 0.7],
            VeilSide::SpaceTime => [0.8, 0.4, 0.2, 0.6],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumSlider {
    pub min_ratio: f64,
    pub max_ratio: f64,
    pub current_ratio: f64,
    pub veil_ratio: f64,
    pub logarithmic: bool,
    pub interactive: bool,
}

impl Default for SpectrumSlider {
    fn default() -> Self {
        SpectrumSlider {
            min_ratio: 0.01,
            max_ratio: 100.0,
            current_ratio: VEIL_RATIO,
            veil_ratio: VEIL_RATIO,
            logarithmic: true,
            interactive: true,
        }
    }
}

impl SpectrumSlider {
    pub fn new(min_ratio: f64, max_ratio: f64) -> Self {
        SpectrumSlider {
            min_ratio,
            max_ratio,
            current_ratio: VEIL_RATIO,
            veil_ratio: VEIL_RATIO,
            logarithmic: true,
            interactive: true,
        }
    }

    pub fn set_ratio(&mut self, ratio: f64) {
        self.current_ratio = ratio.clamp(self.min_ratio, self.max_ratio);
    }

    pub fn normalized_position(&self, ratio: f64) -> f64 {
        if self.logarithmic {
            let min_log = self.min_ratio.log10();
            let max_log = self.max_ratio.log10();
            let ratio_log = ratio.log10();
            (ratio_log - min_log) / (max_log - min_log)
        } else {
            (ratio - self.min_ratio) / (self.max_ratio - self.min_ratio)
        }
    }

    pub fn veil_normalized_position(&self) -> f64 {
        self.normalized_position(self.veil_ratio)
    }

    pub fn is_space_time_side(&self, ratio: f64) -> bool {
        ratio > self.veil_ratio
    }

    pub fn is_time_space_side(&self, ratio: f64) -> bool {
        ratio < self.veil_ratio
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntitySpectrumAccess {
    pub entity_id: EntityId,
    pub min_access: f64,
    pub max_access: f64,
    pub current_ratio: f64,
    pub density: Density,
    pub access_strength: f64,
}

impl EntitySpectrumAccess {
    pub fn new(
        entity_id: EntityId,
        density: Density,
        current_ratio: f64,
        access_strength: f64,
    ) -> Self {
        let (min_access, max_access) = Self::default_access_range(density);

        EntitySpectrumAccess {
            entity_id,
            min_access,
            max_access,
            current_ratio,
            density,
            access_strength: access_strength.clamp(0.0, 1.0),
        }
    }

    fn default_access_range(density: Density) -> (f64, f64) {
        match density {
            Density::First => (0.5, 2.0),
            Density::Second => (0.3, 5.0),
            Density::Third => (0.2, 10.0),
            Density::Fourth => (0.1, 20.0),
            Density::Fifth => (0.05, 50.0),
            Density::Sixth => (0.02, 80.0),
            Density::Seventh => (0.01, 100.0),
            Density::Eighth => (0.01, 100.0),
        }
    }

    pub fn can_access(&self, ratio: f64) -> bool {
        ratio >= self.min_access && ratio <= self.max_access
    }

    pub fn access_color(&self) -> [f32; 4] {
        let alpha = self.access_strength as f32;
        match self.density {
            Density::First => [0.8, 0.8, 0.8, alpha],
            Density::Second => [0.5, 0.5, 1.0, alpha],
            Density::Third => [0.5, 0.8, 0.5, alpha],
            Density::Fourth => [0.8, 0.8, 0.5, alpha],
            Density::Fifth => [1.0, 0.7, 0.3, alpha],
            Density::Sixth => [1.0, 0.5, 0.3, alpha],
            Density::Seventh => [0.8, 0.3, 0.3, alpha],
            Density::Eighth => [0.3, 0.3, 0.8, alpha],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DensityTransition {
    pub entity_id: EntityId,
    pub from_density: Density,
    pub to_density: Density,
    pub progress: f64,
    pub speed: f64,
    pub complete: bool,
    pub start_ratio: f64,
    pub end_ratio: f64,
}

impl DensityTransition {
    pub fn new(
        entity_id: EntityId,
        from_density: Density,
        to_density: Density,
        start_ratio: f64,
        end_ratio: f64,
    ) -> Self {
        DensityTransition {
            entity_id,
            from_density,
            to_density,
            progress: 0.0,
            speed: 0.01,
            complete: false,
            start_ratio,
            end_ratio,
        }
    }

    pub fn update(&mut self) -> bool {
        if self.complete {
            return false;
        }

        self.progress += self.speed;
        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.complete = true;
        }

        true
    }

    pub fn current_ratio(&self) -> f64 {
        let t = self.progress;
        let smooth_t = t * t * (3.0 - 2.0 * t);
        self.start_ratio + (self.end_ratio - self.start_ratio) * smooth_t
    }

    pub fn current_density(&self) -> Density {
        if self.progress < 0.5 {
            self.from_density
        } else {
            self.to_density
        }
    }
}

#[derive(Debug, Clone)]
pub struct VeilVisualization {
    pub veil_data: VeilData,
    pub spectrum_slider: SpectrumSlider,
    pub entity_access: Vec<EntitySpectrumAccess>,
    pub transitions: Vec<DensityTransition>,
    pub show_slider: bool,
    pub show_entity_access: bool,
    pub animate_transitions: bool,
}

impl Default for VeilVisualization {
    fn default() -> Self {
        VeilVisualization {
            veil_data: VeilData::default(),
            spectrum_slider: SpectrumSlider::default(),
            entity_access: Vec::new(),
            transitions: Vec::new(),
            show_slider: true,
            show_entity_access: true,
            animate_transitions: true,
        }
    }
}

impl VeilVisualization {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entity_access(&mut self, access: EntitySpectrumAccess) {
        self.entity_access.push(access);
    }

    pub fn get_entity_access(&self, entity_id: EntityId) -> Option<&EntitySpectrumAccess> {
        self.entity_access.iter().find(|e| e.entity_id == entity_id)
    }

    pub fn add_transition(&mut self, transition: DensityTransition) {
        self.transitions.push(transition);
    }

    pub fn update_transitions(&mut self) {
        for transition in &mut self.transitions {
            transition.update();
        }

        self.transitions.retain(|t| !t.complete);
    }

    pub fn get_active_transitions(&self) -> Vec<&DensityTransition> {
        self.transitions.iter().filter(|t| !t.complete).collect()
    }

    pub fn set_ratio(&mut self, ratio: f64) {
        self.spectrum_slider.set_ratio(ratio);
    }

    pub fn get_current_side(&self) -> VeilSide {
        VeilData::get_side(self.spectrum_slider.current_ratio)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VeilRenderData {
    pub veil_ratio: f32,
    pub veil_position: f32,
    pub thickness: f32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub visible: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpectrumSliderRenderData {
    pub min_ratio: f32,
    pub max_ratio: f32,
    pub current_ratio: f32,
    pub current_position: f32,
    pub veil_position: f32,
    pub logarithmic: u32,
    pub interactive: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityAccessRenderData {
    pub entity_id: u64,
    pub min_access: f32,
    pub max_access: f32,
    pub min_position: f32,
    pub max_position: f32,
    pub current_ratio: f32,
    pub current_position: f32,
    pub color: [f32; 4],
    pub density: u32,
    pub strength: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veil_data_default() {
        let veil = VeilData::default();
        assert_eq!(veil.veil_ratio, VEIL_RATIO);
        assert!(veil.visible);
    }

    #[test]
    fn test_veil_data_is_at_veil() {
        assert!(VeilData::is_at_veil(1.0));
        assert!(!VeilData::is_at_veil(2.0));
    }

    #[test]
    fn test_spectrum_slider_default() {
        let slider = SpectrumSlider::default();
        assert_eq!(slider.current_ratio, VEIL_RATIO);
        assert!(slider.logarithmic);
    }

    #[test]
    fn test_entity_spectrum_access_creation() {
        let access = EntitySpectrumAccess::new(
            EntityId::new("entity_1".to_string()),
            Density::Third,
            1.5,
            0.8,
        );

        assert_eq!(access.entity_id, EntityId::new("entity_1".to_string()));
        assert_eq!(access.density, Density::Third);
    }

    #[test]
    fn test_density_transition_creation() {
        let transition = DensityTransition::new(
            EntityId::new("entity_1".to_string()),
            Density::Second,
            Density::Third,
            0.5,
            2.0,
        );

        assert_eq!(transition.entity_id, EntityId::new("entity_1".to_string()));
        assert!(!transition.complete);
    }

    #[test]
    fn test_veil_visualization_default() {
        let viz = VeilVisualization::new();
        assert_eq!(viz.veil_data.veil_ratio, VEIL_RATIO);
        assert!(viz.show_slider);
    }
}
