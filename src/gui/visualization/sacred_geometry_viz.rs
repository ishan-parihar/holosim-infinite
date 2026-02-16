//! Sacred Geometry Visualization
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 (Weeks 19-20):
//! "Visual patterns show sacred geometry, not random distribution"
//! "φ in all proportions - Spiral patterns in galaxy formation - Vesica Piscis in entity relationships - Flower of Life in layer interactions"
//!
//! This module provides GPU-ready visualization data structures for rendering sacred geometry
//! patterns including Fibonacci sequences, golden ratio spirals, Platonic solids, Vesica Piscis,
//! Flower of Life, and harmonic resonance patterns.

use crate::sacred_geometry::{
    FibonacciSequence, SpiralPattern, PlatonicStructure,
    VesicaPiscis, FlowerOfLife, HarmonicSeries,
    StandingWave, FrequencyResonance, GOLDEN_RATIO
};
use crate::types::Float;

// ============================================================================
// Sacred Geometry Colors
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SacredGeometryColors {
    pub fibonacci_primary: [f32; 4],
    pub fibonacci_secondary: [f32; 4],
    pub golden_ratio: [f32; 4],
    pub platonic_solid: [f32; 4],
    pub vesica_piscis: [f32; 4],
    pub flower_of_life: [f32; 4],
    pub harmonic_resonance: [f32; 4],
    pub standing_wave_constructive: [f32; 4],
    pub standing_wave_destructive: [f32; 4],
    pub standing_wave_partial: [f32; 4],
}

impl Default for SacredGeometryColors {
    fn default() -> Self {
        SacredGeometryColors {
            fibonacci_primary: [0.9, 0.7, 0.2, 0.9],
            fibonacci_secondary: [0.6, 0.4, 0.1, 0.7],
            golden_ratio: [1.0, 0.84, 0.0, 0.95],
            platonic_solid: [0.3, 0.8, 0.9, 0.85],
            vesica_piscis: [0.8, 0.4, 0.9, 0.8],
            flower_of_life: [0.4, 0.9, 0.6, 0.8],
            harmonic_resonance: [0.9, 0.3, 0.5, 0.9],
            standing_wave_constructive: [0.0, 1.0, 0.5, 0.9],
            standing_wave_destructive: [1.0, 0.2, 0.2, 0.8],
            standing_wave_partial: [0.9, 0.7, 0.0, 0.8],
        }
    }
}

// ============================================================================
// Fibonacci Render Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FibonacciRenderPoint {
    pub position: [f32; 3],
    pub generation: u32,
    pub value: f32,
    pub color: [f32; 4],
    pub size: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FibonacciRenderConnection {
    pub from_index: u32,
    pub to_index: u32,
    pub color: [f32; 4],
    pub thickness: f32,
}

#[derive(Debug, Clone)]
pub struct FibonacciRenderData {
    pub points: Vec<FibonacciRenderPoint>,
    pub connections: Vec<FibonacciRenderConnection>,
    pub max_generation: u32,
    pub scale: f32,
    pub visible: bool,
}

impl FibonacciRenderData {
    pub fn from_sequence(sequence: &FibonacciSequence, colors: &SacredGeometryColors, scale: f32) -> Self {
        let max_generation = sequence.len() as u32;
        let mut points = Vec::with_capacity(max_generation as usize);
        let mut connections = Vec::with_capacity((max_generation as usize).saturating_sub(1));
        
        let golden_angle = 2.39996322972865332;
        
        for i in 0..max_generation {
            let gen = i as u64;
            let value = sequence.get(gen) as f32;
            
            let angle = gen as f32 * golden_angle;
            let radius = (gen as f32 + 1.0).sqrt() * scale;
            
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let z = 0.0;
            
            let t = (i as f32 / max_generation as f32).min(1.0);
            let color = [
                colors.fibonacci_primary[0] * (1.0 - t) + colors.fibonacci_secondary[0] * t,
                colors.fibonacci_primary[1] * (1.0 - t) + colors.fibonacci_secondary[1] * t,
                colors.fibonacci_primary[2] * (1.0 - t) + colors.fibonacci_secondary[2] * t,
                colors.fibonacci_primary[3] * (1.0 - t) + colors.fibonacci_secondary[3] * t,
            ];
            
            let size = (value.log10() + 1.0) * scale * 0.1;
            
            points.push(FibonacciRenderPoint {
                position: [x, y, z],
                generation: i,
                value,
                color,
                size,
            });
            
            if i > 0 {
                connections.push(FibonacciRenderConnection {
                    from_index: i - 1,
                    to_index: i,
                    color,
                    thickness: scale * 0.05,
                });
            }
        }
        
        FibonacciRenderData {
            points,
            connections,
            max_generation,
            scale,
            visible: true,
        }
    }
}

// ============================================================================
// Spiral Render Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpiralRenderPoint {
    pub position: [f32; 3],
    pub angle: f32,
    pub radius: f32,
    pub color: [f32; 4],
    pub width: f32,
}

#[derive(Debug, Clone)]
pub struct SpiralRenderData {
    pub points: Vec<SpiralRenderPoint>,
    pub center: [f32; 3],
    pub turns: f32,
    pub spiral_type: u32,
    pub visible: bool,
}

impl SpiralRenderData {
    pub fn from_spiral_pattern(spiral: &SpiralPattern, colors: &SacredGeometryColors, num_points: usize) -> Self {
        let mut points = Vec::with_capacity(num_points);
        
        let center = [spiral.center_x() as f32, spiral.center_y() as f32, 0.0];
        let turns = spiral.total_turns();
        
        for i in 0..num_points {
            let t = i as f64 / num_points as f64;
            let angle = t * turns * 2.0 * std::f64::consts::PI;
            
            let radius = if matches!(spiral.spiral_type(), crate::sacred_geometry::SpiralType::Golden) {
                0.1 * GOLDEN_RATIO.powf(angle / (std::f64::consts::PI / 2.0))
            } else {
                t * 10.0
            };
            
            let x = center[0] + radius.cos() as f32 * radius as f32;
            let y = center[1] + radius.sin() as f32 * radius as f32;
            let z = center[2];
            
            let color = colors.golden_ratio;
            let width = (1.0 - t * 0.7) as f32;
            
            points.push(SpiralRenderPoint {
                position: [x, y, z],
                angle: angle as f32,
                radius: radius as f32,
                color,
                width,
            });
        }
        
        let spiral_type = match spiral.spiral_type() {
            crate::sacred_geometry::SpiralType::Golden => 0,
            crate::sacred_geometry::SpiralType::Fibonacci => 1,
            crate::sacred_geometry::SpiralType::Archimedean => 2,
            crate::sacred_geometry::SpiralType::Logarithmic => 3,
        };
        
        SpiralRenderData {
            points,
            center,
            turns: turns as f32,
            spiral_type,
            visible: true,
        }
    }
}

// ============================================================================
// Platonic Solid Render Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlatonicVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub size: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlatonicEdge {
    pub from_vertex: u32,
    pub to_vertex: u32,
    pub color: [f32; 4],
    pub thickness: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlatonicFace {
    pub vertex_indices: [u32; 6],
    pub num_vertices: u32,
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct PlatonicSolidRenderData {
    pub vertices: Vec<PlatonicVertex>,
    pub edges: Vec<PlatonicEdge>,
    pub faces: Vec<PlatonicFace>,
    pub solid_type: u32,
    pub center: [f32; 3],
    pub scale: f32,
    pub visible: bool,
}

impl PlatonicSolidRenderData {
    pub fn from_structure(structure: &PlatonicStructure, colors: &SacredGeometryColors, scale: f32) -> Self {
        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        let mut faces = Vec::new();
        
        let structure_vertices = structure.vertices();
        for vertex in structure_vertices {
            let pos = vertex.position();
            vertices.push(PlatonicVertex {
                position: [
                    pos.x as f32 * scale,
                    pos.y as f32 * scale,
                    pos.z as f32 * scale,
                ],
                color: colors.platonic_solid,
                size: scale * 0.1,
            });
        }
        
        let structure_edges = structure.edges();
        for edge in structure_edges {
            edges.push(PlatonicEdge {
                from_vertex: edge.0 as u32,
                to_vertex: edge.1 as u32,
                color: colors.platonic_solid,
                thickness: scale * 0.03,
            });
        }
        
        let structure_faces = structure.faces();
        for face in structure_faces {
            let mut vertex_indices = [0u32; 6];
            for (i, &idx) in face.iter().enumerate() {
                if i < 6 {
                    vertex_indices[i] = idx as u32;
                }
            }
            faces.push(PlatonicFace {
                vertex_indices,
                num_vertices: face.len() as u32,
                color: [
                    colors.platonic_solid[0],
                    colors.platonic_solid[1],
                    colors.platonic_solid[2],
                    0.3,
                ],
            });
        }
        
        let solid_type = match structure.solid_type() {
            crate::sacred_geometry::PlatonicSolid::Tetrahedron => 0,
            crate::sacred_geometry::PlatonicSolid::Cube => 1,
            crate::sacred_geometry::PlatonicSolid::Octahedron => 2,
            crate::sacred_geometry::PlatonicSolid::Dodecahedron => 3,
            crate::sacred_geometry::PlatonicSolid::Icosahedron => 4,
        };
        
        PlatonicSolidRenderData {
            vertices,
            edges,
            faces,
            solid_type,
            center: [0.0, 0.0, 0.0],
            scale,
            visible: true,
        }
    }
}

// ============================================================================
// Vesica Piscis Render Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CircleRenderData {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [f32; 4],
    pub thickness: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct IntersectionPoint {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct VesicaPiscisRenderData {
    pub circle_a: CircleRenderData,
    pub circle_b: CircleRenderData,
    pub intersection_points: Vec<IntersectionPoint>,
    pub intersection_area: f32,
    pub visible: bool,
}

impl VesicaPiscisRenderData {
    pub fn from_vesica_piscis(vesica: &VesicaPiscis, colors: &SacredGeometryColors, scale: f32) -> Self {
        let center_a = [
            vesica.center_a().0 as f32 * scale,
            vesica.center_a().1 as f32 * scale,
            0.0,
        ];
        let center_b = [
            vesica.center_b().0 as f32 * scale,
            vesica.center_b().1 as f32 * scale,
            0.0,
        ];
        let radius = vesica.radius() as f32 * scale;
        
        let circle_a = CircleRenderData {
            center: center_a,
            radius,
            color: colors.vesica_piscis,
            thickness: scale * 0.02,
        };
        
        let circle_b = CircleRenderData {
            center: center_b,
            radius,
            color: colors.vesica_piscis,
            thickness: scale * 0.02,
        };
        
        let mut intersection_points = Vec::new();
        for point in vesica.intersection_points() {
            intersection_points.push(IntersectionPoint {
                position: [
                    point.0 as f32 * scale,
                    point.1 as f32 * scale,
                    0.0,
                ],
                color: colors.golden_ratio,
                size: scale * 0.15,
            });
        }
        
        let intersection_area = vesica.area() as f32;
        
        VesicaPiscisRenderData {
            circle_a,
            circle_b,
            intersection_points,
            intersection_area,
            visible: true,
        }
    }
}

// ============================================================================
// Flower of Life Render Data
// ============================================================================

#[derive(Debug, Clone)]
pub struct FlowerOfLifeRenderData {
    pub center: [f32; 3],
    pub base_radius: f32,
    pub num_layers: u32,
    pub num_circles: u32,
    pub visible: bool,
}

impl FlowerOfLifeRenderData {
    pub fn from_flower_of_life(flower: &FlowerOfLife, colors: &SacredGeometryColors, scale: f32) -> Self {
        let center = [flower.center().0 as f32 * scale, flower.center().1 as f32 * scale, 0.0];
        let base_radius = flower.radius() as f32 * scale;
        let num_circles = flower.circle_count() as u32;
        
        FlowerOfLifeRenderData {
            center,
            base_radius,
            num_layers: flower.layers() as u32,
            num_circles,
            visible: true,
        }
    }
}

// ============================================================================
// Harmonic Resonance Render Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrequencyBar {
    pub position: [f32; 3],
    pub frequency: f32,
    pub amplitude: f32,
    pub color: [f32; 4],
    pub width: f32,
}

#[derive(Debug, Clone)]
pub struct HarmonicResonanceRenderData {
    pub fundamental_bar: FrequencyBar,
    pub overtone_bars: Vec<FrequencyBar>,
    pub fundamental_frequency: f32,
    pub resonance_strength: f32,
    pub harmonic_overtone: Option<u32>,
    pub overtone_pattern_type: u32,
    pub visible: bool,
}

impl HarmonicResonanceRenderData {
    pub fn from_resonance(resonance: &FrequencyResonance, harmonic_series: &HarmonicSeries, colors: &SacredGeometryColors, scale: f32) -> Self {
        let fundamental_freq = harmonic_series.fundamental as f32;
        
        let fundamental_bar = FrequencyBar {
            position: [0.0, 0.0, 0.0],
            frequency: fundamental_freq,
            amplitude: 1.0,
            color: colors.harmonic_resonance,
            width: scale * 0.5,
        };
        
        let mut overtone_bars = Vec::new();
        for (i, &overtone_freq) in harmonic_series.overtones.iter().enumerate() {
            let freq = overtone_freq as f32;
            let amplitude = 1.0 / ((i + 2) as f32).sqrt();
            
            let x = (i + 1) as f32 * scale * 0.8;
            let color = if let Some(overtone_num) = resonance.harmonic_overtone {
                if (overtone_num as usize) == i + 1 {
                    colors.standing_wave_constructive
                } else {
                    colors.harmonic_resonance
                }
            } else {
                colors.harmonic_resonance
            };
            
            overtone_bars.push(FrequencyBar {
                position: [x, 0.0, 0.0],
                frequency: freq,
                amplitude,
                color,
                width: scale * 0.4 * amplitude,
            });
        }
        
        HarmonicResonanceRenderData {
            fundamental_bar,
            overtone_bars,
            fundamental_frequency: fundamental_freq,
            resonance_strength: resonance.resonance_strength as f32,
            harmonic_overtone: resonance.harmonic_overtone.map(|x| x as u32),
            overtone_pattern_type: 0,
            visible: true,
        }
    }
}

// ============================================================================
// Standing Wave Render Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WavePoint {
    pub position: [f32; 3],
    pub amplitude: f32,
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct StandingWaveRenderData {
    pub wave_points: Vec<WavePoint>,
    pub nodes: Vec<[f32; 3]>,
    pub antinodes: Vec<[f32; 3]>,
    pub interference_type: u32,
    pub center_amplitude: f32,
    pub visible: bool,
}

impl StandingWaveRenderData {
    pub fn from_standing_wave(wave: &StandingWave, colors: &SacredGeometryColors, num_points: usize, scale: f32) -> Self {
        let mut wave_points = Vec::with_capacity(num_points);
        let mut nodes = Vec::new();
        let mut antinodes = Vec::new();
        
        let interference_color = match wave.interference_type {
            crate::sacred_geometry::InterferenceType::Constructive => colors.standing_wave_constructive,
            crate::sacred_geometry::InterferenceType::Partial => colors.standing_wave_partial,
            crate::sacred_geometry::InterferenceType::Destructive => colors.standing_wave_destructive,
        };
        
        let interference_type = match wave.interference_type {
            crate::sacred_geometry::InterferenceType::Constructive => 0u32,
            crate::sacred_geometry::InterferenceType::Partial => 1u32,
            crate::sacred_geometry::InterferenceType::Destructive => 2u32,
        };
        
        for i in 0..num_points {
            let position = i as f64 / (num_points - 1) as f64;
            let amplitude = wave.get_amplitude_at(position);
            
            let x = ((position - 0.5) * 2.0) as f32 * scale;
            let y = amplitude as f32 * scale * 0.5;
            let z = 0.0;
            
            wave_points.push(WavePoint {
                position: [x, y, z],
                amplitude: amplitude as f32,
                color: interference_color,
            });
        }
        
        for node_pos in &wave.nodes {
            nodes.push([
                (*node_pos - 0.5) as f32 * 2.0 * scale,
                0.0,
                0.0,
            ]);
        }
        
        for antinode_pos in &wave.antinodes {
            let amplitude_at = wave.get_amplitude_at(*antinode_pos);
            antinodes.push([
                (*antinode_pos - 0.5) as f32 * 2.0 * scale,
                amplitude_at as f32 * scale * 0.5,
                0.0,
            ]);
        }
        
        StandingWaveRenderData {
            wave_points,
            nodes,
            antinodes,
            interference_type,
            center_amplitude: wave.center_amplitude as f32,
            visible: true,
        }
    }
}

// ============================================================================
// Sacred Geometry Visualization
// ============================================================================

#[derive(Debug, Clone)]
pub struct SacredGeometryVisualization {
    pub colors: SacredGeometryColors,
    pub fibonacci: Option<FibonacciRenderData>,
    pub spirals: Vec<SpiralRenderData>,
    pub platonic_solids: Vec<PlatonicSolidRenderData>,
    pub vesica_piscis: Vec<VesicaPiscisRenderData>,
    pub flower_of_life: Vec<FlowerOfLifeRenderData>,
    pub harmonic_resonance: Vec<HarmonicResonanceRenderData>,
    pub standing_waves: Vec<StandingWaveRenderData>,
    pub show_fibonacci: bool,
    pub show_spirals: bool,
    pub show_platonic_solids: bool,
    pub show_vesica_piscis: bool,
    pub show_flower_of_life: bool,
    pub show_harmonic_resonance: bool,
    pub show_standing_waves: bool,
    pub global_scale: f32,
}

impl Default for SacredGeometryVisualization {
    fn default() -> Self {
        SacredGeometryVisualization {
            colors: SacredGeometryColors::default(),
            fibonacci: None,
            spirals: Vec::new(),
            platonic_solids: Vec::new(),
            vesica_piscis: Vec::new(),
            flower_of_life: Vec::new(),
            harmonic_resonance: Vec::new(),
            standing_waves: Vec::new(),
            show_fibonacci: true,
            show_spirals: true,
            show_platonic_solids: true,
            show_vesica_piscis: true,
            show_flower_of_life: true,
            show_harmonic_resonance: true,
            show_standing_waves: true,
            global_scale: 1.0,
        }
    }
}

impl SacredGeometryVisualization {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_scale(scale: f32) -> Self {
        let mut viz = Self::default();
        viz.global_scale = scale;
        viz
    }
    
    pub fn add_fibonacci(&mut self, sequence: &FibonacciSequence) {
        self.fibonacci = Some(FibonacciRenderData::from_sequence(sequence, &self.colors, self.global_scale));
    }
    
    pub fn add_spiral(&mut self, spiral: &SpiralPattern, num_points: usize) {
        self.spirals.push(SpiralRenderData::from_spiral_pattern(spiral, &self.colors, num_points));
    }
    
    pub fn add_platonic_solid(&mut self, structure: &PlatonicStructure) {
        self.platonic_solids.push(PlatonicSolidRenderData::from_structure(structure, &self.colors, self.global_scale));
    }
    
    pub fn add_vesica_piscis(&mut self, vesica: &VesicaPiscis) {
        self.vesica_piscis.push(VesicaPiscisRenderData::from_vesica_piscis(vesica, &self.colors, self.global_scale));
    }
    
    pub fn add_flower_of_life(&mut self, flower: &FlowerOfLife) {
        self.flower_of_life.push(FlowerOfLifeRenderData::from_flower_of_life(flower, &self.colors, self.global_scale));
    }
    
    pub fn add_harmonic_resonance(&mut self, resonance: &FrequencyResonance, harmonic_series: &HarmonicSeries) {
        self.harmonic_resonance.push(HarmonicResonanceRenderData::from_resonance(resonance, harmonic_series, &self.colors, self.global_scale));
    }
    
    pub fn add_standing_wave(&mut self, wave: &StandingWave, num_points: usize) {
        self.standing_waves.push(StandingWaveRenderData::from_standing_wave(wave, &self.colors, num_points, self.global_scale));
    }
    
    pub fn clear_all(&mut self) {
        self.fibonacci = None;
        self.spirals.clear();
        self.platonic_solids.clear();
        self.vesica_piscis.clear();
        self.flower_of_life.clear();
        self.harmonic_resonance.clear();
        self.standing_waves.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sacred_geometry::{FibonacciSequence, SpiralPattern, PlatonicStructure, VesicaPiscis, FlowerOfLife, HarmonicSeries, FrequencyResonance};
    
    #[test]
    fn test_sacred_geometry_colors_default() {
        let colors = SacredGeometryColors::default();
        assert_eq!(colors.golden_ratio[3], 0.95);
        assert_eq!(colors.platonic_solid.len(), 4);
    }
    
    #[test]
    fn test_fibonacci_render_data() {
        let sequence = FibonacciSequence::new(10);
        let render_data = FibonacciRenderData::from_sequence(&sequence, &SacredGeometryColors::default(), 1.0);
        assert_eq!(render_data.max_generation, 9);
        assert_eq!(render_data.points.len(), 11);
        assert_eq!(render_data.connections.len(), 9);
    }
    
    #[test]
    fn test_spiral_render_data() {
        let spiral = SpiralPattern::golden_spiral(0.0, 0.0, 5.0);
        let render_data = SpiralRenderData::from_spiral_pattern(&spiral, &SacredGeometryColors::default(), 100);
        assert_eq!(render_data.points.len(), 100);
        assert_eq!(render_data.spiral_type, 0);
    }
    
    #[test]
    fn test_platonic_solid_render_data() {
        let solid = PlatonicStructure::tetrahedron(1.0);
        let render_data = PlatonicSolidRenderData::from_structure(&solid, &SacredGeometryColors::default(), 1.0);
        assert_eq!(render_data.vertices.len(), 4);
        assert_eq!(render_data.edges.len(), 6);
        assert_eq!(render_data.solid_type, 0);
    }
    
    #[test]
    fn test_vesica_piscis_render_data() {
        let vesica = VesicaPiscis::new((0.0, 0.0), (1.0, 0.0), 1.0);
        let render_data = VesicaPiscisRenderData::from_vesica_piscis(&vesica, &SacredGeometryColors::default(), 1.0);
        assert!(!render_data.intersection_points.is_empty());
    }
    
    #[test]
    fn test_flower_of_life_render_data() {
        let flower = FlowerOfLife::new(0.0, 0.0, 1.0, 3);
        let render_data = FlowerOfLifeRenderData::from_flower_of_life(&flower, &SacredGeometryColors::default(), 1.0);
        assert_eq!(render_data.num_layers, 3);
    }
    
    #[test]
    fn test_harmonic_resonance_render_data() {
        let series = HarmonicSeries::new(440.0);
        let resonance = FrequencyResonance::calculate(440.0, 880.0, 0.8, 0.8);
        let render_data = HarmonicResonanceRenderData::from_resonance(&resonance, &series, &SacredGeometryColors::default(), 1.0);
        assert_eq!(render_data.fundamental_frequency, 440.0);
        assert_eq!(render_data.overtone_bars.len(), 12);
    }
    
    #[test]
    fn test_standing_wave_render_data() {
        let wave = crate::sacred_geometry::StandingWave::calculate(440.0, 880.0);
        let render_data = StandingWaveRenderData::from_standing_wave(&wave, &SacredGeometryColors::default(), 100, 1.0);
        assert_eq!(render_data.wave_points.len(), 100);
    }
    
    #[test]
    fn test_sacred_geometry_visualization() {
        let mut viz = SacredGeometryVisualization::with_scale(1.5);
        let sequence = FibonacciSequence::new(5);
        viz.add_fibonacci(&sequence);
        assert!(viz.fibonacci.is_some());
        assert_eq!(viz.global_scale, 1.5);
    }
}
