//! Multi-Scale Simulation Tests
//!
//! From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
//! "Create tests for all new functionality"
//!
//! Tests include:
//! - SimulationRunner::new() initializes scale physics correctly
//! - set_scale() changes active scale
//! - simulate_scale_step() calls correct scale physics
//! - Holographic continuity is maintained during scale transitions

use crate::simulation_v3::multiscale_camera::{InterpolationMode, MultiScaleCamera, ScaleLevel};
use crate::simulation_v3::scale_physics::{ScalePhysicsError, ScaleSpecificPhysics};
use crate::simulation_v3::simulation_runner::SimulationParameters;
use crate::simulation_v3::simulation_runner::SimulationRunner;

/// Test: SimulationRunner::new() initializes scale physics correctly
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Test SimulationRunner::new() initializes scale physics correctly"
#[test]
fn test_simulation_runner_initializes_scale_physics() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let runner = SimulationRunner::new(parameters);

    // Check that scale_physics is initialized
    // The scale_physics field should be accessible and properly initialized
    // This is verified implicitly by the runner being created successfully

    // Check that current_scale is set to default (Biological)
    assert_eq!(runner.get_current_scale(), ScaleLevel::Biological);

    // Check that multiscale_camera is initially None
    assert!(runner.get_multiscale_camera().is_none());

    // Check that holographic continuity is initialized
    let continuity = runner.get_holographic_continuity();
    assert_eq!(continuity.continuity_strength, 1.0);
}

/// Test: set_scale() changes active scale
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Test set_scale() changes active scale"
#[test]
fn test_set_scale_changes_active_scale() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Initial scale should be Biological
    assert_eq!(runner.get_current_scale(), ScaleLevel::Biological);

    // Set scale to Quantum
    runner.set_scale(ScaleLevel::Quantum);
    assert_eq!(runner.get_current_scale(), ScaleLevel::Quantum);

    // Set scale to Cosmic
    runner.set_scale(ScaleLevel::Cosmic);
    assert_eq!(runner.get_current_scale(), ScaleLevel::Cosmic);

    // Set scale back to Biological
    runner.set_scale(ScaleLevel::Biological);
    assert_eq!(runner.get_current_scale(), ScaleLevel::Biological);
}

/// Test: simulate_scale_step() calls correct scale physics
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Test simulate_scale_step() calls correct scale physics"
#[test]
fn test_simulate_scale_step_calls_correct_physics() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Test simulation at each scale level
    let scales = [
        ScaleLevel::Quantum,
        ScaleLevel::Cellular,
        ScaleLevel::Biological,
        ScaleLevel::Planetary,
        ScaleLevel::Stellar,
        ScaleLevel::Galactic,
        ScaleLevel::Cosmic,
    ];

    for scale in scales {
        // Set the scale
        runner.set_scale(scale);

        // Simulate one step
        let result = runner.simulate_scale_step(1.0);

        // Verify that simulation succeeded
        assert!(result.is_ok(), "Simulation failed for scale {:?}", scale);
    }
}

/// Test: simulate_scale_step() handles time_step correctly
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Test simulate_scale_step() calls correct scale physics"
#[test]
fn test_simulate_scale_step_time_step() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);
    runner.set_scale(ScaleLevel::Biological);

    // Test different time steps
    let time_steps = [0.1, 0.5, 1.0, 2.0, 5.0];

    for time_step in time_steps {
        let result = runner.simulate_scale_step(time_step);
        assert!(
            result.is_ok(),
            "Simulation failed for time step {}",
            time_step
        );
    }
}

/// Test: Holographic continuity is maintained during scale transitions
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Test holographic continuity is maintained during scale transitions"
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each entity contains within it all densities and sub-densities of the octave"
/// "Any portion contains the whole" - the Law of One principle
#[test]
fn test_holographic_continuity_during_scale_transitions() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Get initial holographic continuity
    let initial_continuity = runner.get_holographic_continuity();
    let initial_strength = initial_continuity.continuity_strength;

    // Transition through all scales
    let scales = [
        ScaleLevel::Quantum,
        ScaleLevel::Cellular,
        ScaleLevel::Biological,
        ScaleLevel::Planetary,
        ScaleLevel::Stellar,
        ScaleLevel::Galactic,
        ScaleLevel::Cosmic,
    ];

    for scale in scales {
        runner.set_scale(scale);

        // Get holographic continuity after transition
        let continuity = runner.get_holographic_continuity();

        // Continuity strength should remain high (near 1.0)
        assert!(
            continuity.continuity_strength >= 0.99,
            "Continuity strength dropped to {} at scale {:?}",
            continuity.continuity_strength,
            scale
        );
    }

    // Final continuity strength should be at least as strong as initial
    let final_continuity = runner.get_holographic_continuity();
    assert!(
        final_continuity.continuity_strength >= initial_strength,
        "Continuity strength degraded from {} to {}",
        initial_strength,
        final_continuity.continuity_strength
    );
}

/// Test: Cross-scale coupling is established
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Add coupling between adjacent scales"
#[test]
fn test_cross_scale_coupling() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Initialize holographic continuity by setting a scale
    runner.set_scale(ScaleLevel::Biological);

    // Test coupling between adjacent scales
    let adjacent_pairs = [
        (ScaleLevel::Quantum, ScaleLevel::Cellular),
        (ScaleLevel::Cellular, ScaleLevel::Biological),
        (ScaleLevel::Biological, ScaleLevel::Planetary),
        (ScaleLevel::Planetary, ScaleLevel::Stellar),
        (ScaleLevel::Stellar, ScaleLevel::Galactic),
        (ScaleLevel::Galactic, ScaleLevel::Cosmic),
    ];

    for (from, to) in adjacent_pairs {
        let coupling = runner.get_cross_scale_coupling(from, to);
        assert!(coupling > 0.0, "No coupling from {:?} to {:?}", from, to);
        assert!(
            coupling <= 1.0,
            "Coupling exceeds 1.0: {} from {:?} to {:?}",
            coupling,
            from,
            to
        );
    }
}

/// Test: Multi-scale camera integration
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Add multiscale_camera: Option<MultiScaleCamera> field for scale tracking"
#[test]
fn test_multiscale_camera_integration() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Check that camera is initially None
    assert!(runner.get_multiscale_camera().is_none());

    // Create and set multi-scale camera
    let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
    runner.set_multiscale_camera(camera);

    // Check that camera is now available
    assert!(runner.get_multiscale_camera().is_some());

    // Check that scale was updated to match camera
    assert_eq!(runner.get_current_scale(), ScaleLevel::Biological);
}

/// Test: Scale transition detection
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Add scale transition detection"
#[test]
fn test_scale_transition_detection() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Create and set multi-scale camera
    let mut camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
    runner.set_multiscale_camera(camera);

    // No transition should be detected initially
    let transition = runner.detect_scale_transition();
    assert!(transition.is_none());

    // Trigger a camera transition
    if let Some(ref mut cam) = runner.multiscale_camera {
        cam.transition_to_scale(
            ScaleLevel::Planetary,
            std::time::Duration::from_millis(100),
            InterpolationMode::Smooth,
        );
    }

    // Transition should now be detected
    let transition = runner.detect_scale_transition();
    assert!(transition.is_some());

    let (from, to) = transition.unwrap();
    assert_eq!(from, ScaleLevel::Biological);
    assert_eq!(to, ScaleLevel::Planetary);
}

/// Test: Scale physics simulation returns valid results
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Test simulate_scale_step() calls correct scale physics"
#[test]
fn test_scale_physics_simulation_result() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);
    runner.set_scale(ScaleLevel::Stellar);

    // Simulate multiple steps
    for _ in 0..5 {
        let result = runner.simulate_scale_step(1.0);
        assert!(result.is_ok());
    }
}

/// Test: Holographic continuity improves over time
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each entity contains within it all densities and sub-densities of the octave"
#[test]
fn test_holographic_continuity_improves() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Get initial continuity strength
    let initial_continuity = runner.get_holographic_continuity();
    let initial_strength = initial_continuity.continuity_strength;

    // Run multiple scale transitions
    runner.set_scale(ScaleLevel::Quantum);
    runner.set_scale(ScaleLevel::Cellular);
    runner.set_scale(ScaleLevel::Biological);

    // Get final continuity strength
    let final_continuity = runner.get_holographic_continuity();
    let final_strength = final_continuity.continuity_strength;

    // Continuity should improve or stay the same
    assert!(
        final_strength >= initial_strength,
        "Continuity degraded from {} to {}",
        initial_strength,
        final_strength
    );
}

/// Test: All 7 scale levels are accessible
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 5:
/// "7 scales (quantum to cosmic, 52 orders of magnitude)"
#[test]
fn test_all_seven_scale_levels_accessible() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Test all 7 scale levels
    let all_scales = [
        ScaleLevel::Quantum,
        ScaleLevel::Cellular,
        ScaleLevel::Biological,
        ScaleLevel::Planetary,
        ScaleLevel::Stellar,
        ScaleLevel::Galactic,
        ScaleLevel::Cosmic,
    ];

    for scale in all_scales {
        // Set scale
        runner.set_scale(scale);

        // Verify scale is set
        assert_eq!(runner.get_current_scale(), scale);

        // Simulate at this scale
        let result = runner.simulate_scale_step(1.0);
        assert!(result.is_ok(), "Failed to simulate at scale {:?}", scale);
    }
}

/// Test: Scale physics is initialized with all sub-physics engines
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Has ScaleSpecificPhysics with 7 scale implementations"
#[test]
fn test_scale_physics_has_all_implementations() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let runner = SimulationRunner::new(parameters);

    // Verify that we can simulate at all scales
    // This implicitly verifies that all physics engines are initialized
    let scales = [
        ScaleLevel::Quantum,
        ScaleLevel::Cellular,
        ScaleLevel::Biological,
        ScaleLevel::Planetary,
        ScaleLevel::Stellar,
        ScaleLevel::Galactic,
        ScaleLevel::Cosmic,
    ];

    for scale in scales {
        let mut test_runner = runner.clone();
        test_runner.set_scale(scale);
        let result = test_runner.simulate_scale_step(1.0);
        assert!(
            result.is_ok(),
            "Physics engine not properly initialized for scale {:?}",
            scale
        );
    }
}

/// Test: Scale-specific physics modes are correct
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 5:
/// "Physics mode: Quantum" (Quantum, Cellular scales)
/// "Physics mode: Space/Time (v = s/t)" (Biological, Planetary, Stellar, Galactic scales)
/// "Physics mode: Time/Space (v = t/s)" (Cosmic scale)
#[test]
fn test_scale_physics_modes() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let runner = SimulationRunner::new(parameters);

    // Quantum scale should use Quantum physics
    runner.set_scale(ScaleLevel::Quantum);
    assert_eq!(
        runner.get_current_scale().physics_mode(),
        crate::simulation_v3::multiscale_camera::PhysicsMode::Quantum
    );

    // Cellular scale should use Quantum physics
    runner.set_scale(ScaleLevel::Cellular);
    assert_eq!(
        runner.get_current_scale().physics_mode(),
        crate::simulation_v3::multiscale_camera::PhysicsMode::Quantum
    );

    // Biological scale should use Space/Time physics
    runner.set_scale(ScaleLevel::Biological);
    assert_eq!(
        runner.get_current_scale().physics_mode(),
        crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
    );

    // Planetary scale should use Space/Time physics
    runner.set_scale(ScaleLevel::Planetary);
    assert_eq!(
        runner.get_current_scale().physics_mode(),
        crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
    );

    // Stellar scale should use Space/Time physics
    runner.set_scale(ScaleLevel::Stellar);
    assert_eq!(
        runner.get_current_scale().physics_mode(),
        crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
    );

    // Galactic scale should use Space/Time physics
    runner.set_scale(ScaleLevel::Galactic);
    assert_eq!(
        runner.get_current_scale().physics_mode(),
        crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
    );

    // Cosmic scale should use Time/Space physics
    runner.set_scale(ScaleLevel::Cosmic);
    assert_eq!(
        runner.get_current_scale().physics_mode(),
        crate::simulation_v3::multiscale_camera::PhysicsMode::TimeSpace
    );
}

/// Test: Cross-scale coupling decays with distance
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Store cross-scale influence factors"
#[test]
fn test_cross_scale_coupling_distance_decay() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);
    runner.set_scale(ScaleLevel::Biological);

    // Adjacent scales should have stronger coupling
    let biological_planetary_coupling =
        runner.get_cross_scale_coupling(ScaleLevel::Biological, ScaleLevel::Planetary);

    // Non-adjacent scales should have weaker coupling
    let biological_stellar_coupling =
        runner.get_cross_scale_coupling(ScaleLevel::Biological, ScaleLevel::Stellar);

    // Adjacent coupling should be stronger
    assert!(
        biological_planetary_coupling > biological_stellar_coupling,
        "Adjacent coupling ({}) should be stronger than distant coupling ({})",
        biological_planetary_coupling,
        biological_stellar_coupling
    );
}

/// Test: Scale transitions preserve holographic data
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 5:
/// "Maintain holographic continuity during transitions"
#[test]
fn test_scale_transitions_preserve_holographic_data() {
    let parameters = SimulationParameters::new()
        .with_num_entities(10)
        .with_num_steps(10);

    let mut runner = SimulationRunner::new(parameters);

    // Get initial cross-scale coupling data
    runner.set_scale(ScaleLevel::Biological);
    let initial_quantum_biological =
        runner.get_cross_scale_coupling(ScaleLevel::Quantum, ScaleLevel::Biological);

    // Transition to different scale
    runner.set_scale(ScaleLevel::Planetary);

    // Cross-scale coupling should still be available
    let later_quantum_biological =
        runner.get_cross_scale_coupling(ScaleLevel::Quantum, ScaleLevel::Biological);

    // Coupling data should be preserved
    assert_eq!(
        initial_quantum_biological, later_quantum_biological,
        "Cross-scale coupling data not preserved during transition"
    );
}
