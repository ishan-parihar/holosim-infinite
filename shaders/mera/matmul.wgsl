// ============================================================================
// MERA Matrix Multiply — HoloSim Infinite
// ============================================================================
// Tiled GEMM (General Matrix Multiply) using shared memory for performance.
// Computes C = A × B where A is (rows_a × cols_a) and B is (rows_b × cols_b).
// Output C is (rows_a × cols_b).
// TILE_SIZE = 16, workgroup_size(16, 16, 1).
// ============================================================================

#include "common/types.wgsl"
#include "common/math.wgsl"

// Tensor shape uniform — matches Rust TensorShape (16 bytes).
struct TensorShape {
    rows: u32,
    cols: u32,
    depth: u32,
    _pad: u32,
}

// ============================================================================
// Buffer Bindings
// ============================================================================
@group(0) @binding(0) var<storage, read> matrix_a: array<f32>;
@group(0) @binding(1) var<storage, read> matrix_b: array<f32>;
@group(0) @binding(2) var<storage, read_write> matrix_c: array<f32>;
@group(0) @binding(3) var<uniform> shape_a: TensorShape;
@group(0) @binding(4) var<uniform> shape_b: TensorShape;

// ============================================================================
// Tiled Shared Memory
// ============================================================================
const TILE_SIZE: u32 = 16u;

var<workgroup> tile_a: array<f32, 256>;  // 16×16 tile of A
var<workgroup> tile_b: array<f32, 256>;  // 16×16 tile of B

// ============================================================================
// Main — Tiled Matrix Multiply
// ============================================================================
@compute @workgroup_size(16, 16, 1)
fn main_matmul(@builtin(global_invocation_id) gid: vec3<u32>,
               @builtin(local_invocation_id) lid: vec3<u32>,
               @builtin(workgroup_id) wid: vec3<u32>) {
    let row: u32 = gid.y;  // output row index (0..rows_a)
    let col: u32 = gid.x;  // output col index (0..cols_b)

    let local_x: u32 = lid.x;
    let local_y: u32 = lid.y;

    var sum: f32 = 0.0;

    let num_tiles: u32 = (shape_a.cols + TILE_SIZE - 1u) / TILE_SIZE;

    for (var t: u32 = 0u; t < num_tiles; t = t + 1u) {
        // Load tile from A: A[row, t * TILE_SIZE + local_x]
        let a_col: u32 = t * TILE_SIZE + local_x;
        if (row < shape_a.rows && a_col < shape_a.cols) {
            tile_a[local_y * TILE_SIZE + local_x] =
                matrix_a[row * shape_a.cols + a_col];
        } else {
            tile_a[local_y * TILE_SIZE + local_x] = 0.0;
        }

        // Load tile from B: B[t * TILE_SIZE + local_y, col]
        let b_row: u32 = t * TILE_SIZE + local_y;
        if (b_row < shape_b.rows && col < shape_b.cols) {
            tile_b[local_y * TILE_SIZE + local_x] =
                matrix_b[b_row * shape_b.cols + col];
        } else {
            tile_b[local_y * TILE_SIZE + local_x] = 0.0;
        }

        workgroupBarrier();

        // Accumulate partial dot product
        for (var k: u32 = 0u; k < TILE_SIZE; k = k + 1u) {
            sum = sum + tile_a[local_y * TILE_SIZE + k] *
                         tile_b[k * TILE_SIZE + local_x];
        }

        workgroupBarrier();
    }

    // Write result
    if (row < shape_a.rows && col < shape_b.cols) {
        matrix_c[row * shape_b.cols + col] = sum;
    }
}
