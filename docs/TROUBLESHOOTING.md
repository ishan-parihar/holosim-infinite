# Troubleshooting Guide

**Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator

---

## Table of Contents

1. [Common Issues](#common-issues)
2. [Compilation Errors](#compilation-errors)
3. [Runtime Errors](#runtime-errors)
4. [Performance Issues](#performance-issues)
5. [Validation Failures](#validation-failures)
6. [Configuration Issues](#configuration-issues)
7. [Integration Issues](#integration-issues)
8. [Memory Issues](#memory-issues)
9. [Testing Issues](#testing-issues)
10. [Documentation Issues](#documentation-issues)

---

## Common Issues

### Issue: "cannot find function `initialize_global_physics_system`"

**Symptom**: Compilation error when using physics derivation functions.

**Cause**: The function `initialize_global_physics_system` does not exist in the physics engine module.

**Solution**:
```rust
// Remove this import:
// use physics_engine::initialize_global_physics_system;

// Instead, create your physics engine directly:
let physics_engine = PhysicsEngine::new();
```

### Issue: "duplicate definitions found" for functions

**Symptom**: Compilation error about duplicate function definitions.

**Cause**: Functions are defined multiple times in the same module.

**Solution**:
- Check for duplicate function definitions in your module
- Remove or rename duplicate functions
- Use `#[allow(dead_code)]` if you need to keep both

### Issue: "import `Position` from `interference_pattern`"

**Symptom**: Compilation error about Position import.

**Cause**: Position should be imported from `holographic` module, not `interference_pattern`.

**Solution**:
```rust
// Correct import:
use holonic_realms::holographic::Position;

// Incorrect import:
// use holonic_realms::holographic::interference_pattern::Position;
```

### Issue: "expected `ParticleType`, found `Option<_>`"

**Symptom**: Type mismatch error for particle type.

**Cause**: Particle type should be `ParticleType`, not `Option<ParticleType>`.

**Solution**:
```rust
// Correct:
particle_type: ParticleType::Electron,

// Incorrect:
particle_type: None,
```

### Issue: "ComplexArchetype with id/vector fields"

**Symptom**: Compilation error about ComplexArchetype fields.

**Cause**: ComplexArchetype uses `amplitude` and `phase` fields, not `id` and `vector`.

**Solution**:
```rust
// Correct:
ComplexArchetype {
    amplitude: 1.0,
    phase: 0.0,
}

// Incorrect:
// ComplexArchetype {
//     id: "archetype_1".to_string(),
//     vector: ComplexVector::new(1.0, 0.0),
// }
```

---

## Compilation Errors

### Error: "cannot find type `ArchetypicalMindSystem` in module `holographic_properties`"

**Cause**: The module `archetypical_mind` does not exist.

**Solution**:
```rust
// Create a placeholder or remove the import:
// pub type ArchetypicalMindSystem = (); // Placeholder
```

### Error: "no method named `set_aperture_size` found for struct `HolographicField`"

**Cause**: The HolographicField field is not mutable.

**Solution**:
```rust
// Make the field mutable:
let mut field = HolographicField::new(...);
field.set_aperture_size(1.0);
```

### Error: "expected struct `ComplexVector`, found struct `ComplexArchetype`"

**Cause**: ComplexArchetype needs to be converted to ComplexVector.

**Solution**:
```rust
// Convert ComplexArchetype to ComplexVector:
let archetype = ComplexArchetype::new(1.0, 0.0);
let vector = archetype.to_vector();
```

### Error: "cannot borrow `*field` as mutable, as it is behind a shared reference"

**Cause**: The field is passed by reference, but you need mutable access.

**Solution**:
```rust
// Pass mutable reference:
fn modify_field(field: &mut HolographicField) {
    field.set_aperture_size(1.0);
}
```

---

## Runtime Errors

### Error: "Panicked at 'index out of bounds: the len is 22 but the index is 22'"

**Cause**: Trying to access archetype index 22 when valid range is 0-21.

**Solution**:
```rust
// Correct range:
for i in 0..22 {
    // Access archetype_encoding[i]
}

// Incorrect range:
// for i in 0..=22 {
//     // This will panic at i=22
// }
```

### Error: "attempt to divide by zero"

**Cause**: Division by zero in calculations.

**Solution**:
```rust
// Add a check:
if denominator != 0.0 {
    let result = numerator / denominator;
} else {
    // Handle the zero case
}
```

### Error: "Resonance calculation failed"

**Cause**: Invalid input parameters for resonance calculation.

**Solution**:
```rust
// Validate inputs:
if entity1.archetype_encoding.len() == 22 &&
   entity2.archetype_encoding.len() == 22 {
    let resonance = calculate_resonance(&entity1, &entity2);
} else {
    eprintln!("Invalid archetype encoding length");
}
```

---

## Performance Issues

### Issue: "Configuration discovery is slow"

**Symptom**: Configuration discovery takes too long.

**Solutions**:
1. **Reduce spatial frequency range**:
   ```rust
   // Instead of full range, use narrower range
   let field = HolographicField::with_default_spatial_frequency(
       InvolutionLayer::Yellow // More focused search
   );
   ```

2. **Increase resonance threshold**:
   ```rust
   // Filter out low-resonance configurations
   let engine = ConfigurationDiscoveryEngine::new(field, 0.9); // Higher threshold
   ```

3. **Batch operations**:
   ```rust
   // Process multiple configurations at once
   let configs = engine.discover_configurations_batch(&archetypes, 100);
   ```

4. **Use caching**:
   ```rust
   // Cache interference patterns
   let mut pattern_cache = HashMap::new();
   ```

### Issue: "Memory usage is high"

**Symptom**: Simulation uses too much memory.

**Solutions**:
1. **Use memory pools**:
   ```rust
   let pool = MemoryPool::new(1000);
   let allocation = pool.allocate();
   ```

2. **Reduce holographic memory dimension**:
   ```rust
   // Use smaller dimension for hypervectors
   let memory = HolographicMemory::new(500); // Instead of 1000
   ```

3. **Limit exploration scope**:
   ```rust
   // Limit number of variants to explore
   let variants = variants.into_iter().take(10).collect();
   ```

4. **Use efficient data structures**:
   ```rust
   // Use Vec instead of HashMap when possible
   let positions: Vec<Position> = vec![...];
   ```

### Issue: "Oscillator synchronization is slow"

**Symptom**: Oscillator network synchronization takes too long.

**Solutions**:
1. **Reduce coupling strength**:
   ```rust
   let mut network = OscillatorNetwork::new(0.3); // Lower coupling
   ```

2. **Increase time step**:
   ```rust
   let sync_state = network.synchronize(0.2); // Larger time step
   ```

3. **Reduce number of oscillators**:
   ```rust
   // Only use essential oscillators
   for i in 0..10 { // Instead of 0..22
       network.add_oscillator(oscillator);
   }
   ```

---

## Validation Failures

### Issue: "Holographic completeness test failed"

**Symptom**: Validation test reports holographic completeness failure.

**Solutions**:
1. **Check archetype encoding**:
   ```rust
   // Ensure all 22 archetypes are present
   assert_eq!(entity.archetype_encoding.len(), 22);
   ```

2. **Validate amplitude and phase**:
   ```rust
   for archetype in &entity.archetype_encoding {
       assert!(archetype.amplitude >= 0.0);
       assert!(archetype.phase >= 0.0 && archetype.phase <= 2.0 * std::f64::consts::PI);
   }
   ```

3. **Check holographic reference**:
   ```rust
   // Verify holographic reference consistency
   let is_consistent = verify_holographic_ref_consistency(&holographic_ref);
   assert!(is_consistent);
   ```

### Issue: "Phase coherence test failed"

**Symptom**: Validation test reports phase coherence failure.

**Solutions**:
1. **Synchronize oscillators**:
   ```rust
   let mut network = OscillatorNetwork::new(0.5);
   for _ in 0..100 {
       network.synchronize(0.1);
   }
   ```

2. **Check coupling strength**:
   ```rust
   // Increase coupling for better synchronization
   let mut network = OscillatorNetwork::new(0.8);
   ```

3. **Verify phase alignment**:
   ```rust
   // Check if phases are aligned
   let coherence = network.calculate_phase_coherence();
   assert!(coherence > 0.8);
   ```

### Issue: "Physics recovery test failed"

**Symptom**: Validation test reports physics recovery failure.

**Solutions**:
1. **Check configuration discovery**:
   ```rust
   // Ensure configurations are discovered correctly
   let configs = engine.discover_configurations(&archetypes);
   assert!(!configs.is_empty());
   ```

2. **Verify resonance calculation**:
   ```rust
   // Check resonance calculation
   let resonance = calculate_resonance(&entity1, &entity2);
   assert!(resonance >= 0.0 && resonance <= 1.0);
   ```

3. **Validate physics derivation**:
   ```rust
   // Check physics derivation functions
   let mass = derive_mass_from_archetypes(&archetypes);
   assert!(mass > 0.0);
   ```

---

## Configuration Issues

### Issue: "Invalid spatial frequency"

**Symptom**: Error about invalid spatial frequency.

**Solution**:
```rust
// Use default spatial frequency for layer
let field = HolographicField::with_default_spatial_frequency(
    InvolutionLayer::Yellow
);

// Or use valid range (100.0 to 10000.0)
let field = HolographicField::new(
    1.0,
    1000.0, // Valid spatial frequency
    InvolutionLayer::Yellow
);
```

### Issue: "Invalid aperture size"

**Symptom**: Error about invalid aperture size.

**Solution**:
```rust
// Use default aperture size for layer
let field = HolographicField::with_default_aperture(
    InvolutionLayer::Yellow
);

// Or use valid range (0.1 to 10.0)
let field = HolographicField::new(
    1.0, // Valid aperture size
    1000.0,
    InvolutionLayer::Yellow
);
```

### Issue: "Invalid layer"

**Symptom**: Error about invalid involution layer.

**Solution**:
```rust
// Use valid layer enum
let layer = InvolutionLayer::Violet; // Valid
// let layer = InvolutionLayer::Invalid; // Invalid

// Get layer info
let layer_info = field.get_layer_info();
println!("Layer: {:?}", layer_info.layer);
```

---

## Integration Issues

### Issue: "Dual physics system not working"

**Symptom**: Dual physics system returns wrong results.

**Solutions**:
1. **Check physics mode**:
   ```rust
   let mut system = DualPhysicsSystem::new();
   system.set_mode(PhysicsMode::Holographic);
   assert_eq!(system.get_mode(), PhysicsMode::Holographic);
   ```

2. **Verify engines**:
   ```rust
   // Check both engines are initialized
   let comparison = system.compare_modes();
   assert!(comparison.hardcoded_results.is_some());
   assert!(comparison.holographic_results.is_some());
   ```

3. **Test with known values**:
   ```rust
   // Test with known particle properties
   let particle = Particle::new("test".to_string(), ParticleType::Electron);
   let energy = system.calculate_energy(&particle);
   assert!(energy > 0.0);
   ```

### Issue: "Exploration system not discovering configurations"

**Symptom**: Exploration system returns empty results.

**Solutions**:
1. **Check variant parameters**:
   ```rust
   // Ensure variant parameters are reasonable
   let variant = PhysicalLawsVariant {
       gravity_modifier: 1.0, // Valid range: 0.5 to 2.0
       electromagnetic_force_modifier: 1.0,
       // ...
   };
   ```

2. **Verify discovery database**:
   ```rust
   // Check database is initialized
   let explorer = UniverseExplorer::with_default_architecture();
   let db = explorer.get_discovery_database();
   assert!(!db.discoveries.is_empty());
   ```

3. **Adjust thresholds**:
   ```rust
   // Lower thresholds for more discoveries
   let engine = ConfigurationDiscoveryEngine::new(field, 0.7); // Lower threshold
   ```

---

## Memory Issues

### Issue: "Stack overflow"

**Symptom**: Program crashes with stack overflow.

**Solutions**:
1. **Use heap allocation**:
   ```rust
   // Instead of large stack arrays
   let archetypes: [ComplexArchetype; 22] = [...];

   // Use Vec for large data
   let archetypes: Vec<ComplexArchetype> = vec![...];
   ```

2. **Reduce recursion depth**:
   ```rust
   // Use iterative approach instead of recursion
   fn calculate_iterative(&self) -> Float {
       let mut result = 0.0;
       for i in 0..self.max_iterations {
           result += self.step(i);
       }
       result
   }
   ```

3. **Increase stack size**:
   ```bash
   # Run with increased stack size
   RUST_MIN_STACK=8388608 cargo run
   ```

### Issue: "Memory leak"

**Symptom**: Memory usage keeps increasing.

**Solutions**:
1. **Check for circular references**:
   ```rust
   // Avoid circular references in structs
   // Use Weak references if needed
   use std::rc::Rc;
   use std::rc::Weak;
   ```

2. **Use RAII patterns**:
   ```rust
   // Ensure resources are cleaned up
   {
       let resource = Resource::new();
       // Resource is automatically cleaned up here
   }
   ```

3. **Profile memory usage**:
   ```bash
   # Use memory profiler
   cargo install valgrind
   valgrind --leak-check=full ./target/debug/holonic_realms
   ```

---

## Testing Issues

### Issue: "Tests fail intermittently"

**Symptom**: Tests sometimes pass, sometimes fail.

**Solutions**:
1. **Add deterministic random seeds**:
   ```rust
   // Use fixed seed for reproducible tests
   let mut rng = StdRng::seed_from_u64(42);
   ```

2. **Increase test timeouts**:
   ```rust
   #[test]
   #[timeout(10000)] // 10 second timeout
   fn test_slow_operation() {
       // ...
   }
   ```

3. **Isolate tests**:
   ```rust
   // Run tests in isolation
   #[test]
   fn test_isolated() {
       // Setup
       let state = setup_test_state();

       // Test
       let result = perform_test(&state);

       // Cleanup
       cleanup_test_state(state);
   }
   ```

### Issue: "Mock not working correctly"

**Symptom**: Mock objects don't behave as expected.

**Solutions**:
1. **Use trait objects**:
   ```rust
   trait PhysicsEngine {
       fn calculate_force(&self, m1: Float, m2: Float, distance: Float) -> Float;
   }

   struct MockPhysicsEngine {
       force_value: Float,
   }

   impl PhysicsEngine for MockPhysicsEngine {
       fn calculate_force(&self, _m1: Float, _m2: Float, _distance: Float) -> Float {
           self.force_value
       }
   }
   ```

2. **Use dependency injection**:
   ```rust
   fn calculate_physics(engine: &dyn PhysicsEngine) -> Float {
       engine.calculate_force(1.0, 2.0, 1.0)
   }
   ```

---

## Documentation Issues

### Issue: "Rustdoc warnings"

**Symptom**: Rustdoc generates warnings.

**Solutions**:
1. **Add missing documentation**:
   ```rust
   /// Calculates the resonance between two entities.
   ///
   /// # Arguments
   ///
   /// * `entity1` - The first entity
   /// * `entity2` - The second entity
   ///
   /// # Returns
   ///
   /// The resonance score (0.0 to 1.0)
   pub fn calculate_resonance(entity1: &HolographicEntity, entity2: &HolographicEntity) -> Float {
       // ...
   }
   ```

2. **Fix broken links**:
   ```rust
   /// See [`ComplexVector`] for more information.
   ///
   /// [`ComplexVector`]: struct.ComplexVector.html
   ```

3. **Add examples**:
   ```rust
   /// # Examples
   ///
   /// ```
   /// use holonic_realms::holographic::ComplexVector;
   ///
   /// let cv = ComplexVector::new(1.0, 0.0);
   /// assert_eq!(cv.amplitude, 1.0);
   /// ```
   ```

---

## Getting Help

If you're still experiencing issues after trying these solutions:

1. **Check the documentation**:
   - [API Documentation](API.md)
   - [Getting Started Guide](GETTING_STARTED.md)
   - [Tutorial](TUTORIAL.md)

2. **Review the examples**:
   - [Usage Examples](EXAMPLES.md)

3. **Check the architecture**:
   - [Architecture Diagrams](ARCHITECTURE.md)

4. **Run validation tests**:
   ```bash
   cargo test --lib validation
   ```

5. **Check the logs**:
   ```bash
   RUST_LOG=debug cargo run
   ```

6. **Report the issue**:
   - Include error messages
   - Include code snippets
   - Include system information
   - Include steps to reproduce

---

## Best Practices

### 1. Always Initialize Before Use

```rust
// Always initialize holographic field
let field = HolographicField::new(
    1.0,
    1000.0,
    InvolutionLayer::Yellow
);

// Always initialize physics system
let mut physics = DualPhysicsSystem::new();
physics.set_mode(PhysicsMode::Holographic);
```

### 2. Validate Inputs

```rust
// Always validate inputs
fn calculate_resonance(entity1: &HolographicEntity, entity2: &HolographicEntity) -> Result<Float, String> {
    if entity1.archetype_encoding.len() != 22 {
        return Err("Invalid archetype encoding length".to_string());
    }
    // ...
    Ok(resonance)
}
```

### 3. Use Error Handling

```rust
// Use Result for fallible operations
fn discover_configurations(&self, archetypes: &[ComplexArchetype]) -> Result<Vec<StableConfiguration>, String> {
    if archetypes.is_empty() {
        return Err("No archetypes provided".to_string());
    }
    // ...
    Ok(configs)
}
```

### 4. Add Logging

```rust
use log::{info, warn, error};

fn process_entity(&mut self, entity: &HolographicEntity) {
    info!("Processing entity: {}", entity.id);
    if let Err(e) = self.calculate_resonance(entity) {
        error!("Resonance calculation failed: {}", e);
    }
}
```

### 5. Write Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_calculation() {
        let entity1 = create_test_entity();
        let entity2 = create_test_entity();
        let resonance = calculate_resonance(&entity1, &entity2);
        assert!(resonance >= 0.0 && resonance <= 1.0);
    }
}
```

---

## See Also

- [API Documentation](API.md)
- [Getting Started Guide](GETTING_STARTED.md)
- [Tutorial](TUTORIAL.md)
- [Reference Manual](REFERENCE.md)
- [FAQ](FAQ.md)
- [Usage Examples](EXAMPLES.md)
- [Architecture Diagrams](ARCHITECTURE.md)

---

**Troubleshooting Guide Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator