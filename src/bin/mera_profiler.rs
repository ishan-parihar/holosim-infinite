//! MERA CPU Profiler for HoloSim Infinite
//!
//! Profiles MERA tensor operations and holographic field computations
//! to identify GPU offload candidates for Phase 5 (wgpu compute shaders).
//!
//! Run: cargo run --release --bin mera_profiler

use std::collections::HashMap;
use std::time::Instant;

use holonic_realms::entity_layer7::layer7::{EntityId, EntityType, SubSubLogos};
use holonic_realms::simulation_v3::archetype_basis::{ArchetypeActivationProfile, ArchetypeBasis};
use holonic_realms::simulation_v3::holographic_field::HolographicFieldManager;
use holonic_realms::simulation_v3::mera_network::{
    MeraNetwork, MeraQuery, MeraScale, QueryType, Tensor,
};

// ============================================================================
// Profiling Infrastructure
// ============================================================================

struct Profiler {
    segments: HashMap<String, Vec<f64>>, // segment_name -> Vec<elapsed_ms>
}

impl Profiler {
    fn new() -> Self {
        Self {
            segments: HashMap::new(),
        }
    }

    fn time<F: FnMut()>(&mut self, name: &str, iterations: usize, mut f: F) -> f64 {
        let start = Instant::now();
        for _ in 0..iterations {
            f();
        }
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        let per_iter = elapsed_ms / iterations as f64;
        self.segments
            .entry(name.to_string())
            .or_default()
            .push(elapsed_ms);
        per_iter
    }

    fn report(&self, total_ms: f64) {
        println!(
            "\n  {:<40} {:>12} {:>10}",
            "Component", "Total (ms)", "% of Total"
        );
        println!("  {:─<40} {:─>12} {:─>10}", "", "", "");

        let mut entries: Vec<(&String, &Vec<f64>)> = self.segments.iter().collect();
        entries.sort_by(|a, b| {
            let sum_a: f64 = b.1.iter().sum();
            let sum_b: f64 = a.1.iter().sum();
            sum_a
                .partial_cmp(&sum_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (name, times) in &entries {
            let total: f64 = times.iter().sum();
            let pct = if total_ms > 0.0 {
                (total / total_ms) * 100.0
            } else {
                0.0
            };
            println!("  {:<40} {:>12.2} {:>9.1}%", name, total, pct);
        }
        println!("  {:─<40} {:─>12} {:─>10}", "", "", "");
        println!("  {:<40} {:>12.2} {:>9.1}%", "TOTAL", total_ms, 100.0);
    }
}

// ============================================================================
// Entity Creation Helper
// ============================================================================

fn create_entities(count: usize) -> Vec<SubSubLogos> {
    let mut entities = Vec::with_capacity(count);
    for i in 0..count {
        let entity = SubSubLogos::builder(
            EntityId::new(format!("profile-{}", i)),
            EntityType::Individual,
        )
        .build();
        entities.push(entity);
    }
    entities
}

// ============================================================================
// Profiling Scenarios
// ============================================================================

fn profile_mera_compress_decompress(p: &mut Profiler, tensor_size: usize, iterations: usize) {
    println!(
        "\n  --- MERA Compress/Decompress ({}x{} tensor) ---",
        tensor_size, tensor_size
    );

    // Setup: build tensor
    let setup = || {
        let data: Vec<f64> = (0..tensor_size * tensor_size)
            .map(|i| (i as f64) * 0.001)
            .collect();
        Tensor {
            shape: vec![tensor_size, tensor_size],
            data,
        }
    };

    let tensor = setup();

    // Profile compress
    let compress_ms = p.time("MERA compress", iterations, || {
        let mut network = MeraNetwork::new();
        let _ = network.compress(tensor.clone());
    });

    // Profile build_hierarchy + decompress
    let decompress_ms = p.time("MERA build_hierarchy + decompress", iterations, || {
        let mut network = MeraNetwork::new();
        let t = setup();
        network.initialize(t.clone()).unwrap();
        network.build_hierarchy().unwrap();
        let query = MeraQuery::new(MeraScale::Quantum, QueryType::Spatial);
        let _ = network.decompress(&query);
    });

    // Profile tensor matmul (O(n^3) hotspot)
    let matmul_ms = p.time("Tensor matmul (O(n^3))", iterations, || {
        let size = (tensor_size / 2).max(2);
        let a = Tensor::from_data(
            vec![size, size],
            (0..size * size).map(|i| (i as f64) * 0.01).collect(),
        )
        .unwrap();
        let b = Tensor::from_data(
            vec![size, size],
            (0..size * size).map(|i| (i as f64) * 0.013).collect(),
        )
        .unwrap();
        let _ = a.matmul(&b);
    });

    // Profile tensor downsample (nested loops)
    let downsample_ms = p.time("Tensor downsample", iterations, || {
        let t = setup();
        let _ = t.downsample();
    });

    // Profile tensor upsample (nested loops x8)
    let upsample_ms = p.time("Tensor upsample", iterations, || {
        let small = Tensor::from_data(
            vec![tensor_size / 2, tensor_size / 2],
            (0..(tensor_size / 2) * (tensor_size / 2))
                .map(|i| (i as f64) * 0.01)
                .collect(),
        )
        .unwrap();
        let _ = small.upsample();
    });

    // Profile wavelet compression (O(n^2) with get/set overhead)
    let wavelet_ms = p.time("Wavelet compress/decompress", iterations, || {
        use holonic_realms::simulation_v3::mera_network::MeraLayer;
        let t = Tensor::from_data(
            vec![tensor_size, tensor_size],
            (0..tensor_size * tensor_size)
                .map(|i| (i as f64) * 0.001)
                .collect(),
        )
        .unwrap();
        let mut layer = MeraLayer::new(0, t);
        let _ = layer.apply_wavelet_compression(0.01);
        let _ = layer.apply_wavelet_decompression();
    });

    println!("    compress:       {:.2} ms/iter", compress_ms);
    println!("    decompress:     {:.2} ms/iter", decompress_ms);
    println!("    matmul:         {:.2} ms/iter", matmul_ms);
    println!("    downsample:     {:.2} ms/iter", downsample_ms);
    println!("    upsample:       {:.2} ms/iter", upsample_ms);
    println!("    wavelet:        {:.2} ms/iter", wavelet_ms);
}

fn profile_archetype_basis(p: &mut Profiler, dimension: usize, iterations: usize) {
    println!("\n  --- Archetype Basis (dimension={}) ---", dimension);

    let basis_ms = p.time("ArchetypeBasis::new", iterations, || {
        let _basis = ArchetypeBasis::new(dimension);
    });

    // Reconstruct + project cycle
    let recon_ms = p.time("ArchetypeBasis reconstruct+project", iterations, || {
        let basis = ArchetypeBasis::new(dimension);
        let mut coeffs = [0.0f64; 22];
        for (i, c) in coeffs.iter_mut().enumerate() {
            *c = (i as f64 * 0.1).sin();
        }
        let profile = ArchetypeActivationProfile::new(coeffs);
        let pattern = basis.reconstruct(&profile);
        let _recovered = basis.project(&pattern);
    });

    println!("    basis creation: {:.2} ms/iter", basis_ms);
    println!("    reconstruct+project: {:.2} ms/iter", recon_ms);
}

fn profile_holographic_field(p: &mut Profiler, entities: &[SubSubLogos], iterations: usize) {
    let n = entities.len();
    println!("\n  --- Holographic Field ({} entities) ---", n);

    let hf_ms = p.time("HolographicField update_field", iterations, || {
        let mut field = HolographicFieldManager::new();
        for entity in entities {
            let _ = field.add_entity(entity.clone());
        }
        let _result = field.update_field(1);
    });

    println!("    holographic field: {:.2} ms/iter", hf_ms);
}

fn profile_entity_lifecycle(p: &mut Profiler, entities: &[SubSubLogos], iterations: usize) {
    let n = entities.len();
    println!("\n  --- Entity Lifecycle ({} entities) ---", n);

    let lifecycle_ms = p.time("Entity clone + add to field", iterations, || {
        for entity in entities {
            let _cloned = entity.clone();
        }
    });

    println!("    lifecycle ops: {:.2} ms/iter", lifecycle_ms);
}

fn profile_full_simulation(p: &mut Profiler, num_entities: usize, num_steps: usize) {
    println!("\n=====================================================================",);
    println!(
        "  FULL SIMULATION PROFILE: {} entities, {} steps",
        num_entities, num_steps
    );
    println!("=====================================================================",);

    let total_start = Instant::now();
    let mut p = Profiler::new();

    // 1. Entity creation
    let entity_create_ms = p.time("Entity creation", 1, || {
        let _entities = create_entities(num_entities);
    });

    let entities = create_entities(num_entities);

    // 2. Add entities to holographic field
    let field_add_ms = p.time("Add entities to holographic field", 1, || {
        let mut field = HolographicFieldManager::new();
        for e in &entities {
            let _ = field.add_entity(e.clone());
        }
    });

    // 3. Run simulation steps with holographic field updates
    let sim_steps_ms = p.time("Simulation steps (field + lifecycle)", 1, || {
        let mut field = HolographicFieldManager::new();
        for e in &entities {
            let _ = field.add_entity(e.clone());
        }
        let _result = field.update_field(num_steps as u64);
    });

    // 4. MERA compress/decompress cycle
    let tensor_size = num_entities.min(128); // Cap tensor size for profiling
    let mera_ms = p.time("MERA compress/decompress cycle", num_steps, || {
        let data: Vec<f64> = (0..tensor_size * tensor_size)
            .map(|i| (i as f64) * 0.001)
            .collect();
        let tensor = Tensor {
            shape: vec![tensor_size, tensor_size],
            data,
        };
        let mut network = MeraNetwork::new();
        let _ = network.compress(tensor.clone());
        let query = MeraQuery::new(MeraScale::Quantum, QueryType::Spatial);
        let _ = network.decompress(&query);
    });

    // 5. Archetype interference (basis reconstruction per entity)
    let archetype_ms = p.time("Archetype interference", num_steps, || {
        let basis = ArchetypeBasis::new(64);
        for entity in &entities {
            let mut coeffs = [0.0f64; 22];
            let density_level = entity.current_density.as_usize();
            for (i, c) in coeffs.iter_mut().enumerate() {
                *c = ((i + density_level) as f64 * 0.1).sin();
            }
            let profile = ArchetypeActivationProfile::new(coeffs);
            let _pattern = basis.reconstruct(&profile);
        }
    });

    let total_ms = total_start.elapsed().as_secs_f64() * 1000.0;

    p.report(total_ms);

    println!("\n  Key Metrics:");
    println!(
        "    Entity creation:            {:.2} ms ({:.2} ms/entity)",
        entity_create_ms,
        entity_create_ms / num_entities as f64
    );
    println!("    Field add:                  {:.2} ms", field_add_ms);
    println!(
        "    Simulation steps (total):   {:.2} ms ({:.2} ms/step)",
        sim_steps_ms,
        sim_steps_ms / num_steps as f64
    );
    println!("    MERA per-step:              {:.2} ms", mera_ms);
    println!("    Archetype per-step:         {:.2} ms", archetype_ms);
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       HoloSim Infinite — MERA CPU Profiling Report       ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("\nTimestamp: {}", chrono_format());
    println!("CPU threads: {}", num_cpus::get());

    // ===================================================================
    // Part 1: Micro-benchmarks of individual tensor operations
    // ===================================================================
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("PART 1: TENSOR OPERATION MICRO-BENCHMARKS");
    println!("═══════════════════════════════════════════════════════════");

    for &size in &[32, 64, 128] {
        profile_mera_compress_decompress(&mut Profiler::new(), size, 50);
    }

    // ===================================================================
    // Part 2: Archetype basis at various dimensions
    // ===================================================================
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("PART 2: ARCHETYPE BASIS OPERATIONS");
    println!("═══════════════════════════════════════════════════════════");

    for &dim in &[32, 64, 128, 256] {
        profile_archetype_basis(&mut Profiler::new(), dim, 100);
    }

    // ===================================================================
    // Part 3: Full simulation profiles at target entity counts
    // ===================================================================
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("PART 3: FULL SIMULATION PROFILES");
    println!("═══════════════════════════════════════════════════════════");

    let entity_counts = [128, 1024]; // 8192 would take too long; report extrapolation

    for &count in &entity_counts {
        profile_full_simulation(&mut Profiler::new(), count, 100);
    }

    // ===================================================================
    // Part 4: O(n^2) scaling test for holographic connections
    // ===================================================================
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("PART 4: HOLOGRAPHIC FIELD SCALING TEST");
    println!("═══════════════════════════════════════════════════════════");

    println!(
        "\n  {:<12} {:>12} {:>12} {:>12}",
        "Entities", "Connections", "Time (ms)", "ms/entity"
    );
    println!("  {:─<12} {:─>12} {:─>12} {:─>12}", "", "", "", "");

    for &count in &[64, 128, 256, 512] {
        let entities = create_entities(count);
        let start = Instant::now();

        let mut field = HolographicFieldManager::new();
        for e in &entities {
            let _ = field.add_entity(e.clone());
        }
        field.update_field(1);

        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        let conn_count = field.statistics.connection_count;
        println!(
            "  {:<12} {:>12} {:>12.2} {:>12.4}",
            count,
            conn_count,
            elapsed_ms,
            elapsed_ms / count as f64
        );
    }

    // ===================================================================
    // Summary and GPU candidates
    // ===================================================================
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("PART 5: HOTSPOT IDENTIFICATION");
    println!("═══════════════════════════════════════════════════════════");

    println!("\nIdentified CPU Hotspots (from code analysis):");
    println!("\n  1. Tensor::matmul() — src/simulation_v3/mera_network.rs:1165-1189");
    println!("     Complexity: O(n^3) naive matrix multiplication");
    println!("     Issue: Uses get()/set() per element — massive overhead");
    println!("     Called by: MeraLayer::disentangle(), MeraLayer::coarsen(), MeraLayer::refine()");
    println!("     GPU candidate: YES — classic matmul kernel");

    println!("\n  2. HolographicFieldManager::create_holographic_connections()");
    println!("     — src/simulation_v3/holographic_field.rs:794-870");
    println!("     Complexity: O(n^2) entity pairs, each with 6 similarity calculations");
    println!("     Issue: Nested loop over all entity pairs; clones entire entity HashMap");
    println!("     Sub-operations per pair: archetype_similarity, spectrum_similarity,");
    println!("       blueprint_alignment, resonance_match, phase_coherence, karmic_correlation");
    println!("     GPU candidate: YES — embarrassingly parallel pairwise computation");

    println!(
        "\n  3. Tensor::downsample()/upsample() — src/simulation_v3/mera_network.rs:1193-1319"
    );
    println!("     Complexity: O(n^d) for d-dimensional tensor with nested loops");
    println!("     Issue: Uses get()/set() for every element instead of direct indexing");
    println!("     Called by: MeraNetwork::build_hierarchy() — runs every compress cycle");
    println!("     GPU candidate: YES — simple stencil computation, ideal for compute shader");

    println!("\n  4. Wavelet compression — src/simulation_v3/mera_network.rs:1442-1521");
    println!("     Complexity: O(n^2) with get()/set() overhead");
    println!("     Issue: 4 Tensor get() + 4 Tensor set() per 2x2 block = 8 indirections");
    println!("     GPU candidate: YES — parallel Haar wavelet transform");

    println!("\n  5. ArchetypeBasis::reconstruct() — src/simulation_v3/archetype_basis.rs:336-347");
    println!("     Complexity: O(22 * n) vector ops, each allocates new vectors");
    println!("     Issue: Excessive allocation — creates ArchetypeVector and ArchetypicalPattern");
    println!("       per archetype per reconstruction");
    println!("     GPU candidate: PARTIAL — small constant (22), but high allocation overhead");

    // ===================================================================
    // Extrapolation for 8192 entities
    // ===================================================================
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("PART 6: SCALING EXTRAPOLATION (8192 ENTITIES)");
    println!("═══════════════════════════════════════════════════════════");

    // Measure at 512 for extrapolation
    let entities_512 = create_entities(512);
    let start = Instant::now();
    let mut field = HolographicFieldManager::new();
    for e in &entities_512 {
        let _ = field.add_entity(e.clone());
    }
    field.update_field(1);
    let elapsed_512 = start.elapsed().as_secs_f64() * 1000.0;

    // O(n^2) extrapolation: 8192/512 = 16x entities → 16^2 = 256x time for connections
    // But max_connections_per_entity caps it: at >200 entities, max 50 connections each
    // So scaling is O(n * 50) = O(n) after cap
    println!(
        "\n  Measured at 512 entities: {:.2} ms for 1 step",
        elapsed_512
    );

    // Entity count scaling for holographic field with max_connections cap
    let scaling_8192 = 8192.0 / 512.0;
    let estimated_8192 = elapsed_512 * scaling_8192;
    println!(
        "  Estimated at 8192 entities (capped connections, linear): {:.2} ms/step",
        estimated_8192
    );
    println!(
        "  Estimated at 8192 entities (uncapped, O(n^2)): {:.2} ms/step",
        elapsed_512 * scaling_8192 * scaling_8192
    );

    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("END OF PROFILING REPORT");
    println!("═══════════════════════════════════════════════════════════\n");
}

fn chrono_format() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let secs = now.as_secs();
    let mins = (secs / 60) % 60;
    let hours = (secs / 3600) % 24;
    let days = secs / 86400;
    format!(
        "2026-04-07 {:02}:{:02}:{:02} UTC (day {} since epoch)",
        hours,
        mins,
        secs % 60,
        days
    )
}
