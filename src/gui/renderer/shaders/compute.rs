//! Compute Shader for Collective Dynamics and Particle Systems
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 4 Week 8:
//! "Compute shaders - Collective dynamics calculations, Resonance field computation, Particle system updates"

/// Compute shader for collective dynamics
pub const COMPUTE_SHADER: &str = r#"
// Workgroup size
@workgroup_size(64, 1, 1)

// Entity data
struct EntityData {
    position: vec3<f32>,
    velocity: vec3<f32>,
    density: f32,
    polarity: f32,
    archetype: f32,
    consciousness: f32,
    _padding: vec2<f32>,
};

// Collective data
struct CollectiveData {
    center: vec3<f32>,
    radius: f32,
    density: f32,
    polarity: f32,
    resonance: f32,
    coherence: f32,
    entity_count: u32,
    _padding: vec3<f32>,
};

// Simulation parameters
struct SimulationParams {
    dt: f32,
    resonance_threshold: f32,
    coherence_threshold: f32,
    max_interaction_distance: f32,
    _padding: vec4<f32>,
};

// Output buffers
struct ResonanceField {
    position: vec3<f32>,
    resonance_strength: f32,
    resonance_type: u32,
    _padding: vec3<f32>,
};

@group(0) @binding(0)
var<storage, read> entities: array<EntityData>;

@group(0) @binding(1)
var<storage, read_write> collectives: array<CollectiveData>;

@group(0) @binding(2)
var<uniform> params: SimulationParams;

@group(0) @binding(3)
var<storage, read_write> resonance_field: array<ResonanceField>;

@group(0) @binding(4)
var<storage, read_write> entity_velocities: array<vec3<f32>>;

// Compute collective dynamics
@compute
fn compute_collective_dynamics(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    let num_entities = arrayLength(&entities);
    
    if (idx >= num_entities) {
        return;
    }
    
    let entity = entities[idx];
    var total_attraction = vec3<f32>(0.0, 0.0, 0.0);
    var total_repulsion = vec3<f32>(0.0, 0.0, 0.0);
    var total_resonance = 0.0;
    var interaction_count = 0u;
    
    // Calculate interactions with all other entities
    for (var j: u32 = 0u; j < num_entities; j = j + 1u) {
        if (j == idx) {
            continue;
        }
        
        let other = entities[j];
        let direction = other.position - entity.position;
        let distance = length(direction);
        
        if (distance > params.max_interaction_distance) {
            continue;
        }
        
        let normalized_dir = direction / distance;
        
        // Calculate resonance based on density, polarity, and archetype
        let density_resonance = 1.0 - abs(entity.density - other.density);
        let polarity_resonance = 1.0 - abs(entity.polarity - other.polarity);
        let archetype_resonance = 1.0 - abs(entity.archetype - other.archetype);
        let resonance = (density_resonance + polarity_resonance + archetype_resonance) / 3.0;
        
        // Attraction based on resonance (like attracts like)
        total_attraction += normalized_dir * resonance * (distance / params.max_interaction_distance);
        
        // Repulsion based on distance (avoid crowding)
        let repulsion_strength = 1.0 - (distance / params.max_interaction_distance);
        total_repulsion -= normalized_dir * repulsion_strength * 0.5;
        
        total_resonance += resonance;
        interaction_count += 1u;
    }
    
    // Update entity velocity based on collective behavior
    if (interaction_count > 0u) {
        let avg_resonance = total_resonance / f32(interaction_count);
        let collective_factor = avg_resonance * params.dt;
        
        entity_velocities[idx] = entity.velocity + (total_attraction + total_repulsion) * collective_factor;
        
        // Update resonance field
        resonance_field[idx].position = entity.position;
        resonance_field[idx].resonance_strength = avg_resonance;
        resonance_field[idx].resonance_type = u32(entity.density * 10.0);
    }
}

// Compute collective properties
@compute
fn compute_collective_properties(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let collective_idx = global_id.x;
    let num_collectives = arrayLength(&collectives);
    
    if (collective_idx >= num_collectives) {
        return;
    }
    
    let num_entities = arrayLength(&entities);
    var center = vec3<f32>(0.0, 0.0, 0.0);
    var total_density = 0.0;
    var total_polarity = 0.0;
    var entity_count = 0u;
    
    // Find entities within this collective's radius
    for (var i: u32 = 0u; i < num_entities; i = i + 1u) {
        let entity = entities[i];
        let direction = entity.position - collectives[collective_idx].center;
        let distance = length(direction);
        
        if (distance <= collectives[collective_idx].radius) {
            center += entity.position;
            total_density += entity.density;
            total_polarity += entity.polarity;
            entity_count += 1u;
        }
    }
    
    // Update collective properties
    if (entity_count > 0u) {
        collectives[collective_idx].center = center / f32(entity_count);
        collectives[collective_idx].density = total_density / f32(entity_count);
        collectives[collective_idx].polarity = total_polarity / f32(entity_count);
        collectives[collective_idx].entity_count = entity_count;
        
        // Calculate coherence (how aligned entities are)
        var alignment_sum = 0.0;
        for (var i: u32 = 0u; i < num_entities; i = i + 1u) {
            let entity = entities[i];
            let direction = entity.position - collectives[collective_idx].center;
            let distance = length(direction);
            
            if (distance <= collectives[collective_idx].radius) {
                let normalized_dir = direction / distance;
                alignment_sum += abs(dot(normalized_dir, normalize(entity.velocity)));
            }
        }
        collectives[collective_idx].coherence = alignment_sum / f32(entity_count);
    }
}

// Particle system update
struct ParticleData {
    position: vec3<f32>,
    velocity: vec3<f32>,
    life: f32,
    max_life: f32,
    color: vec4<f32>,
    size: f32,
    _padding: vec3<f32>,
};

struct ParticleEmitter {
    position: vec3<f32>,
    rate: f32,
    lifetime: f32,
    color: vec4<f32>,
    size: f32,
    _padding: vec3<f32>,
};

@group(1) @binding(0)
var<storage, read_write> particles: array<ParticleData>;

@group(1) @binding(1)
var<storage, read> emitters: array<ParticleEmitter>;

@group(1) @binding(2)
var<uniform> particle_params: SimulationParams;

@compute
fn update_particles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    let num_particles = arrayLength(&particles);
    
    if (idx >= num_particles) {
        return;
    }
    
    let particle = particles[idx];
    
    // Update particle life
    particles[idx].life -= particle_params.dt;
    
    // Respawn dead particles from emitters
    if (particle.life <= 0.0) {
        let num_emitters = arrayLength(&emitters);
        if (num_emitters > 0u) {
            let emitter_idx = idx % num_emitters;
            let emitter = emitters[emitter_idx];
            
            particles[idx].position = emitter.position;
            
            // Random velocity direction
            let theta = fract(f32(idx) * 0.6180339887498949) * 6.283185307179586;
            let phi = fract(f32(idx) * 0.3141592653589793) * 3.141592653589793;
            let sin_phi = phi.sin();
            let x = theta.cos() * sin_phi;
            let y = phi.sin();
            let z = theta.sin() * sin_phi;
            
            particles[idx].velocity = vec3<f32>(x, y, z) * fract(f32(idx) * 0.1) * 10.0;
            particles[idx].life = emitter.lifetime;
            particles[idx].max_life = emitter.lifetime;
            particles[idx].color = emitter.color;
            particles[idx].size = emitter.size;
        }
    } else {
        // Update particle position
        particles[idx].position += particles[idx].velocity * particle_params.dt;
        
        // Apply gravity
        particles[idx].velocity.y -= 9.81 * particle_params.dt;
        
        // Fade out based on life
        let life_ratio = particles[idx].life / particles[idx].max_life;
        particles[idx].color.a *= life_ratio;
    }
}
"#;

/// Data structures for compute shader
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityData {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub density: f32,
    pub polarity: f32,
    pub archetype: f32,
    pub consciousness: f32,
    pub _padding: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CollectiveData {
    pub center: [f32; 3],
    pub radius: f32,
    pub density: f32,
    pub polarity: f32,
    pub resonance: f32,
    pub coherence: f32,
    pub entity_count: u32,
    pub _padding: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimulationParams {
    pub dt: f32,
    pub resonance_threshold: f32,
    pub coherence_threshold: f32,
    pub max_interaction_distance: f32,
    pub _padding: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ResonanceField {
    pub position: [f32; 3],
    pub resonance_strength: f32,
    pub resonance_type: u32,
    pub _padding: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleData {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub life: f32,
    pub max_life: f32,
    pub color: [f32; 4],
    pub size: f32,
    pub _padding: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleEmitter {
    pub position: [f32; 3],
    pub rate: f32,
    pub lifetime: f32,
    pub color: [f32; 4],
    pub size: f32,
    pub _padding: [f32; 3],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_data_size() {
        assert_eq!(std::mem::size_of::<EntityData>(), 48);
    }

    #[test]
    fn test_collective_data_size() {
        assert_eq!(std::mem::size_of::<CollectiveData>(), 48);
    }

    #[test]
    fn test_simulation_params_size() {
        assert_eq!(std::mem::size_of::<SimulationParams>(), 32);
    }

    #[test]
    fn test_resonance_field_size() {
        assert_eq!(std::mem::size_of::<ResonanceField>(), 32);
    }

    #[test]
    fn test_particle_data_size() {
        assert_eq!(std::mem::size_of::<ParticleData>(), 64);
    }

    #[test]
    fn test_particle_emitter_size() {
        // ParticleEmitter: position[3] (12) + rate (4) + lifetime (4) + color[4] (16) + size (4) + _padding[3] (12) = 52
        assert_eq!(std::mem::size_of::<ParticleEmitter>(), 52);
    }
}
