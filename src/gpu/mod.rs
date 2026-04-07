//! GPU compute subsystem for HoloSim Infinite.
//!
//! Provides GPU acceleration for MERA tensor operations and entity processing
//! via wgpu compute shaders.
//!
//! # Module Organization
//!
//! - `device` — GPU device initialization and context management
//! - `buffers` — Typed GPU buffer wrappers with staging support
//! - `entity_upload` — Entity data extraction and GPU upload
//! - `shader_composer` — Compile-time WGSL composition with #include resolution
//! - `connection_kernel` — Pairwise connection metrics compute kernel
//! - `resonance_kernel` — Pairwise resonance metrics compute kernel
//! - `matmul_kernel` — Tiled matrix multiplication (GEMM) compute kernel
//! - `wavelet_kernel` — 2D Haar wavelet decomposition compute kernel
//! - `upsample_kernel` — Nearest-neighbor 2× upsampling compute kernel

pub mod buffers;
pub mod compute_engine;
pub mod connection_kernel;
pub mod device;
pub mod entity_upload;
pub mod matmul_kernel;
pub mod resonance_kernel;
pub mod upsample_kernel;
pub mod wavelet_kernel;

mod shader_composer;

pub use shader_composer::ShaderComposer;

// Re-export primary types for convenience
pub use buffers::{GpuBuffer, StagingBuffer};
pub use compute_engine::{ComputeBackend, ComputeEngine};
pub use connection_kernel::{ConnectionKernel, ConnectionResult, ConnectionWeights};
pub use device::GpuContext;
pub use entity_upload::{create_entity_buffer, extract_entity_data, EntityGPUData};
pub use matmul_kernel::{MatmulKernel, TensorShape};
pub use resonance_kernel::{ResonanceKernel, ResonanceResult, ResonanceWeights};
pub use upsample_kernel::{UpsampleKernel, UpsampleParams};
pub use wavelet_kernel::{WaveletKernel, WaveletParams};
