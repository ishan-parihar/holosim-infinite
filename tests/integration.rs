//! Integration tests for HoloSim_Infinite cross-module functionality.
//!
//! These tests exercise the public API across multiple modules to verify
//! that components work together correctly.

use holonic_realms::simulation_v3::{
    ArchetypeActivationProfile, ArchetypeBasis, ArchetypicalInterferenceEngine,
    BehavioralObservation, FreeWillSeed, HolographicEntity, MeraNetwork, MeraQuery, MeraScale,
    ObservationLayer, PhysicalObservation, QueryType, SpectrumRatio, Tensor,
};

// ============================================================================
// TEST 1: Archetype Interference produces EmergentBehavior
// ============================================================================

/// Create an ArchetypeBasis, create an ArchetypicalInterferenceEngine, feed it
/// an archetype profile, and verify it produces an EmergentBehavior with
/// non-zero confidence.
#[test]
fn archetype_interference_produces_behavior() {
    // Dimension = 22 to match archetype count, so components stay above
    // the 0.01 threshold after normalization in apply_interference_physics
    let basis = ArchetypeBasis::new(22);

    // Set all 22 coefficients high to ensure every component exceeds 0.01
    // after normalization (total sum of 22 * 0.9 = 19.8, each ~ 0.05)
    let mut profile = ArchetypeActivationProfile::zero();
    for i in 0..22 {
        profile.set(i, 0.9).unwrap();
    }

    // Create the interference engine with a deterministic seed
    let seed = FreeWillSeed::new(42);
    let mut engine = ArchetypicalInterferenceEngine::new(basis, seed);

    // Use mild spectrum_ratio and density to avoid suppressing components
    // below the 0.01 threshold (spectrum_filter = 1/(1+r), density = 1/(d+1))
    let behavior = engine
        .evaluate_behavior(&profile, 0.1, 0, None, None)
        .expect("evaluate_behavior should succeed with valid inputs");

    // Verify emergent behavior properties
    assert!(
        behavior.confidence > 0.0,
        "EmergentBehavior confidence should be non-zero, got {}",
        behavior.confidence
    );
    assert!(
        behavior.confidence <= 1.0,
        "confidence should be <= 1.0, got {}",
        behavior.confidence
    );

    // Interference pattern should have meaningful coherence
    assert!(
        behavior.interference_pattern.coherence >= 0.0,
        "coherence should be non-negative"
    );

    // Action vector should have positive magnitude
    assert!(
        behavior.action_vector.magnitude > 0.0,
        "action magnitude should be positive, got {}",
        behavior.action_vector.magnitude
    );

    // Stability and novelty should be in valid range
    assert!(
        (0.0..=1.0).contains(&behavior.stability),
        "stability should be 0.0-1.0, got {}",
        behavior.stability
    );
    assert!(
        (0.0..=1.0).contains(&behavior.novelty),
        "novelty should be 0.0-1.0, got {}",
        behavior.novelty
    );
}

// ============================================================================
// TEST 2: MERA compress/decompress roundtrip preserves shape
// ============================================================================

/// Create a MeraNetwork, compress a small tensor, decompress it, and verify
/// the output shape matches the input.
#[test]
fn mera_compress_decompress_roundtrip() {
    // Use 3 levels — a 4x4 tensor only supports 2 downsamplings before
    // reaching 1x1, and the 7-level default would hit InsufficientData
    let mut network = MeraNetwork::with_levels(3);

    // Create a 4x4 input tensor
    let shape = vec![4, 4];
    let data: Vec<f64> = (0..16).map(|i| i as f64 * 0.1).collect();
    let input_tensor =
        Tensor::from_data(shape.clone(), data).expect("Tensor creation should succeed");

    let original_shape = input_tensor.shape.clone();
    let original_elements = input_tensor.num_elements();

    // Compress the tensor
    let compress_result = network
        .compress(input_tensor)
        .expect("compress should succeed");

    assert!(
        compress_result.compression_ratio > 0.0,
        "compression ratio should be positive, got {}",
        compress_result.compression_ratio
    );

    // Decompress at the finest scale level (0 = quantum)
    let query = MeraQuery::new(MeraScale::Quantum, QueryType::Global);
    let decompress_result = network
        .decompress(&query)
        .expect("decompress should succeed");

    // Verify output shape matches input shape
    assert_eq!(
        decompress_result.data.shape, original_shape,
        "decompressed shape {:?} should match original shape {:?}",
        decompress_result.data.shape, original_shape
    );

    // Verify element count matches
    assert_eq!(
        decompress_result.data.num_elements(),
        original_elements,
        "decompressed element count should match original"
    );

    // Verify decompression produced non-trivial data
    let norm = decompress_result.data.norm();
    assert!(
        norm > 0.0,
        "decompressed tensor should have non-zero norm, got {}",
        norm
    );

    // Precision should be in valid range
    assert!(
        (0.0..=1.0).contains(&decompress_result.precision),
        "precision should be 0.0-1.0, got {}",
        decompress_result.precision
    );
}

// ============================================================================
// TEST 3: HolographicEntity observation produces PhysicalObservation and
// BehavioralObservation with populated fields
// ============================================================================

/// Create a HolographicEntity, use the ObservationLayer to observe it, and
/// verify PhysicalObservation and BehavioralObservation are populated.
#[test]
fn holographic_entity_observation() {
    // Create a holographic entity with a space-time dominant spectrum
    let spectrum = SpectrumRatio::new(2.0, 1.0);
    let entity = HolographicEntity::new(12345, spectrum);

    // Create an archetype activation profile with elevated values
    let mut archetype_profile: [f64; 22] = [0.0; 22];
    archetype_profile[0] = 0.5; // A1
    archetype_profile[7] = 0.7; // A8
    archetype_profile[14] = 0.4; // A15
    archetype_profile[21] = 0.3; // A22 (Choice)

    // Create observation layer and observe the entity
    let mut observation_layer = ObservationLayer::new();
    let observation = observation_layer.observe_entity(
        entity.entity_id,
        &archetype_profile,
        entity.spectrum_ratio.spectrum_position,
        3, // density level
        entity.position,
        entity.velocity,
        entity.mass,
        entity.energy,
        0.7, // coherence
    );

    // --- Verify PhysicalObservation is populated ---
    let physical: &PhysicalObservation = &observation.physical;
    assert_eq!(physical.entity_id, 12345, "entity_id should match");
    assert_eq!(
        physical.position,
        [0.0, 0.0, 0.0],
        "position should match entity position"
    );
    assert!(
        physical.mass > 0.0,
        "mass should be positive, got {}",
        physical.mass
    );
    assert!(
        physical.temperature > 0.0,
        "temperature should be positive, got {}",
        physical.temperature
    );
    assert!(
        (0.0..=1.0).contains(&physical.health),
        "health should be 0.0-1.0, got {}",
        physical.health
    );
    assert!(
        (0.0..=1.0).contains(&physical.energy_level),
        "energy_level should be 0.0-1.0, got {}",
        physical.energy_level
    );

    // --- Verify BehavioralObservation is populated ---
    let behavioral: &BehavioralObservation = &observation.behavioral;
    assert_eq!(behavioral.entity_id, 12345, "entity_id should match");

    // Needs array should have 5 elements, all in valid range
    assert_eq!(behavioral.needs.len(), 5, "needs should have 5 elements");
    for (i, &need) in behavioral.needs.iter().enumerate() {
        assert!(
            (0.0..=1.0).contains(&need),
            "need[{}] should be 0.0-1.0, got {}",
            i,
            need
        );
    }

    // Mood should be in valid range
    assert!(
        (-1.0..=1.0).contains(&behavioral.mood),
        "mood should be -1.0 to 1.0, got {}",
        behavioral.mood
    );

    // Personality profile should be 22 elements
    assert_eq!(
        behavioral.personality_profile.len(),
        22,
        "personality_profile should have 22 elements"
    );

    // Stability and novelty should be in valid range
    assert!(
        (0.0..=1.0).contains(&behavioral.stability),
        "stability should be 0.0-1.0, got {}",
        behavioral.stability
    );
    assert!(
        (0.0..=1.0).contains(&behavioral.novelty),
        "novelty should be 0.0-1.0, got {}",
        behavioral.novelty
    );

    // Verify observation was stored
    let all_obs = observation_layer.get_all_observations();
    assert!(
        all_obs.contains_key(&12345),
        "observation should be stored in the layer"
    );
    assert_eq!(
        observation_layer.statistics().total_observations,
        1,
        "total_observations should be 1"
    );
}
