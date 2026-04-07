// ============================================================================
// MERA Upsample — HoloSim Infinite
// ============================================================================
// Nearest-neighbor 2× upsampling.
// Output dimensions are 2× input in each direction.
// output[gy, gx] = input[gy/2, gx/2]
// workgroup_size(16, 16, 1) — each invocation writes one output pixel.
// ============================================================================

#include "common/types.wgsl"
#include "common/math.wgsl"

// Upsample parameters — 16 bytes.
struct UpsampleParams {
    input_rows: u32,
    input_cols: u32,
    _pad: u32,
    _pad2: u32,
}

// ============================================================================
// Buffer Bindings
// ============================================================================
@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;
@group(0) @binding(2) var<uniform> params: UpsampleParams;

// ============================================================================
// Main — Nearest-Neighbor 2× Upsample
// ============================================================================
@compute @workgroup_size(16, 16, 1)
fn main_upsample(@builtin(global_invocation_id) gid: vec3<u32>) {
    let gx: u32 = gid.x;  // output column
    let gy: u32 = gid.y;  // output row

    let output_cols: u32 = params.input_cols * 2u;
    let output_rows: u32 = params.input_rows * 2u;

    // Boundary check
    if (gy >= output_rows || gx >= output_cols) {
        return;
    }

    // Nearest-neighbor: sample from input at (gy/2, gx/2)
    let src_row: u32 = gy / 2u;
    let src_col: u32 = gx / 2u;

    output[gy * output_cols + gx] = input[src_row * params.input_cols + src_col];
}
