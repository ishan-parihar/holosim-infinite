//! Harmonic Resonance Module
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
//! "Frequency-based resonance (not just spectrum ratios) - Musical harmonic series - Overtone patterns - Standing wave formations"
//!
//! This module implements true harmonic resonance based on frequency matching, not statistical averaging.
//! Entities resonate when their frequencies align with the harmonic series (2f, 3f, 4f, etc.),
//! creating standing waves and interference patterns that reflect the holographic nature of reality.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The holographic principle means that each entity contains the whole, and harmonic resonance
//! is how entities communicate and align across scales."
//!
//! ## Key Concepts
//!
//! - **Fundamental Frequency**: The base vibration of an entity (0.0-1.0 scaled to Hz)
//! - **Harmonic Series**: Overtones at integer multiples (2f, 3f, 4f, 5f, ...)
//! - **Overtone Pattern**: The strength of each overtone varies by archetype
//! - **Standing Wave**: Interference pattern when frequencies align
//! - **Resonance**: 1.0 = perfect harmonic alignment, 0.0 = no resonance
//!
//! ## Mathematical Foundation
//!
//! ```
//! f₀ = fundamental frequency (scaled from entity frequency)
//! fₙ = n × f₀ (n-th overtone)
//! resonance = 1.0 if |freq_a × n - freq_b| < tolerance for some n
//! ```

use crate::types::Float;
use std::fmt;

/// Golden ratio (φ) for frequency scaling
const PHI: Float = 1.6180339887498948482;

/// Base frequency scale (Hz) - maps entity frequency 0.0-1.0 to this range
const BASE_FREQUENCY_SCALE: Float = 432.0; // A=432Hz (sacred frequency)

/// Harmonic tolerance (±1% for matching)
const HARMONIC_TOLERANCE: Float = 0.01;

/// Maximum overtones to consider
const MAX_OVERTONES: usize = 12;

/// Harmonic Series - Fundamental frequency and overtones
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "Musical harmonic series (2x, 3x, 4x overtones)"
///
/// The harmonic series is the foundation of all musical harmony in the universe.
/// Each overtone is an integer multiple of the fundamental frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct HarmonicSeries {
    /// Fundamental frequency (Hz)
    pub fundamental: Float,
    /// Overtones: [2f, 3f, 4f, ..., (MAX_OVERTONES+1)f]
    pub overtones: Vec<Float>,
}

impl HarmonicSeries {
    /// Create a new harmonic series from a fundamental frequency
    ///
    /// # Arguments
    ///
    /// * `fundamental` - Base frequency in Hz
    ///
    /// # Returns
    ///
    /// A harmonic series with overtones at 2f, 3f, 4f, ..., (MAX_OVERTONES+1)f
    pub fn new(fundamental: Float) -> Self {
        let mut overtones = Vec::with_capacity(MAX_OVERTONES);
        for n in 2..=MAX_OVERTONES + 1 {
            overtones.push(fundamental * n as Float);
        }
        
        HarmonicSeries {
            fundamental,
            overtones,
        }
    }
    
    /// Create a harmonic series from entity frequency (0.0-1.0)
    ///
    /// Scales entity frequency to meaningful Hz using golden ratio scaling.
    ///
    /// # Arguments
    ///
    /// * `entity_frequency` - Entity frequency (0.0-1.0)
    ///
    /// # Returns
    ///
    /// A harmonic series with scaled fundamental frequency
    pub fn from_entity_frequency(entity_frequency: Float) -> Self {
        // Scale using φ to map 0.0-1.0 to BASE_FREQUENCY_SCALE range
        let fundamental = BASE_FREQUENCY_SCALE * (entity_frequency * PHI.powi(2));
        Self::new(fundamental)
    }
    
    /// Get a specific overtone
    ///
    /// # Arguments
    ///
    /// * `overtone_number` - Overtone number (1 = 2f, 2 = 3f, etc.)
    ///
    /// # Returns
    ///
    /// The frequency of the requested overtone, or None if out of range
    pub fn get_overtone(&self, overtone_number: usize) -> Option<Float> {
        if overtone_number == 0 {
            Some(self.fundamental)
        } else if overtone_number <= MAX_OVERTONES {
            self.overtones.get(overtone_number - 1).copied()
        } else {
            None
        }
    }
    
    /// Check if a frequency aligns with this harmonic series
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
    /// "NOT statistical averaging - actual frequency matching"
    ///
    /// # Arguments
    ///
    /// * `frequency` - Frequency to check (Hz)
    ///
    /// # Returns
    ///
    /// The overtone number if aligned, or None if not aligned
    pub fn check_alignment(&self, frequency: Float) -> Option<usize> {
        // Check fundamental
        if (frequency - self.fundamental).abs() < self.fundamental * HARMONIC_TOLERANCE {
            return Some(0);
        }
        
        // Check overtones
        for (i, &overtone) in self.overtones.iter().enumerate() {
            if (frequency - overtone).abs() < overtone * HARMONIC_TOLERANCE {
                return Some(i + 1);
            }
        }
        
        None
    }
}

impl fmt::Display for HarmonicSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HarmonicSeries(f₀={:.2}Hz, overtones={})", 
               self.fundamental, self.overtones.len())
    }
}

/// Musical Note - Represents a frequency with harmonic context
///
/// Maps frequencies to musical notes and provides harmonic relationships.
#[derive(Debug, Clone, PartialEq)]
pub struct MusicalNote {
    /// Frequency in Hz
    pub frequency: Float,
    /// Note name (C, C#, D, etc.) if calculable
    pub note_name: String,
    /// Octave number
    pub octave: i32,
    /// Harmonic series this note belongs to
    pub harmonic_series: HarmonicSeries,
}

impl MusicalNote {
    /// Create a new musical note from frequency
    ///
    /// # Arguments
    ///
    /// * `frequency` - Frequency in Hz
    ///
    /// # Returns
    ///
    /// A musical note with calculated note name and octave
    pub fn new(frequency: Float) -> Self {
        let harmonic_series = HarmonicSeries::new(frequency);
        let (note_name, octave) = Self::calculate_note_name(frequency);
        
        MusicalNote {
            frequency,
            note_name,
            octave,
            harmonic_series,
        }
    }
    
    /// Create a musical note from entity frequency
    ///
    /// # Arguments
    ///
    /// * `entity_frequency` - Entity frequency (0.0-1.0)
    ///
    /// # Returns
    ///
    /// A musical note scaled from entity frequency
    pub fn from_entity_frequency(entity_frequency: Float) -> Self {
        let harmonic_series = HarmonicSeries::from_entity_frequency(entity_frequency);
        let frequency = harmonic_series.fundamental;
        let (note_name, octave) = Self::calculate_note_name(frequency);
        
        MusicalNote {
            frequency,
            note_name,
            octave,
            harmonic_series,
        }
    }
    
    /// Calculate note name and octave from frequency
    ///
    /// Uses A4 = 440Hz as reference (standard tuning).
    ///
    /// # Arguments
    ///
    /// * `frequency` - Frequency in Hz
    ///
    /// # Returns
    ///
    /// (note_name, octave)
    fn calculate_note_name(frequency: Float) -> (String, i32) {
        const A4: Float = 440.0;
        const NOTE_NAMES: [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
        
        // Calculate semitones from A4
        let semitones = (frequency / A4).log2() * 12.0;
        let octave = 4 + (semitones / 12.0).floor() as i32;
        let note_index = ((semitones % 12.0).round() as i32 + 12) % 12;
        
        (NOTE_NAMES[note_index as usize].to_string(), octave)
    }
    
    /// Check if this note is harmonically related to another
    ///
    /// # Arguments
    ///
    /// * `other` - Another musical note
    ///
    /// # Returns
    ///
    /// The overtone relationship if harmonically related, or None
    pub fn is_harmonically_related(&self, other: &MusicalNote) -> Option<(usize, Float)> {
        // Check if other's frequency aligns with this note's harmonic series
        if let Some(overtone) = self.harmonic_series.check_alignment(other.frequency) {
            let ratio = other.frequency / self.frequency;
            return Some((overtone, ratio));
        }
        
        // Check if this note's frequency aligns with other's harmonic series
        if let Some(overtone) = other.harmonic_series.check_alignment(self.frequency) {
            let ratio = self.frequency / other.frequency;
            return Some((overtone, ratio));
        }
        
        None
    }
}

impl fmt::Display for MusicalNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{} ({:.2}Hz)", self.note_name, self.octave, self.frequency)
    }
}

/// Overtone Pattern - The strength of each overtone
///
/// Different archetypes have different overtone patterns, reflecting their
/// unique vibrational signatures in the holographic universe.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each archetype has a unique vibrational signature that influences entity evolution."
#[derive(Debug, Clone, PartialEq)]
pub struct OvertonePattern {
    /// Strength of each overtone (0.0-1.0), index 0 = fundamental
    pub strengths: Vec<Float>,
}

impl OvertonePattern {
    /// Create a new overtone pattern with custom strengths
    ///
    /// # Arguments
    ///
    /// * `strengths` - Strength of each overtone (fundamental + overtones)
    ///
    /// # Returns
    ///
    /// A new overtone pattern
    pub fn new(strengths: Vec<Float>) -> Self {
        OvertonePattern { strengths }
    }
    
    /// Create a natural overtone pattern (1/n falloff)
    ///
    /// Natural harmonics decrease in strength: 1, 1/2, 1/3, 1/4, ...
    ///
    /// # Returns
    ///
    /// A natural overtone pattern
    pub fn natural() -> Self {
        let mut strengths = Vec::with_capacity(MAX_OVERTONES + 1);
        for n in 1..=MAX_OVERTONES + 1 {
            strengths.push(1.0 / n as Float);
        }
        OvertonePattern { strengths }
    }
    
    /// Create an overtone pattern based on Fibonacci sequence
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Fibonacci sequences appear in growth patterns throughout the universe."
    ///
    /// # Returns
    ///
    /// A Fibonacci-based overtone pattern
    pub fn fibonacci() -> Self {
        let mut strengths = Vec::with_capacity(MAX_OVERTONES + 1);
        let mut fib_prev: u64 = 1;
        let mut fib_curr: u64 = 1;
        
        for _ in 0..=MAX_OVERTONES {
            strengths.push(1.0 / fib_curr as Float);
            let fib_next = fib_prev + fib_curr;
            fib_prev = fib_curr;
            fib_curr = fib_next;
        }
        
        OvertonePattern { strengths }
    }
    
    /// Create an overtone pattern based on golden ratio
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The golden ratio (φ) is the fundamental proportion in sacred geometry."
    ///
    /// # Returns
    ///
    /// A golden ratio-based overtone pattern
    pub fn golden_ratio() -> Self {
        let mut strengths = Vec::with_capacity(MAX_OVERTONES + 1);
        for n in 0..=MAX_OVERTONES {
            strengths.push(PHI.powf(-(n as Float)));
        }
        OvertonePattern { strengths }
    }
    
    /// Create an overtone pattern for a specific archetype
    ///
    /// # Arguments
    ///
    /// * `archetype` - Archetype number (1-22)
    ///
    /// # Returns
    ///
    /// An archetype-specific overtone pattern
    pub fn archetype(archetype: usize) -> Self {
        match archetype {
            // Archetype 22 (The Choice) has strong fundamental, balanced overtones
            22 => {
                let mut strengths = vec![1.0]; // Strong fundamental
                for n in 1..=MAX_OVERTONES {
                    // Golden ratio falloff for harmonics
                    strengths.push(PHI.powf(-(n as Float) * 0.5));
                }
                OvertonePattern { strengths }
            }
            // Great Way archetypes (7, 14, 21) have strong higher overtones
            7 | 14 | 21 => {
                let mut strengths = Vec::with_capacity(MAX_OVERTONES + 1);
                for n in 0..=MAX_OVERTONES {
                    // Stronger higher overtones
                    strengths.push((n as Float + 1.0) / (MAX_OVERTONES + 1) as Float);
                }
                OvertonePattern { strengths }
            }
            // Default: Fibonacci pattern
            _ => Self::fibonacci(),
        }
    }
    
    /// Get the strength of a specific overtone
    ///
    /// # Arguments
    ///
    /// * `overtone` - Overtone number (0 = fundamental)
    ///
    /// # Returns
    ///
    /// The strength (0.0-1.0), or 0.0 if out of range
    pub fn get_strength(&self, overtone: usize) -> Float {
        self.strengths.get(overtone).copied().unwrap_or(0.0)
    }
    
    /// Normalize the pattern so total strength = 1.0
    ///
    /// # Returns
    ///
    /// A normalized overtone pattern
    pub fn normalize(&self) -> Self {
        let total: Float = self.strengths.iter().sum();
        if total == 0.0 {
            return self.clone();
        }
        
        let strengths = self.strengths.iter().map(|&s| s / total).collect();
        OvertonePattern { strengths }
    }
}

/// Standing Wave - Wave interference pattern
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "Standing wave formations - interference patterns when frequencies align"
///
/// Standing waves occur when frequencies align harmonically, creating
/// stable interference patterns that reflect the holographic nature of reality.
#[derive(Debug, Clone, PartialEq)]
pub struct StandingWave {
    /// Primary frequency (Hz)
    pub primary_frequency: Float,
    /// Secondary frequency (Hz)
    pub secondary_frequency: Float,
    /// Interference pattern (constructive vs destructive)
    pub interference_type: InterferenceType,
    /// Node positions (where amplitude is zero)
    pub nodes: Vec<Float>,
    /// Antinode positions (where amplitude is maximum)
    pub antinodes: Vec<Float>,
    /// Wave amplitude at center
    pub center_amplitude: Float,
}

/// Type of interference pattern
#[derive(Debug, Clone, PartialEq)]
pub enum InterferenceType {
    /// Constructive interference (waves reinforce)
    Constructive,
    /// Destructive interference (waves cancel)
    Destructive,
    /// Partial interference (mixed)
    Partial,
}

impl StandingWave {
    /// Calculate standing wave pattern between two frequencies
    ///
    /// # Arguments
    ///
    /// * `freq_a` - First frequency (Hz)
    /// * `freq_b` - Second frequency (Hz)
    ///
    /// # Returns
    ///
    /// A standing wave pattern
    pub fn calculate(freq_a: Float, freq_b: Float) -> Self {
        // Determine interference type based on harmonic relationship
        let ratio = if freq_b > 0.0 { freq_a / freq_b } else { 0.0 };
        let (interference_type, center_amplitude) = if ratio.is_finite() && ratio > 0.0 {
            // Check for harmonic relationship
            let is_integer = (ratio - ratio.round()).abs() < HARMONIC_TOLERANCE;
            let is_reciprocal = (1.0/ratio - (1.0/ratio).round()).abs() < HARMONIC_TOLERANCE;
            
            if is_integer || is_reciprocal {
                // Harmonic relationship: constructive interference
                let amplitude = 2.0 * (ratio.min(1.0/ratio).min(1.0));
                (InterferenceType::Constructive, amplitude)
            } else if ratio > 0.9 && ratio < 1.1 {
                // Nearly equal frequencies: beating pattern
                (InterferenceType::Partial, 1.0 + (1.0 - (ratio - 1.0).abs()))
            } else {
                // Non-harmonic: destructive interference
                (InterferenceType::Destructive, (ratio.min(1.0/ratio)).max(0.0))
            }
        } else {
            (InterferenceType::Destructive, 0.0)
        };
        
        // Calculate nodes and antinodes
        let wavelength = if freq_a > freq_b {
            1.0 / freq_a
        } else {
            1.0 / freq_b
        };
        
        let mut nodes = Vec::new();
        let mut antinodes = Vec::new();
        
        // Nodes occur at n * wavelength / 2
        for n in 0..=10 {
            nodes.push(n as Float * wavelength / 2.0);
            // Antinodes occur at (2n + 1) * wavelength / 4
            antinodes.push((2 * n + 1) as Float * wavelength / 4.0);
        }
        
        StandingWave {
            primary_frequency: freq_a,
            secondary_frequency: freq_b,
            interference_type,
            nodes,
            antinodes,
            center_amplitude,
        }
    }
    
    /// Get the interference strength at a specific position
    ///
    /// # Arguments
    ///
    /// * `position` - Position along the wave (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// Amplitude at the position (0.0 to 1.0)
    pub fn get_amplitude_at(&self, position: Float) -> Float {
        let k = 2.0 * std::f64::consts::PI * self.primary_frequency;
        let amplitude = self.center_amplitude * (k * position).cos().abs();
        amplitude.min(1.0)
    }
}

impl fmt::Display for StandingWave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StandingWave({:.2}Hz × {:.2}Hz, {:?}, amplitude={:.2})", 
               self.primary_frequency, self.secondary_frequency, 
               self.interference_type, self.center_amplitude)
    }
}

/// Frequency Resonance - Resonance calculation between entities
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "Entities resonate when their frequencies align (NOT statistical averaging)"
///
/// This is the core harmonic resonance calculation. Resonance is NOT based on
/// statistical similarity, but on actual frequency alignment with the harmonic series.
#[derive(Debug, Clone, PartialEq)]
pub struct FrequencyResonance {
    /// Resonance strength (0.0 = no resonance, 1.0 = perfect harmonic)
    pub resonance_strength: Float,
    /// Harmonic overtone relationship (if any)
    pub harmonic_overtone: Option<usize>,
    /// Frequency ratio
    pub frequency_ratio: Float,
    /// Standing wave pattern
    pub standing_wave: StandingWave,
    /// Coherence contribution (0.0-1.0)
    pub coherence_contribution: Float,
}

impl FrequencyResonance {
    /// Calculate resonance between two frequencies
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
    /// "NOT statistical averaging - actual frequency matching"
    ///
    /// # Arguments
    ///
    /// * `freq_a` - First frequency (Hz)
    /// * `freq_b` - Second frequency (Hz)
    /// * `coherence_a` - First entity's vibrational coherence (0.0-1.0)
    /// * `coherence_b` - Second entity's vibrational coherence (0.0-1.0)
    ///
    /// # Returns
    ///
    /// A frequency resonance calculation
    pub fn calculate(freq_a: Float, freq_b: Float, coherence_a: Float, coherence_b: Float) -> Self {
        // Calculate frequency ratio
        let frequency_ratio = if freq_b > 0.0 { freq_a / freq_b } else { 0.0 };
        
        // Check for harmonic alignment
        let harmonic_series = HarmonicSeries::new(freq_a);
        let harmonic_overtone = harmonic_series.check_alignment(freq_b);
        
        // Calculate resonance strength
        let resonance_strength = if let Some(overtone) = harmonic_overtone {
            // Perfect harmonic alignment: 1.0
            1.0
        } else {
            // Check reciprocal (freq_b harmonic of freq_a)
            let harmonic_series_b = HarmonicSeries::new(freq_b);
            if let Some(overtone_b) = harmonic_series_b.check_alignment(freq_a) {
                // Reciprocal harmonic alignment
                1.0
            } else {
                // Non-harmonic: calculate interference
                let standing_wave = StandingWave::calculate(freq_a, freq_b);
                match standing_wave.interference_type {
                    InterferenceType::Constructive => 0.8,
                    InterferenceType::Partial => 0.5,
                    InterferenceType::Destructive => 0.2,
                }
            }
        };
        
        // Calculate standing wave
        let standing_wave = StandingWave::calculate(freq_a, freq_b);
        
        // Coherence contribution (average of both entities' coherence)
        let coherence_contribution = (coherence_a + coherence_b) / 2.0;
        
        // Final resonance combines harmonic alignment and coherence
        let final_resonance = resonance_strength * (0.5 + coherence_contribution * 0.5);
        
        FrequencyResonance {
            resonance_strength: final_resonance,
            harmonic_overtone,
            frequency_ratio,
            standing_wave,
            coherence_contribution,
        }
    }
    
    /// Calculate resonance between two entity frequencies
    ///
    /// # Arguments
    ///
    /// * `entity_freq_a` - First entity frequency (0.0-1.0)
    /// * `entity_freq_b` - Second entity frequency (0.0-1.0)
    /// * `coherence_a` - First entity's vibrational coherence (0.0-1.0)
    /// * `coherence_b` - Second entity's vibrational coherence (0.0-1.0)
    ///
    /// # Returns
    ///
    /// A frequency resonance calculation
    pub fn calculate_from_entities(
        entity_freq_a: Float,
        entity_freq_b: Float,
        coherence_a: Float,
        coherence_b: Float,
    ) -> Self {
        let freq_a = HarmonicSeries::from_entity_frequency(entity_freq_a).fundamental;
        let freq_b = HarmonicSeries::from_entity_frequency(entity_freq_b).fundamental;
        
        Self::calculate(freq_a, freq_b, coherence_a, coherence_b)
    }
    
    /// Check if resonance is strong (above threshold)
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum resonance strength (default 0.7)
    ///
    /// # Returns
    ///
    /// True if resonance is strong
    pub fn is_strong(&self, threshold: Float) -> bool {
        self.resonance_strength >= threshold
    }
    
    /// Check if resonance is harmonic (exact overtone relationship)
    ///
    /// # Returns
    ///
    /// True if resonance is harmonic
    pub fn is_harmonic(&self) -> bool {
        self.harmonic_overtone.is_some()
    }
}

impl fmt::Display for FrequencyResonance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let harmonic_info = if let Some(overtone) = self.harmonic_overtone {
            format!("harmonic overtone {}", overtone)
        } else {
            "non-harmonic".to_string()
        };
        
        write!(f, "FrequencyResonance(strength={:.3}, {}, ratio={:.3}, coherence={:.3})", 
               self.resonance_strength, harmonic_info, self.frequency_ratio, 
               self.coherence_contribution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_series_creation() {
        let series = HarmonicSeries::new(440.0);
        assert_eq!(series.fundamental, 440.0);
        assert_eq!(series.overtones.len(), MAX_OVERTONES);
        assert!((series.overtones[0] - 880.0).abs() < 0.01); // 2f
        assert!((series.overtones[1] - 1320.0).abs() < 0.01); // 3f
    }

    #[test]
    fn test_harmonic_series_from_entity_frequency() {
        let series = HarmonicSeries::from_entity_frequency(0.5);
        assert!(series.fundamental > 0.0);
        // Should be scaled using φ
        let expected = BASE_FREQUENCY_SCALE * (0.5 * PHI.powi(2));
        assert!((series.fundamental - expected).abs() < 0.01);
    }

    #[test]
    fn test_harmonic_series_alignment() {
        let series = HarmonicSeries::new(440.0);
        
        // Check fundamental alignment
        assert_eq!(series.check_alignment(440.0), Some(0));
        
        // Check overtone alignment (2f = 880Hz)
        assert_eq!(series.check_alignment(880.0), Some(1));
        assert_eq!(series.check_alignment(1320.0), Some(2));
        
        // Check non-alignment
        assert_eq!(series.check_alignment(450.0), None);
    }

    #[test]
    fn test_musical_note_creation() {
        let note = MusicalNote::new(440.0);
        assert_eq!(note.frequency, 440.0);
        assert_eq!(note.note_name, "A");
        assert_eq!(note.octave, 4);
    }

    #[test]
    fn test_musical_note_harmonic_relationship() {
        let note_a = MusicalNote::new(440.0);
        let note_octave = MusicalNote::new(880.0);
        
        let relationship = note_a.is_harmonically_related(&note_octave);
        assert!(relationship.is_some());
        let (overtone, ratio) = relationship.unwrap();
        assert_eq!(overtone, 1); // First overtone
        assert!((ratio - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_overtone_pattern_natural() {
        let pattern = OvertonePattern::natural();
        assert_eq!(pattern.strengths.len(), MAX_OVERTONES + 1);
        assert!((pattern.strengths[0] - 1.0).abs() < 0.01); // Fundamental
        assert!((pattern.strengths[1] - 0.5).abs() < 0.01); // 1/2
        assert!((pattern.strengths[2] - 0.333).abs() < 0.01); // ~1/3
    }

    #[test]
    fn test_overtone_pattern_fibonacci() {
        let pattern = OvertonePattern::fibonacci();
        assert_eq!(pattern.strengths.len(), MAX_OVERTONES + 1);
        // Fibonacci pattern should have specific ratios
        assert!(pattern.strengths[0] > 0.0);
        assert!(pattern.strengths[1] > 0.0);
        assert!(pattern.strengths[1] < pattern.strengths[0]); // Decreasing
    }

    #[test]
    fn test_overtone_pattern_golden_ratio() {
        let pattern = OvertonePattern::golden_ratio();
        assert_eq!(pattern.strengths.len(), MAX_OVERTONES + 1);
        // Golden ratio falloff
        assert!((pattern.strengths[0] - 1.0).abs() < 0.01); // φ^0 = 1
        assert!((pattern.strengths[1] - 1.0/PHI).abs() < 0.01); // φ^-1
        assert!((pattern.strengths[2] - 1.0/PHI.powi(2)).abs() < 0.01); // φ^-2
    }

    #[test]
    fn test_overtone_pattern_archetype() {
        let pattern_22 = OvertonePattern::archetype(22);
        assert_eq!(pattern_22.strengths.len(), MAX_OVERTONES + 1);
        // Archetype 22 should have strong fundamental
        assert!(pattern_22.strengths[0] > 0.9);
        
        let pattern_7 = OvertonePattern::archetype(7);
        // Great Way archetype should have stronger higher overtones
        assert!(pattern_7.strengths[MAX_OVERTONES] > pattern_7.strengths[0]);
    }

    #[test]
    fn test_overtone_pattern_normalize() {
        let pattern = OvertonePattern::natural();
        let normalized = pattern.normalize();
        
        // Check that sum is approximately 1.0
        let sum: Float = normalized.strengths.iter().sum();
        assert!((sum - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_standing_wave_constructive() {
        let wave = StandingWave::calculate(440.0, 880.0);
        assert_eq!(wave.interference_type, InterferenceType::Constructive);
        assert!(wave.center_amplitude > 0.0);
        assert!(!wave.nodes.is_empty());
        assert!(!wave.antinodes.is_empty());
    }

    #[test]
    fn test_standing_wave_destructive() {
        let wave = StandingWave::calculate(440.0, 600.0);
        // Non-harmonic should be destructive
        assert!(wave.center_amplitude < 0.8);
    }

    #[test]
    fn test_standing_wave_amplitude_at_position() {
        let wave = StandingWave::calculate(440.0, 880.0);
        
        // Check amplitude at different positions
        let amp_0 = wave.get_amplitude_at(0.0);
        let amp_0_5 = wave.get_amplitude_at(0.5);
        let amp_1 = wave.get_amplitude_at(1.0);
        
        assert!(amp_0 >= 0.0 && amp_0 <= 1.0);
        assert!(amp_0_5 >= 0.0 && amp_0_5 <= 1.0);
        assert!(amp_1 >= 0.0 && amp_1 <= 1.0);
    }

    #[test]
    fn test_frequency_resonance_harmonic() {
        let resonance = FrequencyResonance::calculate(440.0, 880.0, 0.8, 0.8);
        
        // Should be harmonic (880 = 2 * 440)
        assert!(resonance.is_harmonic());
        assert_eq!(resonance.harmonic_overtone, Some(1));
        assert!((resonance.frequency_ratio - 0.5).abs() < 0.01);
        assert!(resonance.resonance_strength >= 0.9);
    }

    #[test]
    fn test_frequency_resonance_non_harmonic() {
        let resonance = FrequencyResonance::calculate(440.0, 450.0, 0.5, 0.5);
        
        // Should not be harmonic
        assert!(!resonance.is_harmonic());
        assert_eq!(resonance.harmonic_overtone, None);
        assert!(resonance.resonance_strength < 1.0);
    }

    #[test]
    fn test_frequency_resonance_from_entities() {
        let resonance = FrequencyResonance::calculate_from_entities(0.5, 0.5, 0.8, 0.8);
        
        // Same frequency should resonate
        assert!(resonance.resonance_strength > 0.5);
        assert!((resonance.frequency_ratio - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_frequency_resonance_coherence_contribution() {
        // Low coherence
        let resonance_low = FrequencyResonance::calculate(440.0, 880.0, 0.2, 0.2);
        
        // High coherence
        let resonance_high = FrequencyResonance::calculate(440.0, 880.0, 0.9, 0.9);
        
        // Higher coherence should result in stronger resonance
        assert!(resonance_high.resonance_strength > resonance_low.resonance_strength);
    }

    #[test]
    fn test_frequency_resonance_is_strong() {
        let strong = FrequencyResonance::calculate(440.0, 880.0, 0.9, 0.9);
        let weak = FrequencyResonance::calculate(440.0, 450.0, 0.3, 0.3);
        
        assert!(strong.is_strong(0.7));
        assert!(!weak.is_strong(0.7));
    }
}
