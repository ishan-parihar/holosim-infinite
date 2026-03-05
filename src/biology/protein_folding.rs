//! Protein folding guided by holographic blueprint
//!
//! From HOLOSIM_INFINITE_RnD_ROADMAP_V5.md Phase 3.4:
//! "Protein Folding as Blueprint Unfolding"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Native state = minimum interference configuration"
//!
//! This module implements:
//! 1. Protein folding guided by holographic blueprint
//! 2. Native state as minimum interference configuration
//! 3. Blueprint encoding target shape (not hardcoded values)
//! 4. Folding pathway from primary to quaternary structure

use crate::holographic::holographic_field::HolographicField;
use crate::holographic::Position;
use crate::matter::particle::Coordinate3D;
use crate::types::Float;

// ============================================================================
// AMINO ACID TYPES
// ============================================================================

/// Amino acid representation
///
/// Each amino acid has a characteristic archetype signature
/// that determines how it contributes to protein folding.
#[derive(Debug, Clone, PartialEq)]
pub enum AminoAcid {
    // Nonpolar (hydrophobic)
    Alanine,
    Valine,
    Leucine,
    Isoleucine,
    Methionine,
    Phenylalanine,
    Tryptophan,
    Proline,
    // Polar uncharged
    Glycine,
    Serine,
    Threonine,
    Cysteine,
    Tyrosine,
    Asparagine,
    Glutamine,
    // Charged
    Lysine,
    Arginine,
    Histidine,
    AsparticAcid,
    GlutamicAcid,
}

impl AminoAcid {
    /// Get single-letter code
    pub fn code(&self) -> char {
        match self {
            AminoAcid::Alanine => 'A',
            AminoAcid::Arginine => 'R',
            AminoAcid::Asparagine => 'N',
            AminoAcid::AsparticAcid => 'D',
            AminoAcid::Cysteine => 'C',
            AminoAcid::GlutamicAcid => 'E',
            AminoAcid::Glutamine => 'Q',
            AminoAcid::Glycine => 'G',
            AminoAcid::Histidine => 'H',
            AminoAcid::Isoleucine => 'I',
            AminoAcid::Leucine => 'L',
            AminoAcid::Lysine => 'K',
            AminoAcid::Methionine => 'M',
            AminoAcid::Phenylalanine => 'F',
            AminoAcid::Proline => 'P',
            AminoAcid::Serine => 'S',
            AminoAcid::Threonine => 'T',
            AminoAcid::Tryptophan => 'W',
            AminoAcid::Tyrosine => 'Y',
            AminoAcid::Valine => 'V',
        }
    }

    /// Get three-letter code
    pub fn three_letter_code(&self) -> &'static str {
        match self {
            AminoAcid::Alanine => "Ala",
            AminoAcid::Arginine => "Arg",
            AminoAcid::Asparagine => "Asn",
            AminoAcid::AsparticAcid => "Asp",
            AminoAcid::Cysteine => "Cys",
            AminoAcid::GlutamicAcid => "Glu",
            AminoAcid::Glutamine => "Gln",
            AminoAcid::Glycine => "Gly",
            AminoAcid::Histidine => "His",
            AminoAcid::Isoleucine => "Ile",
            AminoAcid::Leucine => "Leu",
            AminoAcid::Lysine => "Lys",
            AminoAcid::Methionine => "Met",
            AminoAcid::Phenylalanine => "Phe",
            AminoAcid::Proline => "Pro",
            AminoAcid::Serine => "Ser",
            AminoAcid::Threonine => "Thr",
            AminoAcid::Tryptophan => "Trp",
            AminoAcid::Tyrosine => "Tyr",
            AminoAcid::Valine => "Val",
        }
    }

    /// Get archetype signature for this amino acid
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// Each amino acid has a characteristic archetype pattern
    /// that determines how it contributes to protein folding.
    ///
    /// The 22 values correspond to the 22 archetypes:
    /// - Indices 0-6: Mind Complex (Matrix, Potentiator, Catalyst, Experience, Significator, Transformation, Great Way)
    /// - Indices 7-13: Body Complex
    /// - Indices 14-20: Spirit Complex
    /// - Index 21: The Choice (Archetype 22)
    pub fn archetype_signature(&self) -> [Float; 22] {
        // Each amino acid has a characteristic archetype pattern
        // This determines how it contributes to protein folding
        let mut sig = [0.5; 22];

        match self {
            // Hydrophobic: Matrix archetype dominant (stability)
            // High Matrix (A1, A8, A15) promotes stable, folded structures
            AminoAcid::Alanine | AminoAcid::Valine | AminoAcid::Leucine | AminoAcid::Isoleucine => {
                sig[0] = 0.8; // Matrix
                sig[6] = 0.7; // Great Way (stability)
                sig[7] = 0.8; // Body Matrix
                sig[14] = 0.75; // Spirit Matrix
            }
            // Methionine: Start codon - high Transformation for initiation
            AminoAcid::Methionine => {
                sig[5] = 0.9; // Transformation (initiation)
                sig[12] = 0.85; // Body Transformation
                sig[0] = 0.7; // Matrix
            }
            // Phenylalanine/Tryptophan: Aromatic - high Significator
            AminoAcid::Phenylalanine | AminoAcid::Tryptophan => {
                sig[4] = 0.85; // Significator (identity/aromatic)
                sig[11] = 0.8; // Body Significator
                sig[0] = 0.75; // Matrix
            }
            // Polar: Experience archetype dominant (interaction)
            AminoAcid::Serine
            | AminoAcid::Threonine
            | AminoAcid::Asparagine
            | AminoAcid::Glutamine => {
                sig[3] = 0.8; // Experience (interaction)
                sig[10] = 0.75; // Body Experience
                sig[17] = 0.7; // Spirit Experience
            }
            // Charged negative (acidic): Catalyst archetype high
            AminoAcid::AsparticAcid | AminoAcid::GlutamicAcid => {
                sig[2] = 0.9; // Catalyst (reactivity)
                sig[9] = 0.85; // Body Catalyst
            }
            // Charged positive (basic): Inverse catalyst pattern
            AminoAcid::Lysine | AminoAcid::Arginine => {
                sig[2] = 0.1; // Inverse catalyst (positive charge)
                sig[9] = 0.15; // Body Catalyst inverse
                sig[1] = 0.8; // Potentiator (activation)
            }
            // Histidine: Special - can be positive or neutral
            AminoAcid::Histidine => {
                sig[2] = 0.5; // Variable catalyst
                sig[1] = 0.7; // Potentiator
                sig[3] = 0.7; // Experience
            }
            // Cysteine: Disulfide bonds - Transformation archetype
            AminoAcid::Cysteine => {
                sig[5] = 0.9; // Transformation (disulfide bonds)
                sig[12] = 0.85; // Body Transformation
            }
            // Proline: Rigid structure - very high Matrix
            AminoAcid::Proline => {
                sig[0] = 0.95; // High Matrix (rigid structure)
                sig[6] = 0.9; // Great Way (structural constraint)
                sig[7] = 0.9; // Body Matrix
            }
            // Glycine: Flexible - low Matrix (most flexible)
            AminoAcid::Glycine => {
                sig[0] = 0.1; // Low Matrix (flexible)
                sig[7] = 0.15; // Body Matrix low
                sig[5] = 0.7; // Transformation (flexibility)
            }
            // Tyrosine: Aromatic + polar
            AminoAcid::Tyrosine => {
                sig[4] = 0.75; // Significator (aromatic)
                sig[3] = 0.7; // Experience (polar)
                sig[10] = 0.7; // Body Experience
            }
        }

        // Set Choice archetype (A22) based on flexibility
        sig[21] = 0.5 + (1.0 - sig[0]) * 0.3; // More choice when less Matrix

        sig
    }

    /// Check if amino acid is hydrophobic
    pub fn is_hydrophobic(&self) -> bool {
        matches!(
            self,
            AminoAcid::Alanine
                | AminoAcid::Valine
                | AminoAcid::Leucine
                | AminoAcid::Isoleucine
                | AminoAcid::Methionine
                | AminoAcid::Phenylalanine
                | AminoAcid::Tryptophan
                | AminoAcid::Proline
        )
    }

    /// Check if amino acid is polar
    pub fn is_polar(&self) -> bool {
        matches!(
            self,
            AminoAcid::Serine
                | AminoAcid::Threonine
                | AminoAcid::Asparagine
                | AminoAcid::Glutamine
                | AminoAcid::Tyrosine
                | AminoAcid::Cysteine
                | AminoAcid::Glycine
        )
    }

    /// Check if amino acid is charged
    pub fn is_charged(&self) -> bool {
        matches!(
            self,
            AminoAcid::Lysine
                | AminoAcid::Arginine
                | AminoAcid::Histidine
                | AminoAcid::AsparticAcid
                | AminoAcid::GlutamicAcid
        )
    }

    /// Get charge at physiological pH (7.4)
    pub fn charge(&self) -> Float {
        match self {
            AminoAcid::Lysine | AminoAcid::Arginine => 1.0,
            AminoAcid::Histidine => 0.1, // Partially charged
            AminoAcid::AsparticAcid | AminoAcid::GlutamicAcid => -1.0,
            _ => 0.0,
        }
    }
}

// ============================================================================
// SECONDARY STRUCTURE
// ============================================================================

/// Secondary structure type
#[derive(Debug, Clone, PartialEq)]
pub enum SecondaryStructure {
    AlphaHelix,
    BetaSheet,
    RandomCoil,
    Turn,
    /// Mixed structure with regions of different types
    Mixed(Vec<SecondaryStructureRegion>),
}

/// A region of secondary structure
#[derive(Debug, Clone, PartialEq)]
pub struct SecondaryStructureRegion {
    pub structure_type: SecondaryStructureType,
    pub start: usize,
    pub end: usize,
}

/// Secondary structure type (for regions)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecondaryStructureType {
    AlphaHelix,
    BetaSheet,
    Coil,
    Turn,
}

// ============================================================================
// FOLDING STATE
// ============================================================================

/// Folding state
#[derive(Debug, Clone, PartialEq)]
pub enum FoldingState {
    /// Unfolded (extended chain)
    Unfolded,
    /// Partially folded with progress indicator
    PartialFolded { progress: Float },
    /// Native state (minimum interference configuration)
    Native,
    /// Misfolded (stuck in local minimum)
    Misfolded { interference_energy: Float },
    /// Aggregated with other proteins
    Aggregated,
}

// ============================================================================
// PROTEIN STRUCTURE
// ============================================================================

/// Protein with blueprint-guided folding
///
/// From ROADMAP Phase 3.4:
/// "Protein folding guided by holographic blueprint"
/// "Native state = minimum interference configuration"
#[derive(Debug, Clone)]
pub struct Protein {
    /// Primary structure (amino acid sequence)
    pub sequence: Vec<AminoAcid>,

    /// Secondary structure prediction
    pub secondary: SecondaryStructure,

    /// Tertiary structure (3D coordinates of each residue)
    pub tertiary: Vec<Coordinate3D>,

    /// Quaternary structure (assembly with other proteins)
    pub quaternary: Option<QuaternaryStructure>,

    /// Current folding state
    pub folding_state: FoldingState,

    /// Interference energy (lower = more stable)
    /// From ROADMAP: "Native state = minimum interference configuration"
    pub interference_energy: Float,

    /// Folding pathway (history of intermediate states)
    pub folding_pathway: Vec<FoldingIntermediate>,

    /// Sequence length
    pub length: usize,
}

/// Quaternary structure (protein assembly)
#[derive(Debug, Clone)]
pub struct QuaternaryStructure {
    /// Subunit indices (which parts of sequence belong to which subunit)
    pub subunits: Vec<ProteinSubunit>,
    /// Assembly type
    pub assembly_type: AssemblyType,
}

/// A protein subunit
#[derive(Debug, Clone)]
pub struct ProteinSubunit {
    /// Start index in sequence
    pub start: usize,
    /// End index in sequence
    pub end: usize,
    /// Subunit name
    pub name: String,
}

/// Assembly type for quaternary structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssemblyType {
    Monomer,
    Dimer,
    Trimer,
    Tetramer,
    Multimer,
}

/// Folding intermediate state
#[derive(Debug, Clone)]
pub struct FoldingIntermediate {
    /// Progress toward native state (0.0 to 1.0)
    pub progress: Float,
    /// Interference energy at this state
    pub interference_energy: Float,
    /// Secondary structure content
    pub secondary_content: SecondaryStructureContent,
    /// Tertiary structure at this state
    pub coordinates: Vec<Coordinate3D>,
}

/// Secondary structure content percentages
#[derive(Debug, Clone, Default)]
pub struct SecondaryStructureContent {
    pub helix_fraction: Float,
    pub sheet_fraction: Float,
    pub coil_fraction: Float,
}

impl Protein {
    /// Create new protein from amino acid sequence
    ///
    /// Initializes as unfolded (extended chain conformation)
    pub fn new(sequence: Vec<AminoAcid>) -> Self {
        let length = sequence.len();
        let tertiary = Self::extended_chain(&sequence);

        Self {
            sequence,
            secondary: SecondaryStructure::RandomCoil,
            tertiary,
            quaternary: None,
            folding_state: FoldingState::Unfolded,
            interference_energy: Float::MAX,
            folding_pathway: Vec::new(),
            length,
        }
    }

    /// Create extended chain conformation
    ///
    /// Standard peptide bond geometry: 3.8 Å per residue
    fn extended_chain(sequence: &[AminoAcid]) -> Vec<Coordinate3D> {
        sequence
            .iter()
            .enumerate()
            .map(|(i, _)| Coordinate3D::new(i as Float * 3.8, 0.0, 0.0))
            .collect()
    }

    /// Predict secondary structure from amino acid archetypes
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// Secondary structure emerges from the archetype signatures
    /// of the amino acid sequence.
    pub fn predict_secondary_structure(&self) -> SecondaryStructure {
        if self.sequence.is_empty() {
            return SecondaryStructure::RandomCoil;
        }

        let mut helix_propensity = 0.0;
        let mut sheet_propensity = 0.0;
        let mut _coil_propensity = 0.0;

        for aa in &self.sequence {
            let sig = aa.archetype_signature();

            // Matrix archetype promotes helix (stable, regular structure)
            helix_propensity += sig[0] + sig[7] + sig[14];

            // Great Way archetype promotes sheet (extended, stable)
            sheet_propensity += sig[6] + sig[13] + sig[20];

            // Transformation and Choice promote coil (flexible)
            _coil_propensity += sig[5] + sig[12] + sig[19] + sig[21];
        }

        let len = self.sequence.len() as Float;
        helix_propensity /= len * 3.0;
        sheet_propensity /= len * 3.0;
        _coil_propensity /= len * 4.0;

        // Determine dominant structure
        if helix_propensity > 0.6 && helix_propensity > sheet_propensity {
            SecondaryStructure::AlphaHelix
        } else if sheet_propensity > 0.6 && sheet_propensity > helix_propensity {
            SecondaryStructure::BetaSheet
        } else if helix_propensity > 0.4 && sheet_propensity > 0.4 {
            // Mixed structure
            SecondaryStructure::Mixed(identify_regions(
                &self.sequence,
                helix_propensity,
                sheet_propensity,
            ))
        } else {
            SecondaryStructure::RandomCoil
        }
    }

    /// Check if protein is in native state
    ///
    /// From ROADMAP: "Native state = minimum interference configuration"
    pub fn is_native(&self) -> bool {
        matches!(self.folding_state, FoldingState::Native) && self.interference_energy < 0.5
    }

    /// Get sequence as string of single-letter codes
    pub fn sequence_string(&self) -> String {
        self.sequence.iter().map(|aa| aa.code()).collect()
    }
}

/// Identify regions of secondary structure (standalone helper)
///
/// Used by both Protein and BlueprintProteinShape to identify
/// secondary structure regions within a sequence.
fn identify_regions(
    sequence: &[AminoAcid],
    _helix_threshold: Float,
    _sheet_threshold: Float,
) -> Vec<SecondaryStructureRegion> {
    let mut regions = Vec::new();

    if sequence.is_empty() {
        return regions;
    }

    // Simple window-based prediction
    let window_size = 7;
    let mut current_type = SecondaryStructureType::Coil;
    let mut region_start = 0;

    for i in 0..sequence.len() {
        let start = i.saturating_sub(window_size / 2);
        let end = (i + window_size / 2 + 1).min(sequence.len());

        let window = &sequence[start..end];
        let helix_count = window.iter().filter(|aa| aa.is_hydrophobic()).count();
        let sheet_count = window.iter().filter(|aa| aa.is_polar()).count();

        let new_type = if helix_count > window.len() / 2 {
            SecondaryStructureType::AlphaHelix
        } else if sheet_count > window.len() / 2 {
            SecondaryStructureType::BetaSheet
        } else {
            SecondaryStructureType::Coil
        };

        if new_type != current_type && i > 0 {
            regions.push(SecondaryStructureRegion {
                structure_type: current_type,
                start: region_start,
                end: i - 1,
            });
            region_start = i;
            current_type = new_type;
        }
    }

    // Add final region
    regions.push(SecondaryStructureRegion {
        structure_type: current_type,
        start: region_start,
        end: sequence.len() - 1,
    });

    regions
}

// ============================================================================
// PROTEIN SHAPE FROM BLUEPRINT
// ============================================================================

/// Target protein shape derived from holographic blueprint
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The holographic blueprint encodes the native conformation"
/// "Native state = minimum interference configuration"
#[derive(Debug, Clone)]
pub struct BlueprintProteinShape {
    /// Target 3D coordinates
    pub coordinates: Vec<Coordinate3D>,
    /// Secondary structure assignments
    pub secondary_structure: SecondaryStructure,
    /// Predicted interference energy at native state
    pub native_interference_energy: Float,
    /// Folding pathway (sequence of intermediate states)
    pub folding_pathway: Vec<BlueprintFoldingStep>,
}

/// A step in the blueprint-guided folding pathway
#[derive(Debug, Clone)]
pub struct BlueprintFoldingStep {
    /// Step number
    pub step: usize,
    /// Target coordinates at this step
    pub coordinates: Vec<Coordinate3D>,
    /// Interference energy target
    pub interference_target: Float,
}

impl BlueprintProteinShape {
    /// Generate target shape from amino acid sequence
    ///
    /// From ROADMAP: "Blueprint encodes target shape, not hardcoded values"
    pub fn from_sequence(sequence: &[AminoAcid]) -> Self {
        if sequence.is_empty() {
            return Self {
                coordinates: Vec::new(),
                secondary_structure: SecondaryStructure::RandomCoil,
                native_interference_energy: 0.0,
                folding_pathway: Vec::new(),
            };
        }

        // Predict secondary structure from archetype signatures
        let secondary_structure = Self::predict_structure_type(sequence);

        // Generate 3D coordinates based on secondary structure
        let coordinates = Self::generate_native_coordinates(sequence, &secondary_structure);

        // Calculate native interference energy
        let native_interference_energy = Self::calculate_native_interference(sequence);

        // Generate folding pathway
        let folding_pathway = Self::generate_folding_pathway(sequence, &coordinates);

        Self {
            coordinates,
            secondary_structure,
            native_interference_energy,
            folding_pathway,
        }
    }

    /// Predict overall structure type
    fn predict_structure_type(sequence: &[AminoAcid]) -> SecondaryStructure {
        let helix_count = sequence.iter().filter(|aa| aa.is_hydrophobic()).count();
        let polar_count = sequence.iter().filter(|aa| aa.is_polar()).count();

        let helix_fraction = helix_count as Float / sequence.len() as Float;
        let polar_fraction = polar_count as Float / sequence.len() as Float;

        if helix_fraction > 0.6 {
            SecondaryStructure::AlphaHelix
        } else if polar_fraction > 0.5 {
            SecondaryStructure::BetaSheet
        } else if helix_fraction > 0.3 && polar_fraction > 0.3 {
            SecondaryStructure::Mixed(identify_regions(sequence, helix_fraction, polar_fraction))
        } else {
            SecondaryStructure::RandomCoil
        }
    }

    /// Generate native 3D coordinates
    fn generate_native_coordinates(
        sequence: &[AminoAcid],
        secondary: &SecondaryStructure,
    ) -> Vec<Coordinate3D> {
        let mut coordinates = Vec::with_capacity(sequence.len());

        match secondary {
            SecondaryStructure::AlphaHelix => {
                // Alpha helix: 1.5 Å rise per residue, 100° rotation
                for (i, _aa) in sequence.iter().enumerate() {
                    let angle = (i as Float) * 100.0_f64.to_radians();
                    let radius = 2.3; // Å (helix radius)
                    coordinates.push(Coordinate3D::new(
                        radius * angle.cos(),
                        radius * angle.sin(),
                        i as Float * 1.5,
                    ));
                }
            }
            SecondaryStructure::BetaSheet => {
                // Beta sheet: extended conformation with alternating pattern
                for (i, _aa) in sequence.iter().enumerate() {
                    let i_f = i as Float;
                    coordinates.push(Coordinate3D::new(
                        i_f * 3.5,
                        (i % 2) as Float * 3.0 - 1.5,
                        0.0,
                    ));
                }
            }
            SecondaryStructure::Mixed(regions) => {
                // Mixed: generate coordinates for each region
                let mut z_offset = 0.0;
                for region in regions {
                    let region_coords = Self::generate_region_coordinates(
                        &sequence[region.start..=region.end],
                        region.structure_type,
                        z_offset,
                    );
                    coordinates.extend(region_coords);

                    // Update z_offset for next region
                    z_offset += (region.end - region.start + 1) as Float * 2.0;
                }
            }
            _ => {
                // Random coil: semi-extended with some randomness
                for (i, _aa) in sequence.iter().enumerate() {
                    coordinates.push(Coordinate3D::new(i as Float * 3.8, 0.0, 0.0));
                }
            }
        }

        coordinates
    }

    /// Generate coordinates for a specific region
    fn generate_region_coordinates(
        sequence: &[AminoAcid],
        structure_type: SecondaryStructureType,
        z_offset: Float,
    ) -> Vec<Coordinate3D> {
        let mut coordinates = Vec::with_capacity(sequence.len());

        match structure_type {
            SecondaryStructureType::AlphaHelix => {
                for (i, _aa) in sequence.iter().enumerate() {
                    let angle = (i as Float) * 100.0_f64.to_radians();
                    let radius = 2.3;
                    coordinates.push(Coordinate3D::new(
                        radius * angle.cos(),
                        radius * angle.sin(),
                        z_offset + i as Float * 1.5,
                    ));
                }
            }
            SecondaryStructureType::BetaSheet => {
                for (i, _aa) in sequence.iter().enumerate() {
                    coordinates.push(Coordinate3D::new(
                        i as Float * 3.5,
                        (i % 2) as Float * 3.0 - 1.5,
                        z_offset,
                    ));
                }
            }
            SecondaryStructureType::Turn => {
                // 180° turn over 4 residues
                for (i, _aa) in sequence.iter().enumerate() {
                    let angle = (i as Float) * std::f64::consts::PI / 4.0;
                    coordinates.push(Coordinate3D::new(
                        angle.cos() * 3.0,
                        angle.sin() * 3.0,
                        z_offset + i as Float * 1.0,
                    ));
                }
            }
            SecondaryStructureType::Coil => {
                for (i, _aa) in sequence.iter().enumerate() {
                    coordinates.push(Coordinate3D::new(i as Float * 3.8, 0.0, z_offset));
                }
            }
        }

        coordinates
    }

    /// Calculate native interference energy
    fn calculate_native_interference(sequence: &[AminoAcid]) -> Float {
        // Lower energy = more stable = minimum interference
        // Based on archetype signature coherence
        let mut total_coherence = 0.0;

        for window in sequence.windows(2) {
            let sig1 = window[0].archetype_signature();
            let sig2 = window[1].archetype_signature();

            // Calculate correlation between adjacent residues
            let correlation: Float = sig1
                .iter()
                .zip(sig2.iter())
                .map(|(a, b)| a * b)
                .sum::<Float>()
                / 22.0;

            total_coherence += correlation;
        }

        // Native interference energy is lower for higher coherence
        // This represents "minimum interference configuration"
        let native_energy = if sequence.len() > 1 {
            1.0 - total_coherence / (sequence.len() - 1) as Float
        } else {
            0.5
        };

        native_energy.clamp(0.1, 1.0)
    }

    /// Generate folding pathway with intermediate states
    fn generate_folding_pathway(
        sequence: &[AminoAcid],
        target_coords: &[Coordinate3D],
    ) -> Vec<BlueprintFoldingStep> {
        let num_steps = 10;
        let mut pathway = Vec::with_capacity(num_steps);

        // Start from extended chain
        let extended: Vec<Coordinate3D> = sequence
            .iter()
            .enumerate()
            .map(|(i, _)| Coordinate3D::new(i as Float * 3.8, 0.0, 0.0))
            .collect();

        for step in 1..=num_steps {
            let progress = step as Float / num_steps as Float;

            // Interpolate coordinates
            let coords: Vec<Coordinate3D> = extended
                .iter()
                .zip(target_coords.iter())
                .map(|(ext, target)| {
                    Coordinate3D::new(
                        ext.x + (target.x - ext.x) * progress,
                        ext.y + (target.y - ext.y) * progress,
                        ext.z + (target.z - ext.z) * progress,
                    )
                })
                .collect();

            // Interference energy decreases as we approach native state
            let interference_target = 1.0 - progress * 0.9;

            pathway.push(BlueprintFoldingStep {
                step,
                coordinates: coords,
                interference_target,
            });
        }

        pathway
    }
}

// ============================================================================
// PROTEIN FOLDING ENGINE
// ============================================================================

/// Protein folding engine using holographic field guidance
///
/// From ROADMAP Phase 3.4:
/// "Protein folding guided by holographic blueprint"
/// "Folding pathway from primary to quaternary structure"
pub struct ProteinFoldingEngine {
    /// Maximum folding iterations
    max_iterations: usize,
    /// Convergence threshold
    convergence_threshold: Float,
    /// Temperature factor for folding kinetics
    #[allow(dead_code)]
    temperature_factor: Float,
}

impl Default for ProteinFoldingEngine {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            convergence_threshold: 0.001,
            temperature_factor: 1.0,
        }
    }
}

impl ProteinFoldingEngine {
    /// Create new folding engine
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom parameters
    pub fn with_parameters(max_iterations: usize, convergence_threshold: Float) -> Self {
        Self {
            max_iterations,
            convergence_threshold,
            temperature_factor: 1.0,
        }
    }

    /// Fold protein using holographic field guidance
    ///
    /// From ROADMAP:
    /// "Fold protein according to blueprint"
    /// "Native state = minimum interference configuration"
    pub fn fold(
        &self,
        protein: &mut Protein,
        field: &HolographicField,
    ) -> Result<FoldingResult, FoldingError> {
        if protein.sequence.is_empty() {
            return Err(FoldingError::EmptySequence);
        }

        // Get target shape from blueprint
        let blueprint_shape = BlueprintProteinShape::from_sequence(&protein.sequence);

        // Initialize folding pathway
        protein.folding_pathway.clear();

        // Predict secondary structure
        protein.secondary = protein.predict_secondary_structure();

        // Get target coordinates
        let target_coords = &blueprint_shape.coordinates;

        // Iterative folding to minimize interference
        let mut current_coords = protein.tertiary.clone();
        let mut prev_energy = Float::MAX;

        for iteration in 0..self.max_iterations {
            // Calculate current interference energy
            let current_energy = self.calculate_interference_energy(&current_coords, field);

            // Record intermediate state
            protein.folding_pathway.push(FoldingIntermediate {
                progress: iteration as Float / self.max_iterations as Float,
                interference_energy: current_energy,
                secondary_content: self.calculate_secondary_content(&current_coords),
                coordinates: current_coords.clone(),
            });

            // Check for convergence
            if (prev_energy - current_energy).abs() < self.convergence_threshold {
                break;
            }
            prev_energy = current_energy;

            // Move toward target coordinates
            let factor = self.calculate_step_factor(iteration);
            for (current, target) in current_coords.iter_mut().zip(target_coords.iter()) {
                current.x += (target.x - current.x) * factor;
                current.y += (target.y - current.y) * factor;
                current.z += (target.z - current.z) * factor;
            }
        }

        // Update protein state
        protein.tertiary = current_coords;
        protein.interference_energy = prev_energy;

        // Determine final folding state
        if prev_energy < blueprint_shape.native_interference_energy * 1.1 {
            protein.folding_state = FoldingState::Native;
        } else if prev_energy < 0.8 {
            protein.folding_state = FoldingState::PartialFolded {
                progress: 1.0 - prev_energy,
            };
        } else {
            protein.folding_state = FoldingState::Misfolded {
                interference_energy: prev_energy,
            };
        }

        Ok(FoldingResult {
            final_state: protein.folding_state.clone(),
            final_interference_energy: prev_energy,
            iterations: protein.folding_pathway.len(),
            native_energy: blueprint_shape.native_interference_energy,
        })
    }

    /// Calculate interference energy of conformation
    fn calculate_interference_energy(
        &self,
        coordinates: &[Coordinate3D],
        field: &HolographicField,
    ) -> Float {
        let mut total_energy = 0.0;

        for coord in coordinates {
            let position = Position::new(coord.x, coord.y, coord.z);
            let intensity = field.calculate_local_intensity(&position);

            // Lower intensity = lower interference = more stable
            total_energy += 1.0 - intensity;
        }

        total_energy / coordinates.len().max(1) as Float
    }

    /// Calculate step factor for folding iteration
    fn calculate_step_factor(&self, iteration: usize) -> Float {
        // Decreasing step factor for convergence
        let base_factor = 0.5;
        let decay = (iteration as Float + 1.0).ln() / 5.0;
        (base_factor / (1.0 + decay)).min(1.0)
    }

    /// Calculate secondary structure content from coordinates
    fn calculate_secondary_content(
        &self,
        coordinates: &[Coordinate3D],
    ) -> SecondaryStructureContent {
        if coordinates.len() < 4 {
            return SecondaryStructureContent::default();
        }

        let mut helix_count = 0;
        let mut sheet_count = 0;

        // Analyze phi/psi angles (simplified)
        for i in 1..coordinates.len() - 2 {
            let prev = &coordinates[i - 1];
            let curr = &coordinates[i];
            let next = &coordinates[i + 1];
            let next2 = &coordinates[i + 2];

            // Simplified: check for helix-like or sheet-like geometry
            let d1 = prev.distance_to(curr);
            let d2 = curr.distance_to(next);
            let d3 = next.distance_to(next2);

            // Helix: regular spacing
            if (d1 - d2).abs() < 0.5 && (d2 - d3).abs() < 0.5 && d1 < 2.0 {
                helix_count += 1;
            }
            // Sheet: extended
            else if d1 > 3.0 && d2 > 3.0 {
                sheet_count += 1;
            }
        }

        let total = (coordinates.len() - 3) as Float;
        SecondaryStructureContent {
            helix_fraction: helix_count as Float / total,
            sheet_fraction: sheet_count as Float / total,
            coil_fraction: 1.0 - (helix_count + sheet_count) as Float / total,
        }
    }
}

// ============================================================================
// FOLDING RESULT AND ERROR
// ============================================================================

/// Result of protein folding
#[derive(Debug, Clone)]
pub struct FoldingResult {
    /// Final folding state
    pub final_state: FoldingState,
    /// Final interference energy
    pub final_interference_energy: Float,
    /// Number of folding iterations
    pub iterations: usize,
    /// Expected native state energy
    pub native_energy: Float,
}

/// Folding error types
#[derive(Debug, Clone, PartialEq)]
pub enum FoldingError {
    /// Empty protein sequence
    EmptySequence,
    /// Folding failed to converge
    ConvergenceFailed,
    /// Invalid conformation
    InvalidConformation,
    /// Field error
    FieldError(String),
}

impl std::fmt::Display for FoldingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FoldingError::EmptySequence => write!(f, "Cannot fold empty protein sequence"),
            FoldingError::ConvergenceFailed => write!(f, "Folding failed to converge"),
            FoldingError::InvalidConformation => write!(f, "Invalid protein conformation"),
            FoldingError::FieldError(msg) => write!(f, "Field error: {}", msg),
        }
    }
}

impl std::error::Error for FoldingError {}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amino_acid_codes() {
        assert_eq!(AminoAcid::Alanine.code(), 'A');
        assert_eq!(AminoAcid::Glycine.code(), 'G');
        assert_eq!(AminoAcid::Lysine.code(), 'K');
        assert_eq!(AminoAcid::Tryptophan.code(), 'W');
    }

    #[test]
    fn test_amino_acid_three_letter_codes() {
        assert_eq!(AminoAcid::Alanine.three_letter_code(), "Ala");
        assert_eq!(AminoAcid::Methionine.three_letter_code(), "Met");
        assert_eq!(AminoAcid::Phenylalanine.three_letter_code(), "Phe");
    }

    #[test]
    fn test_amino_acid_classification() {
        // Hydrophobic
        assert!(AminoAcid::Alanine.is_hydrophobic());
        assert!(AminoAcid::Valine.is_hydrophobic());
        assert!(AminoAcid::Leucine.is_hydrophobic());

        // Polar
        assert!(AminoAcid::Serine.is_polar());
        assert!(AminoAcid::Threonine.is_polar());

        // Charged
        assert!(AminoAcid::Lysine.is_charged());
        assert!(AminoAcid::AsparticAcid.is_charged());
    }

    #[test]
    fn test_amino_acid_charge() {
        assert_eq!(AminoAcid::Lysine.charge(), 1.0);
        assert_eq!(AminoAcid::Arginine.charge(), 1.0);
        assert_eq!(AminoAcid::AsparticAcid.charge(), -1.0);
        assert_eq!(AminoAcid::GlutamicAcid.charge(), -1.0);
        assert_eq!(AminoAcid::Alanine.charge(), 0.0);
    }

    #[test]
    fn test_amino_acid_archetype_signature() {
        // Hydrophobic amino acids should have high Matrix
        let ala_sig = AminoAcid::Alanine.archetype_signature();
        assert!(ala_sig[0] > 0.7, "Alanine should have high Matrix");

        // Charged amino acids should have distinct Catalyst patterns
        let asp_sig = AminoAcid::AsparticAcid.archetype_signature();
        let lys_sig = AminoAcid::Lysine.archetype_signature();
        assert!(
            asp_sig[2] > lys_sig[2],
            "Aspartic acid should have higher Catalyst than Lysine"
        );

        // Glycine should have low Matrix (flexible)
        let gly_sig = AminoAcid::Glycine.archetype_signature();
        assert!(gly_sig[0] < 0.3, "Glycine should have low Matrix");

        // Proline should have very high Matrix (rigid)
        let pro_sig = AminoAcid::Proline.archetype_signature();
        assert!(pro_sig[0] > 0.9, "Proline should have very high Matrix");
    }

    #[test]
    fn test_protein_creation() {
        let protein = Protein::new(vec![
            AminoAcid::Alanine,
            AminoAcid::Glycine,
            AminoAcid::Lysine,
        ]);

        assert_eq!(protein.sequence.len(), 3);
        assert_eq!(protein.folding_state, FoldingState::Unfolded);
        assert_eq!(protein.tertiary.len(), 3);
        assert!(protein.interference_energy > 1e10); // Initially MAX
    }

    #[test]
    fn test_protein_sequence_string() {
        let protein = Protein::new(vec![
            AminoAcid::Alanine,
            AminoAcid::Glycine,
            AminoAcid::Lysine,
        ]);

        assert_eq!(protein.sequence_string(), "AGK");
    }

    #[test]
    fn test_secondary_structure_prediction() {
        // All hydrophobic should predict helix
        let helix_protein = Protein::new(vec![
            AminoAcid::Alanine,
            AminoAcid::Valine,
            AminoAcid::Leucine,
            AminoAcid::Isoleucine,
            AminoAcid::Methionine,
        ]);
        let secondary = helix_protein.predict_secondary_structure();
        assert!(
            matches!(secondary, SecondaryStructure::AlphaHelix),
            "Hydrophobic sequence should predict helix"
        );

        // Mixed should predict mixed or coil
        let mixed_protein = Protein::new(vec![
            AminoAcid::Serine,
            AminoAcid::Threonine,
            AminoAcid::Alanine,
            AminoAcid::Glycine,
        ]);
        let _secondary = mixed_protein.predict_secondary_structure();
        // Just check it doesn't crash
    }

    #[test]
    fn test_coordinate_distance() {
        let a = Coordinate3D::new(0.0, 0.0, 0.0);
        let b = Coordinate3D::new(3.0, 4.0, 0.0);

        assert!((a.distance_to(&b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_blueprint_protein_shape() {
        let sequence = vec![
            AminoAcid::Alanine,
            AminoAcid::Valine,
            AminoAcid::Leucine,
            AminoAcid::Isoleucine,
        ];

        let shape = BlueprintProteinShape::from_sequence(&sequence);

        assert_eq!(shape.coordinates.len(), 4);
        assert!(shape.native_interference_energy >= 0.0);
        assert!(shape.native_interference_energy <= 1.0);
        assert!(!shape.folding_pathway.is_empty());
    }

    #[test]
    fn test_folding_engine_creation() {
        let engine = ProteinFoldingEngine::new();
        assert_eq!(engine.max_iterations, 100);
        assert!(!engine.convergence_threshold.is_nan());
    }

    #[test]
    fn test_folding_engine_custom_parameters() {
        let engine = ProteinFoldingEngine::with_parameters(50, 0.0001);
        assert_eq!(engine.max_iterations, 50);
        assert!((engine.convergence_threshold - 0.0001).abs() < 1e-10);
    }

    #[test]
    fn test_folding_empty_sequence_error() {
        let mut protein = Protein::new(vec![]);

        // Create a minimal holographic field for testing
        use crate::holographic::complex_vectors::ComplexArchetype;
        use crate::holographic::holographic_field::InvolutionLayer;

        let archetypes = [ComplexArchetype::default(); 22];
        let field = HolographicField::new(InvolutionLayer::Green, archetypes);

        let engine = ProteinFoldingEngine::new();
        let result = engine.fold(&mut protein, &field);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FoldingError::EmptySequence);
    }

    #[test]
    fn test_folding_state_progression() {
        let mut protein = Protein::new(vec![
            AminoAcid::Alanine,
            AminoAcid::Glycine,
            AminoAcid::Lysine,
            AminoAcid::GlutamicAcid,
        ]);

        // Initially unfolded
        assert_eq!(protein.folding_state, FoldingState::Unfolded);

        // After folding (simplified test)
        use crate::holographic::complex_vectors::ComplexArchetype;
        use crate::holographic::holographic_field::InvolutionLayer;

        let archetypes = [ComplexArchetype::default(); 22];
        let field = HolographicField::new(InvolutionLayer::Green, archetypes);

        let engine = ProteinFoldingEngine::new();
        let result = engine.fold(&mut protein, &field);

        // Should complete folding (may or may not reach native state)
        assert!(result.is_ok());
        assert!(!protein.folding_pathway.is_empty());
    }

    #[test]
    fn test_secondary_structure_content() {
        let coordinates = vec![
            Coordinate3D::new(0.0, 0.0, 0.0),
            Coordinate3D::new(1.5, 0.0, 1.5),
            Coordinate3D::new(0.0, 0.0, 3.0),
            Coordinate3D::new(1.5, 0.0, 4.5),
            Coordinate3D::new(0.0, 0.0, 6.0),
        ];

        let engine = ProteinFoldingEngine::new();
        let content = engine.calculate_secondary_content(&coordinates);

        // Check that fractions are valid
        assert!(content.helix_fraction >= 0.0 && content.helix_fraction <= 1.0);
        assert!(content.sheet_fraction >= 0.0 && content.sheet_fraction <= 1.0);
        assert!(content.coil_fraction >= 0.0 && content.coil_fraction <= 1.0);
    }
}
