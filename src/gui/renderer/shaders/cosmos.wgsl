//! Cosmos Shader - Renders cosmic structures at multiple scales
//!
//! From HOLOSIM_VISUALIZATION_INTEGRATION_ROADMAP.md Phase 3:
//! "Renders the cosmic hierarchy: Universe → Galaxy → Solar System → Planet"
//!
//! This shader provides:
//! - Star rendering with luminosity-based glow
//! - Planet rendering with orbital positions
//! - Orbital path visualization
//! - Cosmic filament lines

// ============================================================================
// Uniforms
// ============================================================================

/// Camera uniform for cosmos rendering
struct CameraUniform {
    /// View-projection matrix (64 bytes)
    view_proj: mat4x4<f32>,
    /// Camera position (12 bytes) + time (4 bytes) = 16 bytes
    camera_position: vec3<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// ============================================================================
// Star Rendering
// ============================================================================

/// Star vertex data
struct StarVertex {
    /// Position in world space
    @location(0) position: vec3<f32>,
    /// Star color (RGB, derived from spectral class)
    @location(1) color: vec3<f32>,
    /// Luminosity relative to Sun (affects glow size)
    @location(2) luminosity: f32,
    /// Temperature in Kelvin (affects color temperature)
    @location(3) temperature: f32,
    /// Radius relative to Sun
    @location(4) radius: f32,
    /// Evolution stage (0=ProtoStar, 1=MainSequence, etc.)
    @location(5) stage: u32,
};

struct StarVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) luminosity: f32,
    @location(3) temperature: f32,
};

/// Star vertex shader
///
/// Renders stars as point sprites with size based on luminosity and distance.
/// Points are rendered as quads with UV coordinates for glow effect.
@vertex
fn vs_star(vertex: StarVertex) -> StarVertexOutput {
    var output: StarVertexOutput;
    
    // Transform position to clip space
    output.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    
    // Pass through color and properties
    output.color = vertex.color;
    output.luminosity = vertex.luminosity;
    output.temperature = vertex.temperature;
    
    // UV coordinates for point sprite (centered at origin)
    // These will be set by the geometry shader or instanced quad
    output.uv = vec2<f32>(0.0, 0.0);
    
    return output;
}

/// Star fragment shader
///
/// Creates a glow effect for stars based on luminosity and temperature.
/// Uses a Gaussian falloff for realistic stellar appearance.
@fragment
fn fs_star(input: StarVertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from center for glow effect
    let dist = length(input.uv - vec2<f32>(0.5, 0.5));
    
    // Gaussian falloff for glow
    let sigma = 0.2;
    let glow = exp(-dist * dist / (2.0 * sigma * sigma));
    
    // Intensity based on luminosity (log scale for better visibility)
    let intensity = 0.5 + 0.5 * log2(input.luminosity + 1.0) / 10.0;
    
    // Color temperature adjustment
    // Higher temperature = bluer, lower = redder
    let temp_normalized = clamp((input.temperature - 3000.0) / 30000.0, 0.0, 1.0);
    let color_temp = vec3<f32>(
        1.0 - 0.3 * temp_normalized,
        0.8 + 0.2 * temp_normalized,
        0.6 + 0.4 * temp_normalized
    );
    
    // Final color with glow
    let final_color = input.color * color_temp * glow * intensity;
    
    // Alpha based on glow
    let alpha = glow * intensity;
    
    return vec4<f32>(final_color, alpha);
}

// ============================================================================
// Planet Rendering
// ============================================================================

/// Planet vertex data
struct PlanetVertex {
    /// Position in world space
    @location(0) position: vec3<f32>,
    /// Normal for lighting
    @location(1) normal: vec3<f32>,
    /// UV coordinates
    @location(2) uv: vec2<f32>,
    /// Planet type (0=Terrestrial, 1=GasGiant, 2=IceGiant, 3=Dwarf)
    @location(3) planet_type: u32,
    /// Orbital radius in AU
    @location(4) orbital_radius: f32,
    /// Planet radius relative to Earth
    @location(5) radius: f32,
    /// Surface temperature
    @location(6) temperature: f32,
};

struct PlanetVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) planet_type: u32,
    @location(3) temperature: f32,
    @location(4) world_position: vec3<f32>,
};

/// Planet vertex shader
///
/// Renders planets as spheres with lighting from the central star.
@vertex
fn vs_planet(vertex: PlanetVertex) -> PlanetVertexOutput {
    var output: PlanetVertexOutput;
    
    // Scale planet based on radius
    let scale = vertex.radius * 0.01; // Normalize to visible size
    let scaled_position = vertex.position * scale;
    
    // Transform to clip space
    output.clip_position = camera.view_proj * vec4<f32>(scaled_position, 1.0);
    
    // Pass through properties
    output.normal = normalize(vertex.normal);
    output.uv = vertex.uv;
    output.planet_type = vertex.planet_type;
    output.temperature = vertex.temperature;
    output.world_position = scaled_position;
    
    return output;
}

/// Planet fragment shader
///
/// Renders planet surface with type-based coloring and lighting.
@fragment
fn fs_planet(input: PlanetVertexOutput) -> @location(0) vec4<f32> {
    // Base color based on planet type
    let base_color = select_planet_color(input.planet_type, input.temperature, input.uv);
    
    // Simple directional light from star (positioned at origin)
    let light_dir = normalize(-input.world_position);
    let diffuse = max(0.0, dot(input.normal, light_dir));
    
    // Ambient light
    let ambient = 0.2;
    
    // Final lighting
    let lighting = ambient + diffuse * 0.8;
    
    // Apply lighting to base color
    let final_color = base_color * lighting;
    
    return vec4<f32>(final_color, 1.0);
}

/// Select planet color based on type and temperature
fn select_planet_color(planet_type: u32, temperature: f32, uv: vec2<f32>) -> vec3<f32> {
    switch (planet_type) {
        case 0u: {
            // Terrestrial - brown/green based on temperature
            if (temperature < 200.0) {
                return vec3<f32>(0.9, 0.9, 0.95); // Ice world
            } else if (temperature < 300.0) {
                return vec3<f32>(0.3, 0.4, 0.6); // Cold water world
            } else if (temperature < 350.0) {
                return vec3<f32>(0.2, 0.5, 0.3); // Temperate
            } else {
                return vec3<f32>(0.7, 0.4, 0.2); // Hot desert
            }
        }
        case 1u: {
            // Gas giant - banded colors
            let band = sin(uv.y * 20.0) * 0.5 + 0.5;
            return mix(
                vec3<f32>(0.8, 0.6, 0.4), // Orange
                vec3<f32>(0.9, 0.8, 0.6), // Tan
                band
            );
        }
        case 2u: {
            // Ice giant - blue/cyan
            return vec3<f32>(0.4, 0.6, 0.9);
        }
        case 3u: {
            // Dwarf - gray/brown
            return vec3<f32>(0.5, 0.45, 0.4);
        }
        default: {
            return vec3<f32>(0.5, 0.5, 0.5);
        }
    }
}

// ============================================================================
// Orbit Path Rendering
// ============================================================================

/// Orbit vertex data
struct OrbitVertex {
    /// Position on orbital ellipse
    @location(0) position: vec3<f32>,
    /// Color for the orbit line
    @location(1) color: vec3<f32>,
    /// Alpha for fading
    @location(2) alpha: f32,
};

struct OrbitVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) alpha: f32,
};

/// Orbit vertex shader
///
/// Renders orbital paths as elliptical lines.
@vertex
fn vs_orbit(vertex: OrbitVertex) -> OrbitVertexOutput {
    var output: OrbitVertexOutput;
    output.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    output.color = vertex.color;
    output.alpha = vertex.alpha;
    return output;
}

/// Orbit fragment shader
@fragment
fn fs_orbit(input: OrbitVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, input.alpha);
}

// ============================================================================
// Filament Rendering
// ============================================================================

/// Filament vertex data
struct FilamentVertex {
    /// Position on filament line
    @location(0) position: vec3<f32>,
    /// Color based on density
    @location(1) color: vec3<f32>,
    /// Density contrast (affects alpha)
    @location(2) density: f32,
};

struct FilamentVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) density: f32,
};

/// Filament vertex shader
///
/// Renders cosmic web filaments as glowing lines.
@vertex
fn vs_filament(vertex: FilamentVertex) -> FilamentVertexOutput {
    var output: FilamentVertexOutput;
    output.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    output.color = vertex.color;
    output.density = vertex.density;
    return output;
}

/// Filament fragment shader
@fragment
fn fs_filament(input: FilamentVertexOutput) -> @location(0) vec4<f32> {
    // Color based on density (higher density = brighter)
    let intensity = clamp(input.density / 50.0, 0.1, 1.0);
    let final_color = input.color * intensity;
    return vec4<f32>(final_color, intensity * 0.5);
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Mix two vectors
fn mix(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a * (1.0 - t) + b * t;
}

/// Log base 2
fn log2(x: f32) -> f32 {
    return log(x) / log(2.0);
}
