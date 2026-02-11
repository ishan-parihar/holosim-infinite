# Source Code Refactor Plan

**Created**: 2026-02-08
**Status**: Ready for implementation
**Total Errors**: 703 compilation errors in test code
**Library Status**: ✅ Compiles successfully (0 errors, 92 warnings)

---

## Executive Summary

The library code (`cargo check --lib`) compiles successfully with only warnings. However, when compiling tests (`cargo test --no-run`), there are **703 compilation errors**. These errors are primarily due to:

1. **Missing imports** - Types and modules not properly imported
2. **API changes** - Functions with different signatures than expected
3. **Type mismatches** - Using wrong types or type variants
4. **Missing fields** - Struct initializations missing required fields
5. **Deprecated/removed functionality** - Using old APIs that no longer exist

**Good News**: The source code is sound. The errors are in test code using outdated APIs.

---

## Error Analysis

### Error Distribution by Code

| Error Code | Count | Description | Priority |
|------------|-------|-------------|----------|
| E0433 | 135 | Failed to resolve: undeclared type | **HIGH** |
| E0308 | 134 | Type mismatch | **HIGH** |
| E0609 | 104 | No field access | **HIGH** |
| E0061 | 97 | Function argument count mismatch | **HIGH** |
| E0599 | 95 | Method not found | **HIGH** |
| E0560 | 57 | Struct field error | **MEDIUM** |
| E0618 | 17 | Unexpected function name | **MEDIUM** |
| E0063 | 14 | Missing struct fields | **MEDIUM** |
| E0046 | 9 | Not found in this scope | **MEDIUM** |
| E0615 | 8 | Field access on wrong type | **LOW** |
| E0382 | 7 | Use of moved value | **LOW** |
| E0432 | 5 | Unresolved import | **MEDIUM** |
| E0277 | 5 | Trait not implemented | **LOW** |
| E0425 | 4 | Cannot find value | **LOW** |
| E0422 | 3 | Cannot create type | **LOW** |
| E0369 | 2 | Binary operation not implemented | **LOW** |
| E0282 | 2 | Type annotations needed | **LOW** |
| E0034 | 2 | Multiple applicable items | **LOW** |
| E0716 | 1 | Temporary value dropped while borrowed | **MEDIUM** |
| E0624 | 1 | Unexpected tuple pattern | **LOW** |
| E0596 | 1 | Cannot borrow as mutable | **LOW** |
| E0053 | 1 | Method not applicable | **LOW** |
| E0050 | 1 | Method signature mismatch | **LOW** |

---

## Top Issues by Impact

### 1. Missing Imports (E0433, E0432) - 140 errors

#### Problem
Tests are using types without importing them or importing from wrong paths.

#### Examples

**E0433 - Undeclared Types**:
```rust
// src/entity.rs:1475
let soul_stream = SoulStream::new();  // ❌ E0433: undeclared type

// src/entity.rs:1495
let seed = Arc::new(HolographicSeed::new_from_source());  // ❌ E0433: undeclared types
```

**E0432 - Unresolved Imports**:
```rust
// src/audit_runner.rs:4
use holonic_realms::simulation_audit::SimulationAudit;  // ❌ E0432: module doesn't exist

// tests/growth_principle_tests.rs:6
use holonic_realms::growth_principle::{...};  // ❌ E0432: module doesn't exist

// src/holographic_properties.rs:39
use crate::entities::{EntityEmergence, PotentialEntity};  // ❌ E0432: module doesn't exist
```

#### Root Causes

1. **`SoulStream`** - Exists in `src/memory/soul_stream.rs` but not imported
2. **`Arc`** - Standard library type, needs `use std::sync::Arc`
3. **`HolographicSeed`** - Exists in `src/holographic_seed.rs` but not imported
4. **`simulation_audit`** - Module is commented out in `src/lib.rs`
5. **`growth_principle`** - Module doesn't exist (old tests)
6. **`entities`** - Module doesn't exist (old API)
7. **`Position`** - Type doesn't exist in `crate::types`

#### Solutions

```rust
// ✅ Add missing imports
use std::sync::Arc;
use crate::memory::SoulStream;
use crate::holographic_seed::HolographicSeed;

// ✅ Remove/comment broken imports
// use holonic_realms::simulation_audit::SimulationAudit;  // Module disabled
// use holonic_realms::growth_principle::{...};  // Module doesn't exist
// use crate::entities::{...};  // Module doesn't exist
```

---

### 2. Type Mismatches (E0308) - 134 errors

#### Problem
Tests are using types that don't match expected function signatures.

#### Examples

**ArchetypicalMind vs ArchetypicalMindBlueprint**:
```rust
// src/entity_layer7/mod.rs:124
HolographicBlueprint::from_spectrum_configuration(
    &spectrum_config,
    &archetypical_mind  // ❌ E0308: expected ArchetypicalMindBlueprint, found ArchetypicalMind
);
```

**HolographicPattern::new() missing arguments**:
```rust
// src/spectrum/yellow_realm.rs:706
HolographicPattern::new("pattern1".to_string(), 0.5, 0.1)  // ❌ E0308: missing [f64; 3] argument
```

#### Solutions

```rust
// ✅ Convert ArchetypicalMind to blueprint
let blueprint = ArchetypicalMindBlueprint::from(&archetypical_mind);
HolographicBlueprint::from_spectrum_configuration(&spectrum_config, &blueprint);

// ✅ Add missing argument to HolographicPattern::new()
HolographicPattern::new("pattern1".to_string(), [0.5, 0.5, 0.5], 0.1)
```

---

### 3. Missing Field Access (E0609) - 104 errors

#### Problem
Tests are accessing fields that don't exist on types.

#### Example

```rust
// src/spectrum/veil.rs:497
assert_eq!(access.mannyness_access, 0.0);  // ❌ E0609: no field `mannyness_access` on SpectrumAccess
```

#### Root Cause

The `SpectrumAccess` struct has been refactored and no longer has a `mannyness_access` field.

#### Solution

Check the actual field names in `SpectrumAccess` and update the tests accordingly.

---

### 4. Function Argument Mismatches (E0061) - 97 errors

#### Problem
Tests are calling functions with wrong number of arguments.

#### Examples

**ArchetypicalMind::new()**:
```rust
// src/entity_layer7/dna_encoding.rs:637
let archetypical_mind = ArchetypicalMind::new(ArchetypicalSystemType::TwentyTwoArchetypes);
// ❌ E0061: takes 2 arguments but 1 was supplied
```

**Spectrum Realm Constructors**:
```rust
// src/entity_layer7/layer7.rs:3326
let yellow = YellowRealm::new();  // ❌ E0061: takes 1 argument but 0 supplied
let orange = OrangeRealm::new();  // ❌ E0061: takes 1 argument but 0 supplied
let red = RedRealm::new();        // ❌ E0061: takes 1 argument but 0 supplied
```

**SubSubLogos::new()**:
```rust
// src/entity_layer7/mod.rs:70
let entity = SubSubLogos::new(
    // ... 11 arguments ...
);  // ❌ E0061: takes 13 arguments but 11 supplied
```

#### Solutions

```rust
// ✅ Add missing argument to ArchetypicalMind::new()
let archetypical_mind = ArchetypicalMind::new(
    ArchetypicalSystemType::TwentyTwoArchetypes,
    LambdaMeasurement::default()  // Add this
);

// ✅ Add argument to spectrum realm constructors
let yellow = YellowRealm::new(1.0);  // Add spectrum configuration value
let orange = OrangeRealm::new(1.0);
let red = RedRealm::new(1.0);

// ✅ Add missing arguments to SubSubLogos::new()
let entity = SubSubLogos::new(
    // ... existing 11 arguments ...,
    missing_arg_12,
    missing_arg_13
);
```

---

### 5. Method Not Found (E0599) - 95 errors

#### Problem
Tests are calling methods that don't exist on types.

#### Examples

**is_some() on PolarityChoice**:
```rust
// src/foundation/mod.rs:149
assert!(choice.is_some());  // ❌ E0599: no method `is_some` on PolarityChoice
```

**advance() on DensityOctave**:
```rust
// src/evolution_density_octave/density_octave.rs:1277
octave.advance();  // ❌ E0599: no method `advance` on DensityOctave
```

**default() on IntelligentInfinity**:
```rust
// src/evolution_density_octave/integration.rs:322
indigo_realm: IntelligentInfinity::default(),  // ❌ E0599: no function `default` on IntelligentInfinity
```

#### Solutions

```rust
// ✅ PolarityChoice is an enum, not Option
match choice {
    PolarityChoice::Choice(_) => { /* ... */ },
    PolarityChoice::Unchosen => { /* ... */ },
}

// ✅ Check if DensityOctave has a different method name
// or implement the advance() method if needed

// ✅ Check if IntelligentInfinity has a constructor
// or implement Default trait if appropriate
```

---

### 6. Struct Field Errors (E0560) - 57 errors

#### Problem
Tests are initializing structs with incorrect field names or missing required fields.

#### Solutions

Check struct definitions and update field names/counts accordingly.

---

## Refactor Strategy

### Phase 1: Fix Missing Imports (140 errors) - HIGH PRIORITY

**Goal**: Resolve all E0432 and E0433 errors

**Approach**:
1. Search for all undeclared types (E0433)
2. Find correct import paths
3. Add missing imports at top of files
4. Remove/comment broken imports (E0432)

**Files to Fix**:
- `src/entity.rs` - Add `use std::sync::Arc`, `use crate::memory::SoulStream`, `use crate::holographic_seed::HolographicSeed`
- `src/audit_runner.rs` - Remove/comment `simulation_audit` import
- `tests/growth_principle_tests.rs` - Remove/comment `growth_principle` import
- `src/holographic_properties.rs` - Remove/comment `entities` and `environments` imports
- `src/attractors/attractor_fields.rs` - Remove `Position` from import

**Estimated Time**: 2-3 hours

---

### Phase 2: Fix Type Mismatches (134 errors) - HIGH PRIORITY

**Goal**: Resolve all E0308 errors

**Approach**:
1. Find all type mismatch errors
2. Identify the correct types
3. Add type conversions or use correct types directly

**Key Fixes**:
1. `ArchetypicalMind` → `ArchetypicalMindBlueprint`
2. `HolographicPattern::new()` - Add missing `[f64; 3]` argument
3. Other type mismatches as they appear

**Estimated Time**: 3-4 hours

---

### Phase 3: Fix Field Access Errors (104 errors) - HIGH PRIORITY

**Goal**: Resolve all E0609 errors

**Approach**:
1. Find all field access errors
2. Check struct definitions for correct field names
3. Update field names in tests

**Key Fixes**:
1. `mannyness_access` field - Check actual field name in `SpectrumAccess`

**Estimated Time**: 2-3 hours

---

### Phase 4: Fix Function Argument Errors (97 errors) - HIGH PRIORITY

**Goal**: Resolve all E0061 errors

**Approach**:
1. Find all function argument count errors
2. Check function signatures
3. Add missing arguments or remove extra arguments

**Key Fixes**:
1. `ArchetypicalMind::new()` - Add 2nd argument (LambdaMeasurement)
2. `YellowRealm::new()`, `OrangeRealm::new()`, `RedRealm::new()` - Add configuration argument
3. `SubSubLogos::new()` - Add 2 missing arguments

**Estimated Time**: 3-4 hours

---

### Phase 5: Fix Method Not Found Errors (95 errors) - HIGH PRIORITY

**Goal**: Resolve all E0599 errors

**Approach**:
1. Find all method not found errors
2. Check if methods exist with different names
3. Update method calls or implement missing methods

**Key Fixes**:
1. `PolarityChoice.is_some()` - Use pattern matching instead
2. `DensityOctave.advance()` - Find correct method or implement
3. `IntelligentInfinity::default()` - Find constructor or implement Default

**Estimated Time**: 3-4 hours

---

### Phase 6: Fix Struct Field Errors (57 errors) - MEDIUM PRIORITY

**Goal**: Resolve all E0560 errors

**Approach**:
1. Find all struct field errors
2. Check struct definitions
3. Update field names or add missing fields

**Estimated Time**: 2-3 hours

---

### Phase 7: Fix Remaining Errors (73 errors) - MEDIUM/LOW PRIORITY

**Goal**: Resolve all remaining errors (E0618, E0063, E0046, etc.)

**Approach**:
1. Fix errors by type
2. Each error type should be straightforward

**Estimated Time**: 2-3 hours

---

## Implementation Checklist

### Phase 1: Missing Imports (140 errors)
- [ ] Fix `src/entity.rs` imports
  - [ ] Add `use std::sync::Arc;`
  - [ ] Add `use crate::memory::SoulStream;`
  - [ ] Add `use crate::holographic_seed::HolographicSeed;`
- [ ] Fix `src/audit_runner.rs` imports
  - [ ] Remove/comment `simulation_audit` import
- [ ] Fix `tests/growth_principle_tests.rs` imports
  - [ ] Remove/comment `growth_principle` import
- [ ] Fix `src/holographic_properties.rs` imports
  - [ ] Remove/comment `entities` import
  - [ ] Remove/comment `environments` import
- [ ] Fix `src/attractors/attractor_fields.rs` imports
  - [ ] Remove `Position` from import
- [ ] Verify all E0432 and E0433 errors resolved

### Phase 2: Type Mismatches (134 errors)
- [ ] Fix `ArchetypicalMind` → `ArchetypicalMindBlueprint` conversions
- [ ] Fix `HolographicPattern::new()` calls
- [ ] Fix other type mismatches
- [ ] Verify all E0308 errors resolved

### Phase 3: Field Access Errors (104 errors)
- [ ] Check `SpectrumAccess` struct definition
- [ ] Update all `mannyness_access` references
- [ ] Fix other field access errors
- [ ] Verify all E0609 errors resolved

### Phase 4: Function Argument Errors (97 errors)
- [ ] Fix `ArchetypicalMind::new()` calls
- [ ] Fix `YellowRealm::new()`, `OrangeRealm::new()`, `RedRealm::new()` calls
- [ ] Fix `SubSubLogos::new()` calls
- [ ] Fix other function argument errors
- [ ] Verify all E0061 errors resolved

### Phase 5: Method Not Found Errors (95 errors)
- [ ] Fix `PolarityChoice.is_some()` calls
- [ ] Fix `DensityOctave.advance()` calls
- [ ] Fix `IntelligentInfinity::default()` calls
- [ ] Fix other method not found errors
- [ ] Verify all E0599 errors resolved

### Phase 6: Struct Field Errors (57 errors)
- [ ] Fix all struct initialization errors
- [ ] Verify all E0560 errors resolved

### Phase 7: Remaining Errors (73 errors)
- [ ] Fix E0618 errors (unexpected function names)
- [ ] Fix E0063 errors (missing struct fields)
- [ ] Fix E0046 errors (not found in scope)
- [ ] Fix E0615 errors (field access on wrong type)
- [ ] Fix E0382 errors (use of moved value)
- [ ] Fix other remaining errors
- [ ] Verify all remaining errors resolved

### Phase 8: Validation
- [ ] Run `cargo check --lib` to verify library still compiles
- [ ] Run `cargo test --no-run` to verify tests compile
- [ ] Run `cargo test --release` to verify tests pass
- [ ] Fix any remaining issues

---

## Success Criteria

1. **Library Compilation**: ✅ Already passes (0 errors, 92 warnings)
2. **Test Compilation**: ❌ Currently fails (703 errors) → **Target: 0 errors**
3. **Test Execution**: ❌ Cannot run → **Target: All tests pass**
4. **Code Quality**: Warnings should be addressed after errors are fixed

---

## Risk Assessment

### Low Risk
- Adding missing imports
- Fixing function argument counts
- Updating field names

### Medium Risk
- Type conversions (may change behavior)
- Method name changes (may affect logic)

### Mitigation Strategies
1. **Backup**: Create git commit before each phase
2. **Incremental Testing**: After each phase, run `cargo test --no-run` to verify progress
3. **Documentation**: Document all changes in this file
4. **Review**: Review changes before committing

---

## Estimated Timeline

| Phase | Errors | Estimated Time | Priority |
|-------|--------|----------------|----------|
| Phase 1 | 140 | 2-3 hours | HIGH |
| Phase 2 | 134 | 3-4 hours | HIGH |
| Phase 3 | 104 | 2-3 hours | HIGH |
| Phase 4 | 97 | 3-4 hours | HIGH |
| Phase 5 | 95 | 3-4 hours | HIGH |
| Phase 6 | 57 | 2-3 hours | MEDIUM |
| Phase 7 | 73 | 2-3 hours | MEDIUM/LOW |
| Phase 8 | - | 1-2 hours | VALIDATION |
| **Total** | **703** | **18-26 hours** | **-** |

---

## Next Steps

1. **Review this plan** with the user
2. **Get approval** to proceed
3. **Create git commit** for current state
4. **Start Phase 1**: Fix missing imports
5. **Iterate through phases** until all errors are resolved
6. **Run full test suite** to verify success

---

## Notes

- The library code is in good shape - only warnings, no errors
- All errors are in test code using outdated APIs
- This is a systematic refactor that can be done incrementally
- Each phase can be tested independently
- The refactor will not affect the library code, only test code

---

## References

- **Testing Refactor Roadmap**: `TESTING-REFACTOR-ROADMAP.md` (100% complete)
- **Architecture Documentation**: `docs/ARCHITECTURE.md`
- **Cosmological Architecture**: `01_Metaphysics/COSMOLOGICAL-ARCHITECTURE.md`
- **Build Commands**: `AGENTS.md`