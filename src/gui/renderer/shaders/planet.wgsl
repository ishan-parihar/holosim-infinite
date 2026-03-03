//! Planet Surface Shader - Renders planetary surfaces with dynamic systems
//!
//! From HOLOSIM_VISUALIZATION_INTEGRATION_ROADMAP.md Phase 4:
//! "Visualizes the four planetary subsystems:
//! - Lithosphere: Tectonic plates, volcanoes, terrain, earthquakes
//! - Hydrosphere: Oceans, rivers, lakes, glaciers, water cycle
//! - Atmosphere: Clouds, storms, weather fronts, wind
//! - EnergyFlow: Solar radiation, day/night, seasons"
//!
//! RENDER PASSES:
//! 1. Terrain (from Lithosphere) - height-based coloring, tectonic features
//! 2. Water (from Hydrosphere) - ocean surface with depth-based color
//! 3. Clouds (from Atmosphere) - rotating cloud layer
//! 4. Atmosphere glow - Rayleigh scattering
//! 5. Storms (from Atmosphere) - spiral weather systems

// ============================================================================
// Uniforms
// ============================================================================

/// Planet uniform for all surface rendering
struct PlanetUniform {
    /// View-projection matrix (64 bytes)
    view_proj: mat4x4<f32>,
    /// Planet rotation matrix (64 bytes)
    rotation: mat4x4<f32>,
    /// Time for animations (4 bytes)
    time: f32,
    /// Axial tilt in radians (4 bytes)
    axial_tilt: f32,
    /// Angle to star for day/night (4 bytes)
    solar_angle: f32,
    /// Planet radius (4 bytes)
    radius: f32,
    /// Camera position (12 bytes)
    camera_pos: vec3<f32>,
    /// Scale level (0=planet, 1=continent, 2=region, 3=local) (4 bytes)
    scale_level: u32,
    /// Padding (8 bytes)
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> planet: PlanetUniform;

// Texture and sampler at bind group 1
// Used for terrain heightmap in terrain shader
// Used for cloud coverage in cloud shader
@group(1) @binding(0)
var heightmap_texture: texture_2d<f32>;

@group(1) @binding(1)
var texture_sampler: sampler;

// ============================================================================
// Terrain Types (from lithosphere)
// ============================================================================

const TERRAIN_DEEP_OCEAN: f32 = -1.0;
const TERRAIN_OCEAN: f32 = -0.5;
const TERRAIN_COASTAL: f32 = 0.0;
const TERRAIN_BEACH: f32 = 0.05;
const TERRAIN_PLAINS: f32 = 0.2;
const TERRAIN_HILLS: f32 = 0.4;
const TERRAIN_MOUNTAINS: f32 = 0.6;
const TERRAIN_HIGH_PEAKS: f32 = 0.8;
const TERRAIN_VOLCANIC: f32 = 0.9;
const TERRAIN_ICE: f32 = 1.0;

// ============================================================================
// TERRAIN RENDERING
// ============================================================================

struct TerrainVertex {
    /// Position on sphere
    @location(0) position: vec3<f32>,
    /// Surface normal
    @location(1) normal: vec3<f32>,
    /// UV coordinates
    @location(2) uv: vec2<f32>,
    /// Tectonic plate ID
    @location(3) plate_id: f32,
    /// Volcanic activity level
    @location(4) volcanic_activity: f32,
};

struct TerrainOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) volcanic_activity: f32,
    @location(4) latitude: f32,
};

@vertex
fn vs_terrain(vertex: TerrainVertex) -> TerrainOutput {
    var output: TerrainOutput;
    
    // Apply planet rotation
    let rotated_pos = planet.rotation * vec4<f32>(vertex.position, 1.0);
    
    // Scale by planet radius
    let scaled_pos = rotated_pos.xyz * planet.radius;
    
    output.position = planet.view_proj * vec4<f32>(scaled_pos, 1.0);
    output.normal = (planet.rotation * vec4<f32>(vertex.normal, 0.0)).xyz;
    output.uv = vertex.uv;
    output.world_pos = scaled_pos;
    output.volcanic_activity = vertex.volcanic_activity;
    output.latitude = asin(normalize(scaled_pos).y);
    
    return output;
}

@fragment
fn fs_terrain(input: TerrainOutput) -> @location(0) vec4<f32> {
    // Sample heightmap
    let height = textureSample(heightmap_texture, texture_sampler, input.uv).r;
    
    // Determine terrain type color
    var color: vec3<f32>;
    
    if (height < TERRAIN_COASTAL) {
        // Ocean - handled by water pass, show base color
        color = vec3<f32>(0.0, 0.1, 0.3);
    } else if (height < TERRAIN_BEACH) {
        // Beach/coastal
        color = vec3<f32>(0.76, 0.7, 0.5);
    } else if (height < TERRAIN_HILLS) {
        // Plains to hills - grassland to forest
        let t = (height - TERRAIN_PLAINS) / (TERRAIN_HILLS - TERRAIN_PLAINS);
        color = mix(
            vec3<f32>(0.3, 0.6, 0.2),  // Grassland
            vec3<f32>(0.15, 0.4, 0.15),  // Forest
            t
        );
    } else if (height < TERRAIN_HIGH_PEAKS) {
        // Mountains
        let t = (height - TERRAIN_MOUNTAINS) / (TERRAIN_HIGH_PEAKS - TERRAIN_MOUNTAINS);
        color = mix(
            vec3<f32>(0.4, 0.35, 0.3),  // Rock
            vec3<f32>(0.95, 0.95, 1.0),  // Snow
            t
        );
    } else {
        // High peaks / ice
        color = vec3<f32>(0.95, 0.95, 1.0);
    }
    
    // Add volcanic coloring
    if (input.volcanic_activity > 0.5) {
        let glow = sin(planet.time * 2.0) * 0.1 + 0.9;
        color = mix(color, vec3<f32>(0.8, 0.2, 0.0), input.volcanic_activity * glow);
    }
    
    // Lighting from star
    let light_dir = vec3<f32>(cos(planet.solar_angle), 0.3, sin(planet.solar_angle));
    let diffuse = max(0.0, dot(normalize(input.normal), normalize(light_dir)));
    let ambient = 0.15;
    
    // Latitude-based ice caps
    let ice_factor = smoothstep(0.7, 0.9, abs(input.latitude) / 1.5708);
    color = mix(color, vec3<f32>(0.95, 0.95, 1.0), ice_factor * 0.8);
    
    // Final color
    let final_color = color * (ambient + diffuse * 0.85);
    
    return vec4<f32>(final_color, 1.0);
}

// ============================================================================
// WATER RENDERING
// ============================================================================

struct WaterVertex {
    /// Position on sphere
    @location(0) position: vec3<f32>,
    /// Surface normal
    @location(1) normal: vec3<f32>,
    /// UV coordinates
    @location(2) uv: vec2<f32>,
    /// Water depth (from hydrosphere)
    @location(3) depth: f32,
};

struct WaterOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) depth: f32,
};

@vertex
fn vs_water(vertex: WaterVertex) -> WaterOutput {
    var output: WaterOutput;
    
    // Apply wave animation
    let wave_height = sin(vertex.uv.x * 20.0 + planet.time * 2.0) * 0.001;
    let wave_height2 = sin(vertex.uv.y * 15.0 + planet.time * 1.5) * 0.0008;
    
    var pos = vertex.position;
    pos = pos + vertex.normal * (wave_height + wave_height2);
    
    let rotated_pos = planet.rotation * vec4<f32>(pos, 1.0);
    let scaled_pos = rotated_pos.xyz * planet.radius;
    
    output.position = planet.view_proj * vec4<f32>(scaled_pos, 1.0);
    output.normal = (planet.rotation * vec4<f32>(vertex.normal, 0.0)).xyz;
    output.uv = vertex.uv;
    output.world_pos = scaled_pos;
    output.depth = vertex.depth;
    
    return output;
}

@fragment
fn fs_water(input: WaterOutput) -> @location(0) vec4<f32> {
    // Water color based on depth
    let shallow_color = vec3<f32>(0.1, 0.5, 0.6);
    let deep_color = vec3<f32>(0.0, 0.15, 0.3);
    
    let depth_factor = smoothstep(0.0, 1.0, input.depth);
    let water_color = mix(shallow_color, deep_color, depth_factor);
    
    // Specular highlight from star
    let light_dir = vec3<f32>(cos(planet.solar_angle), 0.3, sin(planet.solar_angle));
    let view_dir = normalize(planet.camera_pos - input.world_pos);
    let half_vec = normalize(light_dir + view_dir);
    let specular = pow(max(0.0, dot(input.normal, half_vec)), 64.0);
    
    // Fresnel effect
    let fresnel = pow(1.0 - max(0.0, dot(input.normal, view_dir)), 3.0);
    
    // Final color
    let color = water_color + specular * 0.5 + fresnel * vec3<f32>(0.3, 0.5, 0.7) * 0.3;
    
    return vec4<f32>(color, 0.85);
}

// ============================================================================
// CLOUD RENDERING
// ============================================================================

struct CloudVertex {
    /// Position on sphere (slightly above surface)
    @location(0) position: vec3<f32>,
    /// UV coordinates
    @location(1) uv: vec2<f32>,
    /// Cloud density
    @location(2) density: f32,
};

struct CloudOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) density: f32,
};

@vertex
fn vs_cloud(vertex: CloudVertex) -> CloudOutput {
    var output: CloudOutput;
    
    // Clouds slightly above surface
    let cloud_height = 0.02;
    let pos = vertex.position * (1.0 + cloud_height);
    
    // Rotate clouds independently (faster than planet)
    let cloud_rotation = planet.time * 0.02;
    let rotated = vec3<f32>(
        pos.x * cos(cloud_rotation) - pos.z * sin(cloud_rotation),
        pos.y,
        pos.x * sin(cloud_rotation) + pos.z * cos(cloud_rotation)
    );
    
    let scaled_pos = rotated * planet.radius;
    
    output.position = planet.view_proj * vec4<f32>(scaled_pos, 1.0);
    output.uv = vertex.uv + vec2<f32>(planet.time * 0.01, 0.0);
    output.world_pos = scaled_pos;
    output.density = vertex.density;
    
    return output;
}

@fragment
fn fs_cloud(input: CloudOutput) -> @location(0) vec4<f32> {
    // Sample cloud texture
    let cloud = textureSample(heightmap_texture, texture_sampler, input.uv);
    
    // Cloud density
    let density = cloud.r * input.density;
    
    // Only render where there are clouds
    if (density < 0.1) {
        discard;
    }
    
    // Cloud color (white to gray)
    let cloud_color = mix(
        vec3<f32>(0.9, 0.9, 0.95),
        vec3<f32>(0.6, 0.6, 0.65),
        density
    );
    
    // Lighting
    let light_dir = vec3<f32>(cos(planet.solar_angle), 0.3, sin(planet.solar_angle));
    let normal = normalize(input.world_pos);
    let diffuse = max(0.0, dot(normal, light_dir));
    
    let color = cloud_color * (0.3 + diffuse * 0.7);
    
    return vec4<f32>(color, density * 0.7);
}

// ============================================================================
// ATMOSPHERE GLOW
// ============================================================================

struct AtmosphereOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

// Generate fullscreen quad from vertex index
@vertex
fn vs_atmosphere(@builtin(vertex_index) vertex_index: u32) -> AtmosphereOutput {
    var output: AtmosphereOutput;
    
    // Generate fullscreen quad coordinates
    let x = f32(vertex_index & 1u) * 2.0 - 1.0;
    let y = f32((vertex_index >> 1u) & 1u) * 2.0 - 1.0;
    
    // UV coordinates
    output.uv = vec2<f32>((x + 1.0) * 0.5, (y + 1.0) * 0.5);
    
    // Position (fullscreen quad)
    output.position = vec4<f32>(x, y, 0.999, 1.0);
    
    // World position on sphere (for atmosphere calculation)
    let theta = output.uv.x * 3.14159 * 2.0;
    let phi = output.uv.y * 3.14159;
    output.world_pos = vec3<f32>(
        sin(phi) * cos(theta),
        cos(phi),
        sin(phi) * sin(theta)
    ) * planet.radius * (1.0 + 0.05);  // Slightly above surface
    
    output.normal = normalize(output.world_pos);
    
    return output;
}

@fragment
fn fs_atmosphere(input: AtmosphereOutput) -> @location(0) vec4<f32> {
    // Rayleigh scattering approximation
    let view_dir = normalize(planet.camera_pos - input.world_pos);
    
    // Scattering color (blue sky)
    let scatter_color = vec3<f32>(0.3, 0.5, 1.0);
    
    // Sun direction
    let light_dir = normalize(vec3<f32>(cos(planet.solar_angle), 0.0, sin(planet.solar_angle)));
    
    // Mie scattering (sun glow)
    let sun_glow = pow(max(0.0, dot(view_dir, light_dir)), 32.0) * vec3<f32>(1.0, 0.9, 0.7);
    
    // Rayleigh scattering (blue sky)
    let rayleigh = pow(1.0 - max(0.0, dot(view_dir, input.normal)), 4.0);
    
    // Combine
    let color = scatter_color * rayleigh + sun_glow;
    
    return vec4<f32>(color, rayleigh * 0.3);
}

// ============================================================================
// STORM RENDERING
// ============================================================================

struct StormVertex {
    /// Position on sphere surface
    @location(0) position: vec3<f32>,
    /// Storm size
    @location(1) size: f32,
    /// Storm intensity (0-1)
    @location(2) intensity: f32,
    /// Storm rotation angle
    @location(3) rotation: f32,
};

struct StormOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) intensity: f32,
    @location(2) rotation: f32,
};

@vertex
fn vs_storm(vertex: StormVertex) -> StormOutput {
    var output: StormOutput;
    
    // Position storm on planet surface (above clouds)
    let pos = vertex.position * planet.radius * 1.025;
    
    output.position = planet.view_proj * vec4<f32>(pos, 1.0);
    output.uv = vec2<f32>(0.5, 0.5);  // Center of spiral
    output.intensity = vertex.intensity;
    output.rotation = vertex.rotation;
    
    return output;
}

@fragment
fn fs_storm(input: StormOutput) -> @location(0) vec4<f32> {
    // Spiral pattern for storm
    let angle = input.rotation + planet.time * 0.5;
    
    // Storm color based on intensity
    // Low intensity: light gray
    // High intensity: dark blue/purple (hurricane)
    let low_intensity_color = vec3<f32>(0.7, 0.7, 0.75);
    let high_intensity_color = vec3<f32>(0.1, 0.1, 0.3);
    
    let color = mix(low_intensity_color, high_intensity_color, input.intensity);
    
    // Eye of the storm is clearer
    let eye_factor = 1.0 - input.intensity * 0.5;
    
    return vec4<f32>(color * eye_factor, input.intensity * 0.8);
}

// ============================================================================
// SETTLEMENT RENDERING (Phase 6 preview)
// ============================================================================

struct SettlementVertex {
    /// Position on planet surface
    @location(0) position: vec3<f32>,
    /// Settlement size (based on population)
    @location(1) size: f32,
    /// Technology level (affects color)
    @location(2) tech_level: f32,
    /// Settlement type (0=hamlet, 1=village, 2=town, 3=city)
    @location(3) settlement_type: f32,
};

struct SettlementOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) size: f32,
    @location(1) tech_level: f32,
    @location(2) settlement_type: f32,
};

@vertex
fn vs_settlement(vertex: SettlementVertex) -> SettlementOutput {
    var output: SettlementOutput;
    
    // Position on planet surface
    let pos = vertex.position * planet.radius * 1.001;
    
    output.position = planet.view_proj * vec4<f32>(pos, 1.0);
    output.size = vertex.size;
    output.tech_level = vertex.tech_level;
    output.settlement_type = vertex.settlement_type;
    
    return output;
}

@fragment
fn fs_settlement(input: SettlementOutput) -> @location(0) vec4<f32> {
    // Settlement color based on type
    var color = select(
        vec3<f32>(0.8, 0.7, 0.5),   // Hamlet - earth tones
        vec3<f32>(0.9, 0.85, 0.7),  // Village
        input.settlement_type < 1.5
    );
    
    color = select(
        color,
        vec3<f32>(0.7, 0.75, 0.8),  // Town
        input.settlement_type >= 1.5 && input.settlement_type < 2.5
    );
    
    color = select(
        color,
        vec3<f32>(0.5, 0.6, 0.7),   // City - more metal/glass
        input.settlement_type >= 2.5
    );
    
    // Tech level adds light glow
    let glow = input.tech_level * 0.3;
    
    return vec4<f32>(color + vec3<f32>(glow), 1.0);
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Mix two vectors
fn mix(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a * (1.0 - t) + b * t;
}

/// Select between two values
fn select(a: vec3<f32>, b: vec3<f32>, condition: bool) -> vec3<f32> {
    if (condition) { return b; }
    return a;
}
