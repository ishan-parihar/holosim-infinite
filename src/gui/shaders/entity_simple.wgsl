struct VertexInput {
    @location(0) local_pos: vec2f,
}

struct InstanceInput {
    @location(1) position: vec2f,
    @location(2) size: f32,
    @location(3) color: vec3f,
    @location(4) glow: f32,
    @location(5) shape: u32,
}

struct VsOutput {
    @builtin(position) clip_pos: vec4f,
    @location(0) color: vec3f,
    @location(1) glow: f32,
    @location(2) local_pos: vec2f,
    @location(3) shape: u32,
}

@vertex
fn vs_main(input: VertexInput, instance: InstanceInput) -> VsOutput {
    let scaled = input.local_pos * instance.size;
    let world = scaled + instance.position;
    var out: VsOutput;
    out.clip_pos = vec4f(world, 0.0, 1.0);
    out.color = instance.color;
    out.glow = instance.glow;
    out.local_pos = input.local_pos;
    out.shape = instance.shape;
    return out;
}

// ── SDF Primitives ──

fn dot2(v: vec2f) -> f32 {
    return dot(v, v);
}

/// SDF for circle of radius r.
fn sd_circle(p: vec2f, r: f32) -> f32 {
    return length(p) - r;
}

/// SDF for equilateral triangle centered at origin, fitting in unit circle.
/// Vertices at: top(0, 1), bottom-right(√3/2, -0.5), bottom-left(-√3/2, -0.5)
fn sd_triangle(p: vec2f) -> f32 {
    let sqrt3: f32 = 1.7320508;
    let q = vec2f(abs(p.x), p.y);
    let d0 = q.y - 1.0;
    let d1 = sqrt3 * 0.5 * q.x - 0.5 * q.y - 0.5;
    let d2 = -sqrt3 * 0.5 * q.x - 0.5 * q.y - 0.5;
    let inside = min(max(d1, d2), d0);
    let outside = min(dot2(vec2f(sqrt3 * 0.5 * q.x - 0.5 * q.y - 0.5, sqrt3 * 0.5 * q.x + 0.5 * q.y + 0.5)),
                      min(dot2(vec2f(-sqrt3 * 0.5 * q.x - 0.5 * q.y - 0.5, -sqrt3 * 0.5 * q.x + 0.5 * q.y + 0.5)),
                          dot2(vec2f(q.x, q.y - 1.0))));
    return select(sqrt(outside), inside, inside < 0.0);
}

/// SDF for axis-aligned box filling the unit quad [-1,1]².
fn sd_box(p: vec2f) -> f32 {
    let d = abs(p) - vec2f(1.0, 1.0);
    return length(max(d, vec2f(0.0))) + min(max(d.x, d.y), 0.0);
}

/// SDF for diamond (rotated square), vertices at (0,1), (1,0), (0,-1), (-1,0).
fn sd_diamond(p: vec2f) -> f32 {
    let q = abs(p);
    return (q.x + q.y) - 1.0;
}

/// SDF for flat-topped regular hexagon inscribed in unit circle.
fn sd_hexagon(p: vec2f) -> f32 {
    let q = abs(p);
    let hex = max(q.x * 0.8660254 + q.y * 0.5, q.y);
    return hex - 0.8660254;
}

/// SDF for 5-pointed star inscribed in unit circle.
fn sd_star(p: vec2f) -> f32 {
    let r = length(p);
    if (r < 0.0001) {
        return -1.0; // center is well inside
    }
    var a = atan2(p.x, p.y) + 3.14159265;
    let seg = floor(a * 2.5 / 3.14159265);
    a = a - seg * 1.25663706; // 2π/5
    a = a - 0.62831853; // π/5
    if (a > 0.31415926) { // π/10
        a = 0.62831853 - a;
    }
    let cos_a = cos(a);
    let sin_a = sin(a);
    let inner_r = 0.381966; // 1/φ²
    let star_r = inner_r / (inner_r * cos_a + 0.9238795 * sin_a);
    return r - star_r;
}

@fragment
fn fs_main(input: VsOutput) -> @location(0) vec4f {
    let p = input.local_pos;
    let aa: f32 = 0.03; // anti-aliasing width

    var d: f32;

    switch input.shape {
        case 0u: { // Circle
            d = sd_circle(p, 1.0);
        }
        case 1u: { // Triangle
            d = sd_triangle(p);
        }
        case 2u: { // Square
            d = sd_box(p);
        }
        case 3u: { // Diamond
            d = sd_diamond(p);
        }
        case 4u: { // Hexagon
            d = sd_hexagon(p);
        }
        case 5u: { // Star
            d = sd_star(p);
        }
        default: {
            d = sd_circle(p, 1.0);
        }
    }

    // Anti-aliased alpha: smooth transition at shape boundary
    let alpha = 1.0 - smoothstep(-aa, aa, d);

    return vec4f(input.color, alpha);
}
