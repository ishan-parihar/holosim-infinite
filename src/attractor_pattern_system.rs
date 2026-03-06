// Attractor Pattern System - Phase 0.5.B
//
// This module implements the foundational attractor pattern system that emerges
// from archetype interactions and enables organic evolution.
//
// CRITICAL PRINCIPLE: Attractor patterns must EMERGE from archetype dynamics,
// not be pre-programmed. This is essential for organic simulation.

use crate::archetypes::ArchetypeTrait;
use std::collections::HashMap;

// Type aliases for clarity
pub type Float = f64;
pub type ArchetypeID = u8;
pub type AttractorID = u32;

/// PatternSignature - Unique identifier for a pattern
///
/// The signature captures the essential characteristics of a pattern
/// that distinguish it from other patterns.
///
/// Note: PatternSignature uses a custom hash implementation because
/// f64 (Float) doesn't implement Hash. Use the calculate_id() method
/// to get a hashable ID.
#[derive(Debug, Clone, PartialEq)]
pub struct PatternSignature {
    /// Number of particles/components in the pattern
    pub particle_count: usize,

    /// Energy distribution across the pattern
    pub energy_distribution: EnergyDistribution,

    /// Spatial arrangement of components
    pub spatial_arrangement: SpatialArrangement,

    /// Resonance frequency of the pattern
    pub resonance_frequency: Float,

    /// Resonance with each archetype (archetype ID -> resonance value)
    pub archetype_resonance_map: HashMap<ArchetypeID, Float>,
}

impl PatternSignature {
    /// Create a new pattern signature
    pub fn new(
        particle_count: usize,
        energy_distribution: EnergyDistribution,
        spatial_arrangement: SpatialArrangement,
        resonance_frequency: Float,
        archetype_resonance_map: HashMap<ArchetypeID, Float>,
    ) -> Self {
        Self {
            particle_count,
            energy_distribution,
            spatial_arrangement,
            resonance_frequency,
            archetype_resonance_map,
        }
    }

    /// Calculate a simple hash-based ID for the signature
    pub fn calculate_id(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.particle_count.hash(&mut hasher);
        (self.resonance_frequency.to_bits()).hash(&mut hasher);
        hasher.finish()
    }
}

/// EnergyDistribution - How energy is distributed across the pattern
///
/// Note: EnergyDistribution uses PartialEq but not Hash because
/// f64 (Float) doesn't implement Hash.
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyDistribution {
    /// Mean energy level
    pub mean: Float,

    /// Standard deviation of energy
    pub std_dev: Float,

    /// Minimum energy level
    pub min: Float,

    /// Maximum energy level
    pub max: Float,
}

impl EnergyDistribution {
    /// Create a new energy distribution
    pub fn new(mean: Float, std_dev: Float, min: Float, max: Float) -> Self {
        Self {
            mean,
            std_dev,
            min,
            max,
        }
    }

    /// Create a uniform distribution (all components have same energy)
    pub fn uniform(energy: Float) -> Self {
        Self {
            mean: energy,
            std_dev: 0.0,
            min: energy,
            max: energy,
        }
    }
}

/// SpatialArrangement - How components are arranged in space
///
/// Note: SpatialArrangement uses PartialEq but not Hash because
/// f64 (Float) doesn't implement Hash.
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialArrangement {
    /// Center of mass
    pub center: (Float, Float, Float),

    /// Average distance from center
    pub avg_radius: Float,

    /// Maximum distance from center
    pub max_radius: Float,

    /// Symmetry factor (0.0 = no symmetry, 1.0 = perfect symmetry)
    pub symmetry: Float,
}

impl SpatialArrangement {
    /// Create a new spatial arrangement
    pub fn new(
        center: (Float, Float, Float),
        avg_radius: Float,
        max_radius: Float,
        symmetry: Float,
    ) -> Self {
        Self {
            center,
            avg_radius,
            max_radius,
            symmetry: symmetry.clamp(0.0, 1.0),
        }
    }

    /// Create a point arrangement (single component at origin)
    pub fn point() -> Self {
        Self {
            center: (0.0, 0.0, 0.0),
            avg_radius: 0.0,
            max_radius: 0.0,
            symmetry: 1.0,
        }
    }
}

/// PatternClassification - Evidence-based classification of patterns
///
/// Classification is based on OBSERVED frequency and stability, NOT predetermined types.
///
/// Note: PatternClassification uses PartialEq but not Eq because
/// it contains PatternSignature which has f64 fields.
#[derive(Debug, Clone, PartialEq)]
pub enum PatternClassification {
    /// New pattern - not enough data to classify
    Unclassified,

    /// Rarely seen, unstable patterns
    Transient(PatternSignature),

    /// Seen occasionally, moderately stable patterns
    Emerging(PatternSignature),

    /// Frequently seen, highly stable patterns
    Established(PatternSignature),
}

impl PatternClassification {
    /// Get the pattern signature if classified
    pub fn signature(&self) -> Option<&PatternSignature> {
        match self {
            PatternClassification::Unclassified => None,
            PatternClassification::Transient(sig) => Some(sig),
            PatternClassification::Emerging(sig) => Some(sig),
            PatternClassification::Established(sig) => Some(sig),
        }
    }

    /// Check if pattern is stable (Emerging or Established)
    pub fn is_stable(&self) -> bool {
        matches!(
            self,
            PatternClassification::Emerging(_) | PatternClassification::Established(_)
        )
    }

    /// Check if pattern is well-established
    pub fn is_established(&self) -> bool {
        matches!(self, PatternClassification::Established(_))
    }
}

/// PatternState - State of a pattern in self-organization dynamics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternState {
    /// Balanced pattern (0.9 < balance < 1.1)
    Stable,

    /// Attraction dominates (balance > 1.5) - collapsing
    Collapsing,

    /// Diffusion dominates (balance < 0.5) - diffusing
    Diffusing,

    /// Unbalanced but not extreme - metastable
    Metastable,
}

impl PatternState {
    /// Determine state from balance value
    pub fn from_balance(balance: Float) -> Self {
        if (0.9..=1.1).contains(&balance) {
            PatternState::Stable
        } else if balance > 1.5 {
            PatternState::Collapsing
        } else if balance < 0.5 {
            PatternState::Diffusing
        } else {
            PatternState::Metastable
        }
    }
}

/// AttractorPattern - Emergent attractor from archetype interactions
///
/// CRITICAL: Attractors EMERGE from archetype interactions, they are NOT predefined.
/// Different archetype combinations produce different attractors.
#[derive(Debug, Clone)]
pub struct AttractorPattern {
    /// Unique identifier for this attractor
    pub attractor_id: AttractorID,

    /// Resonance frequency of the attractor
    pub resonance_frequency: Float,

    /// Basin of attraction - how wide the basin is (0.0 - 1.0)
    pub basin_of_attraction: Float,

    /// Attractor strength - how strong the attraction is
    pub attractor_strength: Float,

    /// Resonance with each archetype (archetype ID -> resonance value)
    pub archetype_resonance_map: HashMap<ArchetypeID, Float>,

    /// Energy signature of the attractor
    pub energy_signature: Float,

    /// Phase signature of the attractor
    pub phase_signature: Float,

    /// Which archetypes contributed to this attractor
    pub contributing_archetypes: Vec<ArchetypeID>,
}

impl AttractorPattern {
    /// Create a new attractor pattern
    ///
    /// This is the basic constructor. For organic emergence, use emerge_from_archetypes().
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        attractor_id: AttractorID,
        resonance_frequency: Float,
        basin_of_attraction: Float,
        attractor_strength: Float,
        archetype_resonance_map: HashMap<ArchetypeID, Float>,
        energy_signature: Float,
        phase_signature: Float,
        contributing_archetypes: Vec<ArchetypeID>,
    ) -> Self {
        Self {
            attractor_id,
            resonance_frequency,
            basin_of_attraction: basin_of_attraction.clamp(0.0, 1.0),
            attractor_strength: attractor_strength.max(0.0),
            archetype_resonance_map,
            energy_signature,
            phase_signature,
            contributing_archetypes,
        }
    }

    /// EMERGE an attractor pattern from archetype interactions
    ///
    /// CRITICAL: This is the organic emergence method. Attractors are NOT pre-programmed.
    /// They emerge from the resonance frequencies of interacting archetypes.
    ///
    /// # Arguments
    /// * `archetypes` - Slice of archetypes that are interacting
    /// * `attractor_id` - Unique identifier for the new attractor
    ///
    /// # Returns
    /// * The emerged attractor pattern
    ///
    /// # Emergence Process
    /// 1. Extract resonance frequencies from each archetype
    /// 2. Combine frequencies (weighted average based on archetype influence)
    /// 3. Calculate composite resonance
    /// 4. Determine basin width based on archetype diversity
    /// 5. Calculate strength based on archetype coherence
    /// 6. Create attractor pattern
    pub fn emerge_from_archetypes(
        archetypes: &[&dyn ArchetypeTrait],
        attractor_id: AttractorID,
    ) -> Self {
        if archetypes.is_empty() {
            // Default attractor if no archetypes provided
            return Self::new(
                attractor_id,
                0.5,
                0.3,
                0.5,
                HashMap::new(),
                0.5,
                0.0,
                vec![],
            );
        }

        // Step 1: Extract resonance frequencies from archetypes
        let mut resonance_frequencies: Vec<Float> = Vec::new();
        let mut archetype_resonance_map: HashMap<ArchetypeID, Float> = HashMap::new();
        let mut contributing_archetypes: Vec<ArchetypeID> = Vec::new();

        for archetype in archetypes {
            let lambda_value = archetype.lambda().value;
            resonance_frequencies.push(lambda_value);
            archetype_resonance_map.insert(archetype.archetype_id(), lambda_value);
            contributing_archetypes.push(archetype.archetype_id());
        }

        // Step 2: Combine frequencies (weighted average)
        let combined_resonance = if !resonance_frequencies.is_empty() {
            let sum: Float = resonance_frequencies.iter().sum();
            sum / resonance_frequencies.len() as Float
        } else {
            0.5
        };

        // Step 3: Calculate composite resonance
        let resonance_frequency = combined_resonance;

        // Step 4: Determine basin width based on archetype diversity
        // More diverse archetypes = wider basin (less specific)
        let diversity = if contributing_archetypes.len() > 1 {
            // Calculate variance in resonance frequencies
            let mean = resonance_frequency;
            let variance: Float = resonance_frequencies
                .iter()
                .map(|&f| (f - mean).powi(2))
                .sum::<Float>()
                / resonance_frequencies.len() as Float;
            variance.sqrt()
        } else {
            0.0
        };

        // Basin width: more diversity = wider basin
        let basin_of_attraction = 0.3 + (diversity * 0.4).min(0.5);

        // Step 5: Calculate strength based on archetype coherence
        // Strength = average lambda value (higher = stronger)
        let attractor_strength = resonance_frequency;

        // Step 6: Calculate energy and phase signatures
        let energy_signature = resonance_frequency;
        let phase_signature =
            (resonance_frequency * std::f64::consts::PI * 2.0) % (2.0 * std::f64::consts::PI);

        Self::new(
            attractor_id,
            resonance_frequency,
            basin_of_attraction,
            attractor_strength,
            archetype_resonance_map,
            energy_signature,
            phase_signature,
            contributing_archetypes,
        )
    }

    /// Check if a pattern resonance frequency is within this attractor's basin
    pub fn is_in_basin(&self, pattern_frequency: Float) -> bool {
        let diff = (self.resonance_frequency - pattern_frequency).abs();
        diff <= self.basin_of_attraction
    }

    /// Calculate resonance match with a pattern
    pub fn calculate_resonance_match(&self, pattern_frequency: Float) -> Float {
        let frequency_difference = (self.resonance_frequency - pattern_frequency).abs();
        // Resonance = 1.0 / (1.0 + frequency_difference)
        
        1.0 / (1.0 + frequency_difference)
    }
}

/// DiscoveredPattern - A pattern that has been discovered by the system
#[derive(Debug, Clone)]
pub struct DiscoveredPattern {
    /// Pattern signature
    pub signature: PatternSignature,

    /// Pattern configuration (stored as generic data)
    pub configuration: PatternConfiguration,

    /// Stability of the pattern (0.0 - 1.0)
    pub stability: Float,

    /// Number of times this pattern has been discovered
    pub discovery_count: usize,

    /// When this pattern was last seen (simulated time)
    pub last_seen: Float,
}

impl DiscoveredPattern {
    /// Create a new discovered pattern
    pub fn new(
        signature: PatternSignature,
        configuration: PatternConfiguration,
        stability: Float,
    ) -> Self {
        Self {
            signature,
            configuration,
            stability: stability.clamp(0.0, 1.0),
            discovery_count: 1,
            last_seen: 0.0,
        }
    }
}

/// PatternConfiguration - Generic pattern configuration
///
/// This represents the configuration of a pattern without specifying
/// what type of pattern it is (quantum, particle, atomic, molecular).
#[derive(Debug, Clone)]
pub struct PatternConfiguration {
    /// Configuration data (stored as key-value pairs)
    pub data: HashMap<String, Float>,

    /// Energy level of the configuration
    pub energy: Float,

    /// Stability of the configuration (if known)
    pub stability: Float,
}

impl PatternConfiguration {
    /// Create a new pattern configuration
    pub fn new(data: HashMap<String, Float>, energy: Float, stability: Float) -> Self {
        Self {
            data,
            energy,
            stability: stability.clamp(0.0, 1.0),
        }
    }

    /// Create an empty configuration
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
            energy: 0.0,
            stability: 0.0,
        }
    }

    /// Get a value from the configuration
    pub fn get(&self, key: &str) -> Option<Float> {
        self.data.get(key).copied()
    }

    /// Set a value in the configuration
    pub fn set(&mut self, key: String, value: Float) {
        self.data.insert(key, value);
    }
}

/// DiscoverySystem - Observes and discovers patterns
///
/// The discovery system EXPLORES all configurations, DISCOVERS stable patterns,
/// and LEARNS from experience.
///
/// Note: Uses u64 signature IDs as HashMap keys because PatternSignature
/// contains f64 fields and cannot implement Hash.
#[derive(Debug, Clone)]
pub struct DiscoverySystem {
    /// Discovered patterns (signature ID -> pattern data)
    pub discovered_patterns: HashMap<u64, DiscoveredPattern>,

    /// Pattern frequency (signature ID -> how often seen)
    pub pattern_frequency: HashMap<u64, usize>,

    /// Pattern stability (signature ID -> stability value)
    pub pattern_stability: HashMap<u64, Float>,

    /// Current simulation time
    pub current_time: Float,

    /// Minimum stability threshold for registration
    pub stability_threshold: Float,
}

impl DiscoverySystem {
    /// Create a new discovery system
    pub fn new() -> Self {
        Self {
            discovered_patterns: HashMap::new(),
            pattern_frequency: HashMap::new(),
            pattern_stability: HashMap::new(),
            current_time: 0.0,
            stability_threshold: 0.7,
        }
    }

    /// Set the stability threshold
    pub fn set_stability_threshold(&mut self, threshold: Float) {
        self.stability_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Advance simulation time
    pub fn advance_time(&mut self, delta: Float) {
        self.current_time += delta;
    }

    /// OBSERVE a pattern
    ///
    /// If the pattern has been seen before, update learning.
    /// If it's a new pattern, register it if stable enough.
    ///
    /// # Arguments
    /// * `signature` - Pattern signature
    /// * `configuration` - Pattern configuration
    /// * `stability` - Pattern stability
    pub fn observe_pattern(
        &mut self,
        signature: PatternSignature,
        configuration: PatternConfiguration,
        stability: Float,
    ) {
        let signature_id = signature.calculate_id();

        if self.discovered_patterns.contains_key(&signature_id) {
            // Pattern seen before - update learning
            self.update_pattern_learning(signature_id, signature, configuration, stability);
        } else {
            // New pattern discovered
            self.register_new_pattern(signature_id, signature, configuration, stability);
        }
    }

    /// REGISTER a new pattern
    ///
    /// Only registers patterns that meet the minimum stability threshold.
    ///
    /// # Arguments
    /// * `signature_id` - Hash-based ID of the pattern signature
    /// * `signature` - Pattern signature
    /// * `configuration` - Pattern configuration
    /// * `stability` - Pattern stability
    pub fn register_new_pattern(
        &mut self,
        signature_id: u64,
        signature: PatternSignature,
        configuration: PatternConfiguration,
        stability: Float,
    ) {
        // Only register if stable enough
        if stability >= self.stability_threshold {
            let discovered = DiscoveredPattern::new(signature, configuration, stability);

            self.discovered_patterns.insert(signature_id, discovered);
            self.pattern_frequency.insert(signature_id, 1);
            self.pattern_stability.insert(signature_id, stability);
        }
    }

    /// UPDATE pattern learning
    ///
    /// Updates the learning data for a pattern that has been seen again.
    ///
    /// # Arguments
    /// * `signature_id` - Hash-based ID of the pattern signature
    /// * `signature` - Pattern signature
    /// * `configuration` - Pattern configuration
    /// * `stability` - Pattern stability
    pub fn update_pattern_learning(
        &mut self,
        signature_id: u64,
        signature: PatternSignature,
        configuration: PatternConfiguration,
        stability: Float,
    ) {
        // Increment frequency
        if let Some(freq) = self.pattern_frequency.get_mut(&signature_id) {
            *freq += 1;
        }

        // Update stability (moving average)
        if let Some(current_stability) = self.pattern_stability.get(&signature_id) {
            let new_stability = (current_stability + stability) / 2.0;
            self.pattern_stability.insert(signature_id, new_stability);
        }

        // Update discovered pattern
        if let Some(discovered) = self.discovered_patterns.get_mut(&signature_id) {
            discovered.discovery_count += 1;
            discovered.last_seen = self.current_time;
            discovered.stability = (discovered.stability + stability) / 2.0;
            discovered.signature = signature;
            discovered.configuration = configuration;
        }
    }

    /// CLASSIFY a pattern based on evidence
    ///
    /// Classification is based on OBSERVED frequency and stability,
    /// NOT predetermined types.
    ///
    /// # Arguments
    /// * `signature` - Pattern signature
    ///
    /// # Returns
    /// * PatternClassification based on evidence
    pub fn classify_pattern(&self, signature: &PatternSignature) -> PatternClassification {
        let signature_id = signature.calculate_id();

        // Check if pattern has been discovered
        if let Some(discovered) = self.discovered_patterns.get(&signature_id) {
            // Pattern has been seen before - classify based on frequency and stability
            if discovered.discovery_count > 100 && discovered.stability > 0.9 {
                // Highly stable and frequently seen - well-established pattern
                PatternClassification::Established(signature.clone())
            } else if discovered.stability > 0.8 {
                // Stable but not frequently seen - emerging pattern
                PatternClassification::Emerging(signature.clone())
            } else {
                // Unstable - transient pattern
                PatternClassification::Transient(signature.clone())
            }
        } else {
            // New pattern - unclassified
            PatternClassification::Unclassified
        }
    }

    /// Get pattern stability
    pub fn get_pattern_stability(&self, signature: &PatternSignature) -> Option<Float> {
        let signature_id = signature.calculate_id();
        self.pattern_stability.get(&signature_id).copied()
    }

    /// Get pattern frequency
    pub fn get_pattern_frequency(&self, signature: &PatternSignature) -> Option<usize> {
        let signature_id = signature.calculate_id();
        self.pattern_frequency.get(&signature_id).copied()
    }

    /// Get total number of discovered patterns
    pub fn discovered_count(&self) -> usize {
        self.discovered_patterns.len()
    }
}

impl Default for DiscoverySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// SelfOrganize - Self-organization dynamics
///
/// Implements the self-organization dynamics that enable patterns to
/// emerge, stabilize, and evolve through the balance of attraction and diffusion.
#[derive(Debug, Clone)]
pub struct SelfOrganize {
    /// Attraction constant (controls nonlinear attraction)
    pub attraction_constant: Float,

    /// Diffusion constant (controls diffusion rate)
    pub diffusion_constant: Float,
}

impl SelfOrganize {
    /// Create a new self-organization system
    ///
    /// # Arguments
    /// * `attraction_constant` - Controls nonlinear attraction (default: 1.0)
    /// * `diffusion_constant` - Controls diffusion rate (default: 1.0)
    pub fn new(attraction_constant: Float, diffusion_constant: Float) -> Self {
        Self {
            attraction_constant: attraction_constant.max(0.0),
            diffusion_constant: diffusion_constant.max(0.0),
        }
    }

    /// Create with initial constants
    pub fn initial() -> Self {
        Self::new(1.0, 1.0)
    }

    /// Pattern evolution - determine pattern state through self-organization
    ///
    /// # Arguments
    /// * `density` - Pattern density
    /// * `energy_gradient` - Energy gradient across the pattern
    ///
    /// # Returns
    /// * PatternState (Stable, Collapsing, Diffusing, Metastable)
    pub fn pattern_evolution(&self, density: Float, energy_gradient: Float) -> PatternState {
        // Calculate self-attraction (nonlinear)
        let attraction = self.calculate_self_attraction(density);

        // Calculate self-diffusion
        let diffusion = self.calculate_self_diffusion(energy_gradient);

        // Calculate balance
        let balance = self.calculate_balance(attraction, diffusion);

        // Determine state based on balance
        PatternState::from_balance(balance)
    }

    /// Calculate self-attraction (nonlinear)
    ///
    /// Attraction increases nonlinearly with density.
    /// Formula: Attraction = constant × density^2 (nonlinear, n=2)
    ///
    /// # Arguments
    /// * `density` - Pattern density
    ///
    /// # Returns
    /// * Attraction force
    pub fn calculate_self_attraction(&self, density: Float) -> Float {
        // Nonlinear: density squared
        self.attraction_constant * density.powi(2)
    }

    /// Calculate self-diffusion
    ///
    /// Diffusion increases with energy gradient.
    /// Formula: Diffusion = constant × energy_gradient
    ///
    /// # Arguments
    /// * `energy_gradient` - Energy gradient across the pattern
    ///
    /// # Returns
    /// * Diffusion force
    pub fn calculate_self_diffusion(&self, energy_gradient: Float) -> Float {
        // Linear: proportional to energy gradient
        self.diffusion_constant * energy_gradient
    }

    /// Calculate balance point
    ///
    /// Balance = attraction / diffusion
    ///
    /// # Arguments
    /// * `attraction` - Attraction force
    /// * `diffusion` - Diffusion force
    ///
    /// # Returns
    /// * Balance value
    pub fn calculate_balance(&self, attraction: Float, diffusion: Float) -> Float {
        if diffusion > 0.0 {
            attraction / diffusion
        } else if attraction > 0.0 {
            // Infinite balance (no diffusion, only attraction)
            Float::INFINITY
        } else {
            // Zero balance (no forces)
            0.0
        }
    }

    /// Calculate stability from balance
    ///
    /// Stability = 1.0 - |balance - 1.0|
    /// Range: 0.0 (unstable) to 1.0 (perfectly balanced)
    ///
    /// # Arguments
    /// * `balance` - Balance value
    ///
    /// # Returns
    /// * Stability value (0.0 - 1.0)
    pub fn calculate_stability(&self, balance: Float) -> Float {
        let deviation = (balance - 1.0).abs();

        // Stability decreases as deviation from perfect balance increases
        // Use exponential decay for smoother transition
        1.0 / (1.0 + deviation)
    }

    /// Calculate stability from density and energy gradient
    ///
    /// Convenience method that combines all calculations.
    ///
    /// # Arguments
    /// * `density` - Pattern density
    /// * `energy_gradient` - Energy gradient across the pattern
    ///
    /// # Returns
    /// * Stability value (0.0 - 1.0)
    pub fn calculate_stability_from_density_and_gradient(
        &self,
        density: Float,
        energy_gradient: Float,
    ) -> Float {
        let attraction = self.calculate_self_attraction(density);
        let diffusion = self.calculate_self_diffusion(energy_gradient);
        let balance = self.calculate_balance(attraction, diffusion);
        self.calculate_stability(balance)
    }
}

impl Default for SelfOrganize {
    fn default() -> Self {
        Self::initial()
    }
}

// Helper functions for pattern calculations

/// Calculate pattern resonance frequency from phase, amplitude, spatial, and energy factors
///
/// This is a helper function used by quantum field system to calculate
/// the resonance frequency of a group of fluctuations.
///
/// # Arguments
/// * `avg_phase` - Average phase of the pattern
/// * `avg_amplitude` - Average amplitude of the pattern
/// * `spatial_factor` - Spatial distribution factor
/// * `energy_factor` - Energy level factor
///
/// # Returns
/// * Resonance frequency (0.0 - 1.0)
pub fn calculate_pattern_resonance_frequency(
    avg_phase: Float,
    avg_amplitude: Float,
    spatial_factor: Float,
    energy_factor: Float,
) -> Float {
    // Resonance frequency = weighted combination
    // Phase and amplitude are more important than spatial and energy factors
    let resonance_frequency =
        (avg_phase * 0.3) + (avg_amplitude * 0.3) + (spatial_factor * 0.2) + (energy_factor * 0.2);

    resonance_frequency.clamp(0.0, 1.0)
}

/// Calculate resonance match between pattern and attractor
///
/// # Arguments
/// * `pattern_frequency` - Pattern resonance frequency
/// * `attractor_frequency` - Attractor resonance frequency
///
/// # Returns
/// * Resonance match (0.0 - 1.0)
pub fn calculate_resonance_match(pattern_frequency: Float, attractor_frequency: Float) -> Float {
    // Resonance match = inverse of frequency difference
    let frequency_difference = (pattern_frequency - attractor_frequency).abs();

    // Resonance = 1.0 / (1.0 + frequency_difference)
    

    1.0 / (1.0 + frequency_difference)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::common::{DevelopmentalPosition, HealthStatus};
    use crate::archetypes::{
        ArchetypeComplex, ArchetypeRole, ArchetypeTrait, FunctionalPair, LambdaMeasurement,
        LambdaMeasurementType, SigmaAxis, TarotCorrelation,
    };

    // Mock archetype for testing
    #[allow(dead_code)]
    struct MockArchetype {
        archetype_id: u8,
        lambda: LambdaMeasurement,
        name: String,
        active: bool,
    }

    impl MockArchetype {
        fn new(archetype_id: u8, lambda_value: Float, name: &str) -> Self {
            MockArchetype {
                archetype_id,
                lambda: LambdaMeasurement::new(lambda_value, LambdaMeasurementType::MatrixRigidity),
                name: name.to_string(),
                active: false,
            }
        }
    }

    impl ArchetypeTrait for MockArchetype {
        fn archetype_id(&self) -> u8 {
            self.archetype_id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Mind
        }
        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Matrix
        }
        fn lambda(&self) -> &LambdaMeasurement {
            &self.lambda
        }
        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.clamp(0.0, 1.0);
        }
        fn tarot_correlation(&self) -> TarotCorrelation {
            TarotCorrelation::new("Test".to_string())
        }
        fn sigma_axis(&self) -> SigmaAxis {
            SigmaAxis::SigmaA
        }
        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::StructurePair
        }
        fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {}
        fn description(&self) -> &str {
            &self.name
        }
        fn activate(&mut self, _intensity: Float) {
            self.active = true;
        }
        fn deactivate(&mut self) {
            self.active = false;
        }
        fn is_active(&self) -> bool {
            self.active
        }
        fn is_healthy(&self) -> bool {
            self.lambda.health_status() == HealthStatus::Healthy
        }
        fn health_status(&self) -> HealthStatus {
            self.lambda.health_status()
        }
    }

    #[test]
    fn test_pattern_signature_creation() {
        let signature = PatternSignature::new(
            3,
            EnergyDistribution::new(0.5, 0.1, 0.3, 0.7),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        assert_eq!(signature.particle_count, 3);
        assert_eq!(signature.resonance_frequency, 0.75);
    }

    #[test]
    fn test_energy_distribution_uniform() {
        let dist = EnergyDistribution::uniform(0.5);
        assert_eq!(dist.mean, 0.5);
        assert_eq!(dist.std_dev, 0.0);
        assert_eq!(dist.min, 0.5);
        assert_eq!(dist.max, 0.5);
    }

    #[test]
    fn test_spatial_arrangement_point() {
        let arr = SpatialArrangement::point();
        assert_eq!(arr.center, (0.0, 0.0, 0.0));
        assert_eq!(arr.avg_radius, 0.0);
        assert_eq!(arr.max_radius, 0.0);
        assert_eq!(arr.symmetry, 1.0);
    }

    #[test]
    fn test_pattern_state_from_balance() {
        // Stable (0.9 - 1.1)
        assert_eq!(PatternState::from_balance(1.0), PatternState::Stable);
        assert_eq!(PatternState::from_balance(0.95), PatternState::Stable);
        assert_eq!(PatternState::from_balance(1.05), PatternState::Stable);

        // Collapsing (> 1.5)
        assert_eq!(PatternState::from_balance(2.0), PatternState::Collapsing);
        assert_eq!(PatternState::from_balance(10.0), PatternState::Collapsing);

        // Diffusing (< 0.5)
        assert_eq!(PatternState::from_balance(0.3), PatternState::Diffusing);
        assert_eq!(PatternState::from_balance(0.0), PatternState::Diffusing);

        // Metastable (0.5 - 0.9 or 1.1 - 1.5)
        assert_eq!(PatternState::from_balance(0.7), PatternState::Metastable);
        assert_eq!(PatternState::from_balance(1.3), PatternState::Metastable);
    }

    #[test]
    fn test_attractor_pattern_creation() {
        let attractor =
            AttractorPattern::new(1, 0.75, 0.5, 0.8, HashMap::new(), 0.75, 1.5, vec![1, 2, 3]);

        assert_eq!(attractor.attractor_id, 1);
        assert_eq!(attractor.resonance_frequency, 0.75);
        assert_eq!(attractor.basin_of_attraction, 0.5);
        assert_eq!(attractor.attractor_strength, 0.8);
        assert_eq!(attractor.contributing_archetypes, vec![1, 2, 3]);
    }

    #[test]
    fn test_attractor_emergence_from_archetypes() {
        let arch1 = MockArchetype::new(1, 0.6, "Archetype1");
        let arch2 = MockArchetype::new(2, 0.8, "Archetype2");
        let arch3 = MockArchetype::new(3, 0.7, "Archetype3");

        let archetypes: Vec<&dyn ArchetypeTrait> = vec![&arch1, &arch2, &arch3];

        let attractor = AttractorPattern::emerge_from_archetypes(&archetypes, 100);

        assert_eq!(attractor.attractor_id, 100);
        assert_eq!(attractor.contributing_archetypes, vec![1, 2, 3]);

        // Resonance should be average of archetype lambda values
        let expected_resonance = (0.6 + 0.8 + 0.7) / 3.0;
        assert!((attractor.resonance_frequency - expected_resonance).abs() < 0.01);

        // Archetype resonance map should contain all archetypes
        assert_eq!(attractor.archetype_resonance_map.len(), 3);
        assert_eq!(attractor.archetype_resonance_map[&1], 0.6);
        assert_eq!(attractor.archetype_resonance_map[&2], 0.8);
        assert_eq!(attractor.archetype_resonance_map[&3], 0.7);
    }

    #[test]
    fn test_attractor_is_in_basin() {
        let attractor = AttractorPattern::new(1, 0.5, 0.2, 1.0, HashMap::new(), 0.5, 0.0, vec![]);

        // Pattern within basin
        assert!(attractor.is_in_basin(0.5));
        assert!(attractor.is_in_basin(0.6));
        assert!(attractor.is_in_basin(0.4));

        // Pattern outside basin
        assert!(!attractor.is_in_basin(0.8));
        assert!(!attractor.is_in_basin(0.2));
    }

    #[test]
    fn test_attractor_calculate_resonance_match() {
        let attractor = AttractorPattern::new(1, 0.5, 0.2, 1.0, HashMap::new(), 0.5, 0.0, vec![]);

        // Perfect match
        let match_perfect = attractor.calculate_resonance_match(0.5);
        assert_eq!(match_perfect, 1.0);

        // Close match
        let match_close = attractor.calculate_resonance_match(0.45);
        assert!(match_close > 0.9);

        // Poor match (resonance = 1.0 / (1.0 + 0.5) = 0.666...)
        let match_poor = attractor.calculate_resonance_match(1.0);
        assert!(match_poor < 0.7 && match_poor > 0.5);
    }

    #[test]
    fn test_discovery_system_creation() {
        let discovery = DiscoverySystem::new();

        assert_eq!(discovery.discovered_count(), 0);
        assert_eq!(discovery.stability_threshold, 0.7);
        assert_eq!(discovery.current_time, 0.0);
    }

    #[test]
    fn test_discovery_system_observe_new_pattern() {
        let mut discovery = DiscoverySystem::new();

        let signature = PatternSignature::new(
            1,
            EnergyDistribution::uniform(0.5),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        let config = PatternConfiguration::empty();
        discovery.observe_pattern(signature.clone(), config, 0.8);

        // Pattern should be registered (stability > threshold)
        assert_eq!(discovery.discovered_count(), 1);
        assert_eq!(discovery.get_pattern_frequency(&signature), Some(1));
        let stability = discovery.get_pattern_stability(&signature).unwrap();
        assert!((stability - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_discovery_system_reject_unstable_pattern() {
        let mut discovery = DiscoverySystem::new();

        let signature = PatternSignature::new(
            1,
            EnergyDistribution::uniform(0.5),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        let config = PatternConfiguration::empty();
        discovery.observe_pattern(signature.clone(), config, 0.5); // Below threshold

        // Pattern should NOT be registered (stability < threshold)
        assert_eq!(discovery.discovered_count(), 0);
        assert_eq!(discovery.get_pattern_frequency(&signature), None);
    }

    #[test]
    fn test_discovery_system_update_learning() {
        let mut discovery = DiscoverySystem::new();

        let signature = PatternSignature::new(
            1,
            EnergyDistribution::uniform(0.5),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        let config = PatternConfiguration::empty();

        // First observation
        discovery.observe_pattern(signature.clone(), config.clone(), 0.8);

        // Second observation
        discovery.observe_pattern(signature.clone(), config, 0.9);

        // Frequency should increase
        assert_eq!(discovery.get_pattern_frequency(&signature), Some(2));

        // Stability should be moving average
        let stability = discovery.get_pattern_stability(&signature).unwrap();
        assert!((stability - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_discovery_system_classify_pattern() {
        let mut discovery = DiscoverySystem::new();

        let signature = PatternSignature::new(
            1,
            EnergyDistribution::uniform(0.5),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        let config = PatternConfiguration::empty();

        // Unclassified (not yet observed)
        let classification = discovery.classify_pattern(&signature);
        assert_eq!(classification, PatternClassification::Unclassified);

        // Observe pattern once
        discovery.observe_pattern(signature.clone(), config.clone(), 0.85);

        // Emerging (seen once, stable > 0.8)
        let classification = discovery.classify_pattern(&signature);
        assert!(matches!(classification, PatternClassification::Emerging(_)));

        // Observe pattern many times
        for _ in 0..100 {
            discovery.observe_pattern(signature.clone(), config.clone(), 0.95);
        }

        // Established (seen many times, highly stable)
        let classification = discovery.classify_pattern(&signature);
        assert!(matches!(
            classification,
            PatternClassification::Established(_)
        ));
    }

    #[test]
    fn test_self_organize_creation() {
        let self_organize = SelfOrganize::new(1.0, 1.0);

        assert_eq!(self_organize.attraction_constant, 1.0);
        assert_eq!(self_organize.diffusion_constant, 1.0);
    }

    #[test]
    fn test_self_organize_calculate_self_attraction() {
        let self_organize = SelfOrganize::new(1.0, 1.0);

        // Nonlinear: density squared
        let attraction_1 = self_organize.calculate_self_attraction(1.0);
        assert_eq!(attraction_1, 1.0);

        let attraction_2 = self_organize.calculate_self_attraction(2.0);
        assert_eq!(attraction_2, 4.0);

        let attraction_3 = self_organize.calculate_self_attraction(0.5);
        assert_eq!(attraction_3, 0.25);
    }

    #[test]
    fn test_self_organize_calculate_self_diffusion() {
        let self_organize = SelfOrganize::new(1.0, 1.0);

        // Linear: proportional to energy gradient
        let diffusion_1 = self_organize.calculate_self_diffusion(1.0);
        assert_eq!(diffusion_1, 1.0);

        let diffusion_2 = self_organize.calculate_self_diffusion(2.0);
        assert_eq!(diffusion_2, 2.0);

        let diffusion_3 = self_organize.calculate_self_diffusion(0.5);
        assert_eq!(diffusion_3, 0.5);
    }

    #[test]
    fn test_self_organize_calculate_balance() {
        let self_organize = SelfOrganize::new(1.0, 1.0);

        // Perfect balance
        let balance = self_organize.calculate_balance(1.0, 1.0);
        assert_eq!(balance, 1.0);

        // Attraction dominates
        let balance = self_organize.calculate_balance(2.0, 1.0);
        assert_eq!(balance, 2.0);

        // Diffusion dominates
        let balance = self_organize.calculate_balance(1.0, 2.0);
        assert_eq!(balance, 0.5);
    }

    #[test]
    fn test_self_organize_calculate_stability() {
        let self_organize = SelfOrganize::initial();

        // Perfect balance = perfect stability
        let stability = self_organize.calculate_stability(1.0);
        assert_eq!(stability, 1.0);

        // Some deviation = reduced stability
        let stability = self_organize.calculate_stability(0.5);
        assert!(stability < 1.0 && stability > 0.0);

        // Large deviation = low stability
        let stability = self_organize.calculate_stability(5.0);
        assert!(stability > 0.0 && stability < 1.0);
    }

    #[test]
    fn test_self_organize_pattern_evolution() {
        let self_organize = SelfOrganize::initial();

        // Balanced (stable)
        let state = self_organize.pattern_evolution(1.0, 1.0);
        assert_eq!(state, PatternState::Stable);

        // Collapsing (attraction dominates)
        let state = self_organize.pattern_evolution(2.0, 0.5);
        assert_eq!(state, PatternState::Collapsing);

        // Diffusing (diffusion dominates)
        let state = self_organize.pattern_evolution(0.5, 2.0);
        assert_eq!(state, PatternState::Diffusing);

        // Metastable (unbalanced but not extreme)
        let state = self_organize.pattern_evolution(0.8, 1.0);
        assert_eq!(state, PatternState::Metastable);
    }

    #[test]
    fn test_calculate_pattern_resonance_frequency() {
        let freq = calculate_pattern_resonance_frequency(0.5, 0.6, 0.7, 0.8);

        // Weighted average: 0.5*0.3 + 0.6*0.3 + 0.7*0.2 + 0.8*0.2
        let expected = 0.15 + 0.18 + 0.14 + 0.16;
        assert!((freq - expected).abs() < 0.01);
    }

    #[test]
    fn test_calculate_resonance_match() {
        // Perfect match
        let match_perfect = calculate_resonance_match(0.5, 0.5);
        assert_eq!(match_perfect, 1.0);

        // Close match
        let match_close = calculate_resonance_match(0.5, 0.45);
        assert!(match_close > 0.9);

        // Poor match (resonance = 1.0 / (1.0 + 0.5) = 0.666...)
        let match_poor = calculate_resonance_match(0.5, 1.0);
        assert!(match_poor < 0.7 && match_poor > 0.5);
    }

    #[test]
    fn test_pattern_configuration() {
        let mut config = PatternConfiguration::new(HashMap::new(), 1.0, 0.8);

        config.set("energy".to_string(), 1.5);
        config.set("mass".to_string(), 2.0);

        assert_eq!(config.get("energy"), Some(1.5));
        assert_eq!(config.get("mass"), Some(2.0));
        assert_eq!(config.get("unknown"), None);
    }

    #[test]
    fn test_discovered_pattern_creation() {
        let signature = PatternSignature::new(
            1,
            EnergyDistribution::uniform(0.5),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        let config = PatternConfiguration::empty();
        let discovered = DiscoveredPattern::new(signature.clone(), config, 0.8);

        assert_eq!(discovered.signature, signature);
        assert_eq!(discovered.stability, 0.8);
        assert_eq!(discovered.discovery_count, 1);
    }

    #[test]
    fn test_pattern_classification_is_stable() {
        let signature = PatternSignature::new(
            1,
            EnergyDistribution::uniform(0.5),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        let unclassified = PatternClassification::Unclassified;
        assert!(!unclassified.is_stable());

        let transient = PatternClassification::Transient(signature.clone());
        assert!(!transient.is_stable());

        let emerging = PatternClassification::Emerging(signature.clone());
        assert!(emerging.is_stable());

        let established = PatternClassification::Established(signature);
        assert!(established.is_stable());
    }

    #[test]
    fn test_pattern_classification_is_established() {
        let signature = PatternSignature::new(
            1,
            EnergyDistribution::uniform(0.5),
            SpatialArrangement::point(),
            0.75,
            HashMap::new(),
        );

        let unclassified = PatternClassification::Unclassified;
        assert!(!unclassified.is_established());

        let transient = PatternClassification::Transient(signature.clone());
        assert!(!transient.is_established());

        let emerging = PatternClassification::Emerging(signature.clone());
        assert!(!emerging.is_established());

        let established = PatternClassification::Established(signature);
        assert!(established.is_established());
    }

    #[test]
    fn test_attractor_basin_width_from_archetype_diversity() {
        // Single archetype - narrow basin
        let arch1 = MockArchetype::new(1, 0.6, "Archetype1");
        let archetypes: Vec<&dyn ArchetypeTrait> = vec![&arch1];
        let attractor1 = AttractorPattern::emerge_from_archetypes(&archetypes, 1);
        let basin1 = attractor1.basin_of_attraction;

        // Multiple diverse archetypes - wider basin
        let arch2 = MockArchetype::new(2, 0.9, "Archetype2");
        let arch3 = MockArchetype::new(3, 0.3, "Archetype3");
        let archetypes: Vec<&dyn ArchetypeTrait> = vec![&arch1, &arch2, &arch3];
        let attractor2 = AttractorPattern::emerge_from_archetypes(&archetypes, 2);
        let basin2 = attractor2.basin_of_attraction;

        // More diverse archetypes should produce wider basin
        assert!(basin2 > basin1);
    }

    #[test]
    fn test_attractor_strength_from_archetype_coherence() {
        // High coherence archetypes
        let arch1 = MockArchetype::new(1, 0.8, "Archetype1");
        let arch2 = MockArchetype::new(2, 0.85, "Archetype2");
        let archetypes: Vec<&dyn ArchetypeTrait> = vec![&arch1, &arch2];
        let attractor1 = AttractorPattern::emerge_from_archetypes(&archetypes, 1);

        // Low coherence archetypes
        let arch3 = MockArchetype::new(3, 0.3, "Archetype3");
        let arch4 = MockArchetype::new(4, 0.35, "Archetype4");
        let archetypes: Vec<&dyn ArchetypeTrait> = vec![&arch3, &arch4];
        let attractor2 = AttractorPattern::emerge_from_archetypes(&archetypes, 2);

        // Higher coherence should produce stronger attractor
        assert!(attractor1.attractor_strength > attractor2.attractor_strength);
    }
}
