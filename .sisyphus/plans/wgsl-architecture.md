# WGSL Compute Shader Architecture — HoloSim Infinite MERA Tensor Operations

**Phase:** 5.2 (GPU Acceleration Design)
**Target:** wgpu 0.19 (from Cargo.toml)
**Status:** Architecture specification — ready for Phase 5.3-5.5 implementation

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Shader File Structure](#2-shader-file-structure)
3. [Data Flow Diagram](#3-data-flow-diagram)
4. [Buffer Layout & WGSL Alignment](#4-buffer-layout--wgsl-alignment)
5. [Workgroup Layouts by Scale](#5-workgroup-layouts-by-scale)
6. [Shared Memory Tiling Strategy](#6-shared-memory-tiling-strategy)
7. [BindGroupLayout Specifications](#7-bindgrouplayout-specifications)
8. [CPU↔GPU Sync Points](#8-cpugpu-sync-points)
9. [Operations to Keep on CPU](#9-operations-to-keep-on-cpu)
10. [wgpu Device/Queue Integration Notes](#10-wgpu-devicequeue-integration-notes)
11. [Implementation Phasing](#11-implementation-phasing)

---

## 1. Executive Summary

### Hot Paths Targeted

| # | Operation | File | CPU % | Complexity | GPU Candidate |
|---|-----------|------|-------|------------|---------------|
| 1 | `create_holographic_connections()` | `holographic_field.rs:794` | 40-50% | O(n²) × 6 metrics | **YES** — massive data parallelism |
| 2 | `calculate_resonance_matrix()` | `holographic_field.rs:1782` | 18% | O(n²) × 3 metrics | **YES** — identical pattern to #1 |
| 3 | `Tensor::matmul()` | `mera_network.rs:1165` | 12% | O(n³) naive | **YES** — classic GEMM |
| 4 | `apply_wavelet_compression()` | `mera_network.rs:1442` | ~5% | O(n²) 2D Haar | **YES** — per-tile independent |
| 5 | `upsample()` | `mera_network.rs:1243` | ~3% | O(n) nearest-neighbor | **YES** — embarrassingly parallel |

### Float Precision Decision

**Current code uses `f64` (`crate::types::Float = f64`).**

WGSL natively supports `f32` and `f16` (with extension). There is **no native `f64`** in WGSL.

**Decision: Use `f32` on GPU.** The simulation's archetype activations, spectrum ratios, and connection strengths are all clamped to [0.0, 1.0] or small ranges where `f32` precision (7 decimal digits) is sufficient. The CPU↔GPU bridge will perform `f64 → f32` conversion during upload and `f32 → f64` during readback. This is a standard practice in GPU compute and introduces negligible error for these value ranges.

---

## 2. Shader File Structure

```
shaders/
├── common/
│   ├── types.wgsl              # Shared struct definitions, constants
│   └── math.wgsl               # Utility functions (dot, normalize, clamp helpers)
│
├── holographic/
│   ├── entity_data.wgsl        # EntityGPUData struct + archetype similarity
│   ├── connection_metrics.wgsl # 6-metric connection strength shader
│   └── resonance_matrix.wgsl   # Resonance calculation shader (3 metrics)
│
├── mera/
│   ├── matmul.wgsl             # Blocked/tiled matrix multiplication
│   ├── wavelet.wgsl            # 2D Haar wavelet decomposition
│   └── upsample.wgsl           # Nearest-neighbor tensor upsampling
│
└── observation/
    └── reduce.wgsl             # Reduction kernels for statistics aggregation
```

### File Responsibilities

| File | Purpose | Dispatched By |
|------|---------|---------------|
| `common/types.wgsl` | `EntityGPUData`, `ConnectionOutput`, `ResonanceOutput`, constants | `#include`d by all |
| `common/math.wgsl` | `cosine_similarity()`, `pearson_correlation()`, `harmonic_match()` | `#include`d by holographic shaders |
| `holographic/connection_metrics.wgsl` | Computes 6 similarity metrics per entity pair → strength + type | HolographicFieldManager |
| `holographic/resonance_matrix.wgsl` | Computes spectrum + holographic + polarity resonance per pair | HolographicFieldManager |
| `mera/matmul.wgsl` | Tiled GEMM for MERA disentanglers/coarse-grainers | MeraNetwork |
| `mera/wavelet.wgsl` | 2D Haar wavelet: approximation + 3 detail subbands | MeraLayer |
| `mera/upsample.wgsl` | Nearest-neighbor 2× upsampling (1D/2D/3D) | Tensor |
| `observation/reduce.wgsl` | Parallel sum/mean/max for statistics aggregation | ObservationLayer |

### Include Strategy

wgpu 0.19 does **not** support `#include` natively. The Rust side will use `naga` or string concatenation to compose shaders. Implementation phase will use a `ShaderComposer` that concatenates `common/` + specific shader before `device.create_shader_module()`.

---

## 3. Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        CPU (Rust)                                   │
│                                                                     │
│  HashMap<EntityId, SubSubLogos>  ──┐                                │
│                                     │ extract_gpu_data()             │
│                                     ▼                                │
│                          EntityUploadStaging                        │
│                          (Vec<EntityGPUData>)                       │
│                                     │                                │
│                    queue.write_buffer() per frame                    │
│                                     │                                │
├─────────────────────────────────────┼────────────────────────────────┤
│                        GPU (wgpu)    │                                │
│                                     ▼                                │
│  ┌─────────────────────────────────────────────────────┐            │
│  │  STORAGE BUFFER: entity_data (read)                 │            │
│  │  STORAGE BUFFER: connection_matrix (read/write)     │            │
│  │  STORAGE BUFFER: resonance_matrix (read/write)      │            │
│  │  STORAGE BUFFER: mera_tensors (read/write)          │            │
│  │  UNIFORM BUFFER: dispatch_params                    │            │
│  │                                                     │            │
│  │  ┌──────────────────┐  ┌──────────────────────┐    │            │
│  │  │ connection_metrics│  │   resonance_matrix   │    │            │
│  │  │    shader          │  │      shader          │    │            │
│  │  │  (O(n²) tiled)    │  │   (O(n²) tiled)     │    │            │
│  │  └────────┬──────────┘  └──────────┬───────────┘    │            │
│  │           │                        │                 │            │
│  │  ┌────────▼──────────┐  ┌─────────▼───────────┐    │            │
│  │  │    matmul shader  │  │  wavelet + upsample  │    │            │
│  │  │   (tiled GEMM)    │  │      shaders         │    │            │
│  │  └────────┬──────────┘  └─────────┬───────────┘    │            │
│  │           │                        │                 │            │
│  │           ▼                        ▼                 │            │
│  │  connection_strength[N×N]   tensor_results          │            │
│  └─────────────────────────────────────────────────────┘            │
│                                     │                                │
│              queue.submit() + fence.map_async()                      │
│                                     │                                │
├─────────────────────────────────────┼────────────────────────────────┤
│                        CPU (Rust)    │                                │
│                                     ▼                                │
│                          EntityDownloadStaging                       │
│                          (readback buffers)                          │
│                                     │                                │
│                    convert back to Rust types                        │
│                                     │                                │
│                                     ▼                                │
│  HolographicFieldManager.update_from_gpu()                           │
│  ObservationLayer.consume_results()                                  │
│  → holographic_connections updated                                   │
│  → resonance_tracker updated                                         │
│  → MERA layers updated                                               │
└─────────────────────────────────────────────────────────────────────┘
```

### Per-Frame Data Flow (Evolution Step)

```
Step N:
1. CPU: Extract entity data → Vec<EntityGPUData>   [~0.1ms for 10K entities]
2. CPU: queue.write_buffer(entity_staging)          [~0.05ms]
3. GPU: dispatch connection_metrics                 [~2ms for 10K]
4. GPU: dispatch resonance_matrix (if needed)       [~2ms for 10K]
5. GPU: dispatch mera operations (if MERA active)   [~1ms]
6. GPU: dispatch reduce for statistics              [~0.5ms]
7. CPU: queue.submit() + readback                  [~0.5ms]
8. CPU: Update Rust structures from GPU results     [~0.5ms]

Total GPU-accelerated step: ~7ms (vs ~50-200ms CPU-only for 10K entities)
```

---

## 4. Buffer Layout & WGSL Alignment

### 4.1 WGSL std140 / std430 Alignment Rules

WGSL uses explicit alignment rules (similar to std140):

| WGSL Type | Alignment (bytes) | Size (bytes) |
|-----------|-------------------|--------------|
| `f32` | 4 | 4 |
| `vec2<f32>` | 8 | 8 |
| `vec3<f32>` | 16 | 12 |
| `vec4<f32>` | 16 | 16 |
| `mat2x2<f32>` | 8 | 16 |
| `mat4x4<f32>` | 16 | 64 |
| `u32` | 4 | 4 |
| `vec2<u32>` | 8 | 8 |
| `vec3<u32>` | 16 | 12 |
| `vec4<u32>` | 16 | 16 |

**Struct members must be aligned to their type's alignment.** Padding is automatically inserted.

### 4.2 EntityGPUData — Input Buffer

This is the flattened data uploaded per entity for GPU processing.

```wgsl
// shaders/common/types.wgsl

struct EntityGPUData {
    // Entity ID packed as 2x u32 (hash of UUID string + incarnation_number)
    entity_id_hash: u32,        // offset 0
    incarnation_number: u32,    // offset 4
    _pad0: u32,                 // offset 8 — alignment to 16

    // Spectrum configuration (from spectrum_configuration.ratio)
    spectrum_ratio: f32,        // offset 12
    space_time_access: f32,     // offset 16
    oneness_access: f32,        // offset 20
    veil_transparency: f32,     // offset 24

    // Vibrational state
    frequency: f32,             // offset 28
    coherence: f32,             // offset 32
    _pad1: u32,                 // offset 36 — alignment

    // Archetype activation (22 × f32 = 88 bytes, rounded to 96 for alignment)
    archetype_activation: vec4<f32>,  // A1-A4,   offset 40
    archetype_activation_1: vec4<f32>, // A5-A8,   offset 56
    archetype_activation_2: vec4<f32>, // A9-A12,  offset 72
    archetype_activation_3: vec4<f32>, // A13-A16, offset 88
    archetype_activation_4: vec4<f32>, // A17-A20, offset 104
    archetype_activation_5: vec2<f32>, // A21-A22, offset 120
    _pad2: vec2<f32>,                  // padding, offset 128

    // Polarity (ServiceToOthers=1.0, ServiceToSelf=-1.0, Neutral=0.0)
    polarity_direction: f32,    // offset 136
    polarity_strength: f32,     // offset 140
    evolutionary_rate: f32,     // offset 144
    _pad3: f32,                 // offset 148 — alignment

    // Holographic signature (8 × f32 = 32 bytes)
    holographic_signature_0: vec4<f32>, // offset 152
    holographic_signature_1: vec4<f32>, // offset 168

    // Density level (1-8 encoded as f32)
    density_level: f32,         // offset 184
    _pad4: vec3<f32>,           // offset 188 — alignment to 16
}
// Total: 200 bytes → rounded to 208 for array stride (multiple of 16)
// Actual array_stride: 208 bytes
```

**Array stride:** 208 bytes (next multiple of 16 after 200).

**CPU-side Rust struct for upload:**
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct EntityGPUData {
    entity_id_hash: u32,
    incarnation_number: u32,
    _pad0: u32,
    spectrum_ratio: f32,
    space_time_access: f32,
    oneness_access: f32,
    veil_transparency: f32,
    frequency: f32,
    coherence: f32,
    _pad1: u32,
    archetype_activation: [f32; 22], // padded to 24 internally
    polarity_direction: f32,
    polarity_strength: f32,
    evolutionary_rate: f32,
    _pad3: f32,
    holographic_signature: [f32; 8],
    density_level: f32,
    _pad4: [f32; 3],
}
// sizeof: 208 bytes (matches WGSL array_stride)
```

### 4.3 ConnectionOutput — Result Buffer (O(n²))

```wgsl
struct ConnectionOutput {
    strength: f32,              // Combined connection strength [0, 1]
    archetype_similarity: f32,  // Component metric
    spectrum_similarity: f32,   // Component metric
    blueprint_alignment: f32,   // Component metric
    resonance_match: f32,       // Component metric
    phase_coherence: f32,       // Component metric
    karmic_correlation: f32,    // Component metric
    connection_type: u32,       // 0=Resonant, 1=Harmonic, 2=Entangled, 3=Antiphase, 4=Weak
}
// Size: 32 bytes (naturally aligned, multiple of 16)
```

**Buffer organization:** Linear array indexed by `entity_a * N + entity_b`.
Total size: `N × N × 32` bytes.

### 4.4 ResonanceOutput — Result Buffer (O(n²))

```wgsl
struct ResonanceOutput {
    resonance_score: f32,       // Overall weighted score [0, 1]
    spectrum_resonance: f32,    // Component (30% weight)
    holographic_resonance: f32, // Component (40% weight)
    polarity_resonance: f32,    // Component (30% weight)
    resonance_type: u32,        // 0=None, 1=Low, 2=Medium, 3=High
    _pad: vec3<u32>,
}
// Size: 32 bytes
```

### 4.5 Uniform Buffer — Dispatch Parameters

```wgsl
struct DispatchParams {
    num_entities: u32,          // N
    max_connections: u32,       // Max connections per entity (50/100/N)
    _pad0: u32,
    _pad1: u32,
    // Weights (matching CPU calculation)
    weight_archetype: f32,      // 0.35
    weight_spectrum: f32,       // 0.20
    weight_blueprint: f32,      // 0.15
    weight_resonance: f32,      // 0.10
    weight_phase: f32,          // 0.10
    weight_karmic: f32,         // 0.10
    _pad2: u32,
    _pad3: u32,
    // Resonance weights
    weight_spectrum_res: f32,   // 0.30
    weight_holographic_res: f32,// 0.40
    weight_polarity_res: f32,   // 0.30
    max_spectrum_diff: f32,     // 20.0 (normalization)
}
// Size: 80 bytes (multiple of 16)
```

### 4.6 MERA Tensor Buffers

For matrix operations, tensors are stored as flat `array<f32>` with dimension uniforms:

```wgsl
struct TensorShape {
    rows: u32,
    cols: u32,
    depth: u32,    // 1 for 2D, >1 for 3D
    _pad: u32,
}
// Size: 16 bytes
```

**Buffer layout for `Tensor::matmul(A, B)`:**
- `tensor_A`: `array<f32>` — size `rows_A × cols_A`
- `tensor_B`: `array<f32>` — size `rows_B × cols_B`
- `tensor_C_out`: `array<f32>` — size `rows_A × cols_B`
- Uniforms: `TensorShape` for A, B, C

### 4.7 Summary of All Buffers

| Buffer | Type | Binding | Size Formula | Max at 100K |
|--------|------|---------|--------------|-------------|
| `entity_data` | Storage (read) | 0 | `N × 208` bytes | ~20 MB |
| `connection_matrix` | Storage (rw) | 1 | `N² × 32` bytes | ~320 MB ⚠️ |
| `resonance_matrix` | Storage (rw) | 2 | `N² × 32` bytes | ~320 MB ⚠️ |
| `mera_tensor_a` | Storage (read) | 3 | Variable | ~4 MB typical |
| `mera_tensor_b` | Storage (read) | 4 | Variable | ~4 MB typical |
| `mera_tensor_out` | Storage (rw) | 5 | Variable | ~4 MB typical |
| `dispatch_params` | Uniform | 6 | 80 bytes | 80 B |
| `reduction_scratch` | Storage (rw) | 7 | `N × 4` bytes | ~400 KB |

⚠️ **100K entity O(n²) buffers (320 MB each) exceed typical GPU memory budget.** See [Workgroup Layouts by Scale](#5-workgroup-layouts-by-scale) for chunking strategy.

---

## 5. Workgroup Layouts by Scale

### 5.1 Connection Metrics & Resonance Matrix (O(n²) shaders)

**Problem:** At 100K entities, N² = 10 billion pairs. A single `N×N` output buffer is ~320 MB per matrix. GPU memory budgets (8-16 GB typical) can hold this, but dispatching 10 billion invocations needs careful chunking.

**Solution: Tiled dispatch with upper-triangle optimization.**

Since connections are symmetric (connection A→B = B→A), we only compute the upper triangle and mirror results.

#### Tile Configuration

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| `WORKGROUP_SIZE_X` | 16 | Matches L1 cache line patterns |
| `WORKGROUP_SIZE_Y` | 16 | 256 invocations per workgroup |
| `TILE_SIZE` | 16 | Tile dimension for shared memory |

#### Workgroup Count Calculation

```rust
fn connection_workgroup_counts(n: usize) -> (u32, u32, u32) {
    let tile_size = 16;
    let tiles = (n + tile_size - 1) / tile_size;
    (tiles as u32, tiles as u32, 1)
}
```

#### Workgroup Counts by Entity Count

| Entities | N | Tiles | Workgroups | Total Invocations | Shared Memory |
|----------|---|-------|------------|-------------------|---------------|
| **1K** | 1,000 | 63 | 3,969 | ~1M | 63 × 63 tiles |
| **10K** | 10,000 | 625 | 390,625 | ~100M | Streaming |
| **100K** | 100,000 | 6,250 | 39,062,500 | ~10B | Chunked |

**100K chunking:** Dispatch in batches of `BATCH_SIZE = 256` tiles along the Y axis:

```rust
fn connection_batch_dispatch(n: usize) -> Vec<(u32, u32, u32)> {
    let tile_size = 16;
    let tiles = (n + tile_size - 1) / tile_size;
    let batch_tiles = 256;
    let mut batches = Vec::new();
    for y_start in (0..tiles).step_by(batch_tiles) {
        let y_count = (batch_tiles.min(tiles - y_start)) as u32;
        batches.push((tiles as u32, y_count, 1));
    }
    batches
}
```

At 100K: ~25 batches of ~1.5M workgroups each.

### 5.2 Matrix Multiplication (Tiled GEMM)

**Problem:** `Tensor::matmul()` is O(n³). Current naive triple loop.

**Solution: Standard tiled GEMM with shared memory.**

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| `WORKGROUP_SIZE_X` | 16 | Output tile width |
| `WORKGROUP_SIZE_Y` | 16 | Output tile height |
| `TILE_K` | 16 | Shared memory tile depth |

```rust
fn matmul_workgroup_count(rows_a: usize, cols_b: usize) -> (u32, u32, u32) {
    (
        (cols_b + 15) / 16,  // cols_b / tile_width
        (rows_a + 15) / 16,  // rows_a / tile_height
        1,
    )
}
```

| Matrix Size | Workgroups | Shared Memory per WG |
|-------------|------------|---------------------|
| 64×64 | 16 | 16×16×4×2 = 2 KB |
| 256×256 | 256 | 2 KB |
| 1024×1024 | 4,096 | 2 KB |
| 4096×4096 | 65,536 | 2 KB |

MERA tensors are typically 64×64 to 1024×1024. The tiled approach gives ~10-50× speedup over naive triple loop.

### 5.3 Wavelet Compression

2D Haar wavelet on a `rows × cols` matrix:

| Parameter | Value |
|-----------|-------|
| `WORKGROUP_SIZE_X` | 16 |
| `WORKGROUP_SIZE_Y` | 16 |

```rust
fn wavelet_workgroup_count(rows: usize, cols: usize) -> (u32, u32, u32) {
    (
        (cols / 2 + 15) / 16,  // Each WG computes 16×16 output = 32×32 input
        (rows / 2 + 15) / 16,
        1,
    )
}
```

### 5.4 Upsampling

Nearest-neighbor 2× upsampling:

```rust
fn upsample_workgroup_count(new_rows: usize, new_cols: usize) -> (u32, u32, u32) {
    (
        (new_cols + 15) / 16,
        (new_rows + 15) / 16,
        1,
    )
}
```

Each invocation writes 1 output pixel by reading `input[row/2][col/2]`.

---

## 6. Shared Memory Tiling Strategy

### 6.1 O(n²) Connection/Resonance Shaders

The key optimization for O(n²) operations is loading entity data into `var<workgroup>` shared memory to avoid repeated global memory accesses.

**Strategy:**

```
For a 16×16 tile of output (256 entity pairs):
- Each workgroup loads 16 entities (row entities) into shared memory
- Each workgroup loads 16 entities (col entities) into shared memory
- Total shared memory: 16 × 208 + 16 × 208 = 6,656 bytes (~6.5 KB)

Each invocation (i, j) within the workgroup:
- Reads row_entity[i] from workgroup memory
- Reads col_entity[j] from workgroup memory
- Computes 6 metrics
- Writes to connection_matrix[row*total_col + col]
```

**WGSL shared memory layout:**

```wgsl
var<workgroup> row_entities: array<EntityGPUData, 16>;
var<workgroup> col_entities: array<EntityGPUData, 16>;
```

**Loading phase (executed by each invocation, coordinated):**
```wgsl
// Each invocation loads one row entity and one col entity
let local_i = workgroup_id.x % 16;
let local_j = workgroup_id.y % 16;

// Load row entity
if (local_i < num_row_entities) {
    row_entities[local_i] = entity_data[row_tile_start + local_i];
}
// Load col entity
if (local_j < num_col_entities) {
    col_entities[local_j] = entity_data[col_tile_start + local_j];
}
workgroupBarrier();
```

**Memory savings:** Without tiling, each of the 256 invocations would read 2 entities from global memory = 512 global reads. With tiling: 32 global reads + 512 workgroup reads. Workgroup memory is ~10× faster than global memory on most GPUs.

### 6.2 Upper-Triangle Optimization

Since connection(A,B) = connection(B,A), only compute where `global_row < global_col`:

```wgsl
let global_row = workgroup_id.x * 16 + local_i;
let global_col = workgroup_id.y * 16 + local_j;

if (global_row >= num_entities || global_col >= num_entities) {
    return;
}

if (global_row >= global_col) {
    // Lower triangle or diagonal: copy from upper triangle or set self
    if (global_row == global_col) {
        connection_matrix[global_row * num_entities + global_col] = self_connection();
    }
    // For lower triangle, we skip — CPU or second pass mirrors
    return;
}

// Compute connection for upper triangle only
let conn = compute_connection(row_entities[local_i], col_entities[local_j]);
connection_matrix[global_row * num_entities + global_col] = conn;
```

This reduces work by ~50%.

### 6.3 Matrix Multiplication Tiling

Standard tiled GEMM:

```wgsl
var<workgroup> tile_a: array<array<f32, 16>, 16>;  // 16×16 shared
var<workgroup> tile_b: array<array<f32, 16>, 16>;  // 16×16 shared

var sum: f32 = 0.0;
let num_tiles = (shape_a.cols + 15) / 16;

for (var t: u32 = 0u; t < num_tiles; t = t + 1u) {
    // Load tiles into shared memory
    tile_a[local_y][local_x] = matrix_a[row * shape_a.cols + (t * 16 + local_x)];
    tile_b[local_y][local_x] = matrix_b[(t * 16 + local_y) * shape_b.cols + col];
    workgroupBarrier();

    // Compute partial dot product
    for (var k: u32 = 0u; k < 16u; k = k + 1u) {
        sum = sum + tile_a[local_y][k] * tile_b[k][local_x];
    }
    workgroupBarrier();
}

// Write result
matrix_out[row * shape_b.cols + col] = sum;
```

Shared memory: 2 × 16 × 16 × 4 = 2 KB per workgroup.

---

## 7. BindGroupLayout Specifications

### 7.1 Connection Metrics Shader

```rust
// Rust-side BindGroupLayout for connection_metrics.wgsl
let connection_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
    label: Some("connection_metrics_layout"),
    entries: &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(208).unwrap()),
            },
            count: None,
        },  // entity_data: Storage (read)
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(32).unwrap()),
            },
            count: None,
        },  // connection_matrix: Storage (rw)
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(80).unwrap()),
            },
            count: None,
        },  // dispatch_params: Uniform
    ],
});
```

**WGSL binding declarations:**
```wgsl
@group(0) @binding(0) var<storage, read> entity_data: array<EntityGPUData>;
@group(0) @binding(1) var<storage, read_write> connection_matrix: array<ConnectionOutput>;
@group(0) @binding(2) var<uniform> params: DispatchParams;
```

### 7.2 Resonance Matrix Shader

```rust
let resonance_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
    label: Some("resonance_matrix_layout"),
    entries: &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(208).unwrap()),
            },
            count: None,
        },  // entity_data: Storage (read) — SAME as connection shader
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(32).unwrap()),
            },
            count: None,
        },  // resonance_matrix: Storage (rw)
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(80).unwrap()),
            },
            count: None,
        },  // dispatch_params: Uniform
    ],
});
```

### 7.3 MERA Matmul Shader

```rust
let matmul_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
    label: Some("mera_matmul_layout"),
    entries: &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // tensor_a: array<f32>
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // tensor_b: array<f32>
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // tensor_out: array<f32>
        BindGroupLayoutEntry {
            binding: 3,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(16).unwrap()),
            },
            count: None,
        },  // shape_a: TensorShape
        BindGroupLayoutEntry {
            binding: 4,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(16).unwrap()),
            },
            count: None,
        },  // shape_b: TensorShape
    ],
});
```

### 7.4 Wavelet Shader

```rust
let wavelet_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
    label: Some("mera_wavelet_layout"),
    entries: &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // input_tensor: array<f32>
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // approximation: array<f32>
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // detail_h: array<f32>
        BindGroupLayoutEntry {
            binding: 3,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // detail_v: array<f32>
        BindGroupLayoutEntry {
            binding: 4,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // detail_d: array<f32>
        BindGroupLayoutEntry {
            binding: 5,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(16).unwrap()),
            },
            count: None,
        },  // shape: TensorShape
    ],
});
```

### 7.5 Upsample Shader

```rust
let upsample_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
    label: Some("mera_upsample_layout"),
    entries: &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // input_tensor: array<f32>
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },  // output_tensor: array<f32>
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(32).unwrap()),
            },
            count: None,
        },  // input_shape + output_shape: 2× TensorShape
    ],
});
```

### 7.6 BindGroupLayout Summary

| Shader | Bindings | Reusable Layouts |
|--------|----------|-----------------|
| connection_metrics | 3 (entity_data, connection_matrix, params) | Can share entityData layout |
| resonance_matrix | 3 (entity_data, resonance_matrix, params) | Reuses entity_data binding 0 |
| matmul | 5 (A, B, out, shape_a, shape_b) | Unique |
| wavelet | 6 (input, approx, detail_h, detail_v, detail_d, shape) | Unique |
| upsample | 3 (input, output, shapes) | Unique |

---

## 8. CPU↔GPU Sync Points

### 8.1 Execution Timeline Per Evolution Step

```
┌─ CPU Thread ──────────────────────────────────────────────────────────┐
│                                                                       │
│  1. extract_entity_data()                                             │
│     SubSubLogos → Vec<EntityGPUData>                                 │
│     [0.1-0.5ms for 10K entities]                                     │
│                                                                       │
│  2. queue.write_buffer(entity_buffer, 0, &entity_data)               │
│     [0.05-0.1ms — async, no stall]                                  │
│                                                                       │
│  3. Build compute pass                                               │
│     ┌─────────────────────────────────────────────┐                  │
│     │ set_bind_group(connection)                  │                  │
│     │ dispatch_workgroups(connection_tiles)       │                  │
│     │                                             │                  │
│     │ set_bind_group(resonance)                   │                  │
│     │ dispatch_workgroups(resonance_tiles)        │                  │
│     │                                             │                  │
│     │ set_bind_group(matmul)  [if MERA active]   │                  │
│     │ dispatch_workgroups(matmul_tiles)           │                  │
│     │                                             │                  │
│     │ set_bind_group(wavelet) [if compressing]   │                  │
│     │ dispatch_workgroups(wavelet_tiles)          │                  │
│     │                                             │                  │
│     │ set_bind_group(upsample) [if upscaling]    │                  │
│     │ dispatch_workgroups(upsample_tiles)         │                  │
│     │                                             │                  │
│     │ set_bind_group(reduce)                      │                  │
│     │ dispatch_workgroups(reduce_wgs)             │                  │
│     └─────────────────────────────────────────────┘                  │
│                                                                       │
│  4. queue.submit([encoder.finish()])                                 │
│     [Submits all work to GPU — async, returns immediately]           │
│                                                                       │
│  5. fence.map_async(WaitMode::Active, || {                           │
│        // GPU work complete                                           │
│        readback_connection_matrix()                                   │
│        readback_resonance_matrix()                                    │
│        update_holographic_connections_from_gpu()                      │
│     })                                                                │
│     [Stall point: waits for GPU]                                     │
│     [~2-10ms depending on entity count]                              │
│                                                                       │
│  6. Continue with non-GPU work (catalyst, lifecycle, etc.)           │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

### 8.2 Sync Point Details

| Sync Point | Operation | Blocking? | Duration (10K) |
|------------|-----------|-----------|----------------|
| `write_buffer` | CPU → GPU upload | No (async copy) | ~0.05ms |
| `dispatch_workgroups` | Queue compute shaders | No (queued) | 0ms |
| `queue.submit()` | Submit command buffer | No (async) | ~0.01ms |
| `fence.map_async()` | Wait for GPU completion | **Yes** | ~2-10ms |
| `readback` | GPU → CPU download | Implicit in map_async | ~0.5ms |

### 8.3 Lazy Update Integration

The holographic field already uses lazy updates (`update_connections_lazy()` with `update_interval` of 5-15 steps). GPU dispatches should follow the same pattern:

```rust
// In SimulationRunner::run_evolution_phase()
if self.performance_optimization_enabled {
    self.holographic_manager.set_current_step(step);

    if step == 0 || (step - self.holographic_manager.last_connection_update)
        >= self.holographic_manager.config.update_interval
    {
        // GPU: dispatch connection + resonance shaders
        self.gpu_compute.dispatch_connection_metrics(&entity_data)?;
        self.gpu_compute.dispatch_resonance_matrix(&entity_data)?;

        // Readback and update Rust structures
        let connections = self.gpu_compute.readback_connections()?;
        self.holographic_manager.apply_gpu_connections(connections);
    }
}
```

### 8.4 Staging Buffer Strategy

For 10K+ entities, use double-buffered staging:

```rust
struct GpuComputePipeline {
    entity_buffer: Buffer,          // GPU-visible storage
    connection_buffer: Buffer,      // GPU-visible read/write
    resonance_buffer: Buffer,       // GPU-visible read/write
    staging_readback: Buffer,       // MAP_READ staging buffer
    uniform_buffer: Buffer,         // Uniform data
    bind_groups: Vec<BindGroup>,
}
```

Readback flow:
1. GPU writes to `connection_buffer`
2. Encoder copies `connection_buffer` → `staging_readback` (via `copy_buffer_to_buffer`)
3. `fence.map_async()` on `staging_readback`
4. CPU maps and reads

### 8.5 MERA Tensor Sync

MERA operations are less frequent (compression happens during hierarchy build, not every step):

```rust
// During MeraNetwork::compress()
fn compress_gpu(&mut self, data: Tensor) -> Result<MeraCompressionResult, MeraError> {
    // 1. Upload tensor to GPU
    self.gpu.upload_tensor(&data, BindingSlot::TensorA);

    // 2. Run disentanglers (matmul chain)
    for disentangler in &self.layers[0].disentanglers {
        self.gpu.upload_tensor(disentangler, BindingSlot::TensorB);
        self.gpu.dispatch_matmul();
        self.gpu.swap_buffers(); // output becomes next input
    }

    // 3. Run wavelet compression
    self.gpu.dispatch_wavelet();

    // 4. Readback compressed result
    let result = self.gpu.readback_tensor();

    // 5. Free GPU buffers
    self.gpu.cleanup();

    Ok(compression_stats)
}
```

---

## 9. Operations to Keep on CPU

Not all operations in the hot paths are GPU-friendly. The following should **remain on CPU**:

### 9.1 Definitely Stay CPU

| Operation | Reason | Location |
|-----------|--------|----------|
| `determine_connection_type()` | Branch-heavy if/else chain with 5 conditions. Not data parallel. | `holographic_field.rs:1245` |
| Veil access control checks | Boolean checks per entity pair, but only 2 branches (pass/fail). Could move to GPU but low value. | `holographic_field.rs:949-977` |
| HashMap-based connection storage | HashMap operations (insert, contains_key) are not GPU-friendly. | `holographic_field.rs:842-851` |
| EntityId UUID string hashing | String operations are terrible on GPU. Hash on CPU. | EntityId struct |
| MERA hierarchy building | Tree-structured, sequential dependency (each level depends on previous). | `mera_network.rs:1573` |
| `PredictiveCache` operations | HashMap lookups, LRU eviction, access pattern analysis. | `mera_network.rs:795-943` |
| `CoherenceTracker` | Time-series snapshot management, HashMap operations. | `holographic_field.rs:566-704` |
| `ResonanceTracker::identify_clusters()` | Iterative clustering with dynamic data structures. | `holographic_field.rs:2154-2215` |
| Catalyst system | Random event generation, entity selection logic. | `catalyst_system.rs` |
| Entity lifecycle management | State machine transitions, density progression logic. | `entity_lifecycle.rs` |

### 9.2 Borderline — CPU for Now, GPU Later

| Operation | Reason | Future GPU Potential |
|-----------|--------|---------------------|
| `calculate_interference_patterns()` | Generates variable-length node lists per connection. Dynamic output size is hard on GPU. | **Medium** — if converted to fixed-size output |
| `find_high_resonance_pairs()` | Requires sorting + thresholding. GPU sort possible but complex. | **High** — thrust/cub-style GPU sort |
| Inter-scale interactions | Tree traversal across entity hierarchy. | **Low** — too much control flow |

### 9.3 Split Strategy (Partial GPU)

For `create_holographic_connections()`:

```
GPU: Compute raw strength + 6 metrics for all N² pairs
CPU:
  - Sort pairs by strength (per entity)
  - Truncate to max_connections_per_entity
  - Build HashMap<(EntityId, EntityId), HolographicConnection>
  - Determine connection_type from metrics
  - Update NonLocalMatrix
```

This keeps the data-parallel computation on GPU while leaving the irregular data structure work on CPU.

---

## 10. wgpu Device/Queue Integration Notes

### 10.1 Existing wgpu Infrastructure

HoloSim already uses wgpu 0.19 for the GUI (`holonic_gui_complete`). The `GpuSimulationRenderer` in `src/gui/renderer/` already has:
- `wgpu::Device` and `wgpu::Queue`
- Surface configuration for rendering

**Integration approach:** Create a separate `GpuComputeEngine` that shares the same `Device` and `Queue` but maintains its own compute pipelines, bind groups, and buffers. No surface/render pipeline interaction needed.

### 10.2 Device Limits to Check

Before creating large buffers:
```rust
let limits = device.limits();
assert!(limits.max_storage_buffer_binding_size >= needed_size);
assert!(limits.max_compute_workgroups_per_dimension >= max_tiles);
```

For wgpu 0.19, typical limits:
- `max_storage_buffer_binding_size`: 128 MB - 2 GB (adapter dependent)
- `max_compute_workgroups_per_dimension`: 65,535
- `max_compute_invocations_per_workgroup`: 256

**At 100K entities:** Connection matrix = 100,000² × 32 = 320 MB exceeds 128 MB limit on some adapters. **Chunking is mandatory at 100K.**

### 10.3 Adapter Selection

For compute work, prefer discrete GPU over integrated:
```rust
let adapter = instance
    .request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None, // No surface needed for compute
    })
    .await
    .expect("Failed to find adapter");
```

### 10.4 Feature Requirements

No special WGSL features needed beyond baseline wgpu 0.19:
- `f32` — baseline
- `storage` buffers — baseline
- `var<workgroup>` — baseline
- `workgroupBarrier()` — baseline

No `f16` extension needed (we use `f32`).

### 10.5 Error Handling

GPU compute failures should fall back to CPU:
```rust
impl GpuComputeEngine {
    fn dispatch_connection_metrics(&self, entity_data: &[EntityGPUData]) -> Result<(), GpuError> {
        match self.try_dispatch(entity_data) {
            Ok(_) => Ok(()),
            Err(e) => {
                log::warn!("GPU compute failed, falling back to CPU: {:?}", e);
                Err(GpuError::FallbackToCpu)
            }
        }
    }
}

// In SimulationRunner:
match self.gpu_compute.dispatch_connection_metrics(&data) {
    Ok(_) => { /* use GPU results */ }
    Err(GpuError::FallbackToCpu) => {
        self.holographic_manager.create_holographic_connections(); // CPU fallback
    }
    Err(e) => { return Err(e); }
}
```

---

## 11. Implementation Phasing

### Phase 5.3: Foundation
- [ ] Add `wgpu` compute infrastructure (reuse existing device/queue from GUI)
- [ ] Create `GpuComputeEngine` struct with buffer management
- [ ] Implement `EntityGPUData` conversion (SubSubLogos → GPU format)
- [ ] Write `common/types.wgsl` and `common/math.wgsl`
- [ ] Implement shader composition (string concatenation for `#include`)

### Phase 5.4: Holographic Field Shaders
- [ ] Write `connection_metrics.wgsl` with shared memory tiling
- [ ] Write `resonance_matrix.wgsl` with shared memory tiling
- [ ] Implement upper-triangle optimization
- [ ] Implement chunked dispatch for 100K entities
- [ ] CPU readback + HashMap reconstruction
- [ ] Integration test: GPU results match CPU results (within f32 tolerance)

### Phase 5.5: MERA Tensor Shaders
- [ ] Write `matmul.wgsl` with tiled GEMM
- [ ] Write `wavelet.wgsl` for 2D Haar decomposition
- [ ] Write `upsample.wgsl` for nearest-neighbor upsampling
- [ ] Integrate with `MeraNetwork::compress()`
- [ ] Performance benchmark: compare CPU vs GPU at various tensor sizes

### Phase 5.6: ObservationLayer Integration
- [ ] Write `reduce.wgsl` for parallel statistics aggregation
- [ ] Create ObservationLayer consume path (GPU → ObservationLayer)
- [ ] Full simulation benchmark at 1K, 10K, 100K entities
- [ ] Profiling report: identify remaining CPU bottlenecks

---

## Appendix A: Entity Count Scaling Reference

| Metric | 1K Entities | 10K Entities | 100K Entities |
|--------|-------------|--------------|---------------|
| **Pairs (N²)** | 1M | 100M | 10B |
| **Entity data upload** | 208 KB | 2.08 MB | 20.8 MB |
| **Connection buffer** | 32 MB | 3.2 GB ⚠️ | 320 GB ❌ |
| **Resonance buffer** | 32 MB | 3.2 GB ⚠️ | 320 GB ❌ |
| **Workgroups (conn)** | 3,969 | 390,625 | 39M |
| **Chunks needed** | 1 | 1 | ~150 |
| **Est. GPU time** | ~0.5ms | ~5ms | ~200ms |
| **Est. CPU time** | ~5ms | ~500ms | ~50s |
| **Speedup** | ~10× | ~100× | ~250× |

⚠️ At 10K, full N² buffers (3.2 GB) exceed typical GPU limits. **Even at 10K, chunking is recommended.**

**Recommended chunk sizes:**
- **1K:** Single dispatch (fits in GPU memory)
- **10K:** 4 chunks of 2,500 entities each (reduces buffer to 200 MB/chunk)
- **100K:** 100 chunks of 10,000 entities each, or 400 chunks of 5,000

### Chunked Dispatch Pattern

```rust
fn dispatch_connections_chunked(
    gpu: &GpuComputeEngine,
    entity_data: &[EntityGPUData],
    chunk_size: usize,
) -> Vec<ConnectionOutput> {
    let n = entity_data.len();
    let mut results = vec![ConnectionOutput::default(); n * n];

    for row_start in (0..n).step_by(chunk_size) {
        let row_end = (row_start + chunk_size).min(n);

        for col_start in (0..n).step_by(chunk_size) {
            let col_end = (col_start + chunk_size).min(n);

            // Upload chunk of entity data
            gpu.upload_entity_chunk(entity_data, row_start, row_end, col_start, col_end);

            // Dispatch
            let wg_x = (col_end - col_start + 15) / 16;
            let wg_y = (row_end - row_start + 15) / 16;
            gpu.dispatch_connection_workgroups(wg_x as u32, wg_y as u32);

            // Readback chunk result
            let chunk_result = gpu.readback_connection_chunk(row_start, row_end, col_start, col_end);

            // Write into full result matrix
            for (ri, row) in (row_start..row_end).enumerate() {
                for (ci, col) in (col_start..col_end).enumerate() {
                    results[row * n + col] = chunk_result[ri * (col_end - col_start) + ci];
                }
            }
        }
    }

    results
}
```

## Appendix B: Cosine Similarity WGSL Implementation

The archetype similarity metric (primary factor in connection strength) uses cosine similarity on 22-element vectors:

```wgsl
fn cosine_similarity(a: EntityGPUData, b: EntityGPUData) -> f32 {
    // Reconstruct 22-element vectors from vec4 packing
    let a_vec = vec22(
        a.archetype_activation, a.archetype_activation_1,
        a.archetype_activation_2, a.archetype_activation_3,
        a.archetype_activation_4, a.archetype_activation_5,
    );
    let b_vec = vec22(/* same for b */);

    let dot = dot(a_vec, b_vec);
    let mag_a = length(a_vec);
    let mag_b = length(b_vec);

    if (mag_a < 1e-10 || mag_b < 1e-10) {
        return 0.0;
    }
    return dot / (mag_a * mag_b);
}
```

## Appendix C: File Sizes Estimate

| Shader File | Est. Lines | Complexity |
|-------------|------------|------------|
| `common/types.wgsl` | ~80 | Struct definitions |
| `common/math.wgsl` | ~60 | Math utilities |
| `connection_metrics.wgsl` | ~200 | 6 metrics + tiling |
| `resonance_matrix.wgsl` | ~150 | 3 metrics + tiling |
| `matmul.wgsl` | ~120 | Tiled GEMM |
| `wavelet.wgsl` | ~100 | 2D Haar |
| `upsample.wgsl` | ~60 | Nearest-neighbor |
| `reduce.wgsl` | ~80 | Parallel reduction |
| **Total** | **~850 lines** | |

---

*Document generated for Phase 5.2 — GPU Acceleration Architecture.
Ready for implementation by Phase 5.3-5.5 agents.*
