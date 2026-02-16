// Consciousness-to-Matter Transition - Phase 7: Consciousness-First Cosmology
//
// This module implements the consciousness-to-matter transition algorithm,
// modeling how consciousness (information) exists BEFORE matter.
//
// Key Principles:
// 1. Quantum fluctuations carry consciousness patterns
// 2. Galaxies, planets, celestial bodies exist as quantum information structures before atoms
// 3. The physical universe is the manifestation of the pre-existing consciousness universe
// 4. Quantum energy pools = coherent quantum states with information content
// 5. Attractor fields create stable quantum states (periodic table)
//
// Knowledge Base References:
// - COMPREHENSIVE_REFACTOR_PLAN.md Phase 7
// - COSMOLOGICAL-ARCHITECTURE.md: "Consciousness-First Cosmology"
// - "Consciousness (information) exists BEFORE matter"

use crate::matter::particle::{Complex, Coordinate3D};
use crate::physical_manifestation::atom_formation::Atom;
use crate::types::Float;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// QUANTUM ENERGY POOL
// ============================================================================

/// Quantum energy pool with holographic information
///
/// Quantum energy pools are coherent quantum states that carry consciousness
/// patterns before manifesting as physical matter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEnergyPool {
    /// Unique identifier for this quantum pool
    pub pool_id: String,

    /// Holographic information pattern (22 values from archetype activation)
    pub holographic_information: Vec<Float>,

    /// Information content (bits) - measures consciousness complexity
    pub information_content: Float,

    /// Coherence level (0.0 to 1.0) - measures quantum coherence
    pub coherence_level: Float,

    /// Is the pool decohered (collapsed to classical state)?
    pub is_decohered: bool,

    /// Collapse probability (0.0 to 1.0)
    pub collapse_probability: Float,

    /// Wave function (complex amplitude)
    pub wave_function: Vec<Complex>,

    /// Energy content (Joules)
    pub energy: Float,

    /// Entanglement links to other pools
    pub entanglement_links: Vec<String>,

    /// Creation timestamp
    pub creation_time: u64,

    /// Current age
    pub age: u64,
}

impl QuantumEnergyPool {
    /// Create a new quantum energy pool
    ///
    /// # Arguments
    /// * `pool_id` - Unique identifier
    /// * `information_content` - Information content in bits
    /// * `coherence_level` - Initial coherence level (0.0 to 1.0)
    pub fn new(pool_id: String, information_content: Float, coherence_level: Float) -> Self {
        // Generate holographic information pattern from information content
        let holographic_information = Self::generate_holographic_pattern(information_content);

        // Initialize wave function with coherent state
        let wave_function = Self::initialize_coherent_state();

        Self {
            pool_id,
            holographic_information,
            information_content,
            coherence_level: coherence_level.clamp(0.0, 1.0),
            is_decohered: false,
            collapse_probability: 0.0,
            wave_function,
            energy: Self::calculate_energy_from_information(information_content),
            entanglement_links: Vec::new(),
            creation_time: 0,
            age: 0,
        }
    }

    /// Generate holographic information pattern from information content
    ///
    /// This creates a 22-value pattern that reflects the 22 archetypes
    /// of consciousness, scaled by the information content.
    fn generate_holographic_pattern(information_content: Float) -> Vec<Float> {
        let _rng = rand::thread_rng();
        let base_pattern = (1..=22)
            .map(|i| {
                // Create periodic variation based on archetype position
                let phase = 2.0 * std::f64::consts::PI * i as Float / 22.0;
                let amplitude = (phase.sin() + 1.0) / 2.0; // Normalize to 0.0-1.0

                // Scale by information content
                amplitude * (information_content / 1000.0).min(1.0).max(0.1)
            })
            .collect();

        base_pattern
    }

    /// Initialize coherent quantum state
    fn initialize_coherent_state() -> Vec<Complex> {
        // Create a coherent superposition state
        vec![Complex::new(1.0, 0.0)]
    }

    /// Calculate energy from information content
    ///
    /// E = I * k_B * ln(2) * T
    /// Where I is information content, k_B is Boltzmann constant, T is temperature
    fn calculate_energy_from_information(information_content: Float) -> Float {
        const K_B: Float = 1.380649e-23; // Boltzmann constant (J/K)
        const T: Float = 2.725; // Cosmic microwave background temperature (K)

        information_content * K_B * 2.0_f64.ln() * T
    }

    /// Check if the pool is decohered
    pub fn is_decohered(&self) -> bool {
        self.is_decohered
    }

    /// Collapse wave function
    ///
    /// This simulates the Copenhagen interpretation: measurement causes
    /// wave function collapse from superposition to definite state.
    pub fn collapse_wave_function(&mut self) -> CollapseResult {
        if self.is_decohered {
            return CollapseResult::AlreadyCollapsed;
        }

        // Calculate collapse probability based on coherence level
        let collapse_threshold = 1.0 - self.coherence_level;
        let random_value: Float = rand::thread_rng().gen();

        if random_value > collapse_threshold {
            self.is_decohered = true;
            self.collapse_probability = 1.0;

            // Collapse wave function to definite state
            self.collapse_to_definite_state();

            CollapseResult::Collapsed
        } else {
            self.collapse_probability = random_value;
            CollapseResult::NotCollapsed(self.collapse_probability)
        }
    }

    /// Collapse wave function to definite state
    fn collapse_to_definite_state(&mut self) {
        // Normalize wave function
        if !self.wave_function.is_empty() {
            let magnitude = self.wave_function[0].magnitude();
            if magnitude > 0.0 {
                self.wave_function[0] = self.wave_function[0].scale(1.0 / magnitude);
            }
        }

        // Reduce coherence level
        self.coherence_level *= 0.1;
    }

    /// Update coherence level
    ///
    /// Coherence decays over time due to environmental decoherence
    pub fn update_coherence(&mut self, dt: Float, decoherence_rate: Float) {
        if !self.is_decohered {
            // Exponential decay of coherence
            self.coherence_level *= (-decoherence_rate * dt).exp();
            self.coherence_level = self.coherence_level.clamp(0.0, 1.0);
        }
    }

    /// Evolve wave function
    ///
    /// Time evolution according to Schrödinger equation
    pub fn evolve_wave_function(&mut self, dt: Float) {
        if self.is_decohered {
            return;
        }

        const H_BAR: Float = 1.0545718e-34; // Reduced Planck constant

        // Phase evolution: ψ(t) = ψ(0) * exp(-iEt/ħ)
        let phase = -self.energy * dt / H_BAR;
        let cos_phase = phase.cos();
        let sin_phase = phase.sin();

        let evolution = Complex::new(cos_phase, sin_phase);

        for wave in &mut self.wave_function {
            *wave = wave.multiply(&evolution);
        }
    }

    /// Calculate probability density |ψ|²
    pub fn probability_density(&self) -> Float {
        self.wave_function
            .iter()
            .map(|w| w.magnitude_squared())
            .sum()
    }

    /// Add entanglement link
    pub fn add_entanglement(&mut self, other_pool_id: String) {
        if !self.entanglement_links.contains(&other_pool_id) {
            self.entanglement_links.push(other_pool_id);
        }
    }

    /// Remove entanglement link
    pub fn remove_entanglement(&mut self, other_pool_id: &str) {
        self.entanglement_links
            .retain(|id| id.as_str() != other_pool_id);
    }
}

// ============================================================================
// COLLAPSE RESULT
// ============================================================================

/// Result of wave function collapse
#[derive(Debug, Clone, PartialEq)]
pub enum CollapseResult {
    /// Wave function collapsed successfully
    Collapsed,
    /// Wave function already collapsed
    AlreadyCollapsed,
    /// Wave function did not collapse (probability too low)
    NotCollapsed(Float),
}

// ============================================================================
// CONSCIOUSNESS-TO-MATTER TRANSITION
// ============================================================================

/// Consciousness-to-matter transition
///
/// This represents the transition from quantum information (consciousness)
/// to physical matter (atoms).
#[derive(Debug, Clone)]
pub struct ConsciousnessToMatterTransition {
    /// Unique identifier for this transition
    pub transition_id: String,

    /// Source quantum energy pool
    pub quantum_pool: QuantumEnergyPool,

    /// Resulting atom (if transition completed)
    pub resulting_atom: Option<Atom>,

    /// Current state of the transition
    pub transition_state: TransitionState,

    /// Decoherence level (0.0 to 1.0)
    pub decoherence_level: Float,

    /// Attractor field used for stabilization
    pub attractor_field: Option<AttractorField>,

    /// Transition progress (0.0 to 1.0)
    pub progress: Float,

    /// Creation timestamp
    pub creation_time: u64,

    /// Completion timestamp (if completed)
    pub completion_time: Option<u64>,
}

/// State of consciousness-to-matter transition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionState {
    /// Transition initiated
    Initiated,
    /// Decoherence in progress
    Decohering,
    /// Wave function collapse
    Collapsing,
    /// Attractor field stabilization
    Stabilizing,
    /// Atom formation
    Forming,
    /// Transition complete
    Complete,
    /// Transition failed
    Failed,
}

impl ConsciousnessToMatterTransition {
    /// Create a new consciousness-to-matter transition
    ///
    /// # Arguments
    /// * `transition_id` - Unique identifier
    /// * `quantum_pool` - Source quantum energy pool
    pub fn new(transition_id: String, quantum_pool: QuantumEnergyPool) -> Self {
        Self {
            transition_id,
            quantum_pool,
            resulting_atom: None,
            transition_state: TransitionState::Initiated,
            decoherence_level: 0.0,
            attractor_field: None,
            progress: 0.0,
            creation_time: 0,
            completion_time: None,
        }
    }

    /// Advance the transition
    ///
    /// # Arguments
    /// * `attractor_fields` - Available attractor fields for stabilization
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn advance(
        &mut self,
        attractor_fields: &HashMap<String, AttractorField>,
    ) -> TransitionResult {
        match self.transition_state {
            TransitionState::Initiated => {
                self.transition_state = TransitionState::Decohering;
                self.progress = 0.1;
                TransitionResult::InProgress("Decoherence started".to_string())
            }
            TransitionState::Decohering => {
                // Increase decoherence level
                self.decoherence_level += 0.2;
                self.progress = 0.1 + self.decoherence_level * 0.4;

                if self.decoherence_level >= 0.8 {
                    self.transition_state = TransitionState::Collapsing;
                }

                TransitionResult::InProgress(format!(
                    "Decoherence: {:.0}%",
                    self.decoherence_level * 100.0
                ))
            }
            TransitionState::Collapsing => {
                // Attempt wave function collapse
                let collapse_result = self.quantum_pool.collapse_wave_function();

                match collapse_result {
                    CollapseResult::Collapsed => {
                        self.transition_state = TransitionState::Stabilizing;
                        self.progress = 0.6;
                        TransitionResult::InProgress("Wave function collapsed".to_string())
                    }
                    CollapseResult::AlreadyCollapsed => {
                        self.transition_state = TransitionState::Stabilizing;
                        self.progress = 0.6;
                        TransitionResult::InProgress("Wave function already collapsed".to_string())
                    }
                    CollapseResult::NotCollapsed(prob) => {
                        TransitionResult::InProgress(format!("Collapse probability: {:.2}", prob))
                    }
                }
            }
            TransitionState::Stabilizing => {
                // Find matching attractor field
                if let Some(attractor) = self.find_matching_attractor(attractor_fields) {
                    self.attractor_field = Some(attractor);
                    self.transition_state = TransitionState::Forming;
                    self.progress = 0.8;
                    TransitionResult::InProgress("Attractor field found".to_string())
                } else {
                    self.transition_state = TransitionState::Failed;
                    TransitionResult::Failed("No matching attractor field".to_string())
                }
            }
            TransitionState::Forming => {
                // Form atom from attractor field
                if let Some(attractor) = &self.attractor_field {
                    self.resulting_atom = Some(self.create_atom_from_attractor(attractor));
                    self.transition_state = TransitionState::Complete;
                    self.progress = 1.0;
                    self.completion_time = Some(0);
                    TransitionResult::Complete
                } else {
                    self.transition_state = TransitionState::Failed;
                    TransitionResult::Failed("No attractor field".to_string())
                }
            }
            TransitionState::Complete => TransitionResult::Complete,
            TransitionState::Failed => TransitionResult::Failed("Transition failed".to_string()),
        }
    }

    /// Find matching attractor field
    fn find_matching_attractor(
        &self,
        attractor_fields: &HashMap<String, AttractorField>,
    ) -> Option<AttractorField> {
        // Match based on holographic information pattern
        let pool_pattern = &self.quantum_pool.holographic_information;

        for (_, attractor) in attractor_fields {
            let compatibility =
                calculate_pattern_compatibility(pool_pattern, &attractor.quantum_states);

            if compatibility > 0.7 {
                return Some(attractor.clone());
            }
        }

        None
    }

    /// Create atom from attractor field
    fn create_atom_from_attractor(&self, attractor: &AttractorField) -> Atom {
        // Create atom based on attractor field properties
        // This is a simplified version - in practice, would use AtomFormationSystem

        let archetype_activation: [Float; 22] = self
            .quantum_pool
            .holographic_information
            .iter()
            .enumerate()
            .map(|(i, v)| {
                // Blend pool information with attractor field
                let attractor_val = attractor.quantum_states.get(i).copied().unwrap_or(0.5);
                (v + attractor_val) / 2.0
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([0.5; 22]);

        // Placeholder atom creation
        // In practice, would use AtomFormationSystem::create_atom_from_archetypes
        Atom {
            id: 0,
            position: Coordinate3D::origin(),
            atomic_number: attractor.element_type as u8,
            mass_number: attractor.element_type as u16 * 2,
            nucleus: crate::physical_manifestation::atom_formation::Nucleus::new(vec![], vec![]),
            electrons: vec![],
            orbital_configuration: String::new(),
            chemical_properties:
                crate::physical_manifestation::atom_formation::ChemicalProperties {
                    electronegativity: 0.0,
                    ionization_energy: 0.0,
                    atomic_radius: 0.0,
                    valence_electrons: 0,
                    oxidation_states: vec![],
                    chemical_family:
                        crate::physical_manifestation::atom_formation::ChemicalFamily::Unknown,
                    reactivity: 0.0,
                },
            archetype_activation,
            creation_time: 0,
            age: 0,
        }
    }
}

// ============================================================================
// TRANSITION RESULT
// ============================================================================

/// Result of consciousness-to-matter transition
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionResult {
    /// Transition in progress
    InProgress(String),
    /// Transition completed successfully
    Complete,
    /// Transition failed
    Failed(String),
}

// ============================================================================
// ATTRACTOR FIELD
// ============================================================================

/// Attractor field for stable quantum states
///
/// Attractor fields create stable quantum states that correspond to
/// elements in the periodic table.
#[derive(Debug, Clone)]
pub struct AttractorField {
    /// Unique identifier for this attractor field
    pub field_id: String,

    /// Element type (atomic number)
    pub element_type: u32,

    /// Attractor strength (0.0 to 1.0)
    pub attractor_strength: Float,

    /// Stability (0.0 to 1.0)
    pub stability: Float,

    /// Quantum states (22 values from archetype activation)
    pub quantum_states: Vec<Float>,

    /// Energy well depth (Joules)
    pub energy_well_depth: Float,

    /// Position in configuration space
    pub configuration_position: Coordinate3D,
}

impl AttractorField {
    /// Create a new attractor field
    ///
    /// # Arguments
    /// * `field_id` - Unique identifier
    /// * `element_type` - Atomic number (1-118)
    /// * `attractor_strength` - Strength of the attractor (0.0 to 1.0)
    pub fn new(field_id: String, element_type: u32, attractor_strength: Float) -> Self {
        // Generate quantum states based on element type
        let quantum_states = Self::generate_quantum_states(element_type);

        // Calculate energy well depth
        let energy_well_depth = Self::calculate_energy_well_depth(element_type);

        Self {
            field_id,
            element_type: element_type.clamp(1, 118),
            attractor_strength: attractor_strength.clamp(0.0, 1.0),
            stability: attractor_strength.clamp(0.0, 1.0),
            quantum_states,
            energy_well_depth,
            configuration_position: Coordinate3D::origin(),
        }
    }

    /// Generate quantum states for an element
    fn generate_quantum_states(element_type: u32) -> Vec<Float> {
        // Use periodic table position to generate quantum states
        let period = match element_type {
            1..=2 => 1,
            3..=10 => 2,
            11..=18 => 3,
            19..=36 => 4,
            37..=54 => 5,
            55..=86 => 6,
            87..=118 => 7,
            _ => 1,
        };

        let group = ((element_type as Float - 1.0) % 18.0 + 1.0) as u8;

        (1..=22)
            .map(|i| {
                // Create periodic variation
                let phase = 2.0 * std::f64::consts::PI * i as Float / 22.0;
                let periodic = (phase.sin() + 1.0) / 2.0;

                // Scale by group and period
                let group_factor = group as Float / 18.0;
                let period_factor = period as Float / 7.0;

                periodic * (0.5 + 0.3 * group_factor + 0.2 * period_factor)
            })
            .collect()
    }

    /// Calculate energy well depth for an element
    fn calculate_energy_well_depth(element_type: u32) -> Float {
        // Energy well depth increases with atomic number
        // Simplified model: E = Z * 13.6 eV (hydrogen-like)
        let ionization_energy = element_type as Float * 13.6; // eV
        ionization_energy * 1.602176634e-19 // Convert to Joules
    }

    /// Calculate attraction to a quantum state
    ///
    /// # Arguments
    /// * `quantum_state` - Quantum state to evaluate
    ///
    /// # Returns
    /// Attraction strength (0.0 to 1.0)
    pub fn calculate_attraction(&self, quantum_state: &[Float]) -> Float {
        calculate_pattern_compatibility(quantum_state, &self.quantum_states)
            * self.attractor_strength
    }
}

// ============================================================================
// PERIODIC TABLE ATTRACTOR FIELDS
// ============================================================================

/// Periodic table attractor fields
///
/// This generates attractor fields for all elements in the periodic table.
pub struct PeriodicTableAttractorFields {
    /// Map of attractor fields by element type
    pub attractor_fields: HashMap<String, AttractorField>,
}

impl PeriodicTableAttractorFields {
    /// Create periodic table attractor fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Generate attractor fields for all elements
        for atomic_number in 1..=118 {
            let field_id = format!("attractor_{}", atomic_number);
            let attractor = AttractorField::new(field_id.clone(), atomic_number, 0.8);
            fields.insert(field_id, attractor);
        }

        Self {
            attractor_fields: fields,
        }
    }

    /// Get attractor field for an element
    ///
    /// # Arguments
    /// * `atomic_number` - Atomic number (1-118)
    ///
    /// # Returns
    /// Option containing the attractor field
    pub fn get_attractor_field(&self, atomic_number: u32) -> Option<&AttractorField> {
        let field_id = format!("attractor_{}", atomic_number);
        self.attractor_fields.get(&field_id)
    }

    /// Find best matching attractor field
    ///
    /// # Arguments
    /// * `quantum_state` - Quantum state to match
    ///
    /// # Returns
    /// Option containing the best matching attractor field
    pub fn find_best_match(&self, quantum_state: &[Float]) -> Option<AttractorField> {
        let mut best_match: Option<AttractorField> = None;
        let mut best_compatibility = 0.0;

        for (_, attractor) in &self.attractor_fields {
            let compatibility =
                calculate_pattern_compatibility(quantum_state, &attractor.quantum_states);

            if compatibility > best_compatibility {
                best_compatibility = compatibility;
                best_match = Some(attractor.clone());
            }
        }

        best_match
    }

    /// Get all attractor fields
    pub fn all_fields(&self) -> &HashMap<String, AttractorField> {
        &self.attractor_fields
    }
}

impl Default for PeriodicTableAttractorFields {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Calculate pattern compatibility between two quantum states
///
/// # Arguments
/// * `pattern1` - First quantum state pattern
/// * `pattern2` - Second quantum state pattern
///
/// # Returns
/// Compatibility score (0.0 to 1.0)
fn calculate_pattern_compatibility(pattern1: &[Float], pattern2: &[Float]) -> Float {
    if pattern1.is_empty() || pattern2.is_empty() {
        return 0.0;
    }

    let min_len = pattern1.len().min(pattern2.len());

    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;

    for i in 0..min_len {
        dot_product += pattern1[i] * pattern2[i];
        norm1 += pattern1[i] * pattern1[i];
        norm2 += pattern2[i] * pattern2[i];
    }

    let norm1 = norm1.sqrt();
    let norm2 = norm2.sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }

    let cosine_similarity = dot_product / (norm1 * norm2);

    // Scale to 0.0-1.0 range
    (cosine_similarity + 1.0) / 2.0
}

// ============================================================================
// CONSCIOUSNESS-TO-MATTER MANAGER
// ============================================================================

/// Manager for consciousness-to-matter transitions
pub struct ConsciousnessToMatterManager {
    /// Active transitions
    pub active_transitions: Vec<ConsciousnessToMatterTransition>,

    /// Completed transitions
    pub completed_transitions: Vec<ConsciousnessToMatterTransition>,

    /// Failed transitions
    pub failed_transitions: Vec<ConsciousnessToMatterTransition>,

    /// Quantum energy pools
    pub quantum_pools: HashMap<String, QuantumEnergyPool>,

    /// Periodic table attractor fields
    pub attractor_fields: PeriodicTableAttractorFields,

    /// Decoherence rate (per second)
    pub decoherence_rate: Float,

    /// Transition counter
    transition_counter: AtomicU64,

    /// Pool counter
    pool_counter: AtomicU64,
}

impl ConsciousnessToMatterManager {
    /// Create a new consciousness-to-matter manager
    pub fn new() -> Self {
        Self {
            active_transitions: Vec::new(),
            completed_transitions: Vec::new(),
            failed_transitions: Vec::new(),
            quantum_pools: HashMap::new(),
            attractor_fields: PeriodicTableAttractorFields::new(),
            decoherence_rate: 0.1, // Default decoherence rate
            transition_counter: AtomicU64::new(0),
            pool_counter: AtomicU64::new(0),
        }
    }

    /// Create a quantum energy pool
    ///
    /// # Arguments
    /// * `information_content` - Information content in bits
    /// * `coherence_level` - Initial coherence level (0.0 to 1.0)
    ///
    /// # Returns
    /// ID of the created pool
    pub fn create_quantum_pool(
        &mut self,
        information_content: Float,
        coherence_level: Float,
    ) -> String {
        let pool_id = format!("pool_{}", self.pool_counter.fetch_add(1, Ordering::SeqCst));
        let pool = QuantumEnergyPool::new(pool_id.clone(), information_content, coherence_level);
        self.quantum_pools.insert(pool_id.clone(), pool);
        pool_id
    }

    /// Start a consciousness-to-matter transition
    ///
    /// # Arguments
    /// * `pool_id` - ID of the quantum pool to transition
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn start_transition(&mut self, pool_id: &str) -> Result<String, String> {
        // Get the quantum pool
        let pool = self
            .quantum_pools
            .get(pool_id)
            .ok_or_else(|| format!("Pool not found: {}", pool_id))?
            .clone();

        // Check if pool is already decohered
        if pool.is_decohered() {
            return Err(format!("Pool already decohered: {}", pool_id));
        }

        // Create transition
        let transition_id = format!(
            "transition_{}",
            self.transition_counter.fetch_add(1, Ordering::SeqCst)
        );
        let transition = ConsciousnessToMatterTransition::new(transition_id.clone(), pool);

        self.active_transitions.push(transition);

        Ok(transition_id)
    }

    /// Update all active transitions
    ///
    /// # Arguments
    /// * `dt` - Time step in seconds
    pub fn update_transitions(&mut self, dt: Float) {
        // Update quantum pools
        for pool in self.quantum_pools.values_mut() {
            pool.update_coherence(dt, self.decoherence_rate);
            pool.evolve_wave_function(dt);
        }

        // Update active transitions
        let mut completed_indices = Vec::new();
        let mut failed_indices = Vec::new();

        for (i, transition) in self.active_transitions.iter_mut().enumerate() {
            // Update quantum pool in transition
            transition
                .quantum_pool
                .update_coherence(dt, self.decoherence_rate);
            transition.quantum_pool.evolve_wave_function(dt);

            // Advance transition
            let result = transition.advance(self.attractor_fields.all_fields());

            match result {
                TransitionResult::Complete => {
                    completed_indices.push(i);
                }
                TransitionResult::Failed(_) => {
                    failed_indices.push(i);
                }
                _ => {}
            }
        }

        // Move completed transitions
        completed_indices.sort_by(|a, b| b.cmp(a)); // Sort in reverse
        for i in completed_indices {
            let transition = self.active_transitions.remove(i);
            self.completed_transitions.push(transition);
        }

        // Move failed transitions
        failed_indices.sort_by(|a, b| b.cmp(a)); // Sort in reverse
        for i in failed_indices {
            let transition = self.active_transitions.remove(i);
            self.failed_transitions.push(transition);
        }
    }

    /// Get statistics
    ///
    /// # Returns
    /// Statistics about consciousness-to-matter transitions
    pub fn get_statistics(&self) -> ConsciousnessToMatterStatistics {
        ConsciousnessToMatterStatistics {
            total_pools: self.quantum_pools.len(),
            coherent_pools: self
                .quantum_pools
                .values()
                .filter(|p| !p.is_decohered())
                .count(),
            decohered_pools: self
                .quantum_pools
                .values()
                .filter(|p| p.is_decohered())
                .count(),
            active_transitions: self.active_transitions.len(),
            completed_transitions: self.completed_transitions.len(),
            failed_transitions: self.failed_transitions.len(),
            total_information_content: self
                .quantum_pools
                .values()
                .map(|p| p.information_content)
                .sum(),
            average_coherence: if self.quantum_pools.is_empty() {
                0.0
            } else {
                self.quantum_pools
                    .values()
                    .map(|p| p.coherence_level)
                    .sum::<Float>()
                    / self.quantum_pools.len() as Float
            },
        }
    }
}

impl Default for ConsciousnessToMatterManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Statistics for consciousness-to-matter transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessToMatterStatistics {
    /// Total number of quantum pools
    pub total_pools: usize,

    /// Number of coherent pools
    pub coherent_pools: usize,

    /// Number of decohered pools
    pub decohered_pools: usize,

    /// Number of active transitions
    pub active_transitions: usize,

    /// Number of completed transitions
    pub completed_transitions: usize,

    /// Number of failed transitions
    pub failed_transitions: usize,

    /// Total information content (bits)
    pub total_information_content: Float,

    /// Average coherence level
    pub average_coherence: Float,
}

impl Default for ConsciousnessToMatterStatistics {
    fn default() -> Self {
        ConsciousnessToMatterStatistics {
            total_pools: 0,
            coherent_pools: 0,
            decohered_pools: 0,
            active_transitions: 0,
            completed_transitions: 0,
            failed_transitions: 0,
            total_information_content: 0.0,
            average_coherence: 0.0,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_pool_creation() {
        let pool = QuantumEnergyPool::new("test_pool".to_string(), 100.0, 0.9);

        assert_eq!(pool.pool_id, "test_pool");
        assert_eq!(pool.information_content, 100.0);
        assert_eq!(pool.coherence_level, 0.9);
        assert!(!pool.is_decohered);
        assert_eq!(pool.holographic_information.len(), 22);
    }

    #[test]
    fn test_wave_function_collapse() {
        let mut pool = QuantumEnergyPool::new("test_pool".to_string(), 100.0, 0.9);

        // Collapse wave function
        let result = pool.collapse_wave_function();

        match result {
            CollapseResult::Collapsed => {
                assert!(pool.is_decohered);
                assert_eq!(pool.collapse_probability, 1.0);
            }
            CollapseResult::NotCollapsed(_) => {
                assert!(!pool.is_decohered);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_coherence_decay() {
        let mut pool = QuantumEnergyPool::new("test_pool".to_string(), 100.0, 0.9);

        let initial_coherence = pool.coherence_level;
        pool.update_coherence(1.0, 0.1);

        assert!(pool.coherence_level < initial_coherence);
    }

    #[test]
    fn test_attractor_field_creation() {
        let attractor = AttractorField::new("test_attractor".to_string(), 6, 0.8);

        assert_eq!(attractor.field_id, "test_attractor");
        assert_eq!(attractor.element_type, 6);
        assert_eq!(attractor.attractor_strength, 0.8);
        assert_eq!(attractor.quantum_states.len(), 22);
    }

    #[test]
    fn test_periodic_table_attractor_fields() {
        let fields = PeriodicTableAttractorFields::new();

        assert_eq!(fields.all_fields().len(), 118);

        // Test getting specific element
        let carbon = fields.get_attractor_field(6);
        assert!(carbon.is_some());
        assert_eq!(carbon.unwrap().element_type, 6);
    }

    #[test]
    fn test_consciousness_to_matter_manager() {
        let mut manager = ConsciousnessToMatterManager::new();

        // Create quantum pool
        let pool_id = manager.create_quantum_pool(100.0, 0.9);
        assert!(manager.quantum_pools.contains_key(&pool_id));

        // Start transition
        let transition_id = manager.start_transition(&pool_id);
        assert!(transition_id.is_ok());
        assert_eq!(manager.active_transitions.len(), 1);

        // Update transitions
        manager.update_transitions(1.0);
    }

    #[test]
    fn test_pattern_compatibility() {
        let pattern1 = vec![1.0, 0.5, 0.0];
        let pattern2 = vec![1.0, 0.5, 0.0];
        let pattern3 = vec![0.0, 0.5, 1.0];

        let compat12 = calculate_pattern_compatibility(&pattern1, &pattern2);
        let compat13 = calculate_pattern_compatibility(&pattern1, &pattern3);

        assert!((compat12 - 1.0).abs() < 1e-9);
        assert!(compat13 < 1.0);
        assert!(compat13 > 0.0);
    }
}
