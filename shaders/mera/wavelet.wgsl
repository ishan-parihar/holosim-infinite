// ============================================================================
// MERA Wavelet Transform — HoloSim Infinite
// ============================================================================
// 2D Haar wavelet decomposition on a matrix.
// For each 2×2 block at (r*2, c*2):
//   LL = (a + b + c + d) / 4   (approximation)
//   LH = (a - b + c - d) / 4   (horizontal detail)
//   HL = (a + b - c - d) / 4   (vertical detail)
//   HH = (a - b - c + d) / 4   (diagonal detail)
// Output quadrants: top-left=LL, top-right=LH, bottom-left=HL, bottom-right=HH
// workgroup_size(16, 16, 1) — each invocation processes one 2×2 block.
// ============================================================================

#include "common/types.wgsl"
#include "common/math.wgsl"

// Wavelet parameters — 16 bytes.
struct WaveletParams {
    rows: u32,
    cols: u32,
    _pad: u32,
    _pad2: u32,
}

// ============================================================================
// Buffer Bindings
// ============================================================================
@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;
@group(0) @binding(2) var<uniform> params: WaveletParams;

// ============================================================================
// Main — 2D Haar Wavelet
// ============================================================================
@compute @workgroup_size(16, 16, 1)
fn main_wavelet(@builtin(global_invocation_id) gid: vec3<u32>) {
    let col: u32 = gid.x;
    let row: u32 = gid.y;

    // Each invocation handles a 2×2 block
    let r0: u32 = row * 2u;
    let c0: u32 = col * 2u;

    // Boundary check
    if (r0 >= params.rows || c0 >= params.cols) {
        return;
    }

    // Clamp to valid indices for non-even dimensions
    let r1: u32 = min(r0 + 1u, params.rows - 1u);
    let c1: u32 = min(c0 + 1u, params.cols - 1u);

    // Read 2×2 block
    let a: f32 = input[r0 * params.cols + c0];  // top-left
    let b: f32 = input[r0 * params.cols + c1];  // top-right
    let c: f32 = input[r1 * params.cols + c0];  // bottom-left
    let d: f32 = input[r1 * params.cols + c1];  // bottom-right

    // Haar wavelet coefficients
    let ll: f32 = (a + b + c + d) * 0.25;  // approximation
    let lh: f32 = (a - b + c - d) * 0.25;  // horizontal detail
    let hl: f32 = (a + b - c - d) * 0.25;  // vertical detail
    let hh: f32 = (a - b - c + d) * 0.25;  // diagonal detail

    // Write to quadrants
    let half_rows: u32 = (params.rows + 1u) / 2u;
    let half_cols: u32 = (params.cols + 1u) / 2u;

    // Top-left quadrant: LL
    output[row * half_cols + col] = ll;

    // Top-right quadrant: LH
    if (col < half_cols) {
        output[row * half_cols + half_cols + col] = lh;
    }

    // Bottom-left quadrant: HL
    if (row < half_rows) {
        output[(half_rows + row) * half_cols + col] = hl;
    }

    // Bottom-right quadrant: HH
    if (row < half_rows && col < half_cols) {
        output[(half_rows + row) * half_cols + half_cols + col] = hh;
    }
}
