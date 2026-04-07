//! HoloSim Infinite — GPU vs CPU Benchmark Suite (Phase 5.7)
//!
//! Benchmarks 5 compute operations (connections, resonance, matmul, wavelet, upsample)
//! comparing CPU fallback implementations against GPU compute shaders.
//!
//! # Running
//! ```bash
//! # All benchmarks (CPU always runs, GPU skipped if unavailable)
//! cargo bench --bench gpu_bench
//!
//! # CPU only (faster, no GPU needed)
//! cargo bench --bench gpu_bench -- --profile-time=1
//!
//! # Dry-run to verify compilation
//! cargo bench --bench gpu_bench -- --noplot
//! ```
//!
//! # Output
//! - CPU time vs GPU time per operation
//! - Speedup factor (CPU_time / GPU_time)
//! - Memory transfer overhead (host→device + device→host)

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use holonic_realms::gpu::buffers::{GpuBuffer, StagingBuffer};
use holonic_realms::gpu::compute_engine::{
    cpu_compute_connections, cpu_compute_matmul, cpu_compute_resonance, cpu_compute_upsample,
    cpu_compute_wavelet,
};
use holonic_realms::gpu::connection_kernel::{ConnectionKernel, ConnectionWeights, ConnectionResult};
use holonic_realms::gpu::device::GpuContext;
use holonic_realms::gpu::entity_upload::EntityGPUData;
use holonic_realms::gpu::matmul_kernel::MatmulKernel;
use holonic_realms::gpu::resonance_kernel::{ResonanceKernel, ResonanceWeights, ResonanceResult};
use holonic_realms::gpu::upsample_kernel::UpsampleKernel;
use holonic_realms::gpu::wavelet_kernel::WaveletKernel;
use holonic_realms::gpu::ShaderComposer;

// ============================================================================
// Test data generation
// ============================================================================

/// Generate `n` random `EntityGPUData` structs with a fixed RNG seed.
fn generate_entities(n: usize) -> Vec<EntityGPUData> {
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    (0..n).map(|_| random_entity(&mut rng)).collect()
}

fn random_entity(rng: &mut ChaCha8Rng) -> EntityGPUData {
    let mut archetype = [0.0f32; 22];
    for a in &mut archetype {
        *a = rng.gen_range(0.0..1.0);
    }
    let mut signature = [0.0f32; 8];
    for s in &mut signature {
        *s = rng.gen_range(0.0..1.0);
    }

    EntityGPUData {
        entity_id_hash: rng.gen(),
        incarnation_number: rng.gen_range(1..100),
        _pad0: 0,
        spectrum_ratio: rng.gen_range(0.1..20.0),
        space_time_access: rng.gen_range(0.0..1.0),
        oneness_access: rng.gen_range(0.0..1.0),
        veil_transparency: rng.gen_range(0.0..1.0),
        frequency: rng.gen_range(0.5..10.0),
        coherence: rng.gen_range(0.0..1.0),
        _pad1: 0,
        archetype_activation: archetype,
        _pad2: [0.0; 2],
        polarity_direction: rng.gen_range(-1.0..1.0),
        polarity_strength: rng.gen_range(0.0..1.0),
        evolutionary_rate: rng.gen_range(0.0..1.0),
        _pad3: 0.0,
        holographic_signature: signature,
        density_level: rng.gen_range(1.0..8.0),
        _pad4: [0.0; 3],
        _pad5: [0.0; 2],
    }
}

/// Generate random f32 matrix data (row-major).
fn generate_matrix_data(rows: usize, cols: usize) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(123);
    (0..rows * cols).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

// ============================================================================
// GPU availability guard
// ============================================================================

static GPU_AVAILABILITY: std::sync::OnceLock<Option<GpuResources>> = std::sync::OnceLock::new();

struct GpuResources {
    context: GpuContext,
    #[allow(dead_code)]
    composer: ShaderComposer,
    connection_kernel: ConnectionKernel,
    resonance_kernel: ResonanceKernel,
    matmul_kernel: MatmulKernel,
    wavelet_kernel: WaveletKernel,
    upsample_kernel: UpsampleKernel,
}

fn gpu_resources() -> Option<&'static GpuResources> {
    GPU_AVAILABILITY.get_or_init(|| {
        // Use a fresh runtime for async initialization
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("GPU bench: Failed to create tokio runtime: {}", e);
                return None;
            }
        };

        rt.block_on(async {
            match GpuContext::new().await {
                Some(context) => {
                    eprintln!(
                        "GPU bench: GPU available — {:?}",
                        context.adapter_info().name
                    );
                    let composer = ShaderComposer::new();
                    Some(GpuResources {
                        connection_kernel: ConnectionKernel::from_composer(
                            &context.device,
                            &composer,
                        ),
                        resonance_kernel: ResonanceKernel::from_composer(
                            &context.device,
                            &composer,
                        ),
                        matmul_kernel: MatmulKernel::from_composer(&context.device, &composer),
                        wavelet_kernel: WaveletKernel::from_composer(&context.device, &composer),
                        upsample_kernel: UpsampleKernel::from_composer(&context.device, &composer),
                        context,
                        composer,
                    })
                }
                None => {
                    eprintln!("GPU bench: No GPU available — GPU benchmarks will be skipped");
                    None
                }
            }
        })
    })
    .as_ref()
}

// ============================================================================
// Connection Metrics Benchmark
// ============================================================================

fn benchmark_connections(c: &mut Criterion) {
    let mut group = c.benchmark_group("connections");
    group.sample_size(10);

    for &n in &[256, 512] {
        let entities = generate_entities(n);
        let weights = ConnectionWeights::default();

        // CPU
        group.bench_with_input(BenchmarkId::new("cpu", n), &n, |b, _| {
            b.iter_batched(
                || (entities.clone(), weights),
                |(ref ents, ref w)| cpu_compute_connections(ents, w),
                BatchSize::LargeInput,
            );
        });

        // GPU
        if let Some(gpu) = gpu_resources() {
            let device = &gpu.context.device;
            let queue = &gpu.context.queue;
            let kernel = &gpu.connection_kernel;

            let n_val = n;
            group.bench_with_input(
                BenchmarkId::new("gpu_full", n_val),
                &n_val,
                |b, _| {
                    let entities_clone = entities.clone();
                    let weights_clone = weights;

                    b.iter_batched(
                        || (),
                        |_| {
                            let entity_buf = GpuBuffer::from_slice(
                                device,
                                &entities_clone,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                                Some("bench_conn_entity"),
                            );

                            let output_count = ConnectionKernel::output_count(n_val);
                            let output_size = ConnectionKernel::output_size(n_val);
                            let output_buf = GpuBuffer::new(
                                device,
                                output_size as u64,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                                Some("bench_conn_output"),
                            );

                            let staging = StagingBuffer::new::<ConnectionResult>(device, output_count);

                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor {
                                    label: Some("bench_conn_encoder"),
                                },
                            );

                            kernel.dispatch(
                                device,
                                queue,
                                &mut encoder,
                                &entity_buf,
                                &output_buf,
                                n_val,
                                &weights_clone,
                            );

                            encoder.copy_buffer_to_buffer(
                                &output_buf.buffer,
                                0,
                                &staging.download().buffer,
                                0,
                                output_size as u64,
                            );

                            queue.submit(Some(encoder.finish()));
                            pollster::block_on(staging.read::<ConnectionResult>(device))
                        },
                        BatchSize::SmallInput,
                    );
                },
            );

            // GPU kernel-only (buffers created outside timing)
            group.bench_with_input(
                BenchmarkId::new("gpu_kernel_only", n_val),
                &n_val,
                |b, _| {
                    let entity_buf = GpuBuffer::from_slice(
                        device,
                        &entities,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        Some("bench_conn_entity_cached"),
                    );

                    let output_size = ConnectionKernel::output_size(n_val);
                    let output_buf = GpuBuffer::new(
                        device,
                        output_size as u64,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                        Some("bench_conn_output_cached"),
                    );

                    b.iter(|| {
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("bench_conn_kernel_only"),
                            },
                        );

                        kernel.dispatch(
                            device,
                            queue,
                            &mut encoder,
                            &entity_buf,
                            &output_buf,
                            n_val,
                            &weights,
                        );

                        queue.submit(Some(encoder.finish()));
                        device.poll(wgpu::Maintain::Wait);
                    });
                },
            );
        }
    }

    group.finish();
}

// ============================================================================
// Resonance Matrix Benchmark
// ============================================================================

fn benchmark_resonance(c: &mut Criterion) {
    let mut group = c.benchmark_group("resonance");
    group.sample_size(10);

    for &n in &[256, 512] {
        let entities = generate_entities(n);
        let weights = ResonanceWeights::default();

        // CPU
        group.bench_with_input(BenchmarkId::new("cpu", n), &n, |b, _| {
            b.iter_batched(
                || (entities.clone(), weights),
                |(ref ents, ref w)| cpu_compute_resonance(ents, w),
                BatchSize::LargeInput,
            );
        });

        // GPU
        if let Some(gpu) = gpu_resources() {
            let n_val = n;
            let device = &gpu.context.device;
            let queue = &gpu.context.queue;
            let kernel = &gpu.resonance_kernel;

            group.bench_with_input(
                BenchmarkId::new("gpu_full", n_val),
                &n_val,
                |b, _| {
                    let entities_clone = entities.clone();
                    let weights_clone = weights;

                    b.iter_batched(
                        || (),
                        |_| {
                            let entity_buf = GpuBuffer::from_slice(
                                device,
                                &entities_clone,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                                Some("bench_res_entity"),
                            );

                            let output_count = ResonanceKernel::output_count(n_val);
                            let output_size = ResonanceKernel::output_size(n_val);
                            let output_buf = GpuBuffer::new(
                                device,
                                output_size as u64,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                                Some("bench_res_output"),
                            );

                            let staging = StagingBuffer::new::<ResonanceResult>(device, output_count);

                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor {
                                    label: Some("bench_res_encoder"),
                                },
                            );

                            kernel.dispatch(
                                device,
                                queue,
                                &mut encoder,
                                &entity_buf,
                                &output_buf,
                                n_val,
                                &weights_clone,
                            );

                            encoder.copy_buffer_to_buffer(
                                &output_buf.buffer,
                                0,
                                &staging.download().buffer,
                                0,
                                output_size as u64,
                            );

                            queue.submit(Some(encoder.finish()));
                            pollster::block_on(staging.read::<ResonanceResult>(device))
                        },
                        BatchSize::SmallInput,
                    );
                },
            );

            // GPU kernel-only
            group.bench_with_input(
                BenchmarkId::new("gpu_kernel_only", n_val),
                &n_val,
                |b, _| {
                    let entity_buf = GpuBuffer::from_slice(
                        device,
                        &entities,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        Some("bench_res_entity_cached"),
                    );

                    let _output_count = ResonanceKernel::output_count(n_val);
                    let output_size = ResonanceKernel::output_size(n_val);
                    let output_buf = GpuBuffer::new(
                        device,
                        output_size as u64,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                        Some("bench_res_output_cached"),
                    );

                    b.iter(|| {
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("bench_res_kernel_only"),
                            },
                        );

                        kernel.dispatch(
                            device,
                            queue,
                            &mut encoder,
                            &entity_buf,
                            &output_buf,
                            n_val,
                            &weights,
                        );

                        queue.submit(Some(encoder.finish()));
                        device.poll(wgpu::Maintain::Wait);
                    });
                },
            );
        }
    }

    group.finish();
}

// ============================================================================
// Matrix Multiplication Benchmark
// ============================================================================

fn benchmark_matmul(c: &mut Criterion) {
    let mut group = c.benchmark_group("matmul");
    group.sample_size(10);

    // Test sizes: (rows_a, cols_a, cols_b)
    let sizes: &[(u32, u32, u32)] = &[(64, 64, 64), (128, 128, 128), (256, 256, 256)];

    for &(rows_a, cols_a, cols_b) in sizes {
        let size_label = format!("{}x{}", rows_a, cols_b);
        let mat_a = generate_matrix_data(rows_a as usize, cols_a as usize);
        let mat_b = generate_matrix_data(cols_a as usize, cols_b as usize);

        // CPU
        group.bench_with_input(
            BenchmarkId::new("cpu", &size_label),
            &(rows_a, cols_a, cols_b),
            |b, &(r_a, c_a, c_b)| {
                let a = mat_a.clone();
                let b_data = mat_b.clone();
                b.iter_batched(
                    || (a.clone(), b_data.clone()),
                    |(ref va, ref vb)| cpu_compute_matmul(va, vb, r_a, c_a, c_b),
                    BatchSize::LargeInput,
                );
            },
        );

        // GPU
        if let Some(gpu) = gpu_resources() {
            let device = &gpu.context.device;
            let queue = &gpu.context.queue;
            let kernel = &gpu.matmul_kernel;
            let label = size_label.clone();

            group.bench_with_input(
                BenchmarkId::new("gpu_full", &label),
                &(rows_a, cols_a, cols_b),
                |b, &(r_a, c_a, c_b)| {
                    let a_clone = mat_a.clone();
                    let b_clone = mat_b.clone();

                    b.iter_batched(
                        || (),
                        |_| {
                            let buf_a = GpuBuffer::from_slice(
                                device,
                                &a_clone,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                                Some("bench_matmul_a"),
                            );
                            let buf_b = GpuBuffer::from_slice(
                                device,
                                &b_clone,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                                Some("bench_matmul_b"),
                            );

                            let output_count = MatmulKernel::output_count(r_a, c_b);
                            let output_size = MatmulKernel::output_size(r_a, c_b);
                            let buf_c = GpuBuffer::new(
                                device,
                                output_size as u64,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                                Some("bench_matmul_c"),
                            );

                            let staging = StagingBuffer::new::<f32>(device, output_count);

                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor {
                                    label: Some("bench_matmul_encoder"),
                                },
                            );

                            kernel.dispatch(
                                device,
                                queue,
                                &mut encoder,
                                &buf_a,
                                &buf_b,
                                &buf_c,
                                r_a,
                                c_a,
                                c_b,
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
                        },
                        BatchSize::SmallInput,
                    );
                },
            );

            // GPU kernel-only
            group.bench_with_input(
                BenchmarkId::new("gpu_kernel_only", &label),
                &(rows_a, cols_a, cols_b),
                |b, &(r_a, c_a, c_b)| {
                    let buf_a = GpuBuffer::from_slice(
                        device,
                        &mat_a,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        Some("bench_matmul_a_cached"),
                    );
                    let buf_b = GpuBuffer::from_slice(
                        device,
                        &mat_b,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        Some("bench_matmul_b_cached"),
                    );

                    let _output_count = MatmulKernel::output_count(r_a, c_b);
                    let output_size = MatmulKernel::output_size(r_a, c_b);
                    let buf_c = GpuBuffer::new(
                        device,
                        output_size as u64,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                        Some("bench_matmul_c_cached"),
                    );

                    b.iter(|| {
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("bench_matmul_kernel_only"),
                            },
                        );

                        kernel.dispatch(
                            device,
                            queue,
                            &mut encoder,
                            &buf_a,
                            &buf_b,
                            &buf_c,
                            r_a,
                            c_a,
                            c_b,
                        );

                        queue.submit(Some(encoder.finish()));
                        device.poll(wgpu::Maintain::Wait);
                    });
                },
            );
        }
    }

    group.finish();
}

// ============================================================================
// Wavelet Decomposition Benchmark
// ============================================================================

fn benchmark_wavelet(c: &mut Criterion) {
    let mut group = c.benchmark_group("wavelet");
    group.sample_size(10);

    let sizes: &[(u32, u32)] = &[(64, 64), (128, 128), (256, 256)];

    for &(rows, cols) in sizes {
        let size_label = format!("{}x{}", rows, cols);
        let data = generate_matrix_data(rows as usize, cols as usize);

        // CPU
        group.bench_with_input(
            BenchmarkId::new("cpu", &size_label),
            &(rows, cols),
            |b, &(r, c)| {
                let input = data.clone();
                b.iter_batched(
                    || input.clone(),
                    |ref v| cpu_compute_wavelet(v, r, c),
                    BatchSize::LargeInput,
                );
            },
        );

        // GPU
        if let Some(gpu) = gpu_resources() {
            let device = &gpu.context.device;
            let queue = &gpu.context.queue;
            let kernel = &gpu.wavelet_kernel;
            let label = size_label.clone();

            group.bench_with_input(
                BenchmarkId::new("gpu_full", &label),
                &(rows, cols),
                |b, &(r, c)| {
                    let input_clone = data.clone();

                    b.iter_batched(
                        || (),
                        |_| {
                            let input_buf = GpuBuffer::from_slice(
                                device,
                                &input_clone,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                                Some("bench_wavelet_input"),
                            );

                            let output_count = WaveletKernel::output_count(r, c);
                            let output_size = WaveletKernel::output_size(r, c);
                            let output_buf = GpuBuffer::new(
                                device,
                                output_size as u64,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                                Some("bench_wavelet_output"),
                            );

                            let staging = StagingBuffer::new::<f32>(device, output_count);

                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor {
                                    label: Some("bench_wavelet_encoder"),
                                },
                            );

                            kernel.dispatch(
                                device,
                                queue,
                                &mut encoder,
                                &input_buf,
                                &output_buf,
                                r,
                                c,
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
                        },
                        BatchSize::SmallInput,
                    );
                },
            );

            // GPU kernel-only
            group.bench_with_input(
                BenchmarkId::new("gpu_kernel_only", &label),
                &(rows, cols),
                |b, &(r, c)| {
                    let input_buf = GpuBuffer::from_slice(
                        device,
                        &data,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        Some("bench_wavelet_input_cached"),
                    );

                    let _output_count = WaveletKernel::output_count(r, c);
                    let output_size = WaveletKernel::output_size(r, c);
                    let output_buf = GpuBuffer::new(
                        device,
                        output_size as u64,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                        Some("bench_wavelet_output_cached"),
                    );

                    b.iter(|| {
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("bench_wavelet_kernel_only"),
                            },
                        );

                        kernel.dispatch(
                            device,
                            queue,
                            &mut encoder,
                            &input_buf,
                            &output_buf,
                            r,
                            c,
                        );

                        queue.submit(Some(encoder.finish()));
                        device.poll(wgpu::Maintain::Wait);
                    });
                },
            );
        }
    }

    group.finish();
}

// ============================================================================
// Upsampling Benchmark
// ============================================================================

fn benchmark_upsample(c: &mut Criterion) {
    let mut group = c.benchmark_group("upsample");
    group.sample_size(10);

    let sizes: &[(u32, u32)] = &[(32, 32), (64, 64), (128, 128)];

    for &(input_rows, input_cols) in sizes {
        let size_label = format!("{}x{}", input_rows, input_cols);
        let data = generate_matrix_data(input_rows as usize, input_cols as usize);

        // CPU
        group.bench_with_input(
            BenchmarkId::new("cpu", &size_label),
            &(input_rows, input_cols),
            |b, &(ir, ic)| {
                let input = data.clone();
                b.iter_batched(
                    || input.clone(),
                    |ref v| cpu_compute_upsample(v, ir, ic),
                    BatchSize::LargeInput,
                );
            },
        );

        // GPU
        if let Some(gpu) = gpu_resources() {
            let device = &gpu.context.device;
            let queue = &gpu.context.queue;
            let kernel = &gpu.upsample_kernel;
            let label = size_label.clone();

            group.bench_with_input(
                BenchmarkId::new("gpu_full", &label),
                &(input_rows, input_cols),
                |b, &(ir, ic)| {
                    let input_clone = data.clone();

                    b.iter_batched(
                        || (),
                        |_| {
                            let input_buf = GpuBuffer::from_slice(
                                device,
                                &input_clone,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                                Some("bench_upsample_input"),
                            );

                            let output_count = UpsampleKernel::output_count(ir, ic);
                            let output_size = UpsampleKernel::output_size(ir, ic);
                            let output_buf = GpuBuffer::new(
                                device,
                                output_size as u64,
                                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                                Some("bench_upsample_output"),
                            );

                            let staging = StagingBuffer::new::<f32>(device, output_count);

                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor {
                                    label: Some("bench_upsample_encoder"),
                                },
                            );

                            kernel.dispatch(
                                device,
                                queue,
                                &mut encoder,
                                &input_buf,
                                &output_buf,
                                ir,
                                ic,
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
                        },
                        BatchSize::SmallInput,
                    );
                },
            );

            // GPU kernel-only
            group.bench_with_input(
                BenchmarkId::new("gpu_kernel_only", &label),
                &(input_rows, input_cols),
                |b, &(ir, ic)| {
                    let input_buf = GpuBuffer::from_slice(
                        device,
                        &data,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        Some("bench_upsample_input_cached"),
                    );

                    let _output_count = UpsampleKernel::output_count(ir, ic);
                    let output_size = UpsampleKernel::output_size(ir, ic);
                    let output_buf = GpuBuffer::new(
                        device,
                        output_size as u64,
                        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                        Some("bench_upsample_output_cached"),
                    );

                    b.iter(|| {
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("bench_upsample_kernel_only"),
                            },
                        );

                        kernel.dispatch(
                            device,
                            queue,
                            &mut encoder,
                            &input_buf,
                            &output_buf,
                            ir,
                            ic,
                        );

                        queue.submit(Some(encoder.finish()));
                        device.poll(wgpu::Maintain::Wait);
                    });
                },
            );
        }
    }

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    name = gpu_benchmarks;
    config = Criterion::default().sample_size(10);
    targets = benchmark_connections, benchmark_resonance, benchmark_matmul, benchmark_wavelet, benchmark_upsample
);

criterion_main!(gpu_benchmarks);
