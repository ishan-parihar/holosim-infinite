//! Cellular Level Visualization
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.4:
//! "Cellular Level Visualization: Blueprint, gene expression, protein folding, cell manifestation, Gaia resonance"
//!
//! This module provides visualization for:
//! - Holographic blueprint for morphogenesis
//! - Archetype-to-gene encoding (A1-A7=Mind→regulatory, A8-A14=Body→structural, A15-A21=Spirit→epigenetic, A22=Choice→mutation)
//! - 3D protein field configurations
//! - Cell membranes as field boundaries
//! - Cellular/Gaia consciousness co-emergence
//!
//! # Design Principles
//!
//! - DNA/RNA unfold from pre-existing holographic blueprint
//! - A1-A7 (Mind): Regulatory genes - control when/where genes express
//! - A8-A14 (Body): Structural genes - encode proteins for physical structure
//! - A15-A21 (Spirit): Epigenetic control - modulate expression without changing sequence
//! - A22 (Choice): Mutation/recombination operators - drive evolutionary change
//! - Cells as field boundaries with membranes
//! - Simultaneous emergence: cells AND Gaia consciousness from the same field

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::cellular_emergence::archetype_genes::{
    ArchetypeGene, GeneCategory, GeneExpressionProfile, GeneId, GeneRegulatoryNetwork,
};
use crate::holographic_foundation::cellular_emergence::cell_manifestation::{
    CellBoundary, CellId, CellManifestation, CellMembrane, CellOrganelle, CellState,
    CellularFieldConfiguration,
};
use crate::holographic_foundation::cellular_emergence::gene_expression::{
    ExpressionCondition, ExpressionResult, GeneExpressionEngine,
};
use crate::holographic_foundation::cellular_emergence::holographic_blueprint::{
    BlueprintId, DevelopmentalStage, HolographicBlueprint, OrganismManifestation,
};
use crate::holographic_foundation::cellular_emergence::protein_field::{
    AminoAcid, ProteinFoldingField, ProteinManifestation, ProteinStructure, SecondaryStructure,
};
use crate::holographic_foundation::cellular_emergence::simultaneous_emergence::{
    CellularGaiaPair, CellularPlanetaryResonance, GaiaConsciousness, PlanetaryCellField,
    PlanetaryCellType,
};
use crate::holographic_foundation::field_state::Position3D;
use egui::Color32;
use egui::Stroke;
use std::collections::HashMap;

// ============================================================================
// Blueprint Renderer - Visualize Holographic Blueprint for Morphogenesis
// ============================================================================

/// Color scheme for blueprint visualization
#[derive(Debug, Clone)]
pub struct BlueprintColors;

impl BlueprintColors {
    pub fn mind_archetype() -> Color32 {
        Color32::from_rgb(100, 149, 237) // Cornflower blue
    }

    pub fn body_archetype() -> Color32 {
        Color32::from_rgb(34, 139, 34) // Forest green
    }

    pub fn spirit_archetype() -> Color32 {
        Color32::from_rgb(218, 165, 32) // Goldenrod
    }

    pub fn choice_archetype() -> Color32 {
        Color32::from_rgb(255, 99, 71) // Tomato
    }

    pub fn blueprint_grid() -> Color32 {
        Color32::from_rgb(70, 130, 180) // Steel blue
    }

    pub fn active_gene() -> Color32 {
        Color32::from_rgb(50, 205, 50) // Lime green
    }

    pub fn inactive_gene() -> Color32 {
        Color32::from_rgb(105, 105, 105) // Dim gray
    }
}

/// Data for rendering a developmental stage
#[derive(Debug, Clone)]
pub struct DevelopmentalStageData {
    pub stage: DevelopmentalStage,
    pub complexity_factor: f32,
    pub activation_threshold: f32,
    pub cell_count: usize,
    pub active_gene_count: usize,
}

impl From<&DevelopmentalStage> for DevelopmentalStageData {
    fn from(stage: &DevelopmentalStage) -> Self {
        Self {
            stage: stage.clone(),
            complexity_factor: stage.complexity_factor() as f32,
            activation_threshold: stage.gene_activation_threshold() as f32,
            cell_count: match stage {
                DevelopmentalStage::Zygote => 1,
                DevelopmentalStage::Blastula => 8,
                DevelopmentalStage::Gastrula => 16,
                DevelopmentalStage::Neurula => 64,
                DevelopmentalStage::Organogenesis => 256,
                DevelopmentalStage::Fetal => 1024,
                DevelopmentalStage::Mature => 10000,
            },
            active_gene_count: 0,
        }
    }
}

/// Blueprint rendering data
#[derive(Debug, Clone)]
pub struct BlueprintRenderData {
    pub blueprint_id: u64,
    pub archetype_pattern: Vec<f32>,
    pub mind_dominance: f32,
    pub body_dominance: f32,
    pub spirit_dominance: f32,
    pub choice_capacity: f32,
    pub complexity_index: f32,
    pub developmental_stages: Vec<DevelopmentalStageData>,
    pub species_signature: u64,
}

impl From<&HolographicBlueprint> for BlueprintRenderData {
    fn from(blueprint: &HolographicBlueprint) -> Self {
        Self {
            blueprint_id: blueprint.id.raw(),
            archetype_pattern: blueprint
                .archetype_pattern
                .iter()
                .map(|&x| x as f32)
                .collect(),
            mind_dominance: blueprint.mind_complex_dominance() as f32,
            body_dominance: blueprint.body_complex_dominance() as f32,
            spirit_dominance: blueprint.spirit_complex_dominance() as f32,
            choice_capacity: blueprint.choice_capacity() as f32,
            complexity_index: blueprint.complexity_index() as f32,
            developmental_stages: blueprint
                .developmental_sequence
                .iter()
                .map(|s| DevelopmentalStageData::from(s))
                .collect(),
            species_signature: blueprint.species_signature,
        }
    }
}

/// Blueprint renderer widget
pub struct BlueprintRenderer {
    pub selected_stage: Option<DevelopmentalStage>,
}

impl BlueprintRenderer {
    pub fn new() -> Self {
        Self {
            selected_stage: Some(DevelopmentalStage::Zygote),
        }
    }

    /// Render the blueprint visualization panel
    pub fn render(&mut self, ui: &mut egui::Ui, blueprint: &HolographicBlueprint) {
        let data = BlueprintRenderData::from(blueprint);

        egui::CollapsingHeader::new("Blueprint Overview")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Holographic Blueprint for Morphogenesis").strong());
                ui.separator();

                // Complexity metrics
                ui.columns(2, |cols| {
                    cols[0].label("Complexity Index:");
                    cols[1].label(format!("{:.3}", data.complexity_index));

                    cols[0].label("Species Signature:");
                    cols[1].label(format!("{:#018x}", data.species_signature));
                });

                ui.separator();

                // Archetype dominance visualization
                ui.label(egui::RichText::new("Archetype Complex Dominance").strong());
                self.render_dominance_bars(ui, &data);

                ui.separator();

                // Archetype encoding visualization
                ui.label(egui::RichText::new("Archetype Encoding (A1-A22)").strong());
                self.render_archetype_grid(ui, &data);
            });

        // Developmental stages
        egui::CollapsingHeader::new("Developmental Sequence")
            .default_open(true)
            .show(ui, |ui| {
                self.render_developmental_stages(ui, &data);
            });

        // DNA unfolding preview
        egui::CollapsingHeader::new("DNA Unfolding Preview")
            .default_open(false)
            .show(ui, |ui| {
                self.render_dna_preview(ui, blueprint);
            });
    }

    fn render_dominance_bars(&self, ui: &mut egui::Ui, data: &BlueprintRenderData) {
        let bar_width = 150.0;

        ui.horizontal(|ui| {
            ui.label("Mind (A1-A7):");
            ui.add(
                egui::ProgressBar::new(data.mind_dominance)
                    .desired_width(bar_width)
                    .fill(BlueprintColors::mind_archetype()),
            );
            ui.label(format!("{:.2}", data.mind_dominance));
        });

        ui.horizontal(|ui| {
            ui.label("Body (A8-A14):");
            ui.add(
                egui::ProgressBar::new(data.body_dominance)
                    .desired_width(bar_width)
                    .fill(BlueprintColors::body_archetype()),
            );
            ui.label(format!("{:.2}", data.body_dominance));
        });

        ui.horizontal(|ui| {
            ui.label("Spirit (A15-A21):");
            ui.add(
                egui::ProgressBar::new(data.spirit_dominance)
                    .desired_width(bar_width)
                    .fill(BlueprintColors::spirit_archetype()),
            );
            ui.label(format!("{:.2}", data.spirit_dominance));
        });

        ui.horizontal(|ui| {
            ui.label("Choice (A22):");
            ui.add(
                egui::ProgressBar::new(data.choice_capacity)
                    .desired_width(bar_width)
                    .fill(BlueprintColors::choice_archetype()),
            );
            ui.label(format!("{:.2}", data.choice_capacity));
        });
    }

    fn render_archetype_grid(&self, ui: &mut egui::Ui, data: &BlueprintRenderData) {
        let grid_size = 22;
        let cell_size = 20.0;

        egui::Grid::new("archetype_grid")
            .num_columns(11)
            .spacing([5.0, 5.0])
            .show(ui, |ui| {
                for i in 0..grid_size {
                    if i > 0 && i % 11 == 0 {
                        ui.end_row();
                    }

                    let activation = data.archetype_pattern.get(i).copied().unwrap_or(0.5);
                    let color = match i {
                        0..=6 => BlueprintColors::mind_archetype(),
                        7..=13 => BlueprintColors::body_archetype(),
                        14..=20 => BlueprintColors::spirit_archetype(),
                        21 => BlueprintColors::choice_archetype(),
                        _ => BlueprintColors::inactive_gene(),
                    };

                    let alpha = (activation * 255.0) as u8;
                    let cell_color =
                        Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

                    let rect = ui.available_rect_before_wrap();
                    let cell_rect =
                        egui::Rect::from_min_size(rect.min, egui::vec2(cell_size, cell_size));

                    ui.painter().rect_filled(cell_rect, 2.0, cell_color);
                    ui.label(format!("A{}", i + 1));
                }
            });
    }

    fn render_developmental_stages(&mut self, ui: &mut egui::Ui, data: &BlueprintRenderData) {
        for stage_data in &data.developmental_stages {
            let stage_name = match stage_data.stage {
                DevelopmentalStage::Zygote => "Zygote",
                DevelopmentalStage::Blastula => "Blastula",
                DevelopmentalStage::Gastrula => "Gastrula",
                DevelopmentalStage::Neurula => "Neurula",
                DevelopmentalStage::Organogenesis => "Organogenesis",
                DevelopmentalStage::Fetal => "Fetal",
                DevelopmentalStage::Mature => "Mature",
            };

            let is_selected = self
                .selected_stage
                .as_ref()
                .map(|s| *s == stage_data.stage)
                .unwrap_or(false);

            ui.horizontal(|ui| {
                let response =
                    ui.selectable_label(is_selected, egui::RichText::new(stage_name).strong());
                if response.clicked() {
                    self.selected_stage = Some(stage_data.stage);
                }

                ui.label(format!(
                    " | Cells: {} | Threshold: {:.2}",
                    stage_data.cell_count, stage_data.activation_threshold
                ));
            });
        }
    }

    fn render_dna_preview(&self, ui: &mut egui::Ui, blueprint: &HolographicBlueprint) {
        let dna = blueprint.unfold_dna(50);
        let nucleotides = dna.nucleotides();
        let sequence: String = nucleotides.iter().map(|n| n.symbol()).collect();

        ui.label("DNA Sequence (first 50 nucleotides):");
        ui.add(
            egui::TextEdit::multiline(&mut sequence.clone())
                .desired_width(400.0)
                .desired_rows(3)
                .interactive(false),
        );
    }
}

impl Default for BlueprintRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Gene Expression View - Show Archetype-to-Gene Encoding
// ============================================================================

/// Gene category colors
#[derive(Debug, Clone)]
pub struct GeneCategoryColors;

impl GeneCategoryColors {
    pub fn regulatory() -> Color32 {
        Color32::from_rgb(65, 105, 225) // Royal blue
    }

    pub fn structural() -> Color32 {
        Color32::from_rgb(34, 139, 34) // Forest green
    }

    pub fn epigenetic() -> Color32 {
        Color32::from_rgb(148, 0, 211) // Dark violet
    }

    pub fn mutation() -> Color32 {
        Color32::from_rgb(255, 69, 0) // Red-orange
    }

    pub fn for_category(category: &GeneCategory) -> Color32 {
        match category {
            GeneCategory::Regulatory => Self::regulatory(),
            GeneCategory::Structural => Self::structural(),
            GeneCategory::Epigenetic => Self::epigenetic(),
            GeneCategory::Mutation => Self::mutation(),
        }
    }
}

/// Gene expression view data
#[derive(Debug, Clone)]
pub struct GeneViewData {
    pub gene_id: u64,
    pub archetype_source: usize,
    pub category: GeneCategory,
    pub expression_threshold: f32,
    pub expression_strength: f32,
    pub regulatory_target_count: usize,
}

impl From<&ArchetypeGene> for GeneViewData {
    fn from(gene: &ArchetypeGene) -> Self {
        Self {
            gene_id: gene.id.0,
            archetype_source: gene.archetype_source,
            category: gene.category,
            expression_threshold: gene.expression_threshold as f32,
            expression_strength: gene.expression_strength as f32,
            regulatory_target_count: gene.regulatory_targets.len(),
        }
    }
}

/// Gene regulatory network visualization data
#[derive(Debug, Clone)]
pub struct NetworkViewData {
    pub genes: Vec<GeneViewData>,
    pub connections: Vec<(u64, u64, f32)>,
    pub coherence: f32,
    pub gene_count: usize,
    pub connection_count: usize,
}

impl From<&GeneRegulatoryNetwork> for NetworkViewData {
    fn from(network: &GeneRegulatoryNetwork) -> Self {
        let genes: Vec<GeneViewData> = network.genes.values().map(GeneViewData::from).collect();

        let mut connections = Vec::new();
        for (source_id, targets) in network.connections().iter() {
            for (target_id, strength) in targets {
                connections.push((source_id.0, target_id.0, *strength as f32));
            }
        }

        Self {
            genes,
            connections,
            coherence: network.coherence() as f32,
            gene_count: network.gene_count(),
            connection_count: network.connection_count(),
        }
    }
}

/// Gene expression view widget
pub struct GeneExpressionView {
    selected_category: Option<GeneCategory>,
    pub show_network: bool,
    pub show_expression_levels: bool,
}

impl GeneExpressionView {
    pub fn new() -> Self {
        Self {
            selected_category: None,
            show_network: true,
            show_expression_levels: true,
        }
    }

    /// Render the gene expression panel
    pub fn render(&mut self, ui: &mut egui::Ui, profile: &GeneExpressionProfile) {
        egui::CollapsingHeader::new("Gene Expression Profile")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Archetype-to-Gene Encoding").strong());
                ui.separator();

                // Category legend
                self.render_legend(ui);

                ui.separator();

                // Expression potential
                let potential = profile.total_expression_potential() as f32;
                ui.label(format!("Total Expression Potential: {:.3}", potential));

                ui.add(egui::ProgressBar::new(potential).desired_width(200.0));

                ui.separator();

                // Category breakdown
                ui.label(egui::RichText::new("Gene Categories").strong());
                self.render_category_counts(ui, profile);
            });

        // Gene list
        egui::CollapsingHeader::new("Gene List (A1-A22)")
            .default_open(true)
            .show(ui, |ui| {
                self.render_gene_list(ui, profile);
            });

        // Regulatory network visualization
        if self.show_network {
            egui::CollapsingHeader::new("Gene Regulatory Network")
                .default_open(false)
                .show(ui, |ui| {
                    self.render_network_visualization(ui, profile);
                });
        }
    }

    fn render_legend(&self, ui: &mut egui::Ui) {
        ui.label("Gene Category Legend:");

        ui.horizontal(|ui| {
            let rect_min = ui.cursor().min;
            ui.painter().rect_filled(
                egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                2.0,
                GeneCategoryColors::regulatory(),
            );
            ui.label("A1-A7: Regulatory (Mind) - Control when/where genes express");
        });

        ui.horizontal(|ui| {
            let rect_min = ui.cursor().min;
            ui.painter().rect_filled(
                egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                2.0,
                GeneCategoryColors::structural(),
            );
            ui.label("A8-A14: Structural (Body) - Encode proteins for physical structure");
        });

        ui.horizontal(|ui| {
            let rect_min = ui.cursor().min;
            ui.painter().rect_filled(
                egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                2.0,
                GeneCategoryColors::epigenetic(),
            );
            ui.label("A15-A21: Epigenetic (Spirit) - Modulate expression without sequence change");
        });

        ui.horizontal(|ui| {
            let rect_min = ui.cursor().min;
            ui.painter().rect_filled(
                egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                2.0,
                GeneCategoryColors::mutation(),
            );
            ui.label("A22: Mutation (Choice) - Evolutionary change operators");
        });
    }

    fn render_category_counts(&self, ui: &mut egui::Ui, profile: &GeneExpressionProfile) {
        let categories = [
            (
                GeneCategory::Regulatory,
                "Regulatory (Mind)",
                profile.regulatory_genes.len(),
            ),
            (
                GeneCategory::Structural,
                "Structural (Body)",
                profile.structural_genes.len(),
            ),
            (
                GeneCategory::Epigenetic,
                "Epigenetic (Spirit)",
                profile.epigenetic_genes.len(),
            ),
            (
                GeneCategory::Mutation,
                "Mutation (Choice)",
                if profile.mutation_gene.is_some() {
                    1
                } else {
                    0
                },
            ),
        ];

        for (category, name, count) in categories {
            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(12.0, 12.0)),
                    2.0,
                    GeneCategoryColors::for_category(&category),
                );
                ui.label(format!("{}: {} genes", name, count));
            });
        }
    }

    fn render_gene_list(&self, ui: &mut egui::Ui, profile: &GeneExpressionProfile) {
        let all_genes: Vec<(&ArchetypeGene, &str)> = profile
            .regulatory_genes
            .iter()
            .map(|g| (g as &ArchetypeGene, "Regulatory"))
            .chain(
                profile
                    .structural_genes
                    .iter()
                    .map(|g| (g as &ArchetypeGene, "Structural")),
            )
            .chain(
                profile
                    .epigenetic_genes
                    .iter()
                    .map(|g| (g as &ArchetypeGene, "Epigenetic")),
            )
            .chain(
                profile
                    .mutation_gene
                    .as_ref()
                    .map(|g| (g as &ArchetypeGene, "Mutation")),
            )
            .collect();

        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                egui::Grid::new("gene_grid")
                    .num_columns(5)
                    .spacing([10.0, 5.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Gene").strong());
                        ui.label(egui::RichText::new("Archetype").strong());
                        ui.label(egui::RichText::new("Category").strong());
                        ui.label(egui::RichText::new("Threshold").strong());
                        ui.label(egui::RichText::new("Strength").strong());
                        ui.end_row();

                        for (gene, _category_str) in &all_genes {
                            let data = GeneViewData::from(*gene);
                            let color = GeneCategoryColors::for_category(&data.category);

                            let rect_min = ui.cursor().min;
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(rect_min, egui::vec2(10.0, 10.0)),
                                1.0,
                                color,
                            );

                            ui.label(format!("G{}", data.archetype_source + 1));
                            ui.label(format!("A{}", data.archetype_source + 1));

                            let category_str = match data.category {
                                GeneCategory::Regulatory => "Reg",
                                GeneCategory::Structural => "Struct",
                                GeneCategory::Epigenetic => "Epi",
                                GeneCategory::Mutation => "Mut",
                            };
                            ui.label(category_str);

                            ui.label(format!("{:.2}", data.expression_threshold));
                            ui.label(format!("{:.2}", data.expression_strength));
                            ui.end_row();
                        }
                    });
            });
    }

    fn render_network_visualization(&self, ui: &mut egui::Ui, profile: &GeneExpressionProfile) {
        // Create a simple network diagram
        let network = GeneRegulatoryNetwork::from_profile(profile);
        let data = NetworkViewData::from(&network);

        ui.label(format!(
            "Network: {} genes, {} connections, coherence: {:.3}",
            data.gene_count, data.connection_count, data.coherence
        ));

        // Simple node visualization
        let available_width = ui.available_width();
        let node_spacing = available_width / 22.0;

        for (i, gene) in data.genes.iter().enumerate() {
            let x = i as f32 * node_spacing + node_spacing / 2.0;
            let y = ui.cursor().min.y + 30.0;

            let color = GeneCategoryColors::for_category(&gene.category);
            let radius = 8.0 + gene.expression_strength * 6.0;

            ui.painter().circle_filled(egui::pos2(x, y), radius, color);

            if i % 3 == 0 {
                ui.label(format!("A{}", i + 1));
            }
        }
    }
}

impl Default for GeneExpressionView {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Protein Folding View - Show 3D Protein Field Configurations
// ============================================================================

/// Protein structure visualization data
#[derive(Debug, Clone)]
pub struct ProteinViewData {
    pub name: String,
    pub amino_acid_count: usize,
    pub helix_content: f32,
    pub sheet_content: f32,
    pub folding_coherence: f32,
    pub field_energy: f32,
    pub sequence: String,
}

impl From<&ProteinManifestation> for ProteinViewData {
    fn from(protein: &ProteinManifestation) -> Self {
        let sequence: String = protein
            .amino_acid_sequence()
            .iter()
            .map(|aa| aa.one_letter_code())
            .collect();

        Self {
            name: format!("Protein_{}", protein.id.0),
            amino_acid_count: protein.len(),
            helix_content: protein.structure.helix_content() as f32,
            sheet_content: protein.structure.sheet_content() as f32,
            folding_coherence: protein.coherence() as f32,
            field_energy: protein.structure.field_energy() as f32,
            sequence,
        }
    }
}

/// Secondary structure colors
#[derive(Debug, Clone)]
pub struct SecondaryStructureColors;

impl SecondaryStructureColors {
    pub fn alpha_helix() -> Color32 {
        Color32::from_rgb(255, 100, 100) // Light red
    }

    pub fn beta_sheet() -> Color32 {
        Color32::from_rgb(100, 100, 255) // Light blue
    }

    pub fn beta_turn() -> Color32 {
        Color32::from_rgb(255, 255, 100) // Yellow
    }

    pub fn random_coil() -> Color32 {
        Color32::from_rgb(150, 150, 150) // Gray
    }

    pub fn for_structure(ss: &SecondaryStructure) -> Color32 {
        match ss {
            SecondaryStructure::AlphaHelix => Self::alpha_helix(),
            SecondaryStructure::BetaSheet => Self::beta_sheet(),
            SecondaryStructure::BetaTurn => Self::beta_turn(),
            SecondaryStructure::RandomCoil => Self::random_coil(),
            SecondaryStructure::Unknown => Color32::BLACK,
        }
    }
}

/// Protein folding view widget
pub struct ProteinFoldingView {
    selected_protein_index: usize,
    pub show_sequence: bool,
    pub show_structure_legend: bool,
}

impl ProteinFoldingView {
    pub fn new() -> Self {
        Self {
            selected_protein_index: 0,
            show_sequence: true,
            show_structure_legend: true,
        }
    }

    /// Render the protein folding panel
    pub fn render(&mut self, ui: &mut egui::Ui, proteins: &[ProteinManifestation]) {
        if proteins.is_empty() {
            ui.label("No proteins to display");
            return;
        }

        egui::CollapsingHeader::new("Protein Folding Field")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("3D Protein Field Configurations").strong());
                ui.separator();

                // Protein list
                ui.label("Select Protein:");
                egui::ScrollArea::horizontal()
                    .max_height(40.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for (i, protein) in proteins.iter().enumerate() {
                                let label = format!("P{}", i + 1);
                                let is_selected = i == self.selected_protein_index;
                                if ui.selectable_label(is_selected, label).clicked() {
                                    self.selected_protein_index = i;
                                }
                            }
                        });
                    });

                ui.separator();

                // Selected protein details
                if let Some(protein) = proteins.get(self.selected_protein_index) {
                    self.render_protein_details(ui, protein);
                }
            });

        // Structure legend
        if self.show_structure_legend {
            egui::CollapsingHeader::new("Structure Legend")
                .default_open(false)
                .show(ui, |ui| {
                    self.render_structure_legend(ui);
                });
        }

        // Field coherence
        egui::CollapsingHeader::new("Folding Field Summary")
            .default_open(false)
            .show(ui, |ui| {
                self.render_field_summary(ui, proteins);
            });
    }

    fn render_protein_details(&self, ui: &mut egui::Ui, protein: &ProteinManifestation) {
        let data = ProteinViewData::from(protein);

        ui.columns(2, |cols| {
            cols[0].label("Amino Acids:");
            cols[1].label(format!("{}", data.amino_acid_count));

            cols[0].label("Folding Coherence:");
            cols[1].add(
                egui::ProgressBar::new(data.folding_coherence)
                    .desired_width(150.0)
                    .fill(BlueprintColors::active_gene()),
            );

            cols[0].label("Field Energy:");
            cols[1].label(format!("{:.2}", data.field_energy));
        });

        ui.separator();

        // Secondary structure content
        ui.label("Secondary Structure Content:");

        ui.horizontal(|ui| {
            ui.label("α-Helix:");
            ui.add(
                egui::ProgressBar::new(data.helix_content)
                    .desired_width(120.0)
                    .fill(SecondaryStructureColors::alpha_helix()),
            );
            ui.label(format!("{:.1}%", data.helix_content * 100.0));
        });

        ui.horizontal(|ui| {
            ui.label("β-Sheet:");
            ui.add(
                egui::ProgressBar::new(data.sheet_content)
                    .desired_width(120.0)
                    .fill(SecondaryStructureColors::beta_sheet()),
            );
            ui.label(format!("{:.1}%", data.sheet_content * 100.0));
        });

        // Sequence display
        if self.show_sequence && !data.sequence.is_empty() {
            ui.separator();
            ui.label("Primary Sequence:");

            let display_seq = if data.sequence.len() > 60 {
                format!("{}...", &data.sequence[..60])
            } else {
                data.sequence.clone()
            };

            ui.add(
                egui::TextEdit::multiline(&mut display_seq.clone())
                    .desired_width(400.0)
                    .desired_rows(2)
                    .interactive(false),
            );
        }
    }

    fn render_structure_legend(&self, ui: &mut egui::Ui) {
        ui.label("Secondary Structure Types:");

        let structures = [
            (
                SecondaryStructure::AlphaHelix,
                "α-Helix",
                "Field minimum spiral",
            ),
            (
                SecondaryStructure::BetaSheet,
                "β-Sheet",
                "Extended strand network",
            ),
            (SecondaryStructure::BetaTurn, "β-Turn", "Direction change"),
            (
                SecondaryStructure::RandomCoil,
                "Random Coil",
                "Unstructured region",
            ),
        ];

        for (ss, name, desc) in structures {
            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    SecondaryStructureColors::for_structure(&ss),
                );
                ui.label(egui::RichText::new(name).strong());
                ui.label(desc);
            });
        }
    }

    fn render_field_summary(&self, ui: &mut egui::Ui, proteins: &[ProteinManifestation]) {
        let total_aa: usize = proteins.iter().map(|p| p.len()).sum();
        let avg_coherence: f32 = if proteins.is_empty() {
            0.0
        } else {
            proteins.iter().map(|p| p.coherence()).sum::<f64>() as f32 / proteins.len() as f32
        };

        ui.columns(2, |cols| {
            cols[0].label("Total Proteins:");
            cols[1].label(format!("{}", proteins.len()));

            cols[0].label("Total Amino Acids:");
            cols[1].label(format!("{}", total_aa));

            cols[0].label("Average Coherence:");
            cols[1].add(
                egui::ProgressBar::new(avg_coherence)
                    .desired_width(150.0)
                    .fill(BlueprintColors::active_gene()),
            );
        });
    }
}

impl Default for ProteinFoldingView {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Cell Manifestation View - Cells as Field Boundaries
// ============================================================================

/// Cell state colors
#[derive(Debug, Clone)]
pub struct CellStateColors;

impl CellStateColors {
    pub fn quiescent() -> Color32 {
        Color32::from_rgb(128, 128, 128) // Gray
    }

    pub fn growing() -> Color32 {
        Color32::from_rgb(50, 205, 50) // Lime green
    }

    pub fn dividing() -> Color32 {
        Color32::from_rgb(255, 165, 0) // Orange
    }

    pub fn differentiating() -> Color32 {
        Color32::from_rgb(138, 43, 226) // Blue violet
    }

    pub fn apoptotic() -> Color32 {
        Color32::from_rgb(255, 0, 0) // Red
    }

    pub fn senescent() -> Color32 {
        Color32::from_rgb(139, 69, 19) // Brown
    }

    pub fn for_state(state: &CellState) -> Color32 {
        match state {
            CellState::Quiescent => Self::quiescent(),
            CellState::Growing => Self::growing(),
            CellState::Dividing => Self::dividing(),
            CellState::Differentiating => Self::differentiating(),
            CellState::Apoptotic => Self::apoptotic(),
            CellState::Senescent => Self::senescent(),
        }
    }
}

/// Organelle colors
#[derive(Debug, Clone)]
pub struct OrganelleColors;

impl OrganelleColors {
    pub fn for_organelle(organelle: &CellOrganelle) -> Color32 {
        match organelle {
            CellOrganelle::Nucleus => Color32::from_rgb(139, 0, 0), // Dark red
            CellOrganelle::Mitochondria => Color32::from_rgb(255, 140, 0), // Dark orange
            CellOrganelle::EndoplasmicReticulum => Color32::from_rgb(0, 100, 0), // Dark green
            CellOrganelle::GolgiApparatus => Color32::from_rgb(255, 215, 0), // Gold
            CellOrganelle::Lysosome => Color32::from_rgb(128, 0, 128), // Purple
            CellOrganelle::Ribosome => Color32::from_rgb(0, 0, 139), // Dark blue
            CellOrganelle::Cytoskeleton => Color32::from_rgb(70, 130, 180), // Steel blue
            CellOrganelle::Centrosome => Color32::from_rgb(255, 20, 147), // Deep pink
            CellOrganelle::Vacuole => Color32::from_rgb(0, 255, 255), // Cyan
            CellOrganelle::Chloroplast => Color32::from_rgb(0, 128, 0), // Green
        }
    }
}

/// Cell visualization data
#[derive(Debug, Clone)]
pub struct CellViewData {
    pub cell_id: u64,
    pub position: (f32, f32, f32),
    pub radius: f32,
    pub state: CellState,
    pub coherence: f32,
    pub generation: u32,
    pub has_dna: bool,
    pub protein_count: usize,
}

impl From<&CellManifestation> for CellViewData {
    fn from(cell: &CellManifestation) -> Self {
        let pos = cell.position();
        Self {
            cell_id: cell.id.0,
            position: (pos.x as f32, pos.y as f32, pos.z as f32),
            radius: cell.radius() as f32,
            state: cell.state(),
            coherence: cell.coherence() as f32,
            generation: cell.generation(),
            has_dna: cell.has_dna(),
            protein_count: cell.protein_count(),
        }
    }
}

/// Cell manifestation view widget
pub struct CellManifestationView {
    selected_cell_id: Option<u64>,
    pub show_membrane: bool,
    pub show_organelles: bool,
    pub show_state_legend: bool,
}

impl CellManifestationView {
    pub fn new() -> Self {
        Self {
            selected_cell_id: None,
            show_membrane: true,
            show_organelles: true,
            show_state_legend: true,
        }
    }

    /// Render the cell manifestation panel
    pub fn render(&mut self, ui: &mut egui::Ui, cells: &[CellManifestation]) {
        if cells.is_empty() {
            ui.label("No cells to display");
            return;
        }

        egui::CollapsingHeader::new("Cell Manifestation")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Cells as Field Boundaries").strong());
                ui.separator();

                // Cell count summary
                self.render_cell_summary(ui, cells);

                ui.separator();

                // Cell list
                ui.label("Cell Field Configuration:");
                self.render_cell_list(ui, cells);
            });

        // Cell state legend
        if self.show_state_legend {
            egui::CollapsingHeader::new("Cell State Legend")
                .default_open(false)
                .show(ui, |ui| {
                    self.render_state_legend(ui);
                });
        }

        // Selected cell details
        if let Some(selected_id) = self.selected_cell_id {
            if let Some(cell) = cells.iter().find(|c| c.id.0 == selected_id) {
                egui::CollapsingHeader::new("Selected Cell Details")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.render_cell_details(ui, cell);
                    });
            }
        }
    }

    fn render_cell_summary(&self, ui: &mut egui::Ui, cells: &[CellManifestation]) {
        let total = cells.len();
        let avg_coherence: f32 =
            cells.iter().map(|c| c.coherence()).sum::<f64>() as f32 / total.max(1) as f32;
        let with_dna = cells.iter().filter(|c| c.has_dna()).count();

        ui.columns(3, |cols| {
            cols[0].label(format!("Total Cells: {}", total));
            cols[1].label(format!("Avg Coherence: {:.2}", avg_coherence));
            cols[2].label(format!("With DNA: {}", with_dna));
        });
    }

    fn render_cell_list(&mut self, ui: &mut egui::Ui, cells: &[CellManifestation]) {
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                egui::Grid::new("cell_grid")
                    .num_columns(6)
                    .spacing([10.0, 5.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("ID").strong());
                        ui.label(egui::RichText::new("State").strong());
                        ui.label(egui::RichText::new("Coherence").strong());
                        ui.label(egui::RichText::new("Gen").strong());
                        ui.label(egui::RichText::new("Proteins").strong());
                        ui.label(egui::RichText::new("DNA").strong());
                        ui.end_row();

                        for cell in cells.iter().take(20) {
                            let data = CellViewData::from(cell);
                            let is_selected = self.selected_cell_id == Some(data.cell_id);

                            let response = ui
                                .selectable_label(is_selected, format!("C{}", data.cell_id % 1000));
                            if response.clicked() {
                                self.selected_cell_id = Some(data.cell_id);
                            }

                            let state_str = match data.state {
                                CellState::Quiescent => "Q",
                                CellState::Growing => "G",
                                CellState::Dividing => "D",
                                CellState::Differentiating => "Di",
                                CellState::Apoptotic => "A",
                                CellState::Senescent => "S",
                            };
                            ui.label(state_str);

                            ui.add(
                                egui::ProgressBar::new(data.coherence)
                                    .desired_width(40.0)
                                    .fill(CellStateColors::for_state(&data.state)),
                            );

                            ui.label(format!("{}", data.generation));
                            ui.label(format!("{}", data.protein_count));
                            ui.label(if data.has_dna { "Yes" } else { "No" });
                            ui.end_row();
                        }
                    });
            });
    }

    fn render_state_legend(&self, ui: &mut egui::Ui) {
        ui.label("Cell States:");

        let states = [
            (CellState::Quiescent, "Quiescent", "Resting state"),
            (CellState::Growing, "Growing", "Active growth"),
            (CellState::Dividing, "Dividing", "Mitosis"),
            (
                CellState::Differentiating,
                "Differentiating",
                "Specializing",
            ),
            (CellState::Apoptotic, "Apoptotic", "Programmed death"),
            (CellState::Senescent, "Senescent", "Aged state"),
        ];

        for (state, name, desc) in states {
            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    CellStateColors::for_state(&state),
                );
                ui.label(egui::RichText::new(name).strong());
                ui.label(desc);
            });
        }
    }

    fn render_cell_details(&self, ui: &mut egui::Ui, cell: &CellManifestation) {
        let data = CellViewData::from(cell);

        ui.columns(2, |cols| {
            cols[0].label("Cell ID:");
            cols[1].label(format!("{}", data.cell_id));

            cols[0].label("Position:");
            cols[1].label(format!(
                "({:.2}, {:.2}, {:.2})",
                data.position.0, data.position.1, data.position.2
            ));

            cols[0].label("Radius:");
            cols[1].label(format!("{:.2}", data.radius));

            cols[0].label("Generation:");
            cols[1].label(format!("{}", data.generation));

            cols[0].label("State:");
            let state_name = match data.state {
                CellState::Quiescent => "Quiescent",
                CellState::Growing => "Growing",
                CellState::Dividing => "Dividing",
                CellState::Differentiating => "Differentiating",
                CellState::Apoptotic => "Apoptotic",
                CellState::Senescent => "Senescent",
            };
            cols[1].label(state_name);
        });

        ui.separator();

        // Membrane properties - use boundary accessor
        ui.label("Membrane Properties:");
        let boundary = cell.boundary();
        let membrane = boundary.membrane();
        ui.columns(2, |cols| {
            cols[0].label("Thickness:");
            cols[1].label(format!("{:.1} nm", membrane.thickness()));

            cols[0].label("Permeability:");
            cols[1]
                .add(egui::ProgressBar::new(membrane.permeability() as f32).desired_width(100.0));

            cols[0].label("Membrane Potential:");
            cols[1].label(format!("{:.1} mV", membrane.potential()));

            cols[0].label("Receptor Density:");
            cols[1].add(
                egui::ProgressBar::new(membrane.receptor_density() as f32).desired_width(100.0),
            );
        });

        // Organelles
        ui.separator();
        ui.label("Organelles:");
        self.render_organelles(ui, cell);
    }

    fn render_organelles(&self, ui: &mut egui::Ui, cell: &CellManifestation) {
        let organelles = cell.field_config.organelles();

        for (organelle, position) in organelles {
            ui.horizontal(|ui| {
                let name = match organelle {
                    CellOrganelle::Nucleus => "Nucleus",
                    CellOrganelle::Mitochondria => "Mitochondria",
                    CellOrganelle::EndoplasmicReticulum => "ER",
                    CellOrganelle::GolgiApparatus => "Golgi",
                    CellOrganelle::Lysosome => "Lysosome",
                    CellOrganelle::Ribosome => "Ribosome",
                    CellOrganelle::Cytoskeleton => "Cytoskeleton",
                    CellOrganelle::Centrosome => "Centrosome",
                    CellOrganelle::Vacuole => "Vacuole",
                    CellOrganelle::Chloroplast => "Chloroplast",
                };

                let circle_pos = ui.cursor().min + egui::vec2(8.0, 8.0);
                ui.painter().circle_filled(
                    circle_pos,
                    6.0,
                    OrganelleColors::for_organelle(organelle),
                );
                ui.label(format!(
                    "{} at ({:.1}, {:.1}, {:.1})",
                    name, position.x, position.y, position.z
                ));
            });
        }
    }
}

impl Default for CellManifestationView {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Gaia Resonance View - Cellular/Gaia Consciousness Co-emergence
// ============================================================================

/// Gaia resonance visualization data
#[derive(Debug, Clone)]
pub struct GaiaResonanceData {
    pub coherence: f32,
    pub biomass: f32,
    pub biodiversity: f32,
    pub network_density: f32,
    pub planetary_health: f32,
    pub archetype_resonance: Vec<f32>,
}

impl From<&GaiaConsciousness> for GaiaResonanceData {
    fn from(gaia: &GaiaConsciousness) -> Self {
        let resonance = gaia.archetype_resonance();
        Self {
            coherence: gaia.coherence() as f32,
            biomass: gaia.biomass() as f32,
            biodiversity: gaia.biodiversity() as f32,
            network_density: gaia.network_density() as f32,
            planetary_health: gaia.planetary_health() as f32,
            archetype_resonance: resonance.iter().map(|&x| x as f32).collect(),
        }
    }
}

/// Planetary cell field view data
#[derive(Debug, Clone)]
pub struct PlanetaryFieldData {
    pub cell_count: usize,
    pub coherence: f32,
    pub avg_cell_coherence: f32,
    pub dividing_cells: usize,
    pub gaia: GaiaResonanceData,
}

impl From<&PlanetaryCellField> for PlanetaryFieldData {
    fn from(field: &PlanetaryCellField) -> Self {
        Self {
            cell_count: field.cell_count(),
            coherence: field.coherence() as f32,
            avg_cell_coherence: field.average_cell_coherence() as f32,
            dividing_cells: field.dividing_cells().len(),
            gaia: GaiaResonanceData::from(field.gaia()),
        }
    }
}

/// Resonance view widget
pub struct GaiaResonanceView {
    pub show_archetype_resonance: bool,
    pub show_planetary_health: bool,
}

impl GaiaResonanceView {
    pub fn new() -> Self {
        Self {
            show_archetype_resonance: true,
            show_planetary_health: true,
        }
    }

    /// Render the Gaia resonance panel
    pub fn render(&mut self, ui: &mut egui::Ui, field: &PlanetaryCellField) {
        let data = PlanetaryFieldData::from(field);

        egui::CollapsingHeader::new("Cellular-Gaia Co-emergence")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Simultaneous Emergence: Cells + Planetary Consciousness")
                        .strong(),
                );
                ui.separator();

                // Field overview
                ui.columns(2, |cols| {
                    cols[0].label("Cell Count:");
                    cols[1].label(format!("{}", data.cell_count));

                    cols[0].label("Field Coherence:");
                    cols[1].add(
                        egui::ProgressBar::new(data.coherence)
                            .desired_width(150.0)
                            .fill(BlueprintColors::active_gene()),
                    );

                    cols[0].label("Dividing Cells:");
                    cols[1].label(format!("{}", data.dividing_cells));
                });

                ui.separator();

                // Gaia metrics
                self.render_gaia_metrics(ui, &data.gaia);
            });

        // Planetary health
        if self.show_planetary_health {
            egui::CollapsingHeader::new("Planetary Health")
                .default_open(true)
                .show(ui, |ui| {
                    self.render_planetary_health(ui, &data.gaia);
                });
        }

        // Archetype resonance
        if self.show_archetype_resonance {
            egui::CollapsingHeader::new("Archetype Resonance")
                .default_open(false)
                .show(ui, |ui| {
                    self.render_archetype_resonance(ui, &data.gaia);
                });
        }
    }

    fn render_gaia_metrics(&self, ui: &mut egui::Ui, gaia: &GaiaResonanceData) {
        ui.label("Gaia Consciousness Metrics:");

        ui.columns(2, |cols| {
            cols[0].label("Gaia Coherence:");
            cols[1].add(
                egui::ProgressBar::new(gaia.coherence)
                    .desired_width(150.0)
                    .fill(BlueprintColors::spirit_archetype()),
            );

            cols[0].label("Biomass:");
            cols[1].label(format!("{:.0} cells", gaia.biomass));

            cols[0].label("Biodiversity:");
            cols[1].label(format!("{:.0} states", gaia.biodiversity));

            cols[0].label("Network Density:");
            cols[1].add(
                egui::ProgressBar::new(gaia.network_density)
                    .desired_width(150.0)
                    .fill(BlueprintColors::mind_archetype()),
            );
        });
    }

    fn render_planetary_health(&self, ui: &mut egui::Ui, gaia: &GaiaResonanceData) {
        ui.label("Planetary Health:");

        ui.horizontal(|ui| {
            ui.add(egui::ProgressBar::new(gaia.planetary_health).desired_width(200.0));
            ui.label(format!("{:.1}%", gaia.planetary_health * 100.0));
        });

        // Health factors
        ui.label("Contributing Factors:");
        ui.horizontal(|ui| {
            ui.label("Coherence:");
            ui.add(egui::ProgressBar::new(gaia.coherence).desired_width(80.0));
        });
        ui.horizontal(|ui| {
            ui.label("Diversity:");
            ui.add(egui::ProgressBar::new((gaia.biodiversity / 10.0).min(1.0)).desired_width(80.0));
        });
        ui.horizontal(|ui| {
            ui.label("Network:");
            ui.add(egui::ProgressBar::new(gaia.network_density).desired_width(80.0));
        });
    }

    fn render_archetype_resonance(&self, ui: &mut egui::Ui, gaia: &GaiaResonanceData) {
        ui.label("Archetype Resonance Profile:");

        // Group by complex
        let mind: f32 = gaia.archetype_resonance[0..7].iter().sum();
        let body: f32 = gaia.archetype_resonance[7..14].iter().sum();
        let spirit: f32 = gaia.archetype_resonance[14..21].iter().sum();
        let choice = gaia.archetype_resonance[21];

        ui.columns(4, |cols| {
            cols[0].label("Mind (A1-A7):");
            cols[1].add(egui::ProgressBar::new(mind / 7.0).desired_width(60.0));

            cols[2].label("Body (A8-A14):");
            cols[3].add(egui::ProgressBar::new(body / 7.0).desired_width(60.0));
        });

        ui.columns(2, |cols| {
            cols[0].label("Spirit (A15-A21):");
            cols[1].add(egui::ProgressBar::new(spirit / 7.0).desired_width(120.0));
        });

        ui.horizontal(|ui| {
            ui.label("Choice (A22):");
            ui.add(egui::ProgressBar::new(choice).desired_width(120.0));
        });

        // Individual archetypes
        ui.separator();
        ui.label("Individual Archetype Resonance:");

        egui::Grid::new("archetype_resonance_grid")
            .num_columns(8)
            .spacing([5.0, 2.0])
            .show(ui, |ui| {
                for i in 0..22 {
                    let resonance = gaia.archetype_resonance.get(i).copied().unwrap_or(0.5);
                    let color = match i {
                        0..=6 => BlueprintColors::mind_archetype(),
                        7..=13 => BlueprintColors::body_archetype(),
                        14..=20 => BlueprintColors::spirit_archetype(),
                        21 => BlueprintColors::choice_archetype(),
                        _ => Color32::GRAY,
                    };

                    ui.label(format!("A{}", i + 1));
                    ui.add(
                        egui::ProgressBar::new(resonance)
                            .desired_width(20.0)
                            .fill(color),
                    );

                    if (i + 1) % 8 == 0 {
                        ui.end_row();
                    }
                }
            });
    }
}

impl Default for GaiaResonanceView {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Main Cellular Visualization Panel
// ============================================================================

/// Main cellular visualization panel combining all views
pub struct CellularVisualizationPanel {
    pub blueprint_renderer: BlueprintRenderer,
    pub gene_expression_view: GeneExpressionView,
    pub protein_folding_view: ProteinFoldingView,
    pub cell_manifestation_view: CellManifestationView,
    pub gaia_resonance_view: GaiaResonanceView,
    active_tab: CellularTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellularTab {
    Blueprint,
    GeneExpression,
    ProteinFolding,
    CellManifestation,
    GaiaResonance,
}

impl CellularTab {
    pub fn name(&self) -> &'static str {
        match self {
            CellularTab::Blueprint => "Blueprint",
            CellularTab::GeneExpression => "Gene Expression",
            CellularTab::ProteinFolding => "Protein Folding",
            CellularTab::CellManifestation => "Cell Manifestation",
            CellularTab::GaiaResonance => "Gaia Resonance",
        }
    }
}

impl CellularVisualizationPanel {
    pub fn new() -> Self {
        Self {
            blueprint_renderer: BlueprintRenderer::new(),
            gene_expression_view: GeneExpressionView::new(),
            protein_folding_view: ProteinFoldingView::new(),
            cell_manifestation_view: CellManifestationView::new(),
            gaia_resonance_view: GaiaResonanceView::new(),
            active_tab: CellularTab::Blueprint,
        }
    }

    /// Render the complete cellular visualization panel
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        blueprint: &HolographicBlueprint,
        organism: &OrganismManifestation,
        field: &PlanetaryCellField,
    ) {
        egui::TopBottomPanel::top("cellular_tabs").show_inside(ui, |ui| {
            ui.label(egui::RichText::new("Phase C.4: Cellular Level Visualization").heading());
            ui.separator();

            // Tab buttons
            ui.horizontal(|ui| {
                for tab in &[
                    CellularTab::Blueprint,
                    CellularTab::GeneExpression,
                    CellularTab::ProteinFolding,
                    CellularTab::CellManifestation,
                    CellularTab::GaiaResonance,
                ] {
                    let is_active = self.active_tab == *tab;
                    if ui
                        .selectable_label(is_active, egui::RichText::new(tab.name()).strong())
                        .clicked()
                    {
                        self.active_tab = *tab;
                    }
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| match self.active_tab {
            CellularTab::Blueprint => {
                self.blueprint_renderer.render(ui, blueprint);
            }
            CellularTab::GeneExpression => {
                self.gene_expression_view.render(
                    ui,
                    &organism
                        .cells
                        .first()
                        .map(|c| &c.expression_profile)
                        .unwrap_or(&GeneExpressionProfile::default()),
                );
            }
            CellularTab::ProteinFolding => {
                let proteins: Vec<ProteinManifestation> =
                    organism.proteins.iter().take(10).cloned().collect();
                self.protein_folding_view.render(ui, &proteins);
            }
            CellularTab::CellManifestation => {
                self.cell_manifestation_view.render(ui, &organism.cells);
            }
            CellularTab::GaiaResonance => {
                self.gaia_resonance_view.render(ui, field);
            }
        });
    }

    /// Render with custom data sources
    pub fn render_with_data(
        &mut self,
        ui: &mut egui::Ui,
        blueprint: Option<&HolographicBlueprint>,
        profile: Option<&GeneExpressionProfile>,
        proteins: &[ProteinManifestation],
        cells: &[CellManifestation],
        field: Option<&PlanetaryCellField>,
    ) {
        egui::TopBottomPanel::top("cellular_tabs").show_inside(ui, |ui| {
            ui.label(egui::RichText::new("Phase C.4: Cellular Level Visualization").heading());
            ui.separator();

            ui.horizontal(|ui| {
                for tab in &[
                    CellularTab::Blueprint,
                    CellularTab::GeneExpression,
                    CellularTab::ProteinFolding,
                    CellularTab::CellManifestation,
                    CellularTab::GaiaResonance,
                ] {
                    let is_active = self.active_tab == *tab;
                    if ui
                        .selectable_label(is_active, egui::RichText::new(tab.name()).strong())
                        .clicked()
                    {
                        self.active_tab = *tab;
                    }
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| match self.active_tab {
            CellularTab::Blueprint => {
                if let Some(bp) = blueprint {
                    self.blueprint_renderer.render(ui, bp);
                } else {
                    ui.label("No blueprint available");
                }
            }
            CellularTab::GeneExpression => {
                if let Some(p) = profile {
                    self.gene_expression_view.render(ui, p);
                } else {
                    ui.label("No gene expression profile available");
                }
            }
            CellularTab::ProteinFolding => {
                self.protein_folding_view.render(ui, proteins);
            }
            CellularTab::CellManifestation => {
                self.cell_manifestation_view.render(ui, cells);
            }
            CellularTab::GaiaResonance => {
                if let Some(f) = field {
                    self.gaia_resonance_view.render(ui, f);
                } else {
                    ui.label("No planetary field available");
                }
            }
        });
    }
}

impl Default for CellularVisualizationPanel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a demo blueprint for visualization
pub fn create_demo_blueprint() -> HolographicBlueprint {
    HolographicBlueprint::for_human()
}

/// Create demo cells for visualization
pub fn create_demo_cells(count: usize) -> Vec<CellManifestation> {
    (0..count)
        .map(|i| {
            let position = Position3D::new(
                (i % 10) as f64 * 20.0,
                ((i / 10) % 10) as f64 * 20.0,
                (i / 100) as f64 * 20.0,
            );
            CellManifestation::new(position, 10.0 + (i % 5) as f64 * 2.0)
        })
        .collect()
}

/// Create demo proteins for visualization
pub fn create_demo_proteins(count: usize) -> Vec<ProteinManifestation> {
    (0..count)
        .map(|i| {
            let sequence: Vec<AminoAcid> = (0..20)
                .map(|j| {
                    AminoAcid::from_index(((i + j) % 20 + 1) as u8).unwrap_or(AminoAcid::Glycine)
                })
                .collect();
            let position = Position3D::new(i as f64 * 5.0, 0.0, 0.0);
            ProteinManifestation::from_sequence(&sequence, position)
        })
        .collect()
}

/// Create demo planetary field for visualization
pub fn create_demo_planetary_field() -> PlanetaryCellField {
    let cells = create_demo_cells(20);
    let mut field = PlanetaryCellField::new(Position3D::new(0.0, 0.0, 0.0));
    field.add_cells(cells);
    field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blueprint_render_data() {
        let blueprint = create_demo_blueprint();
        let data = BlueprintRenderData::from(&blueprint);
        assert_eq!(data.archetype_pattern.len(), 22);
    }

    #[test]
    fn test_gene_category_colors() {
        let regulatory = GeneCategoryColors::regulatory();
        assert!(regulatory.r() > 0 || regulatory.g() > 0 || regulatory.b() > 0);
    }

    #[test]
    fn test_cell_state_colors() {
        let growing = CellStateColors::for_state(&CellState::Growing);
        assert!(growing.g() > 0);
    }

    #[test]
    fn test_demo_functions() {
        let cells = create_demo_cells(10);
        assert_eq!(cells.len(), 10);

        let proteins = create_demo_proteins(5);
        assert_eq!(proteins.len(), 5);

        let field = create_demo_planetary_field();
        assert!(field.cell_count() > 0);
    }

    #[test]
    fn test_cellular_tabs() {
        assert_eq!(CellularTab::Blueprint.name(), "Blueprint");
        assert_eq!(CellularTab::GeneExpression.name(), "Gene Expression");
    }
}
