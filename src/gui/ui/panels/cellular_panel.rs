//! Cellular Panel - Cellular Level Visualization Panel
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.4:
//! "Cellular Level Visualization: Blueprint, gene expression, protein folding, cell manifestation, Gaia resonance"
//!
//! This module provides:
//! - Tabbed interface for different cellular views
//! - Holographic blueprint for morphogenesis
//! - Archetype-to-gene encoding (A1-A7=Mind→regulatory, A8-A14=Body→structural, A15-A21=Spirit→epigenetic, A22=Choice→mutation)
//! - 3D protein field configurations
//! - Cell membranes as field boundaries
//! - Cellular/Gaia consciousness co-emergence
//! - Entity integration for cellular property display

use egui::{Color32, Context, Ui};
use std::collections::HashMap;

use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::visualization::cellular_viz::{
    BlueprintRenderer, CellManifestationView, GaiaResonanceView, GeneExpressionView,
    ProteinFoldingView,
};
use crate::holographic_foundation::archetype_profile::ArchetypeActivationProfile;
use crate::holographic_foundation::cellular_emergence::cell_manifestation::{
    CellManifestation, CellState,
};
use crate::holographic_foundation::cellular_emergence::gene_expression::{
    ExpressionCondition, GeneExpressionEngine,
};
use crate::holographic_foundation::cellular_emergence::holographic_blueprint::HolographicBlueprint;
use crate::holographic_foundation::cellular_emergence::simultaneous_emergence::GaiaConsciousness;
use crate::holographic_foundation::field_state::{HolographicFieldState, Position3D};

/// View mode for cellular panel tabs
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CellularViewMode {
    /// Holographic blueprint for morphogenesis
    #[default]
    Blueprint,
    /// Archetype-to-gene encoding
    GeneExpression,
    /// 3D protein field configurations
    ProteinFolding,
    /// Cell membranes as field boundaries
    CellManifestation,
    /// Cellular/Gaia consciousness co-emergence
    GaiaResonance,
}

/// Cellular Panel for Phase C.4
///
/// Provides integrated cellular visualization with tabbed interface for:
/// - Blueprint: Holographic blueprint for morphogenesis
/// - GeneExpression: Archetype-to-gene encoding (A1-A22)
/// - ProteinFolding: 3D protein field configurations
/// - CellManifestation: Cell membranes as field boundaries
/// - GaiaResonance: Cellular/Gaia consciousness co-emergence
pub struct CellularPanel {
    /// Panel visibility toggle
    pub visible: bool,

    /// Current tab/view mode
    pub view_mode: CellularViewMode,

    /// Blueprint renderer for morphogenesis visualization
    blueprint_renderer: BlueprintRenderer,

    /// Gene expression view for archetype-to-gene encoding
    gene_expression_view: GeneExpressionView,

    /// Protein folding view for 3D field configurations
    protein_folding_view: ProteinFoldingView,

    /// Cell manifestation view for membrane visualization
    cell_manifestation_view: CellManifestationView,

    /// Gaia resonance view for consciousness co-emergence
    gaia_resonance_view: GaiaResonanceView,

    /// Animation time accumulator
    animation_time: f32,

    /// Display controls
    pub show_controls: bool,

    /// Current entity being displayed
    current_entity: Option<SubSubLogos>,

    /// Entity cache for switching between entities
    entity_cache: HashMap<u64, SubSubLogos>,

    /// Selected entity ID
    selected_entity_id: Option<u64>,

    /// Selected cell type for demo
    selected_cell_type: String,

    /// Selected developmental stage
    #[allow(dead_code)]
    selected_stage: String,

    /// Blueprint data cache
    cached_blueprint_data: Option<BlueprintPanelData>,

    /// Gene profile data cache
    cached_gene_data: Option<GeneExpressionPanelData>,

    /// Protein data cache
    cached_protein_data: Option<ProteinPanelData>,

    /// Cell data cache
    cached_cell_data: Option<CellPanelData>,

    /// Gaia field data cache
    cached_gaia_data: Option<GaiaPanelData>,

    /// Field-derived blueprint cache for cellular displays
    field_blueprint: Option<HolographicBlueprint>,
}

/// Blueprint panel rendering data
#[derive(Debug, Clone)]
struct BlueprintPanelData {
    pub archetype_pattern: Vec<f32>,
    pub mind_dominance: f32,
    pub body_dominance: f32,
    pub spirit_dominance: f32,
    pub choice_capacity: f32,
    pub complexity_index: f32,
}

/// Gene expression panel data
#[derive(Debug, Clone)]
struct GeneExpressionPanelData {
    pub regulatory_count: usize,
    pub structural_count: usize,
    pub epigenetic_count: usize,
    pub mutation_count: usize,
    pub total_expression_potential: f32,
}

/// Protein panel data
#[derive(Debug, Clone)]
struct ProteinPanelData {
    pub protein_count: usize,
    pub avg_folding_coherence: f32,
    pub avg_helix_content: f32,
    pub avg_sheet_content: f32,
    pub total_amino_acids: usize,
}

/// Cell panel data
#[derive(Debug, Clone)]
struct CellPanelData {
    pub cell_count: usize,
    pub avg_coherence: f32,
    pub cells_with_dna: usize,
    #[allow(dead_code)]
    pub dividing_cells: usize,
    pub quiescent_ratio: f32,
    pub growing_ratio: f32,
    pub dividing_ratio: f32,
    pub differentiating_ratio: f32,
    pub senescent_ratio: f32,
}

/// Gaia panel data
#[derive(Debug, Clone)]
struct GaiaPanelData {
    pub coherence: f32,
    pub biomass: f32,
    pub biodiversity: f32,
    pub network_density: f32,
    pub planetary_health: f32,
}

impl CellularPanel {
    /// Create a new cellular panel
    pub fn new() -> Self {
        Self {
            visible: true,
            view_mode: CellularViewMode::Blueprint,
            blueprint_renderer: BlueprintRenderer::new(),
            gene_expression_view: GeneExpressionView::new(),
            protein_folding_view: ProteinFoldingView::new(),
            cell_manifestation_view: CellManifestationView::new(),
            gaia_resonance_view: GaiaResonanceView::new(),
            animation_time: 0.0,
            show_controls: true,
            current_entity: None,
            entity_cache: HashMap::new(),
            selected_entity_id: None,
            selected_cell_type: "Stem Cell".to_string(),
            selected_stage: "Zygote".to_string(),
            cached_blueprint_data: None,
            cached_gene_data: None,
            cached_protein_data: None,
            cached_cell_data: None,
            cached_gaia_data: None,
            field_blueprint: None,
        }
    }

    /// Update from entity - extract cellular-relevant data
    pub fn update_from_entity(&mut self, entity: &SubSubLogos) {
        self.current_entity = Some(entity.clone());
        self.selected_entity_id = Some(entity.entity_id.as_u64());

        // Cache entity
        self.entity_cache
            .insert(entity.entity_id.as_u64(), entity.clone());

        // Map entity state to cell type (for visualization)
        self.selected_cell_type = self.map_entity_to_cell_type(entity);

        // Generate cached data from entity
        self.generate_cached_data_from_entity(entity);
    }

    /// Map entity state to a cell type for visualization
    fn map_entity_to_cell_type(&self, entity: &SubSubLogos) -> String {
        // Use archetype activations to determine a representative cell type
        let archetype_sum: f64 = entity.archetype_activations.iter().sum();

        // Map to common cell types
        let cell_types = [
            "Stem Cell",
            "Neuron",
            "Cardiomyocyte",
            "Hepatocyte",
            "Fibroblast",
            "T-Cell",
            "Erythrocyte",
            "Epithelial Cell",
        ];
        let idx = ((archetype_sum * 10.0) as usize) % cell_types.len();

        cell_types[idx].to_string()
    }

    /// Generate cached data from entity
    fn generate_cached_data_from_entity(&mut self, entity: &SubSubLogos) {
        // Generate blueprint data
        let archetype_pattern: Vec<f32> = entity
            .archetype_activations
            .iter()
            .map(|&x| x as f32)
            .collect();

        let mind_dominance = archetype_pattern.iter().take(7).sum::<f32>() / 7.0;
        let body_dominance = archetype_pattern.iter().skip(7).take(7).sum::<f32>() / 7.0;
        let spirit_dominance = archetype_pattern.iter().skip(14).take(7).sum::<f32>() / 7.0;
        let choice_capacity = archetype_pattern.get(21).copied().unwrap_or(0.5);

        self.cached_blueprint_data = Some(BlueprintPanelData {
            archetype_pattern,
            mind_dominance,
            body_dominance,
            spirit_dominance,
            choice_capacity,
            complexity_index: (mind_dominance + body_dominance + spirit_dominance) / 3.0,
        });

        // Generate gene expression data
        let regulatory_count = 7;
        let structural_count = 7;
        let epigenetic_count = 7;
        let mutation_count = 1;

        self.cached_gene_data = Some(GeneExpressionPanelData {
            regulatory_count,
            structural_count,
            epigenetic_count,
            mutation_count,
            total_expression_potential: 0.75,
        });

        // Generate protein data
        self.cached_protein_data = Some(ProteinPanelData {
            protein_count: 50,
            avg_folding_coherence: 0.82,
            avg_helix_content: 0.35,
            avg_sheet_content: 0.25,
            total_amino_acids: 5000,
        });

        // Generate cell data
        self.cached_cell_data = Some(CellPanelData {
            cell_count: 100,
            avg_coherence: 0.78,
            cells_with_dna: 100,
            dividing_cells: 5,
            quiescent_ratio: 0.40,
            growing_ratio: 0.25,
            dividing_ratio: 0.05,
            differentiating_ratio: 0.20,
            senescent_ratio: 0.10,
        });

        // Generate Gaia data
        self.cached_gaia_data = Some(GaiaPanelData {
            coherence: 0.85,
            biomass: 1000000.0,
            biodiversity: 8.0,
            network_density: 0.65,
            planetary_health: 0.75,
        });
    }

    /// Update animations
    pub fn update(&mut self, delta_time: f32) {
        self.update_with_field(delta_time, None, None);
    }

    /// Update animations and optionally refresh from field context
    pub fn update_with_field(
        &mut self,
        delta_time: f32,
        field_state: Option<&HolographicFieldState>,
        focus_position: Option<&Position3D>,
    ) {
        self.animation_time += delta_time;

        if let Some(field_state) = field_state {
            let focus = focus_position
                .cloned()
                .unwrap_or_else(|| field_state.root().position);
            self.generate_cached_data_from_field(field_state, &focus);
        }

        // Update all visualization components
        // Note: These would update their internal state if they had update methods
        // For now, we just advance the animation time
    }

    fn generate_cached_data_from_field(
        &mut self,
        field_state: &HolographicFieldState,
        focus: &Position3D,
    ) {
        let node = field_state
            .get_node_at(focus)
            .unwrap_or_else(|| field_state.root());

        let blueprint = HolographicBlueprint::from_archetype_pattern(node.archetype_vector);
        self.field_blueprint = Some(blueprint.clone());

        let archetype_pattern: Vec<f32> = blueprint
            .archetype_pattern
            .iter()
            .map(|&x| x as f32)
            .collect();
        let mind_dominance = archetype_pattern.iter().take(7).sum::<f32>() / 7.0;
        let body_dominance = archetype_pattern.iter().skip(7).take(7).sum::<f32>() / 7.0;
        let spirit_dominance = archetype_pattern.iter().skip(14).take(7).sum::<f32>() / 7.0;
        let choice_capacity = archetype_pattern.get(21).copied().unwrap_or(0.5);

        self.cached_blueprint_data = Some(BlueprintPanelData {
            archetype_pattern,
            mind_dominance,
            body_dominance,
            spirit_dominance,
            choice_capacity,
            complexity_index: blueprint.complexity_index() as f32,
        });

        let mut expression_engine = GeneExpressionEngine::new(blueprint.gene_network.clone());
        let expression_condition = ExpressionCondition::from_field(field_state, focus);
        let expression_results = expression_engine.evaluate_all(&expression_condition);

        let mut regulatory_count = 0usize;
        let mut structural_count = 0usize;
        let mut epigenetic_count = 0usize;
        let mut mutation_count = 0usize;
        for result in &expression_results {
            let archetype_source = result.gene_id.archetype_source();
            match archetype_source {
                0..=6 => regulatory_count += 1,
                7..=13 => structural_count += 1,
                14..=20 => epigenetic_count += 1,
                _ => mutation_count += 1,
            }
        }

        self.cached_gene_data = Some(GeneExpressionPanelData {
            regulatory_count,
            structural_count,
            epigenetic_count,
            mutation_count,
            total_expression_potential: expression_engine.global_expression_level() as f32,
        });

        let profile = ArchetypeActivationProfile::new(node.archetype_vector);
        let mut seed_cell = CellManifestation::new(*focus, 10.0).with_profile(&profile);
        let protein_count = (4.0 + (blueprint.complexity_index() * 8.0)).round() as usize;
        for i in 0..protein_count.max(1) {
            let protein = blueprint.unfold_protein(&format!("field_protein_{}", i), 120);
            seed_cell.add_protein(protein);
        }

        let cells = vec![seed_cell.clone()];
        let gaia = GaiaConsciousness::from_cells(&cells);

        self.selected_cell_type = match seed_cell.state() {
            CellState::Quiescent => "Stem Cell",
            CellState::Growing => "Fibroblast",
            CellState::Dividing => "Stem Cell",
            CellState::Differentiating => "Epithelial Cell",
            CellState::Senescent => "Neuron",
            CellState::Apoptotic => "Apoptotic Cell",
        }
        .to_string();

        let field_resonance = expression_condition.field_resonance as f32;
        let dividing_ratio = if seed_cell.can_divide() { 0.30 } else { 0.10 };
        let growing_ratio = (0.20 + field_resonance * 0.25).clamp(0.05, 0.45);
        let differentiating_ratio = (0.10 + spirit_dominance * 0.20).clamp(0.05, 0.35);
        let senescent_ratio = (0.05 + (1.0 - field_resonance) * 0.20).clamp(0.02, 0.25);
        let quiescent_ratio =
            (1.0 - growing_ratio - dividing_ratio - differentiating_ratio - senescent_ratio)
                .max(0.05);
        let ratio_sum = quiescent_ratio
            + growing_ratio
            + dividing_ratio
            + differentiating_ratio
            + senescent_ratio;

        self.cached_protein_data = Some(ProteinPanelData {
            protein_count: seed_cell.protein_count(),
            avg_folding_coherence: (0.55 + field_resonance * 0.4).clamp(0.0, 1.0),
            avg_helix_content: (0.25 + mind_dominance * 0.35).clamp(0.0, 1.0),
            avg_sheet_content: (0.20 + body_dominance * 0.35).clamp(0.0, 1.0),
            total_amino_acids: seed_cell.protein_count() * 120,
        });

        self.cached_cell_data = Some(CellPanelData {
            cell_count: cells.len(),
            avg_coherence: seed_cell.coherence() as f32,
            cells_with_dna: usize::from(seed_cell.has_dna()),
            dividing_cells: usize::from(seed_cell.can_divide()),
            quiescent_ratio: quiescent_ratio / ratio_sum,
            growing_ratio: growing_ratio / ratio_sum,
            dividing_ratio: dividing_ratio / ratio_sum,
            differentiating_ratio: differentiating_ratio / ratio_sum,
            senescent_ratio: senescent_ratio / ratio_sum,
        });

        self.cached_gaia_data = Some(GaiaPanelData {
            coherence: gaia.coherence() as f32,
            biomass: gaia.biomass() as f32,
            biodiversity: gaia.biodiversity() as f32,
            network_density: gaia.network_density() as f32,
            planetary_health: gaia.planetary_health() as f32,
        });
    }

    /// Show the panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("🧫 Cellular Level")
            .collapsible(true)
            .resizable(true)
            .default_width(600.0)
            .default_height(700.0)
            .show(ctx, |ui| {
                self.show_content(ui);
            });
    }

    /// Show panel content
    fn show_content(&mut self, ui: &mut Ui) {
        // Entity selector
        self.show_entity_selector(ui);
        ui.separator();

        // Cell type selector
        self.show_cell_type_selector(ui);
        ui.separator();

        // Tab navigation
        self.show_tab_navigation(ui);
        ui.separator();

        // Show controls toggle
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_controls, "Show Controls");
        });

        // Render current tab content
        match self.view_mode {
            CellularViewMode::Blueprint => {
                self.show_blueprint_tab(ui);
            }
            CellularViewMode::GeneExpression => {
                self.show_gene_expression_tab(ui);
            }
            CellularViewMode::ProteinFolding => {
                self.show_protein_folding_tab(ui);
            }
            CellularViewMode::CellManifestation => {
                self.show_cell_manifestation_tab(ui);
            }
            CellularViewMode::GaiaResonance => {
                self.show_gaia_resonance_tab(ui);
            }
        }
    }

    /// Show entity selector
    fn show_entity_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Entity:");

            // Build entity list
            let entity_ids: Vec<u64> = self.entity_cache.keys().cloned().collect();

            if entity_ids.is_empty() {
                ui.label("No entity selected (using demo data)");
                return;
            }

            // Simple selector using radio buttons
            for &id in &entity_ids {
                let label = format!("Entity {}", id);
                ui.selectable_value(&mut self.selected_entity_id, Some(id), &label);
            }
        });
    }

    /// Show cell type selector
    fn show_cell_type_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Cell Type:");

            let cell_types = [
                "Stem Cell",
                "Neuron",
                "Cardiomyocyte",
                "Hepatocyte",
                "Fibroblast",
                "T-Cell",
                "Erythrocyte",
                "Epithelial Cell",
            ];

            for ct in cell_types {
                ui.selectable_value(&mut self.selected_cell_type, ct.to_string(), ct);
            }
        });
    }

    /// Show tab navigation
    fn show_tab_navigation(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.view_mode,
                CellularViewMode::Blueprint,
                "📋 Blueprint",
            );
            ui.selectable_value(
                &mut self.view_mode,
                CellularViewMode::GeneExpression,
                "🧬 Gene Expr",
            );
            ui.selectable_value(
                &mut self.view_mode,
                CellularViewMode::ProteinFolding,
                "🔬 Protein",
            );
            ui.selectable_value(
                &mut self.view_mode,
                CellularViewMode::CellManifestation,
                "🧫 Cell",
            );
            ui.selectable_value(
                &mut self.view_mode,
                CellularViewMode::GaiaResonance,
                "🌍 Gaia",
            );
        });
    }

    /// Show blueprint tab
    fn show_blueprint_tab(&mut self, ui: &mut Ui) {
        ui.heading("Holographic Blueprint for Morphogenesis");

        // Controls
        if self.show_controls {
            self.show_blueprint_controls(ui);
            ui.separator();
        }

        // Render blueprint visualization using cached data
        self.render_blueprint_visualization(ui);

        // Show blueprint explanation
        ui.add_space(10.0);
        ui.label("Archetype Encoding (A1-A22):");
        ui.label("A1-A7 (Mind): Regulatory genes - control when/where genes express");
        ui.label("A8-A14 (Body): Structural genes - encode proteins for physical structure");
        ui.label(
            "A15-A21 (Spirit): Epigenetic control - modulate expression without sequence change",
        );
        ui.label("A22 (Choice): Mutation/recombination operators - drive evolutionary change");
    }

    /// Show blueprint controls
    fn show_blueprint_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Blueprint Controls", |ui| {
            ui.label("View Options:");
            ui.checkbox(
                &mut self.blueprint_renderer.selected_stage.is_some(),
                "Show Developmental Stages",
            );
        });
    }

    /// Render blueprint visualization
    fn render_blueprint_visualization(&self, ui: &mut Ui) {
        let data = self.cached_blueprint_data.as_ref();

        if let Some(blueprint) = data {
            // Complexity index
            ui.label("Complexity Index:");
            ui.add(
                egui::ProgressBar::new(blueprint.complexity_index)
                    .desired_width(200.0)
                    .fill(Color32::from_rgb(100, 149, 237)),
            );
            ui.label(format!("{:.3}", blueprint.complexity_index));

            ui.separator();

            // Archetype dominance visualization
            ui.label(egui::RichText::new("Archetype Complex Dominance").strong());
            self.render_dominance_bars(ui, blueprint);

            ui.separator();

            // Archetype encoding visualization
            ui.label(egui::RichText::new("Archetype Encoding (A1-A22)").strong());
            self.render_archetype_grid(ui, blueprint);
        } else {
            ui.label("No blueprint data available");
        }
    }

    /// Render dominance bars
    fn render_dominance_bars(&self, ui: &mut Ui, data: &BlueprintPanelData) {
        let bar_width = 150.0;

        ui.horizontal(|ui| {
            ui.label("Mind (A1-A7):");
            ui.add(
                egui::ProgressBar::new(data.mind_dominance)
                    .desired_width(bar_width)
                    .fill(Color32::from_rgb(100, 149, 237)), // Cornflower blue
            );
            ui.label(format!("{:.2}", data.mind_dominance));
        });

        ui.horizontal(|ui| {
            ui.label("Body (A8-A14):");
            ui.add(
                egui::ProgressBar::new(data.body_dominance)
                    .desired_width(bar_width)
                    .fill(Color32::from_rgb(34, 139, 34)), // Forest green
            );
            ui.label(format!("{:.2}", data.body_dominance));
        });

        ui.horizontal(|ui| {
            ui.label("Spirit (A15-A21):");
            ui.add(
                egui::ProgressBar::new(data.spirit_dominance)
                    .desired_width(bar_width)
                    .fill(Color32::from_rgb(218, 165, 32)), // Goldenrod
            );
            ui.label(format!("{:.2}", data.spirit_dominance));
        });

        ui.horizontal(|ui| {
            ui.label("Choice (A22):");
            ui.add(
                egui::ProgressBar::new(data.choice_capacity)
                    .desired_width(bar_width)
                    .fill(Color32::from_rgb(255, 99, 71)), // Tomato
            );
            ui.label(format!("{:.2}", data.choice_capacity));
        });
    }

    /// Render archetype grid
    fn render_archetype_grid(&self, ui: &mut Ui, data: &BlueprintPanelData) {
        let grid_size = 22;
        let cell_size = 20.0;

        egui::Grid::new("cellular_archetype_grid")
            .num_columns(11)
            .spacing([5.0, 5.0])
            .show(ui, |ui| {
                for i in 0..grid_size {
                    if i > 0 && i % 11 == 0 {
                        ui.end_row();
                    }

                    let activation = data.archetype_pattern.get(i).copied().unwrap_or(0.5);
                    let color = match i {
                        0..=6 => Color32::from_rgb(100, 149, 237),  // Mind - Blue
                        7..=13 => Color32::from_rgb(34, 139, 34),   // Body - Green
                        14..=20 => Color32::from_rgb(218, 165, 32), // Spirit - Gold
                        21 => Color32::from_rgb(255, 99, 71),       // Choice - Tomato
                        _ => Color32::GRAY,
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

    /// Show gene expression tab
    fn show_gene_expression_tab(&mut self, ui: &mut Ui) {
        ui.heading("Gene Expression Profile");

        // Controls
        if self.show_controls {
            self.show_gene_expression_controls(ui);
            ui.separator();
        }

        // Render gene expression using cached data
        self.render_gene_expression_visualization(ui);

        // Show gene expression explanation
        ui.add_space(10.0);
        ui.label("Gene Categories:");
        ui.label("A1-A7 (Regulatory/Mind): Control when/where genes express");
        ui.label("A8-A14 (Structural/Body): Encode proteins for physical structure");
        ui.label("A15-A21 (Epigenetic/Spirit): Modulate expression without sequence change");
        ui.label("A22 (Mutation/Choice): Evolutionary change operators");
    }

    /// Show gene expression controls
    fn show_gene_expression_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Gene Expression Controls", |ui| {
            ui.checkbox(
                &mut self.gene_expression_view.show_network,
                "Show Regulatory Network",
            );
            ui.checkbox(
                &mut self.gene_expression_view.show_expression_levels,
                "Show Expression Levels",
            );
        });
    }

    /// Render gene expression visualization
    fn render_gene_expression_visualization(&self, ui: &mut Ui) {
        let data = self.cached_gene_data.as_ref();

        if let Some(gene) = data {
            // Expression potential
            ui.label("Total Expression Potential:");
            ui.add(
                egui::ProgressBar::new(gene.total_expression_potential)
                    .desired_width(200.0)
                    .fill(Color32::from_rgb(50, 205, 50)),
            );
            ui.label(format!("{:.3}", gene.total_expression_potential));

            ui.separator();

            // Category breakdown
            ui.label(egui::RichText::new("Gene Categories").strong());

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(65, 105, 225), // Royal blue
                );
                ui.label(format!(
                    "Regulatory (Mind/A1-A7): {} genes",
                    gene.regulatory_count
                ));
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(34, 139, 34), // Forest green
                );
                ui.label(format!(
                    "Structural (Body/A8-A14): {} genes",
                    gene.structural_count
                ));
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(148, 0, 211), // Dark violet
                );
                ui.label(format!(
                    "Epigenetic (Spirit/A15-A21): {} genes",
                    gene.epigenetic_count
                ));
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(255, 69, 0), // Red-orange
                );
                ui.label(format!(
                    "Mutation (Choice/A22): {} genes",
                    gene.mutation_count
                ));
            });

            ui.separator();

            // Gene list (simplified)
            ui.label(egui::RichText::new("Gene List (A1-A22)").strong());
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    egui::Grid::new("gene_list_grid")
                        .num_columns(5)
                        .spacing([10.0, 5.0])
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Gene").strong());
                            ui.label(egui::RichText::new("Archetype").strong());
                            ui.label(egui::RichText::new("Category").strong());
                            ui.label(egui::RichText::new("Threshold").strong());
                            ui.label(egui::RichText::new("Strength").strong());
                            ui.end_row();

                            for i in 0..22 {
                                let (archetype, category, threshold, strength) = if i < 7 {
                                    (
                                        format!("A{}", i + 1),
                                        "Reg".to_string(),
                                        0.5 + (i as f32 * 0.05),
                                        0.6 + (i as f32 * 0.03),
                                    )
                                } else if i < 14 {
                                    (
                                        format!("A{}", i + 1),
                                        "Struct".to_string(),
                                        0.4 + ((i - 7) as f32 * 0.05),
                                        0.7 + ((i - 7) as f32 * 0.02),
                                    )
                                } else if i < 21 {
                                    (
                                        format!("A{}", i + 1),
                                        "Epi".to_string(),
                                        0.6 + ((i - 14) as f32 * 0.04),
                                        0.5 + ((i - 14) as f32 * 0.04),
                                    )
                                } else {
                                    ("A22".to_string(), "Mut".to_string(), 0.8, 0.3)
                                };

                                let color = if i < 7 {
                                    Color32::from_rgb(65, 105, 225)
                                } else if i < 14 {
                                    Color32::from_rgb(34, 139, 34)
                                } else if i < 21 {
                                    Color32::from_rgb(148, 0, 211)
                                } else {
                                    Color32::from_rgb(255, 69, 0)
                                };

                                let rect_min = ui.cursor().min;
                                ui.painter().rect_filled(
                                    egui::Rect::from_min_size(rect_min, egui::vec2(10.0, 10.0)),
                                    1.0,
                                    color,
                                );

                                ui.label(format!("G{}", i + 1));
                                ui.label(archetype);
                                ui.label(category);
                                ui.label(format!("{:.2}", threshold));
                                ui.label(format!("{:.2}", strength));
                                ui.end_row();
                            }
                        });
                });
        } else {
            ui.label("No gene expression data available");
        }
    }

    /// Show protein folding tab
    fn show_protein_folding_tab(&mut self, ui: &mut Ui) {
        ui.heading("Protein Folding Field");

        // Controls
        if self.show_controls {
            self.show_protein_folding_controls(ui);
            ui.separator();
        }

        // Render protein folding visualization using cached data
        self.render_protein_folding_visualization(ui);

        // Show protein folding explanation
        ui.add_space(10.0);
        ui.label("3D Protein Field Configurations:");
        ui.label("Proteins fold into field interference minima");
        ui.label("α-Helix: Spiral field minimum");
        ui.label("β-Sheet: Extended strand network");
    }

    /// Show protein folding controls
    fn show_protein_folding_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Protein Folding Controls", |ui| {
            ui.checkbox(
                &mut self.protein_folding_view.show_sequence,
                "Show Sequence",
            );
            ui.checkbox(
                &mut self.protein_folding_view.show_structure_legend,
                "Show Structure Legend",
            );
        });
    }

    /// Render protein folding visualization
    fn render_protein_folding_visualization(&self, ui: &mut Ui) {
        let data = self.cached_protein_data.as_ref();

        if let Some(protein) = data {
            // Summary
            ui.columns(2, |cols| {
                cols[0].label("Total Proteins:");
                cols[1].label(format!("{}", protein.protein_count));

                cols[0].label("Total Amino Acids:");
                cols[1].label(format!("{}", protein.total_amino_acids));

                cols[0].label("Avg Folding Coherence:");
                cols[1].add(
                    egui::ProgressBar::new(protein.avg_folding_coherence)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(50, 205, 50)),
                );
            });

            ui.separator();

            // Secondary structure content
            ui.label("Secondary Structure Content:");

            ui.horizontal(|ui| {
                ui.label("α-Helix:");
                ui.add(
                    egui::ProgressBar::new(protein.avg_helix_content)
                        .desired_width(120.0)
                        .fill(Color32::from_rgb(255, 100, 100)), // Light red
                );
                ui.label(format!("{:.1}%", protein.avg_helix_content * 100.0));
            });

            ui.horizontal(|ui| {
                ui.label("β-Sheet:");
                ui.add(
                    egui::ProgressBar::new(protein.avg_sheet_content)
                        .desired_width(120.0)
                        .fill(Color32::from_rgb(100, 100, 255)), // Light blue
                );
                ui.label(format!("{:.1}%", protein.avg_sheet_content * 100.0));
            });

            ui.horizontal(|ui| {
                ui.label("Random Coil:");
                let random_coil = 1.0 - protein.avg_helix_content - protein.avg_sheet_content;
                ui.add(
                    egui::ProgressBar::new(random_coil)
                        .desired_width(120.0)
                        .fill(Color32::from_rgb(150, 150, 150)), // Gray
                );
                ui.label(format!("{:.1}%", random_coil * 100.0));
            });

            ui.separator();

            // Structure legend
            ui.label("Structure Legend:");
            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(255, 100, 100),
                );
                ui.label("α-Helix: Field minimum spiral");
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(100, 100, 255),
                );
                ui.label("β-Sheet: Extended strand network");
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(255, 255, 100),
                );
                ui.label("β-Turn: Direction change");
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(150, 150, 150),
                );
                ui.label("Random Coil: Unstructured region");
            });
        } else {
            ui.label("No protein data available");
        }
    }

    /// Show cell manifestation tab
    fn show_cell_manifestation_tab(&mut self, ui: &mut Ui) {
        ui.heading("Cell Manifestation");

        // Controls
        if self.show_controls {
            self.show_cell_manifestation_controls(ui);
            ui.separator();
        }

        // Render cell manifestation using cached data
        self.render_cell_manifestation_visualization(ui);

        // Show cell explanation
        ui.add_space(10.0);
        ui.label("Cells as Field Boundaries:");
        ui.label("Cell membranes emerge as field interference boundaries");
        ui.label("Organelles as sub-field structures");
    }

    /// Show cell manifestation controls
    fn show_cell_manifestation_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Cell Manifestation Controls", |ui| {
            ui.checkbox(
                &mut self.cell_manifestation_view.show_membrane,
                "Show Membrane",
            );
            ui.checkbox(
                &mut self.cell_manifestation_view.show_organelles,
                "Show Organelles",
            );
            ui.checkbox(
                &mut self.cell_manifestation_view.show_state_legend,
                "Show State Legend",
            );
        });
    }

    /// Render cell manifestation visualization
    fn render_cell_manifestation_visualization(&self, ui: &mut Ui) {
        let data = self.cached_cell_data.as_ref();

        if let Some(cell) = data {
            // Cell summary
            ui.columns(3, |cols| {
                cols[0].label(format!("Total Cells: {}", cell.cell_count));
                cols[1].label(format!("Avg Coherence: {:.2}", cell.avg_coherence));
                cols[2].label(format!("With DNA: {}", cell.cells_with_dna));
            });

            ui.separator();

            // Cell state breakdown
            ui.label("Cell States:");

            ui.horizontal(|ui| {
                ui.label("Quiescent:");
                ui.add(
                    egui::ProgressBar::new(cell.quiescent_ratio)
                        .desired_width(80.0)
                        .fill(Color32::GRAY),
                );
                ui.label(format!("{:.0}%", cell.quiescent_ratio * 100.0));
            });

            ui.horizontal(|ui| {
                ui.label("Growing:");
                ui.add(
                    egui::ProgressBar::new(cell.growing_ratio)
                        .desired_width(80.0)
                        .fill(Color32::from_rgb(50, 205, 50)),
                );
                ui.label(format!("{:.0}%", cell.growing_ratio * 100.0));
            });

            ui.horizontal(|ui| {
                ui.label("Dividing:");
                ui.add(
                    egui::ProgressBar::new(cell.dividing_ratio)
                        .desired_width(80.0)
                        .fill(Color32::from_rgb(255, 165, 0)),
                );
                ui.label(format!("{:.0}%", cell.dividing_ratio * 100.0));
            });

            ui.horizontal(|ui| {
                ui.label("Differentiating:");
                ui.add(
                    egui::ProgressBar::new(cell.differentiating_ratio)
                        .desired_width(80.0)
                        .fill(Color32::from_rgb(138, 43, 226)),
                );
                ui.label(format!("{:.0}%", cell.differentiating_ratio * 100.0));
            });

            ui.horizontal(|ui| {
                ui.label("Senescent:");
                ui.add(
                    egui::ProgressBar::new(cell.senescent_ratio)
                        .desired_width(80.0)
                        .fill(Color32::from_rgb(139, 69, 19)),
                );
                ui.label(format!("{:.0}%", cell.senescent_ratio * 100.0));
            });

            ui.separator();

            // Cell state legend
            ui.label("Cell State Legend:");
            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::GRAY,
                );
                ui.label("Quiescent: Resting state");
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(50, 205, 50),
                );
                ui.label("Growing: Active growth");
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(255, 165, 0),
                );
                ui.label("Dividing: Mitosis");
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(138, 43, 226),
                );
                ui.label("Differentiating: Specializing");
            });

            ui.horizontal(|ui| {
                let rect_min = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_min, egui::vec2(16.0, 16.0)),
                    2.0,
                    Color32::from_rgb(139, 69, 19),
                );
                ui.label("Senescent: Aged state");
            });
        } else {
            ui.label("No cell data available");
        }
    }

    /// Show Gaia resonance tab
    fn show_gaia_resonance_tab(&mut self, ui: &mut Ui) {
        ui.heading("Cellular-Gaia Co-emergence");

        // Controls
        if self.show_controls {
            self.show_gaia_resonance_controls(ui);
            ui.separator();
        }

        // Render Gaia resonance using cached data
        self.render_gaia_resonance_visualization(ui);

        // Show Gaia explanation
        ui.add_space(10.0);
        ui.label("Simultaneous Emergence: Cells + Planetary Consciousness");
        ui.label("Cells and Gaia emerge from the same field");
        ui.label("Archetype patterns resonate across scales");
    }

    /// Show Gaia resonance controls
    fn show_gaia_resonance_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Gaia Resonance Controls", |ui| {
            ui.checkbox(
                &mut self.gaia_resonance_view.show_archetype_resonance,
                "Show Archetype Resonance",
            );
            ui.checkbox(
                &mut self.gaia_resonance_view.show_planetary_health,
                "Show Planetary Health",
            );
        });
    }

    /// Render Gaia resonance visualization
    fn render_gaia_resonance_visualization(&self, ui: &mut Ui) {
        let data = self.cached_gaia_data.as_ref();

        if let Some(gaia) = data {
            // Field overview
            ui.columns(2, |cols| {
                cols[0].label("Field Coherence:");
                cols[1].add(
                    egui::ProgressBar::new(gaia.coherence)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(50, 205, 50)),
                );

                cols[0].label("Biomass:");
                cols[1].label(format!("{:.0} cells", gaia.biomass));

                cols[0].label("Biodiversity:");
                cols[1].label(format!("{:.0} states", gaia.biodiversity));

                cols[0].label("Network Density:");
                cols[1].add(
                    egui::ProgressBar::new(gaia.network_density)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(100, 149, 237)),
                );
            });

            ui.separator();

            // Planetary health
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
                ui.add(
                    egui::ProgressBar::new((gaia.biodiversity / 10.0).min(1.0)).desired_width(80.0),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Network:");
                ui.add(egui::ProgressBar::new(gaia.network_density).desired_width(80.0));
            });

            ui.separator();

            // Archetype resonance
            ui.label("Archetype Resonance (A1-A22):");
            self.render_archetype_resonance_bars(ui);
        } else {
            ui.label("No Gaia data available");
        }
    }

    /// Render archetype resonance bars
    fn render_archetype_resonance_bars(&self, ui: &mut Ui) {
        let blueprint_data = self.cached_blueprint_data.as_ref();

        if let Some(blueprint) = blueprint_data {
            let bar_width = 15.0;

            egui::Grid::new("gaia_resonance_grid")
                .num_columns(11)
                .spacing([3.0, 3.0])
                .show(ui, |ui| {
                    for i in 0..22 {
                        if i > 0 && i % 11 == 0 {
                            ui.end_row();
                        }

                        let activation = blueprint.archetype_pattern.get(i).copied().unwrap_or(0.5);

                        let color = match i {
                            0..=6 => Color32::from_rgb(100, 149, 237),  // Mind - Blue
                            7..=13 => Color32::from_rgb(34, 139, 34),   // Body - Green
                            14..=20 => Color32::from_rgb(218, 165, 32), // Spirit - Gold
                            21 => Color32::from_rgb(255, 99, 71),       // Choice - Tomato
                            _ => Color32::GRAY,
                        };

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::ProgressBar::new(activation)
                                    .desired_width(bar_width)
                                    .fill(color),
                            );
                            ui.label(format!("{}", i + 1));
                        });
                    }
                });
        }
    }

    /// Toggle visibility
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get current entity
    pub fn current_entity(&self) -> Option<&SubSubLogos> {
        self.current_entity.as_ref()
    }

    /// Get selected cell type
    pub fn get_selected_cell_type(&self) -> &str {
        &self.selected_cell_type
    }

    /// Set selected cell type
    pub fn set_selected_cell_type(&mut self, cell_type: String) {
        self.selected_cell_type = cell_type;
    }
}

impl Default for CellularPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cellular_panel_creation() {
        let panel = CellularPanel::new();
        assert!(!panel.visible);
        assert_eq!(panel.view_mode, CellularViewMode::Blueprint);
    }

    #[test]
    fn test_view_mode_switching() {
        let mut panel = CellularPanel::new();
        panel.view_mode = CellularViewMode::GeneExpression;
        assert_eq!(panel.view_mode, CellularViewMode::GeneExpression);

        panel.view_mode = CellularViewMode::ProteinFolding;
        assert_eq!(panel.view_mode, CellularViewMode::ProteinFolding);

        panel.view_mode = CellularViewMode::CellManifestation;
        assert_eq!(panel.view_mode, CellularViewMode::CellManifestation);

        panel.view_mode = CellularViewMode::GaiaResonance;
        assert_eq!(panel.view_mode, CellularViewMode::GaiaResonance);
    }

    #[test]
    fn test_visibility_toggle() {
        let mut panel = CellularPanel::new();
        assert!(!panel.is_visible());

        panel.toggle_visibility();
        assert!(panel.is_visible());

        panel.set_visible(false);
        assert!(!panel.is_visible());
    }

    #[test]
    fn test_animation_update() {
        let mut panel = CellularPanel::new();
        panel.update(0.1);
        assert!(panel.animation_time > 0.0);
    }

    #[test]
    fn test_cell_type_selection() {
        let mut panel = CellularPanel::new();

        panel.set_selected_cell_type("Neuron".to_string());
        assert_eq!(panel.get_selected_cell_type(), "Neuron");

        panel.set_selected_cell_type("Cardiomyocyte".to_string());
        assert_eq!(panel.get_selected_cell_type(), "Cardiomyocyte");
    }

    #[test]
    fn test_all_view_modes_render() {
        let mut panel = CellularPanel::new();
        panel.visible = true;

        let modes = [
            CellularViewMode::Blueprint,
            CellularViewMode::GeneExpression,
            CellularViewMode::ProteinFolding,
            CellularViewMode::CellManifestation,
            CellularViewMode::GaiaResonance,
        ];

        for mode in modes {
            panel.view_mode = mode;
            assert_eq!(panel.view_mode, mode);
        }
    }

    #[test]
    fn test_cached_data_generation() {
        let mut panel = CellularPanel::new();

        // Create a mock entity using create_test_entity
        let entity = SubSubLogos::create_test_entity();

        panel.update_from_entity(&entity);

        // Check that cached data was generated
        assert!(panel.cached_blueprint_data.is_some());
        assert!(panel.cached_gene_data.is_some());
        assert!(panel.cached_protein_data.is_some());
        assert!(panel.cached_cell_data.is_some());
        assert!(panel.cached_gaia_data.is_some());
    }
}
