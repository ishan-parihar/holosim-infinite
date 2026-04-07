//! Unified compute engine with GPU/CPU fallback.
//!
//! Attempts GPU initialization on creation. If GPU is unavailable, gracefully
//! falls back to CPU computation with identical input/output semantics.
//!
//! # Usage
//! ```ignore
//! let engine = ComputeEngine::new().await;
//! println!("Using backend: {}", engine.backend());
//!
//! let connections = engine.compute_connections(&entities, &weights);
//! ```

use crate::gpu::buffers::GpuBuffer;
use crate::gpu::connection_kernel::{ConnectionKernel, ConnectionResult, ConnectionWeights};
use crate::gpu::device::GpuContext;
use crate::gpu::entity_upload::EntityGPUData;
use crate::gpu::matmul_kernel::MatmulKernel;
use crate::gpu::resonance_kernel::{ResonanceKernel, ResonanceResult, ResonanceWeights};
use crate::gpu::upsample_kernel::UpsampleKernel;
use crate::gpu::wavelet_kernel::WaveletKernel;
use crate::gpu::ShaderComposer;

pub enum ComputeBackend {
    Gpu {
        context: GpuContext,
        composer: ShaderComposer,
        connection_kernel: ConnectionKernel,
        resonance_kernel: ResonanceKernel,
        matmul_kernel: MatmulKernel,
        wavelet_kernel: WaveletKernel,
        upsample_kernel: UpsampleKernel,
    },
    Cpu,
}

pub struct ComputeEngine {
    backend: ComputeBackend,
}

impl ComputeEngine {
    pub async fn new() -> Self {
        match GpuContext::new().await {
            Some(context) => {
                let composer = ShaderComposer::new();
                let connection_kernel =
                    ConnectionKernel::from_composer(&context.device, &composer);
                let resonance_kernel =
                    ResonanceKernel::from_composer(&context.device, &composer);
                let matmul_kernel = MatmulKernel::from_composer(&context.device, &composer);
                let wavelet_kernel = WaveletKernel::from_composer(&context.device, &composer);
                let upsample_kernel = UpsampleKernel::from_composer(&context.device, &composer);

                eprintln!(
                    "ComputeEngine: GPU backend initialized ({:?})",
                    context.adapter_info().name
                );

                ComputeEngine {
                    backend: ComputeBackend::Gpu {
                        context,
                        composer,
                        connection_kernel,
                        resonance_kernel,
                        matmul_kernel,
                        wavelet_kernel,
                        upsample_kernel,
                    },
                }
            }
            None => {
                eprintln!("ComputeEngine: No GPU available, falling back to CPU");
                ComputeEngine {
                    backend: ComputeBackend::Cpu,
                }
            }
        }
    }

    /// Returns the active backend: `"gpu"` or `"cpu"`.
    pub fn backend(&self) -> &str {
        match &self.backend {
            ComputeBackend::Gpu { .. } => "gpu",
            ComputeBackend::Cpu => "cpu",
        }
    }

    /// Compute pairwise connection metrics between entities.
    ///
    /// Returns an N×N matrix of `ConnectionResult` (flattened), where N is
    /// the number of entities.
    pub fn compute_connections(
        &self,
        entities: &[EntityGPUData],
        weights: &ConnectionWeights,
    ) -> Vec<ConnectionResult> {
        match &self.backend {
            ComputeBackend::Gpu {
                context,
                connection_kernel,
                ..
            } => {
                let num_entities = entities.len();
                if num_entities == 0 {
                    return vec![];
                }

                let device = &context.device;
                let queue = &context.queue;

                let entity_buffer = GpuBuffer::from_slice(
                    device,
                    entities,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    Some("compute_engine_entity_buffer"),
                );

                let output_size = ConnectionKernel::output_size(num_entities);
                let output_buffer = GpuBuffer::new(
                    device,
                    output_size as u64,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    Some("compute_engine_connection_output"),
                );

                let staging = crate::gpu::buffers::StagingBuffer::new::<ConnectionResult>(
                    device,
                    ConnectionKernel::output_count(num_entities),
                );

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("compute_engine_connection_encoder"),
                    });

                connection_kernel.dispatch(
                    device,
                    queue,
                    &mut encoder,
                    &entity_buffer,
                    &output_buffer,
                    num_entities,
                    weights,
                );

                encoder.copy_buffer_to_buffer(
                    &output_buffer.buffer,
                    0,
                    &staging.download().buffer,
                    0,
                    output_size as u64,
                );

                queue.submit(Some(encoder.finish()));

                pollster::block_on(staging.read::<ConnectionResult>(device))
            }
            ComputeBackend::Cpu => cpu_compute_connections(entities, weights),
        }
    }

    /// Compute pairwise resonance scores between entities.
    ///
    /// Returns an N×N matrix of `ResonanceResult` (flattened).
    pub fn compute_resonance(
        &self,
        entities: &[EntityGPUData],
        weights: &ResonanceWeights,
    ) -> Vec<ResonanceResult> {
        match &self.backend {
            ComputeBackend::Gpu {
                context,
                resonance_kernel,
                ..
            } => {
                let num_entities = entities.len();
                if num_entities == 0 {
                    return vec![];
                }

                let device = &context.device;
                let queue = &context.queue;

                let entity_buffer = GpuBuffer::from_slice(
                    device,
                    entities,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    Some("compute_engine_resonance_entity_buffer"),
                );

                let output_size = ResonanceKernel::output_size(num_entities);
                let output_buffer = GpuBuffer::new(
                    device,
                    output_size as u64,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    Some("compute_engine_resonance_output"),
                );

                let staging = crate::gpu::buffers::StagingBuffer::new::<ResonanceResult>(
                    device,
                    ResonanceKernel::output_count(num_entities),
                );

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("compute_engine_resonance_encoder"),
                    });

                resonance_kernel.dispatch(
                    device,
                    queue,
                    &mut encoder,
                    &entity_buffer,
                    &output_buffer,
                    num_entities,
                    weights,
                );

                encoder.copy_buffer_to_buffer(
                    &output_buffer.buffer,
                    0,
                    &staging.download().buffer,
                    0,
                    output_size as u64,
                );

                queue.submit(Some(encoder.finish()));

                pollster::block_on(staging.read::<ResonanceResult>(device))
            }
            ComputeBackend::Cpu => cpu_compute_resonance(entities, weights),
        }
    }

    /// Matrix multiplication: C = A × B.
    ///
    /// - `a`: Matrix A data, row-major, `rows_a × cols_a`
    /// - `b`: Matrix B data, row-major, `cols_a × cols_b`
    /// - `rows_a`, `cols_a`, `cols_b`: dimensions
    ///
    /// Returns C as a flattened row-major `Vec<f32>` of size `rows_a × cols_b`.
    pub fn compute_matmul(
        &self,
        a: &[f32],
        b: &[f32],
        rows_a: u32,
        cols_a: u32,
        cols_b: u32,
    ) -> Vec<f32> {
        match &self.backend {
            ComputeBackend::Gpu {
                context,
                matmul_kernel,
                ..
            } => {
                if a.is_empty() || b.is_empty() {
                    return vec![];
                }

                let device = &context.device;
                let queue = &context.queue;

                let buf_a = GpuBuffer::from_slice(
                    device,
                    a,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    Some("compute_engine_matmul_a"),
                );
                let buf_b = GpuBuffer::from_slice(
                    device,
                    b,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    Some("compute_engine_matmul_b"),
                );

                let output_count = MatmulKernel::output_count(rows_a, cols_b);
                let output_size = MatmulKernel::output_size(rows_a, cols_b);
                let buf_c = GpuBuffer::new(
                    device,
                    output_size as u64,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    Some("compute_engine_matmul_c"),
                );

                let staging = crate::gpu::buffers::StagingBuffer::new::<f32>(device, output_count);

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("compute_engine_matmul_encoder"),
                    });

                matmul_kernel.dispatch(
                    device,
                    queue,
                    &mut encoder,
                    &buf_a,
                    &buf_b,
                    &buf_c,
                    rows_a,
                    cols_a,
                    cols_b,
                );

                encoder.copy_buffer_to_buffer(
                    &buf_c.buffer,
                    0,
                    &staging.download().buffer,
                    0,
                    output_size as u64,
                );

                queue.submit(Some(encoder.finish()));

                pollster::block_on(staging.read::<f32>(device))
            }
            ComputeBackend::Cpu => cpu_compute_matmul(a, b, rows_a, cols_a, cols_b),
        }
    }

    /// 2D Haar wavelet decomposition.
    ///
    /// - `input`: Input matrix data, row-major, `rows × cols`
    /// - `rows`, `cols`: dimensions
    ///
    /// Returns decomposed matrix with quadrant layout:
    /// top-left=LL, top-right=LH, bottom-left=HL, bottom-right=HH.
    pub fn compute_wavelet(&self, input: &[f32], rows: u32, cols: u32) -> Vec<f32> {
        match &self.backend {
            ComputeBackend::Gpu {
                context,
                wavelet_kernel,
                ..
            } => {
                if input.is_empty() {
                    return vec![];
                }

                let device = &context.device;
                let queue = &context.queue;

                let input_buf = GpuBuffer::from_slice(
                    device,
                    input,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    Some("compute_engine_wavelet_input"),
                );

                let output_count = WaveletKernel::output_count(rows, cols);
                let output_size = WaveletKernel::output_size(rows, cols);
                let output_buf = GpuBuffer::new(
                    device,
                    output_size as u64,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    Some("compute_engine_wavelet_output"),
                );

                let staging = crate::gpu::buffers::StagingBuffer::new::<f32>(device, output_count);

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("compute_engine_wavelet_encoder"),
                    });

                wavelet_kernel.dispatch(
                    device,
                    queue,
                    &mut encoder,
                    &input_buf,
                    &output_buf,
                    rows,
                    cols,
                );

                encoder.copy_buffer_to_buffer(
                    &output_buf.buffer,
                    0,
                    &staging.download().buffer,
                    0,
                    output_size as u64,
                );

                queue.submit(Some(encoder.finish()));

                pollster::block_on(staging.read::<f32>(device))
            }
            ComputeBackend::Cpu => cpu_compute_wavelet(input, rows, cols),
        }
    }

    /// Nearest-neighbor 2× upsampling.
    ///
    /// - `input`: Input matrix data, row-major, `input_rows × input_cols`
    /// - `input_rows`, `input_cols`: input dimensions
    ///
    /// Returns upsampled matrix of size `(input_rows*2) × (input_cols*2)`.
    pub fn compute_upsample(
        &self,
        input: &[f32],
        input_rows: u32,
        input_cols: u32,
    ) -> Vec<f32> {
        match &self.backend {
            ComputeBackend::Gpu {
                context,
                upsample_kernel,
                ..
            } => {
                if input.is_empty() {
                    return vec![];
                }

                let device = &context.device;
                let queue = &context.queue;

                let input_buf = GpuBuffer::from_slice(
                    device,
                    input,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    Some("compute_engine_upsample_input"),
                );

                let output_count = UpsampleKernel::output_count(input_rows, input_cols);
                let output_size = UpsampleKernel::output_size(input_rows, input_cols);
                let output_buf = GpuBuffer::new(
                    device,
                    output_size as u64,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    Some("compute_engine_upsample_output"),
                );

                let staging = crate::gpu::buffers::StagingBuffer::new::<f32>(device, output_count);

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("compute_engine_upsample_encoder"),
                    });

                upsample_kernel.dispatch(
                    device,
                    queue,
                    &mut encoder,
                    &input_buf,
                    &output_buf,
                    input_rows,
                    input_cols,
                );

                encoder.copy_buffer_to_buffer(
                    &output_buf.buffer,
                    0,
                    &staging.download().buffer,
                    0,
                    output_size as u64,
                );

                queue.submit(Some(encoder.finish()));

                pollster::block_on(staging.read::<f32>(device))
            }
            ComputeBackend::Cpu => cpu_compute_upsample(input, input_rows, input_cols),
        }
    }
}

// ============================================================================
// CPU fallback functions
// ============================================================================

/// Clamp a value to [0, 1] range.
#[inline]
fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

/// Harmonic match between two frequencies.
/// Returns 1.0 when frequencies are identical, 0.0 when maximally different.
#[inline]
fn harmonic_match(freq_a: f32, freq_b: f32) -> f32 {
    let diff = (freq_a - freq_b).abs();
    let max_f = freq_a.max(freq_b).max(1e-8);
    1.0 - (diff / max_f).min(1.0)
}

/// Cosine similarity of two slices, normalized to [0, 1] using abs().
#[inline]
fn cosine_similarity_abs(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    (dot / (mag_a * mag_b + 1e-8)).abs()
}

/// Cosine similarity of two slices, clamped to [0, 1].
#[inline]
fn cosine_similarity_clamped(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    clamp01(dot / (mag_a * mag_b + 1e-8))
}

/// Determine connection type from metric values.
///
/// Mapping (matching WGSL shader):
/// - 0 = Resonant
/// - 1 = Harmonic
/// - 2 = Entangled
/// - 3 = Antiphase
/// - 4 = Weak
fn determine_connection_type(strength: f32, archetype_sim: f32, resonance: f32, phase: f32) -> u32 {
    if archetype_sim > 0.7 && resonance > 0.6 && phase > 0.6 {
        return 0;
    }
    if archetype_sim > 0.4 && resonance > 0.5 {
        return 1;
    }
    if strength > 0.6 && phase > 0.6 {
        return 2;
    }
    if phase < 0.4 {
        return 3;
    }
    4
}

/// Classify resonance type based on score.
///
/// Mapping (matching WGSL shader):
/// - 0 = None
/// - 1 = Low
/// - 2 = Medium
/// - 3 = High
fn classify_resonance(score: f32) -> u32 {
    if score > 0.8 {
        return 3;
    }
    if score > 0.5 {
        return 2;
    }
    if score > 0.2 {
        return 1;
    }
    0
}

/// CPU implementation of pairwise connection metrics.
///
/// Computes an N×N matrix of `ConnectionResult` (flattened), matching the
/// GPU shader's `main_connection_metrics` exactly.
pub fn cpu_compute_connections(
    entities: &[EntityGPUData],
    weights: &ConnectionWeights,
) -> Vec<ConnectionResult> {
    let n = entities.len();
    if n == 0 {
        return vec![];
    }

    let mut results = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            if i == j {
                results.push(ConnectionResult {
                    strength: 1.0,
                    archetype_similarity: 1.0,
                    spectrum_similarity: 1.0,
                    blueprint_alignment: 1.0,
                    resonance_match: 1.0,
                    phase_coherence: 1.0,
                    karmic_correlation: 1.0,
                    connection_type: 0,
                });
                continue;
            }

            let a = &entities[i];
            let b = &entities[j];

            let arch_sim = cosine_similarity_abs(&a.archetype_activation, &b.archetype_activation);

            let spec_ratio_sum = a.spectrum_ratio + b.spectrum_ratio + 1e-8;
            let spec_sim = clamp01(
                1.0 - (a.spectrum_ratio - b.spectrum_ratio).abs() / spec_ratio_sum,
            );

            let blueprint = cosine_similarity_clamped(
                &a.holographic_signature,
                &b.holographic_signature,
            );

            let resonance = harmonic_match(a.frequency, b.frequency);

            let phase = (a.coherence + b.coherence) / 2.0;

            let karmic = clamp01(1.0 - (a.evolutionary_rate - b.evolutionary_rate).abs());

            let strength = clamp01(
                arch_sim * weights.weight_archetype
                    + spec_sim * weights.weight_spectrum
                    + blueprint * weights.weight_blueprint
                    + resonance * weights.weight_resonance
                    + phase * weights.weight_phase
                    + karmic * weights.weight_karmic,
            );

            let conn_type = determine_connection_type(strength, arch_sim, resonance, phase);

            results.push(ConnectionResult {
                strength,
                archetype_similarity: arch_sim,
                spectrum_similarity: spec_sim,
                blueprint_alignment: blueprint,
                resonance_match: resonance,
                phase_coherence: phase,
                karmic_correlation: karmic,
                connection_type: conn_type,
            });
        }
    }

    results
}

fn compute_spectrum_resonance(a: &EntityGPUData, b: &EntityGPUData) -> f32 {
    let diff = (a.spectrum_ratio - b.spectrum_ratio).abs();
    1.0 - (diff / 20.0).min(1.0)
}

fn compute_holographic_resonance(a: &EntityGPUData, b: &EntityGPUData) -> f32 {
    cosine_similarity_abs(&a.archetype_activation, &b.archetype_activation)
}

fn compute_polarity_resonance(a: &EntityGPUData, b: &EntityGPUData) -> f32 {
    let dir_a = a.polarity_direction;
    let dir_b = b.polarity_direction;
    let abs_a = dir_a.abs();
    let abs_b = dir_b.abs();
    let neutral_threshold = 0.1;

    if abs_a < neutral_threshold && abs_b < neutral_threshold {
        return 0.5;
    }
    if abs_a < neutral_threshold || abs_b < neutral_threshold {
        return 0.3;
    }
    if (dir_a > 0.0 && dir_b > 0.0) || (dir_a < 0.0 && dir_b < 0.0) {
        return 1.0;
    }
    0.0
}

/// CPU implementation of pairwise resonance scores.
///
/// Computes an N×N matrix of `ResonanceResult` (flattened), matching the
/// GPU shader's `main_resonance_matrix` exactly.
pub fn cpu_compute_resonance(
    entities: &[EntityGPUData],
    weights: &ResonanceWeights,
) -> Vec<ResonanceResult> {
    let n = entities.len();
    if n == 0 {
        return vec![];
    }

    let mut results = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            if i == j {
                results.push(ResonanceResult {
                    resonance_score: 1.0,
                    spectrum_resonance: 1.0,
                    holographic_resonance: 1.0,
                    polarity_resonance: 1.0,
                    resonance_type: 3,
                    _pad: [0, 0, 0],
                });
                continue;
            }

            let a = &entities[i];
            let b = &entities[j];

            let spectrum_res = compute_spectrum_resonance(a, b);
            let holographic_res = compute_holographic_resonance(a, b);
            let polarity_res = compute_polarity_resonance(a, b);

            let score = weights.weight_spectrum_res * spectrum_res
                + weights.weight_holographic_res * holographic_res
                + weights.weight_polarity_res * polarity_res;

            let clamped_score = clamp01(score);
            let res_type = classify_resonance(clamped_score);

            results.push(ResonanceResult {
                resonance_score: clamped_score,
                spectrum_resonance: spectrum_res,
                holographic_resonance: holographic_res,
                polarity_resonance: polarity_res,
                resonance_type: res_type,
                _pad: [0, 0, 0],
            });
        }
    }

    results
}

/// CPU matrix multiplication: C = A × B (naive O(n³)).
///
/// - `a`: Row-major matrix A of size `rows_a × cols_a`
/// - `b`: Row-major matrix B of size `cols_a × cols_b`
/// - Returns row-major matrix C of size `rows_a × cols_b`
pub fn cpu_compute_matmul(
    a: &[f32],
    b: &[f32],
    rows_a: u32,
    cols_a: u32,
    cols_b: u32,
) -> Vec<f32> {
    let rows_a = rows_a as usize;
    let cols_a = cols_a as usize;
    let cols_b = cols_b as usize;

    if rows_a == 0 || cols_a == 0 || cols_b == 0 {
        return vec![];
    }

    let mut c = vec![0.0f32; rows_a * cols_b];

    for i in 0..rows_a {
        for j in 0..cols_b {
            let mut sum = 0.0f32;
            for k in 0..cols_a {
                sum += a[i * cols_a + k] * b[k * cols_b + j];
            }
            c[i * cols_b + j] = sum;
        }
    }

    c
}

/// CPU 2D Haar wavelet decomposition.
///
/// For each 2×2 block at (r*2, c*2):
///   LL = (a + b + c + d) / 4   (approximation)
///   LH = (a - b + c - d) / 4   (horizontal detail)
///   HL = (a + b - c - d) / 4   (vertical detail)
///   HH = (a - b - c + d) / 4   (diagonal detail)
///
/// Output quadrants: top-left=LL, top-right=LH, bottom-left=HL, bottom-right=HH.
pub fn cpu_compute_wavelet(input: &[f32], rows: u32, cols: u32) -> Vec<f32> {
    let rows = rows as usize;
    let cols = cols as usize;

    if rows == 0 || cols == 0 {
        return vec![];
    }

    let half_rows = (rows + 1) / 2;
    let half_cols = (cols + 1) / 2;
    let total = rows * cols;
    let mut output = vec![0.0f32; total];

    let mut block_row = 0;
    let mut r0 = 0;
    while r0 < rows {
        let r1 = (r0 + 1).min(rows - 1);

        let mut block_col = 0;
        let mut c0 = 0;
        while c0 < cols {
            let c1 = (c0 + 1).min(cols - 1);

            let a = input[r0 * cols + c0];
            let b = input[r0 * cols + c1];
            let c = input[r1 * cols + c0];
            let d = input[r1 * cols + c1];

            let ll = (a + b + c + d) * 0.25;
            let lh = (a - b + c - d) * 0.25;
            let hl = (a + b - c - d) * 0.25;
            let hh = (a - b - c + d) * 0.25;

            let idx_ll = block_row * half_cols + block_col;
            let idx_lh = block_row * half_cols + half_cols + block_col;
            let idx_hl = (half_rows + block_row) * half_cols + block_col;
            let idx_hh = (half_rows + block_row) * half_cols + half_cols + block_col;

            if idx_ll < total {
                output[idx_ll] = ll;
            }
            if block_col < half_cols && idx_lh < total {
                output[idx_lh] = lh;
            }
            if block_row < half_rows && idx_hl < total {
                output[idx_hl] = hl;
            }
            if block_row < half_rows && block_col < half_cols && idx_hh < total {
                output[idx_hh] = hh;
            }

            block_col += 1;
            c0 += 2;
        }
        block_row += 1;
        r0 += 2;
    }

    output
}

/// CPU nearest-neighbor 2× upsampling.
///
/// Each input pixel is replicated into a 2×2 block in the output.
/// output[gy, gx] = input[gy/2, gx/2]
pub fn cpu_compute_upsample(input: &[f32], input_rows: u32, input_cols: u32) -> Vec<f32> {
    let input_rows = input_rows as usize;
    let input_cols = input_cols as usize;

    if input_rows == 0 || input_cols == 0 {
        return vec![];
    }

    let output_rows = input_rows * 2;
    let output_cols = input_cols * 2;
    let mut output = vec![0.0f32; output_rows * output_cols];

    for gy in 0..output_rows {
        for gx in 0..output_cols {
            let src_row = gy / 2;
            let src_col = gx / 2;
            output[gy * output_cols + gx] = input[src_row * input_cols + src_col];
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_entity(
        id_hash: u32,
        spectrum_ratio: f32,
        frequency: f32,
        coherence: f32,
        archetype: [f32; 22],
        polarity_direction: f32,
        evolutionary_rate: f32,
        holographic_signature: [f32; 8],
    ) -> EntityGPUData {
        EntityGPUData {
            entity_id_hash: id_hash,
            incarnation_number: 1,
            _pad0: 0,
            spectrum_ratio,
            space_time_access: 0.5,
            oneness_access: 0.5,
            veil_transparency: 0.0,
            frequency,
            coherence,
            _pad1: 0,
            archetype_activation: archetype,
            _pad2: [0.0; 2],
            polarity_direction,
            polarity_strength: 1.0,
            evolutionary_rate,
            _pad3: 0.0,
            holographic_signature,
            density_level: 3.0,
            _pad4: [0.0; 3],
            _pad5: [0.0; 2],
        }
    }

    #[test]
    fn test_cpu_backend_identification() {
        let engine = ComputeEngine {
            backend: ComputeBackend::Cpu,
        };
        assert_eq!(engine.backend(), "cpu");
    }

    #[test]
    fn test_cpu_connections_self_identity() {
        let archetype = [0.5; 22];
        let entity = make_test_entity(
            1, 1.0, 1.0, 1.0, archetype, 1.0, 0.5, [0.5; 8],
        );
        let entities = vec![entity];
        let weights = ConnectionWeights::default();

        let results = cpu_compute_connections(&entities, &weights);

        assert_eq!(results.len(), 1);
        let r = &results[0];
        assert!((r.strength - 1.0).abs() < 1e-6);
        assert!((r.archetype_similarity - 1.0).abs() < 1e-6);
        assert!((r.spectrum_similarity - 1.0).abs() < 1e-6);
        assert!((r.blueprint_alignment - 1.0).abs() < 1e-6);
        assert!((r.resonance_match - 1.0).abs() < 1e-6);
        assert!((r.phase_coherence - 1.0).abs() < 1e-6);
        assert!((r.karmic_correlation - 1.0).abs() < 1e-6);
        assert_eq!(r.connection_type, 0);
    }

    #[test]
    fn test_cpu_connections_pairwise() {
        let arch_a = [1.0; 22];
        let arch_b = [1.0; 22];
        let a = make_test_entity(1, 1.0, 1.0, 0.8, arch_a, 1.0, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 1.5, 1.2, 0.6, arch_b, 1.0, 0.5, [0.5; 8]);
        let entities = vec![a, b];
        let weights = ConnectionWeights::default();

        let results = cpu_compute_connections(&entities, &weights);

        assert_eq!(results.len(), 4);

        assert!((results[0].strength - 1.0).abs() < 1e-6);
        assert!((results[3].strength - 1.0).abs() < 1e-6);

        assert!((results[1].archetype_similarity - 1.0).abs() < 1e-6);
        assert!((results[2].archetype_similarity - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_connections_empty() {
        let entities: Vec<EntityGPUData> = vec![];
        let weights = ConnectionWeights::default();
        let results = cpu_compute_connections(&entities, &weights);
        assert!(results.is_empty());
    }

    #[test]
    fn test_cpu_connection_type_classification() {
        let arch = [1.0; 22];
        let a = make_test_entity(1, 1.0, 1.0, 0.9, arch, 1.0, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 1.0, 1.0, 0.9, arch, 1.0, 0.5, [0.5; 8]);
        let results = cpu_compute_connections(&[a, b], &ConnectionWeights::default());
        assert_eq!(results[1].connection_type, 0);

        let arch_low = [0.01; 22];
        let c = make_test_entity(3, 1.0, 1.0, 0.1, arch_low, 1.0, 0.9, [0.1; 8]);
        let d = make_test_entity(4, 20.0, 10.0, 0.1, arch_low, -1.0, 0.1, [0.9; 8]);
        let results2 = cpu_compute_connections(&[c, d], &ConnectionWeights::default());
        assert_eq!(results2[1].connection_type, 3);
    }

    #[test]
    fn test_cpu_resonance_self_identity() {
        let archetype = [0.5; 22];
        let entity = make_test_entity(1, 1.0, 1.0, 1.0, archetype, 1.0, 0.5, [0.5; 8]);
        let entities = vec![entity];
        let weights = ResonanceWeights::default();

        let results = cpu_compute_resonance(&entities, &weights);

        assert_eq!(results.len(), 1);
        let r = &results[0];
        assert!((r.resonance_score - 1.0).abs() < 1e-6);
        assert!((r.spectrum_resonance - 1.0).abs() < 1e-6);
        assert!((r.holographic_resonance - 1.0).abs() < 1e-6);
        assert!((r.polarity_resonance - 1.0).abs() < 1e-6);
        assert_eq!(r.resonance_type, 3);
    }

    #[test]
    fn test_cpu_resonance_polarity_same() {
        let arch = [0.5; 22];
        let a = make_test_entity(1, 1.0, 1.0, 1.0, arch, 1.0, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 1.0, 1.0, 1.0, arch, 1.0, 0.5, [0.5; 8]);
        let entities = vec![a, b];
        let weights = ResonanceWeights::default();

        let results = cpu_compute_resonance(&entities, &weights);

        assert!((results[1].spectrum_resonance - 1.0).abs() < 1e-6);
        assert!((results[1].polarity_resonance - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_resonance_polarity_opposite() {
        let arch = [0.5; 22];
        let a = make_test_entity(1, 1.0, 1.0, 1.0, arch, 1.0, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 1.0, 1.0, 1.0, arch, -1.0, 0.5, [0.5; 8]);
        let entities = vec![a, b];
        let weights = ResonanceWeights::default();

        let results = cpu_compute_resonance(&entities, &weights);

        assert!((results[1].polarity_resonance - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_resonance_empty() {
        let entities: Vec<EntityGPUData> = vec![];
        let weights = ResonanceWeights::default();
        let results = cpu_compute_resonance(&entities, &weights);
        assert!(results.is_empty());
    }

    #[test]
    fn test_cpu_resonance_type_classification() {
        let arch = [0.5; 22];
        let a = make_test_entity(1, 1.0, 1.0, 1.0, arch, 1.0, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 1.0, 1.0, 1.0, arch, 1.0, 0.5, [0.5; 8]);
        let results = cpu_compute_resonance(&[a, b], &ResonanceWeights::default());
        assert_eq!(results[1].resonance_type, 3);

        let c = make_test_entity(3, 1.0, 1.0, 1.0, arch, 1.0, 0.5, [0.5; 8]);
        let d = make_test_entity(4, 20.0, 1.0, 1.0, arch, 1.0, 0.5, [0.5; 8]);
        let results2 = cpu_compute_resonance(&[c, d], &ResonanceWeights::default());
        assert_eq!(results2[1].resonance_type, 2);
    }

    #[test]
    fn test_cpu_matmul_2x2() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![5.0, 6.0, 7.0, 8.0];
        let c = cpu_compute_matmul(&a, &b, 2, 2, 2);
        assert_eq!(c.len(), 4);
        assert!((c[0] - 19.0).abs() < 1e-6);
        assert!((c[1] - 22.0).abs() < 1e-6);
        assert!((c[2] - 43.0).abs() < 1e-6);
        assert!((c[3] - 50.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_matmul_non_square() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
        let c = cpu_compute_matmul(&a, &b, 2, 3, 2);
        assert_eq!(c.len(), 4);
        assert!((c[0] - 58.0).abs() < 1e-6);
        assert!((c[1] - 64.0).abs() < 1e-6);
        assert!((c[2] - 139.0).abs() < 1e-6);
        assert!((c[3] - 154.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_matmul_identity() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let identity = vec![1.0, 0.0, 0.0, 1.0];
        let c = cpu_compute_matmul(&a, &identity, 2, 2, 2);
        assert!((c[0] - 1.0).abs() < 1e-6);
        assert!((c[1] - 2.0).abs() < 1e-6);
        assert!((c[2] - 3.0).abs() < 1e-6);
        assert!((c[3] - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_matmul_empty() {
        let c = cpu_compute_matmul(&[], &[], 0, 0, 0);
        assert!(c.is_empty());
    }

    #[test]
    fn test_cpu_wavelet_2x2_uniform() {
        let input = vec![1.0, 1.0, 1.0, 1.0];
        let output = cpu_compute_wavelet(&input, 2, 2);
        assert_eq!(output.len(), 4);
        assert!((output[0] - 1.0).abs() < 1e-6);
        assert!((output[1] - 0.0).abs() < 1e-6);
        assert!((output[2] - 0.0).abs() < 1e-6);
        assert!((output[3] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_wavelet_2x2_checkerboard() {
        let input = vec![1.0, 0.0, 0.0, 1.0];
        let output = cpu_compute_wavelet(&input, 2, 2);
        // half_rows=1, half_cols=1: LH and HL collide at index 1, HH writes to index 2
        assert!((output[0] - 0.5).abs() < 1e-6);
        assert!((output[1] - 0.0).abs() < 1e-6);
        assert!((output[2] - 0.5).abs() < 1e-6);
        assert!((output[3] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_wavelet_4x4() {
        let input: Vec<f32> = (0..16).map(|x| x as f32).collect();
        let output = cpu_compute_wavelet(&input, 4, 4);
        assert_eq!(output.len(), 16);
        assert!((output[0] - 2.5).abs() < 1e-6);
        assert!((output[1] - 4.5).abs() < 1e-6);
        assert!((output[2] - 10.5).abs() < 1e-6);
        assert!((output[3] - 12.5).abs() < 1e-6);
        assert!((output[4] - (-0.5)).abs() < 1e-6);
        assert!((output[5] - (-0.5)).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_wavelet_empty() {
        let output = cpu_compute_wavelet(&[], 0, 0);
        assert!(output.is_empty());
    }

    #[test]
    fn test_cpu_wavelet_odd_dimensions() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let output = cpu_compute_wavelet(&input, 3, 3);
        assert_eq!(output.len(), 9);
        assert!((output[0] - 3.0).abs() < 1e-6);
        assert!((output[1] - 4.5).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_upsample_2x2() {
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let output = cpu_compute_upsample(&input, 2, 2);
        assert_eq!(output.len(), 16);
        assert!((output[0] - 1.0).abs() < 1e-6);
        assert!((output[1] - 1.0).abs() < 1e-6);
        assert!((output[2] - 2.0).abs() < 1e-6);
        assert!((output[3] - 2.0).abs() < 1e-6);
        assert!((output[4] - 1.0).abs() < 1e-6);
        assert!((output[5] - 1.0).abs() < 1e-6);
        assert!((output[10] - 4.0).abs() < 1e-6);
        assert!((output[15] - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_upsample_1x1() {
        let input = vec![42.0];
        let output = cpu_compute_upsample(&input, 1, 1);
        assert_eq!(output.len(), 4);
        assert!((output[0] - 42.0).abs() < 1e-6);
        assert!((output[1] - 42.0).abs() < 1e-6);
        assert!((output[2] - 42.0).abs() < 1e-6);
        assert!((output[3] - 42.0).abs() < 1e-6);
    }

    #[test]
    fn test_cpu_upsample_empty() {
        let output = cpu_compute_upsample(&[], 0, 0);
        assert!(output.is_empty());
    }

    #[test]
    fn test_cpu_upsample_3x2() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let output = cpu_compute_upsample(&input, 3, 2);
        assert_eq!(output.len(), 24);
        for col in 0..4 {
            let expected = if col < 2 { 1.0 } else { 2.0 };
            assert!((output[col] - expected).abs() < 1e-6);
            assert!((output[4 + col] - expected).abs() < 1e-6);
        }
    }

    #[test]
    fn test_clamp01() {
        assert!((clamp01(-1.0) - 0.0).abs() < 1e-6);
        assert!((clamp01(0.0) - 0.0).abs() < 1e-6);
        assert!((clamp01(0.5) - 0.5).abs() < 1e-6);
        assert!((clamp01(1.0) - 1.0).abs() < 1e-6);
        assert!((clamp01(2.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_harmonic_match() {
        assert!((harmonic_match(1.0, 1.0) - 1.0).abs() < 1e-6);
        assert!((harmonic_match(1.0, 2.0) - 0.5).abs() < 1e-6);
        assert!(harmonic_match(1.0, 100.0) < 0.1);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!((cosine_similarity_abs(&a, &b) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![-1.0, 0.0, 0.0];
        assert!((cosine_similarity_abs(&a, &b) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_determine_connection_type_resonant() {
        assert_eq!(determine_connection_type(0.8, 0.8, 0.7, 0.7), 0);
    }

    #[test]
    fn test_determine_connection_type_harmonic() {
        assert_eq!(determine_connection_type(0.5, 0.5, 0.6, 0.5), 1);
    }

    #[test]
    fn test_determine_connection_type_entangled() {
        assert_eq!(determine_connection_type(0.7, 0.3, 0.4, 0.7), 2);
    }

    #[test]
    fn test_determine_connection_type_antiphase() {
        assert_eq!(determine_connection_type(0.3, 0.3, 0.3, 0.3), 3);
    }

    #[test]
    fn test_determine_connection_type_weak() {
        assert_eq!(determine_connection_type(0.5, 0.3, 0.4, 0.4), 4);
    }

    #[test]
    fn test_classify_resonance() {
        assert_eq!(classify_resonance(0.0), 0);
        assert_eq!(classify_resonance(0.1), 0);
        assert_eq!(classify_resonance(0.3), 1);
        assert_eq!(classify_resonance(0.6), 2);
        assert_eq!(classify_resonance(0.9), 3);
    }

    #[test]
    fn test_compute_spectrum_resonance_identical() {
        let a = make_test_entity(1, 1.0, 1.0, 1.0, [0.5; 22], 1.0, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 1.0, 1.0, 1.0, [0.5; 22], 1.0, 0.5, [0.5; 8]);
        assert!((compute_spectrum_resonance(&a, &b) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_compute_spectrum_resonance_max_diff() {
        let a = make_test_entity(1, 0.0, 1.0, 1.0, [0.5; 22], 1.0, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 20.0, 1.0, 1.0, [0.5; 22], 1.0, 0.5, [0.5; 8]);
        assert!((compute_spectrum_resonance(&a, &b) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_compute_polarity_resonance_neutral() {
        let a = make_test_entity(1, 1.0, 1.0, 1.0, [0.5; 22], 0.05, 0.5, [0.5; 8]);
        let b = make_test_entity(2, 1.0, 1.0, 1.0, [0.5; 22], 0.05, 0.5, [0.5; 8]);
        assert!((compute_polarity_resonance(&a, &b) - 0.5).abs() < 1e-6);
    }
}
