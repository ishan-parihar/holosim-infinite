// Catalyst Intensity Scaling System - Task 1.5
//
// This module implements catalyst intensity scaling that increases as densities decrease.
// Catalyst difficulty scales with polarization, and catalyst type distribution varies by density.
//
// Roadmap Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.5

use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;
use crate::veil::density_variation::PolarizationState;

/// Catalyst types that can be generated for entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalystType {
    /// Emotional catalyst - triggers emotional responses
    Emotional = 0,
    /// Mental catalyst - challenges beliefs and mental patterns
    Mental = 1,
    /// Physical catalyst - challenges physical body and environment
    Physical = 2,
    /// Relational catalyst - challenges relationships and social interactions
    Relational = 3,
    /// Spiritual catalyst - challenges spiritual beliefs and practices
    Spiritual = 4,
    /// Karmic catalyst - brings up past karma for processing
    Karmic = 5,
    /// Service catalyst - opportunities for service to others
    Service = 6,
    /// Control catalyst - opportunities for control over others
    Control = 7,
}

impl CatalystType {
    /// Get all catalyst types
    pub fn all() -> Vec<CatalystType> {
        vec![
            CatalystType::Emotional,
            CatalystType::Mental,
            CatalystType::Physical,
            CatalystType::Relational,
            CatalystType::Spiritual,
            CatalystType::Karmic,
            CatalystType::Service,
            CatalystType::Control,
        ]
    }

    /// Get catalyst type as index
    pub fn as_index(self) -> usize {
        self as usize
    }

    /// Get catalyst types suitable for STO polarization
    pub fn sto_types() -> Vec<CatalystType> {
        vec![
            CatalystType::Emotional,
            CatalystType::Mental,
            CatalystType::Physical,
            CatalystType::Relational,
            CatalystType::Spiritual,
            CatalystType::Karmic,
            CatalystType::Service,
        ]
    }

    /// Get catalyst types suitable for STS polarization
    pub fn sts_types() -> Vec<CatalystType> {
        vec![
            CatalystType::Emotional,
            CatalystType::Mental,
            CatalystType::Physical,
            CatalystType::Relational,
            CatalystType::Spiritual,
            CatalystType::Karmic,
            CatalystType::Control,
        ]
    }
}

/// A catalyst event for an entity
#[derive(Debug, Clone)]
pub struct Catalyst {
    /// Type of catalyst
    pub catalyst_type: CatalystType,
    /// Intensity of catalyst (0.0 to 1.0)
    pub intensity: Float,
    /// Difficulty modifier (1.0 to 1.9, from veil system)
    pub difficulty_modifier: Float,
    /// Effective difficulty (intensity * difficulty_modifier)
    pub effective_difficulty: Float,
    /// Description of catalyst event
    pub description: String,
    /// Whether catalyst has been processed
    pub processed: bool,
    /// Processing progress (0.0 to 1.0)
    pub processing_progress: Float,
}

impl Catalyst {
    /// Create a new catalyst
    pub fn new(
        catalyst_type: CatalystType,
        intensity: Float,
        difficulty_modifier: Float,
        description: String,
    ) -> Self {
        let effective_difficulty = intensity * difficulty_modifier;
        Catalyst {
            catalyst_type,
            intensity,
            difficulty_modifier,
            effective_difficulty,
            description,
            processed: false,
            processing_progress: 0.0,
        }
    }

    /// Process the catalyst by a certain amount
    pub fn process(&mut self, amount: Float) {
        self.processing_progress = (self.processing_progress + amount).min(1.0);
        if self.processing_progress >= 1.0 {
            self.processed = true;
        }
    }

    /// Check if catalyst is fully processed
    pub fn is_fully_processed(&self) -> bool {
        self.processed
    }
}

/// Catalyst intensity curve by density
#[derive(Debug, Clone)]
pub struct CatalystIntensityCurve {
    /// Base intensity for each density (D1-D7)
    base_intensity: [Float; 7],
}

impl CatalystIntensityCurve {
    /// Create new catalyst intensity curve with default values from roadmap
    pub fn new() -> Self {
        CatalystIntensityCurve {
            // D7: 0.1, D6: 0.3, D5: 0.5, D4: 0.7, D3: 0.85, D2: 0.95, D1: 1.0
            // Index 0 = D1, Index 6 = D7
            base_intensity: [1.0, 0.95, 0.85, 0.7, 0.5, 0.3, 0.1],
        }
    }

    /// Get base intensity for a density
    pub fn get_intensity(&self, density: Density) -> Float {
        self.base_intensity[density.as_usize() - 1]
    }

    /// Set base intensity for a specific density
    pub fn set_intensity(&mut self, density: Density, intensity: Float) {
        self.base_intensity[density.as_usize() - 1] = intensity.clamp(0.0, 1.0);
    }

    /// Get the complete intensity curve
    pub fn curve(&self) -> [Float; 7] {
        self.base_intensity
    }

    /// Calculate interpolated intensity during density transition
    pub fn transition_intensity(
        &self,
        from_density: Density,
        to_density: Density,
        progress: Float,
    ) -> Float {
        let from_intensity = self.get_intensity(from_density);
        let to_intensity = self.get_intensity(to_density);
        from_intensity + (to_intensity - from_intensity) * progress.clamp(0.0, 1.0)
    }
}

impl Default for CatalystIntensityCurve {
    fn default() -> Self {
        Self::new()
    }
}

/// Catalyst type distribution by density
#[derive(Debug, Clone)]
pub struct CatalystTypeDistribution {
    /// Probability distribution for each density and catalyst type
    /// distribution[density][type_index] = probability
    distribution: [[Float; 8]; 7],
}

impl CatalystTypeDistribution {
    /// Create new catalyst type distribution with default values
    pub fn new() -> Self {
        let mut distribution = CatalystTypeDistribution {
            distribution: [[0.0; 8]; 7],
        };

        // D7 (Gateway): Mostly spiritual and emotional
        distribution.set_distribution(
            Density::Seventh,
            &[
                (CatalystType::Spiritual, 0.40),
                (CatalystType::Emotional, 0.30),
                (CatalystType::Karmic, 0.15),
                (CatalystType::Service, 0.10),
                (CatalystType::Mental, 0.05),
                (CatalystType::Control, 0.0),
                (CatalystType::Physical, 0.0),
                (CatalystType::Relational, 0.0),
            ],
        );

        // D6 (Unity): Balanced spiritual and relational
        distribution.set_distribution(
            Density::Sixth,
            &[
                (CatalystType::Spiritual, 0.30),
                (CatalystType::Relational, 0.25),
                (CatalystType::Emotional, 0.20),
                (CatalystType::Service, 0.15),
                (CatalystType::Karmic, 0.10),
                (CatalystType::Mental, 0.0),
                (CatalystType::Control, 0.0),
                (CatalystType::Physical, 0.0),
            ],
        );

        // D5 (Wisdom): Mental and spiritual focus
        distribution.set_distribution(
            Density::Fifth,
            &[
                (CatalystType::Mental, 0.30),
                (CatalystType::Spiritual, 0.25),
                (CatalystType::Emotional, 0.20),
                (CatalystType::Relational, 0.15),
                (CatalystType::Karmic, 0.10),
                (CatalystType::Service, 0.0),
                (CatalystType::Control, 0.0),
                (CatalystType::Physical, 0.0),
            ],
        );

        // D4 (Love): Relational and emotional catalysts
        distribution.set_distribution(
            Density::Fourth,
            &[
                (CatalystType::Relational, 0.30),
                (CatalystType::Emotional, 0.25),
                (CatalystType::Service, 0.20),
                (CatalystType::Mental, 0.15),
                (CatalystType::Physical, 0.10),
                (CatalystType::Karmic, 0.0),
                (CatalystType::Spiritual, 0.0),
                (CatalystType::Control, 0.0),
            ],
        );

        // D3 (Self-Aware): Balanced all types
        distribution.set_distribution(
            Density::Third,
            &[
                (CatalystType::Emotional, 0.20),
                (CatalystType::Mental, 0.20),
                (CatalystType::Physical, 0.20),
                (CatalystType::Relational, 0.20),
                (CatalystType::Karmic, 0.10),
                (CatalystType::Spiritual, 0.05),
                (CatalystType::Service, 0.05),
                (CatalystType::Control, 0.0),
            ],
        );

        // D2 (Growth): More physical and relational
        distribution.set_distribution(
            Density::Second(Density2SubLevel::Cellular),
            &[
                (CatalystType::Physical, 0.25),
                (CatalystType::Relational, 0.25),
                (CatalystType::Emotional, 0.20),
                (CatalystType::Mental, 0.15),
                (CatalystType::Karmic, 0.10),
                (CatalystType::Service, 0.0),
                (CatalystType::Control, 0.05),
                (CatalystType::Spiritual, 0.0),
            ],
        );

        // D1 (Elemental): Maximum physical and emotional
        distribution.set_distribution(
            Density::First(Density1SubLevel::Quantum),
            &[
                (CatalystType::Physical, 0.30),
                (CatalystType::Emotional, 0.30),
                (CatalystType::Relational, 0.20),
                (CatalystType::Mental, 0.10),
                (CatalystType::Karmic, 0.10),
                (CatalystType::Service, 0.0),
                (CatalystType::Control, 0.0),
                (CatalystType::Spiritual, 0.0),
            ],
        );

        distribution
    }

    /// Set distribution for a specific density
    fn set_distribution(&mut self, density: Density, types: &[(CatalystType, Float)]) {
        let density_idx = density.as_usize() - 1;
        for (catalyst_type, probability) in types {
            let type_idx = catalyst_type.as_index();
            self.distribution[density_idx][type_idx] = *probability;
        }
    }

    /// Get probability for a specific catalyst type at a density
    pub fn get_probability(&self, density: Density, catalyst_type: CatalystType) -> Float {
        self.distribution[density.as_usize() - 1][catalyst_type.as_index()]
    }

    /// Get all probabilities for a density
    pub fn get_distribution(&self, density: Density) -> [Float; 8] {
        self.distribution[density.as_usize() - 1]
    }

    /// Select a random catalyst type based on distribution
    pub fn select_type(&self, density: Density, polarization: &PolarizationState) -> CatalystType {
        let distribution = self.get_distribution(density);

        // Adjust distribution based on polarization
        let adjusted_distribution = match polarization.polarization {
            crate::types::Polarity::STO | crate::types::Polarity::ServiceToOthers => {
                // Increase service catalyst probability
                let mut dist = distribution;
                dist[CatalystType::Service.as_index()] += 0.1;
                dist[CatalystType::Control.as_index()] = 0.0;
                Self::normalize_distribution(dist)
            }
            crate::types::Polarity::STS | crate::types::Polarity::ServiceToSelf => {
                // Increase control catalyst probability
                let mut dist = distribution;
                dist[CatalystType::Control.as_index()] += 0.1;
                dist[CatalystType::Service.as_index()] = 0.0;
                Self::normalize_distribution(dist)
            }
            crate::types::Polarity::SinkholeOfIndifference | crate::types::Polarity::Neutral => {
                distribution
            }
        };

        // Select based on adjusted distribution
        let random_value: Float = rand::random();
        let mut cumulative = 0.0;
        for (idx, &prob) in adjusted_distribution.iter().enumerate() {
            cumulative += prob;
            if random_value < cumulative {
                return CatalystType::all()[idx];
            }
        }

        // Fallback to first non-zero type
        CatalystType::all()
            .iter()
            .find(|&&t| adjusted_distribution[t.as_index()] > 0.0)
            .copied()
            .unwrap_or(CatalystType::Emotional)
    }

    /// Normalize distribution so probabilities sum to 1.0
    fn normalize_distribution(distribution: [Float; 8]) -> [Float; 8] {
        let total: Float = distribution.iter().sum();
        if total == 0.0 {
            return [0.125; 8]; // Equal distribution
        }
        distribution.map(|p| p / total)
    }
}

impl Default for CatalystTypeDistribution {
    fn default() -> Self {
        Self::new()
    }
}

/// Catalyst Intensity Scaling System
///
/// This system manages catalyst generation, intensity calculation, and difficulty scaling.
pub struct CatalystIntensitySystem {
    /// Catalyst intensity curve by density
    intensity_curve: CatalystIntensityCurve,

    /// Catalyst type distribution by density
    type_distribution: CatalystTypeDistribution,

    /// Generated catalysts by entity
    catalysts: std::collections::HashMap<u64, Vec<Catalyst>>,
}

impl CatalystIntensitySystem {
    /// Create a new catalyst intensity system
    pub fn new() -> Self {
        CatalystIntensitySystem {
            intensity_curve: CatalystIntensityCurve::new(),
            type_distribution: CatalystTypeDistribution::new(),
            catalysts: std::collections::HashMap::new(),
        }
    }

    /// Calculate catalyst intensity for a density
    pub fn calculate_catalyst_intensity(&self, density: Density) -> Float {
        self.intensity_curve.get_intensity(density)
    }

    /// Calculate transition intensity during density change
    pub fn calculate_transition_intensity(
        &self,
        from_density: Density,
        to_density: Density,
        progress: Float,
    ) -> Float {
        self.intensity_curve
            .transition_intensity(from_density, to_density, progress)
    }

    /// Scale catalyst difficulty based on intensity and polarization
    pub fn scale_catalyst_difficulty(
        &self,
        intensity: Float,
        polarization: &PolarizationState,
        veil_difficulty_modifier: Float,
    ) -> Float {
        // Base difficulty from intensity and veil modifier
        let base_difficulty = intensity * veil_difficulty_modifier;

        // Adjust based on polarization intensity
        // Higher polarization = catalysts easier to process (more focused)
        let polarization_modifier = 1.0 - (polarization.intensity * 0.3);

        base_difficulty * polarization_modifier
    }

    /// Generate a catalyst for an entity
    pub fn generate_catalyst(
        &mut self,
        entity_id: u64,
        density: Density,
        polarization: &PolarizationState,
        veil_difficulty_modifier: Float,
    ) -> Option<Catalyst> {
        let intensity = self.calculate_catalyst_intensity(density);
        let catalyst_type = self.type_distribution.select_type(density, polarization);
        let effective_difficulty =
            self.scale_catalyst_difficulty(intensity, polarization, veil_difficulty_modifier);

        let description = self.generate_description(catalyst_type, intensity, density);

        let catalyst = Catalyst::new(catalyst_type, intensity, effective_difficulty, description);

        // Store catalyst
        self.catalysts
            .entry(entity_id)
            .or_default()
            .push(catalyst.clone());

        Some(catalyst)
    }

    /// Generate a description for a catalyst
    fn generate_description(
        &self,
        catalyst_type: CatalystType,
        intensity: Float,
        density: Density,
    ) -> String {
        let intensity_desc = match intensity {
            x if x < 0.3 => "gentle",
            x if x < 0.6 => "moderate",
            x if x < 0.9 => "intense",
            _ => "extreme",
        };

        let type_desc = match catalyst_type {
            CatalystType::Emotional => "emotional challenge",
            CatalystType::Mental => "mental challenge",
            CatalystType::Physical => "physical challenge",
            CatalystType::Relational => "relational challenge",
            CatalystType::Spiritual => "spiritual challenge",
            CatalystType::Karmic => "karmic pattern",
            CatalystType::Service => "opportunity for service",
            CatalystType::Control => "opportunity for control",
        };

        format!(
            "{} {} at density {} (intensity: {:.2})",
            intensity_desc, type_desc, density, intensity
        )
    }

    /// Process a catalyst for an entity
    pub fn process_catalyst(
        &mut self,
        entity_id: u64,
        catalyst_index: usize,
        amount: Float,
    ) -> bool {
        if let Some(catalysts) = self.catalysts.get_mut(&entity_id) {
            if catalyst_index < catalysts.len() {
                catalysts[catalyst_index].process(amount);
                return true;
            }
        }
        false
    }

    /// Get all catalysts for an entity
    pub fn get_catalysts(&self, entity_id: u64) -> Vec<Catalyst> {
        self.catalysts.get(&entity_id).cloned().unwrap_or_default()
    }

    /// Get unprocessed catalysts for an entity
    pub fn get_unprocessed_catalysts(&self, entity_id: u64) -> Vec<Catalyst> {
        self.catalysts
            .get(&entity_id)
            .map(|c| c.iter().filter(|cat| !cat.processed).cloned().collect())
            .unwrap_or_default()
    }

    /// Get processed catalysts for an entity
    pub fn get_processed_catalysts(&self, entity_id: u64) -> Vec<Catalyst> {
        self.catalysts
            .get(&entity_id)
            .map(|c| c.iter().filter(|cat| cat.processed).cloned().collect())
            .unwrap_or_default()
    }

    /// Calculate catalyst processing statistics for an entity
    pub fn get_statistics(&self, entity_id: u64) -> CatalystStatistics {
        let catalysts = self.get_catalysts(entity_id);
        let total = catalysts.len();
        let processed = catalysts.iter().filter(|c| c.processed).count();

        let avg_intensity = if total > 0 {
            catalysts.iter().map(|c| c.intensity).sum::<Float>() / total as Float
        } else {
            0.0
        };

        let avg_difficulty = if total > 0 {
            catalysts
                .iter()
                .map(|c| c.effective_difficulty)
                .sum::<Float>()
                / total as Float
        } else {
            0.0
        };

        let total_processing_progress = catalysts
            .iter()
            .map(|c| c.processing_progress)
            .sum::<Float>();

        CatalystStatistics {
            total_catalysts: total,
            processed_catalysts: processed,
            unprocessed_catalysts: total - processed,
            average_intensity: avg_intensity,
            average_difficulty: avg_difficulty,
            total_processing_progress,
        }
    }

    /// Remove an entity from the system
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.catalysts.remove(&entity_id);
    }

    /// Clear all catalysts
    pub fn clear_all(&mut self) {
        self.catalysts.clear();
    }

    /// Get the intensity curve
    pub fn get_intensity_curve(&self) -> CatalystIntensityCurve {
        self.intensity_curve.clone()
    }

    /// Get the type distribution
    pub fn get_type_distribution(&self) -> CatalystTypeDistribution {
        self.type_distribution.clone()
    }
}

impl Default for CatalystIntensitySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Catalyst statistics for an entity
#[derive(Debug, Clone, Default)]
pub struct CatalystStatistics {
    /// Total number of catalysts
    pub total_catalysts: usize,
    /// Number of processed catalysts
    pub processed_catalysts: usize,
    /// Number of unprocessed catalysts
    pub unprocessed_catalysts: usize,
    /// Average intensity of all catalysts
    pub average_intensity: Float,
    /// Average difficulty of all catalysts
    pub average_difficulty: Float,
    /// Total processing progress across all catalysts
    pub total_processing_progress: Float,
}

impl CatalystStatistics {
    /// Calculate processing percentage
    pub fn processing_percentage(&self) -> Float {
        if self.total_catalysts == 0 {
            0.0
        } else {
            (self.processed_catalysts as Float / self.total_catalysts as Float) * 100.0
        }
    }

    /// Calculate completion percentage
    pub fn completion_percentage(&self) -> Float {
        if self.total_catalysts == 0 {
            0.0
        } else {
            (self.total_processing_progress / self.total_catalysts as Float) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalyst_type_all() {
        let types = CatalystType::all();
        assert_eq!(types.len(), 8);
    }

    #[test]
    fn test_catalyst_type_sto_types() {
        let types = CatalystType::sto_types();
        assert_eq!(types.len(), 7);
        assert!(!types.contains(&CatalystType::Control));
    }

    #[test]
    fn test_catalyst_type_sts_types() {
        let types = CatalystType::sts_types();
        assert_eq!(types.len(), 7);
        assert!(!types.contains(&CatalystType::Service));
    }

    #[test]
    fn test_catalyst_creation() {
        let catalyst = Catalyst::new(
            CatalystType::Emotional,
            0.5,
            1.0,
            "Test catalyst".to_string(),
        );
        assert_eq!(catalyst.catalyst_type, CatalystType::Emotional);
        assert_eq!(catalyst.intensity, 0.5);
        assert_eq!(catalyst.effective_difficulty, 0.5);
        assert_eq!(catalyst.description, "Test catalyst");
        assert!(!catalyst.processed);
        assert_eq!(catalyst.processing_progress, 0.0);
    }

    #[test]
    fn test_catalyst_processing() {
        let mut catalyst =
            Catalyst::new(CatalystType::Mental, 0.7, 1.2, "Test catalyst".to_string());
        assert!(!catalyst.is_fully_processed());

        catalyst.process(0.5);
        assert_eq!(catalyst.processing_progress, 0.5);
        assert!(!catalyst.is_fully_processed());

        catalyst.process(0.6);
        assert_eq!(catalyst.processing_progress, 1.0);
        assert!(catalyst.is_fully_processed());
    }

    #[test]
    fn test_intensity_curve_default() {
        let curve = CatalystIntensityCurve::new();
        assert_eq!(
            curve.get_intensity(Density::First(Density1SubLevel::Quantum)),
            1.0
        );
        assert_eq!(
            curve.get_intensity(Density::Second(Density2SubLevel::Cellular)),
            0.95
        );
        assert_eq!(curve.get_intensity(Density::Third), 0.85);
        assert_eq!(curve.get_intensity(Density::Fourth), 0.7);
        assert_eq!(curve.get_intensity(Density::Fifth), 0.5);
        assert_eq!(curve.get_intensity(Density::Sixth), 0.3);
        assert_eq!(curve.get_intensity(Density::Seventh), 0.1);
    }

    #[test]
    fn test_intensity_curve_set() {
        let mut curve = CatalystIntensityCurve::new();
        curve.set_intensity(Density::Fourth, 0.8);
        assert_eq!(curve.get_intensity(Density::Fourth), 0.8);
    }

    #[test]
    fn test_intensity_curve_clamping() {
        let mut curve = CatalystIntensityCurve::new();
        curve.set_intensity(Density::Fourth, 1.5);
        assert_eq!(curve.get_intensity(Density::Fourth), 1.0);

        curve.set_intensity(Density::Fourth, -0.5);
        assert_eq!(curve.get_intensity(Density::Fourth), 0.0);
    }

    #[test]
    fn test_intensity_transition() {
        let curve = CatalystIntensityCurve::new();
        let intensity = curve.transition_intensity(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            0.5,
        );
        // D7: 0.1, D1: 1.0, 50% progress = 0.55
        assert!((intensity - 0.55).abs() < 0.01);
    }

    #[test]
    fn test_intensity_transition_clamping() {
        let curve = CatalystIntensityCurve::new();
        let intensity_low = curve.transition_intensity(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            -0.5,
        );
        assert_eq!(intensity_low, 0.1);

        let intensity_high = curve.transition_intensity(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            1.5,
        );
        assert_eq!(intensity_high, 1.0);
    }

    #[test]
    fn test_type_distribution_creation() {
        let dist = CatalystTypeDistribution::new();
        let d7_dist = dist.get_distribution(Density::Seventh);
        assert!(d7_dist[CatalystType::Spiritual as usize] > 0.3);
        assert!(d7_dist[CatalystType::Physical as usize] == 0.0);
    }

    #[test]
    fn test_type_distribution_d1() {
        let dist = CatalystTypeDistribution::new();
        let d1_dist = dist.get_distribution(Density::First(Density1SubLevel::Quantum));
        assert!(d1_dist[CatalystType::Physical as usize] > 0.2);
        assert!(d1_dist[CatalystType::Emotional as usize] > 0.2);
        assert!(d1_dist[CatalystType::Spiritual as usize] == 0.0);
    }

    #[test]
    fn test_system_creation() {
        let system = CatalystIntensitySystem::new();
        assert_eq!(
            system.calculate_catalyst_intensity(Density::First(Density1SubLevel::Quantum)),
            1.0
        );
        assert_eq!(system.calculate_catalyst_intensity(Density::Seventh), 0.1);
    }

    #[test]
    fn test_system_generate_catalyst() {
        let mut system = CatalystIntensitySystem::new();
        let polarization = PolarizationState::sto(0.5);
        let catalyst = system.generate_catalyst(1, Density::Fourth, &polarization, 1.0);

        assert!(catalyst.is_some());
        let cat = catalyst.unwrap();
        assert_eq!(cat.intensity, 0.7);
        assert!(!cat.processed);
    }

    #[test]
    fn test_system_scale_difficulty() {
        let system = CatalystIntensitySystem::new();
        let polarization = PolarizationState::sto(0.5);
        let difficulty = system.scale_catalyst_difficulty(0.7, &polarization, 1.2);

        // Base: 0.7 * 1.2 = 0.84
        // Polarity modifier: 1.0 - (0.5 * 0.3) = 0.85
        // Result: 0.84 * 0.85 = 0.714
        assert!((difficulty - 0.714).abs() < 0.01);
    }

    #[test]
    fn test_system_process_catalyst() {
        let mut system = CatalystIntensitySystem::new();
        let polarization = PolarizationState::neutral();
        system.generate_catalyst(1, Density::Fourth, &polarization, 1.0);

        let success = system.process_catalyst(1, 0, 0.5);
        assert!(success);

        let catalysts = system.get_catalysts(1);
        assert_eq!(catalysts[0].processing_progress, 0.5);
    }

    #[test]
    fn test_system_statistics() {
        let mut system = CatalystIntensitySystem::new();
        let polarization = PolarizationState::neutral();
        system.generate_catalyst(1, Density::Third, &polarization, 1.0);
        system.generate_catalyst(1, Density::Third, &polarization, 1.0);
        system.process_catalyst(1, 0, 1.0);

        let stats = system.get_statistics(1);
        assert_eq!(stats.total_catalysts, 2);
        assert_eq!(stats.processed_catalysts, 1);
        assert_eq!(stats.unprocessed_catalysts, 1);
        assert_eq!(stats.processing_percentage(), 50.0);
    }

    #[test]
    fn test_intensity_increases_as_density_decreases() {
        let system = CatalystIntensitySystem::new();

        let d7_intensity = system.calculate_catalyst_intensity(Density::Seventh);
        let d6_intensity = system.calculate_catalyst_intensity(Density::Sixth);
        let d5_intensity = system.calculate_catalyst_intensity(Density::Fifth);
        let d4_intensity = system.calculate_catalyst_intensity(Density::Fourth);
        let d3_intensity = system.calculate_catalyst_intensity(Density::Third);
        let d2_intensity =
            system.calculate_catalyst_intensity(Density::Second(Density2SubLevel::Cellular));
        let d1_intensity =
            system.calculate_catalyst_intensity(Density::First(Density1SubLevel::Quantum));

        assert!(d7_intensity < d6_intensity);
        assert!(d6_intensity < d5_intensity);
        assert!(d5_intensity < d4_intensity);
        assert!(d4_intensity < d3_intensity);
        assert!(d3_intensity < d2_intensity);
        assert!(d2_intensity < d1_intensity);
    }

    #[test]
    fn test_remove_entity() {
        let mut system = CatalystIntensitySystem::new();
        let polarization = PolarizationState::neutral();
        system.generate_catalyst(1, Density::Fourth, &polarization, 1.0);

        assert_eq!(system.get_catalysts(1).len(), 1);
        system.remove_entity(1);
        assert_eq!(system.get_catalysts(1).len(), 0);
    }

    #[test]
    fn test_clear_all() {
        let mut system = CatalystIntensitySystem::new();
        let polarization = PolarizationState::neutral();
        system.generate_catalyst(1, Density::Fourth, &polarization, 1.0);
        system.generate_catalyst(2, Density::Third, &polarization, 1.0);

        assert!(!system.get_catalysts(1).is_empty());
        assert!(!system.get_catalysts(2).is_empty());

        system.clear_all();

        assert!(system.get_catalysts(1).is_empty());
        assert!(system.get_catalysts(2).is_empty());
    }
}
