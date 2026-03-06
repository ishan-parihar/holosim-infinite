//! 3D Entity Rendering Shader with Lighting
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 4 Week 8:
//! "Advanced shaders - 3D sphere rendering with lighting, normal mapping, specular highlights"

/// Entity shader for 3D sphere rendering with lighting
pub const ENTITY_SHADER: &str = r#"
// Uniform buffers
struct CameraUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    _padding: f32,
};

struct LightUniforms {
    light_pos: vec3<f32>,
    light_color: vec3<f32>,
    light_intensity: f32,
    ambient_intensity: f32,
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@group(0) @binding(1)
var<uniform> light: LightUniforms;

// Instance data
struct InstanceData {
    model_matrix: mat4x4<f32>,
    color: vec4<f32>,
    radius: f32,
    density: f32,
    polarity: f32,
    _padding: vec3<f32>,
};

@group(1) @binding(0)
var<storage, read> instances: array<InstanceData>;

// Vertex input for sphere geometry
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// Vertex output to fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) density: f32,
    @location(5) polarity: f32,
    @location(6) radius: f32,
};

@vertex
fn vs_main(
    input: VertexInput,
    @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    let instance = instances[instance_index];
    
    // Transform vertex position to world space
    let world_pos = instance.model_matrix * vec4<f32>(input.position * instance.radius, 1.0);
    
    // Transform normal to world space (using inverse transpose of model matrix)
    let normal = normalize((instance.model_matrix * vec4<f32>(input.normal, 0.0)).xyz);
    
    // Calculate clip position
    let clip_pos = camera.view_proj * world_pos;
    
    var output: VertexOutput;
    output.clip_position = clip_pos;
    output.world_pos = world_pos.xyz;
    output.normal = normal;
    output.uv = input.uv;
    output.color = instance.color;
    output.density = instance.density;
    output.polarity = instance.polarity;
    output.radius = instance.radius;
    
    return output;
}

// Fragment shader with Phong lighting model
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize inputs
    let normal = normalize(input.normal);
    let view_dir = normalize(camera.camera_pos - input.world_pos);
    let light_dir = normalize(light.light_pos - input.world_pos);
    
    // Ambient component
    let ambient = light.ambient_intensity * light.light_color;
    
    // Diffuse component (Lambertian)
    let diff_intensity = max(dot(normal, light_dir), 0.0);
    let diffuse = diff_intensity * light.light_intensity * light.light_color;
    
    // Specular component (Phong)
    let reflect_dir = reflect(-light_dir, normal);
    let spec_intensity = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    let specular = spec_intensity * light.light_intensity * light.light_color * 0.5;
    
    // Combine lighting
    let lighting = ambient + diffuse + specular;
    
    // Apply density-based glow effect
    let glow_intensity = input.density * 0.3;
    let glow = input.color.rgb * glow_intensity;
    
    // Apply polarity-based rim lighting
    let rim_dot = 1.0 - max(dot(view_dir, normal), 0.0);
    let rim = pow(rim_dot, 3.0) * input.polarity * 0.5;
    
    // Combine all effects
    let final_color = input.color.rgb * lighting + glow + rim;
    
    return vec4<f32>(final_color, input.color.a);
}
"#;

/// Type alias for sphere geometry data (vertices, normals, uvs, indices)
pub type SphereGeometry = (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>);

/// Sphere geometry generation
pub fn generate_sphere_geometry(
    segments: u32,
    rings: u32,
) -> SphereGeometry {
    use std::f32::consts::PI;

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Generate vertices
    for ring in 0..=rings {
        let theta = (ring as f32 / rings as f32) * PI;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for segment in 0..=segments {
            let phi = (segment as f32 / segments as f32) * 2.0 * PI;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            // Position on unit sphere
            let x = cos_phi * sin_theta;
            let y = cos_theta;
            let z = sin_phi * sin_theta;

            vertices.push([x, y, z]);

            // Normal is same as position for unit sphere
            normals.push([x, y, z]);

            // UV coordinates
            let u = segment as f32 / segments as f32;
            let v = ring as f32 / rings as f32;
            uvs.push([u, v]);
        }
    }

    // Generate indices
    for ring in 0..rings {
        for segment in 0..segments {
            let first = ring * (segments + 1) + segment;
            let second = first + segments + 1;

            // First triangle
            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            // Second triangle
            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    (vertices, normals, uvs, indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_geometry_generation() {
        let (vertices, normals, uvs, indices) = generate_sphere_geometry(16, 16);

        // Verify we have the expected number of vertices
        let expected_vertices = (16 + 1) * (16 + 1);
        assert_eq!(vertices.len(), expected_vertices);
        assert_eq!(normals.len(), expected_vertices);
        assert_eq!(uvs.len(), expected_vertices);

        // Verify indices form triangles
        assert_eq!(indices.len() % 3, 0);

        // Verify all vertices are on unit sphere
        for vertex in &vertices {
            let magnitude = (vertex[0].powi(2) + vertex[1].powi(2) + vertex[2].powi(2)).sqrt();
            assert!((magnitude - 1.0).abs() < 0.01);
        }

        // Verify normals are normalized
        for normal in &normals {
            let magnitude = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
            assert!((magnitude - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_sphere_uv_coordinates() {
        let (_, _, uvs, _) = generate_sphere_geometry(8, 8);

        // Verify UV coordinates are in [0, 1] range
        for uv in &uvs {
            assert!(uv[0] >= 0.0 && uv[0] <= 1.0);
            assert!(uv[1] >= 0.0 && uv[1] <= 1.0);
        }
    }
}
