//! Molecular Panel - Molecular Level Visualization Panel
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.3:
//! "Design molecular visualization from archetype patterns, Implement bond visualization
//! with type-based colors, Show molecular geometry as field interference minima,
//! Visualize functional groups as archetype signatures, Simultaneous molecular/planetary emergence"
//!
//! This module provides:
//! - Tabbed interface for different molecular views
//! - Bond visualization from archetype interference patterns
//! - Molecular geometry showing VSEPR as field interference minima
//! - Functional group archetype pattern radar charts
//! - Simultaneous molecular/planetary emergence visualization
//! - Entity integration for molecular property display

use egui::{Context, Ui};
use std::collections::HashMap;

use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::visualization::molecular_viz::{
    BondRenderer, FunctionalGroupVisualizer, GeometryRenderData, MolecularColors,
    MolecularManifestationView, ReactivityRenderData,
};
use crate::holographic_foundation::field_state::{HolographicFieldState, Position3D};
use crate::holographic_foundation::molecular_emergence::functional_groups::ReactivityProfile;
use crate::holographic_foundation::molecular_emergence::simultaneous_emergence::{
    MolecularManifestation, MolecularPlanetaryPair, PlanetaryEmergence,
};

/// View mode for molecular panel tabs
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MolecularViewMode {
    /// Bond visualization from archetype interference
    #[default]
    Bonds,
    /// Molecular geometry as field interference minima
    Geometry,
    /// Functional group archetype signatures
    FunctionalGroups,
    /// Simultaneous molecular/planetary manifestation
    Manifestation,
}

/// Molecular Panel for Phase C.3
///
/// Provides integrated molecular visualization with tabbed interface for:
/// - Bonds: Chemical bonds derived from archetype interference patterns
/// - Geometry: VSEPR as field interference minima
/// - FunctionalGroups: Archetype pattern radar charts
/// - Manifestation: Simultaneous molecular and planetary emergence
pub struct MolecularPanel {
    /// Panel visibility toggle
    pub visible: bool,

    /// Current tab/view mode
    pub view_mode: MolecularViewMode,

    /// Bond renderer for chemical bond visualization
    bond_renderer: BondRenderer,

    /// Molecular geometry view
    geometry_view: MolecularGeometryViewWrapper,

    /// Functional group visualizer
    functional_group_visualizer: FunctionalGroupVisualizer,

    /// Molecular manifestation view
    manifestation_view: MolecularManifestationView,

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

    /// Selected molecule formula for demo
    selected_molecule: String,

    /// Selected geometry data
    selected_geometry: GeometryRenderData,

    /// Field-derived molecule manifestation
    field_molecule: Option<MolecularManifestation>,

    /// Field-derived molecule-planet pair
    field_pair: Option<MolecularPlanetaryPair>,

    /// Field-derived bond data
    field_bond_data: Option<crate::gui::visualization::molecular_viz::BondRenderData>,

    /// Field-derived geometry data
    field_geometry: Option<GeometryRenderData>,

    /// Field-derived functional group data
    field_functional_group:
        Option<crate::gui::visualization::molecular_viz::FunctionalGroupRenderData>,
}

/// Wrapper for MolecularGeometryView to provide update method
#[derive(Debug, Clone)]
struct MolecularGeometryViewWrapper {
    /// Show interference minima
    pub show_interference_minima: bool,
    /// Show bond angles
    pub show_bond_angles: bool,
    /// Show shape name
    pub show_shape_name: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for MolecularGeometryViewWrapper {
    fn default() -> Self {
        Self {
            show_interference_minima: true,
            show_bond_angles: true,
            show_shape_name: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl MolecularGeometryViewWrapper {
    /// Create a new geometry view wrapper
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render molecular geometry
    pub fn render(&self, ui: &mut Ui, data: &GeometryRenderData) {
        // Simplified rendering using available data
        ui.horizontal(|ui| {
            ui.label("Shape:");
            let shape_name = match data.shape {
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::Linear => "Linear",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::Bent => "Bent",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::TrigonalPlanar => "Trigonal Planar",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::TrigonalPyramidal => "Trigonal Pyramidal",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::Tetrahedral => "Tetrahedral",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::SquarePlanar => "Square Planar",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::SquarePyramidal => "Square Pyramidal",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::TrigonalBipyramidal => "Trigonal Bipyramidal",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::Octahedral => "Octahedral",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::Seesaw => "Seesaw",
                crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::TShaped => "T-Shaped",
                _ => "Unknown",
            };
            ui.label(shape_name);
        });

        ui.label(format!(
            "Steric Number: {} | Bonding Pairs: {} | Lone Pairs: {}",
            data.steric_number, data.bonding_pairs, data.lone_pairs
        ));

        if !data.bond_angles.is_empty() {
            ui.label(format!("Bond Angles: {:?}", data.bond_angles));
        }

        ui.label(format!("Confidence: {:.1}%", data.confidence * 100.0));
    }
}

impl MolecularPanel {
    /// Create a new molecular panel
    pub fn new() -> Self {
        Self {
            visible: true,
            view_mode: MolecularViewMode::Bonds,
            bond_renderer: BondRenderer::new(),
            geometry_view: MolecularGeometryViewWrapper::new(),
            functional_group_visualizer: FunctionalGroupVisualizer::new(),
            manifestation_view: MolecularManifestationView::new(),
            animation_time: 0.0,
            show_controls: true,
            current_entity: None,
            entity_cache: HashMap::new(),
            selected_entity_id: None,
            selected_molecule: "H2O".to_string(),
            selected_geometry: GeometryRenderData {
                shape: crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape::Bent,
                bond_angles: vec![104.5],
                steric_number: 4,
                bonding_pairs: 2,
                lone_pairs: 2,
                confidence: 0.95,
                interference_minima_count: 4,
            },
            field_molecule: None,
            field_pair: None,
            field_bond_data: None,
            field_geometry: None,
            field_functional_group: None,
        }
    }

    /// Update from entity - extract molecular-relevant data
    pub fn update_from_entity(&mut self, entity: &SubSubLogos) {
        self.current_entity = Some(entity.clone());
        self.selected_entity_id = Some(entity.entity_id.as_u64());

        // Cache entity
        self.entity_cache
            .insert(entity.entity_id.as_u64(), entity.clone());

        // Map entity state to molecule type (for visualization)
        self.selected_molecule = self.map_entity_to_molecule(entity);
    }

    /// Map entity state to a molecule formula for visualization
    fn map_entity_to_molecule(&self, entity: &SubSubLogos) -> String {
        // Use archetype activations to determine a representative molecule
        // This is a heuristic mapping - in practice, would be more sophisticated
        let archetype_sum: f64 = entity.archetype_activations.iter().sum();

        // Map to common molecules
        let molecules = ["H2O", "CO2", "CH4", "NH3", "H2", "O2", "N2", "C2H6"];
        let idx = ((archetype_sum * 10.0) as usize) % molecules.len();

        molecules[idx].to_string()
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
            self.refresh_from_field(field_state, &focus);
        }

        // Update all visualization components
        self.bond_renderer.update(delta_time);
        self.geometry_view.update(delta_time);
        self.functional_group_visualizer.update(delta_time);
        self.manifestation_view.update(delta_time);
    }

    fn refresh_from_field(&mut self, field_state: &HolographicFieldState, focus: &Position3D) {
        let Some(planet) = PlanetaryEmergence::from_field(field_state, *focus, 0.0) else {
            return;
        };

        let dominant_element = planet
            .dominant_elements()
            .first()
            .map(|(z, _)| *z)
            .unwrap_or(8);

        let molecule = if dominant_element == 6 {
            MolecularManifestation::methane(*focus)
        } else {
            MolecularManifestation::water(*focus)
        };

        let pair = MolecularPlanetaryPair::new(molecule.clone(), planet);

        self.selected_molecule = self.formula_from_molecule(&molecule);
        self.field_geometry = Some(GeometryRenderData::from(&molecule.geometry));
        self.selected_geometry = self
            .field_geometry
            .clone()
            .unwrap_or_else(|| self.create_geometry_for_molecule(&self.selected_molecule));

        self.field_bond_data = molecule.bonds.first().map(|bond| bond.into());

        let fg = &molecule.functional_groups;
        let fg_data = if let Some(group) = fg.dominant_group() {
            crate::gui::visualization::molecular_viz::FunctionalGroupRenderData {
                group_name: group.name().to_string(),
                formula: group.formula().to_string(),
                archetype_pattern: group.archetype_signature().map(|v| v as f32),
                reactivity: ReactivityRenderData::from(&ReactivityProfile::for_group(group)),
            }
        } else {
            crate::gui::visualization::molecular_viz::FunctionalGroupRenderData {
                group_name: "Unresolved Group".to_string(),
                formula: self.selected_molecule.clone(),
                archetype_pattern: molecule.archetype_pattern.map(|v| v as f32),
                reactivity: ReactivityRenderData {
                    electrophilicity: 0.5,
                    nucleophilicity: 0.5,
                    acidity: 0.5,
                    basicity: 0.5,
                    dominant_type: "Balanced".to_string(),
                },
            }
        };

        self.field_functional_group = Some(fg_data);
        self.field_pair = Some(pair);
        self.field_molecule = Some(molecule);
    }

    fn formula_from_molecule(&self, molecule: &MolecularManifestation) -> String {
        use std::collections::BTreeMap;

        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for element in &molecule.elements {
            let symbol = element.symbol().to_string();
            *counts.entry(symbol).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .map(|(symbol, count)| {
                if count > 1 {
                    format!("{}{}", symbol, count)
                } else {
                    symbol
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Show the panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("🧬 Molecular Level")
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

        // Molecule selector
        self.show_molecule_selector(ui);
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
            MolecularViewMode::Bonds => {
                self.show_bonds_tab(ui);
            }
            MolecularViewMode::Geometry => {
                self.show_geometry_tab(ui);
            }
            MolecularViewMode::FunctionalGroups => {
                self.show_functional_groups_tab(ui);
            }
            MolecularViewMode::Manifestation => {
                self.show_manifestation_tab(ui);
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
                ui.label("No entity selected");
                return;
            }

            // Simple selector using radio buttons
            for &id in &entity_ids {
                let label = format!("Entity {}", id);
                ui.selectable_value(&mut self.selected_entity_id, Some(id), &label);
            }
        });
    }

    /// Show molecule selector
    fn show_molecule_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Molecule:");

            let molecules = [
                "H2O", "CO2", "CH4", "NH3", "H2", "O2", "N2", "C2H6", "C2H4", "C6H6",
            ];

            for mol in molecules {
                ui.selectable_value(&mut self.selected_molecule, mol.to_string(), mol);
            }
        });

        // Update geometry based on selected molecule
        if self.field_geometry.is_none() {
            self.selected_geometry = self.create_geometry_for_molecule(&self.selected_molecule);
        }
    }

    /// Show tab navigation
    fn show_tab_navigation(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.view_mode, MolecularViewMode::Bonds, "🔗 Bonds");
            ui.selectable_value(
                &mut self.view_mode,
                MolecularViewMode::Geometry,
                "📐 Geometry",
            );
            ui.selectable_value(
                &mut self.view_mode,
                MolecularViewMode::FunctionalGroups,
                "⚡ Groups",
            );
            ui.selectable_value(
                &mut self.view_mode,
                MolecularViewMode::Manifestation,
                "🌍 Manifestation",
            );
        });
    }

    /// Show bonds tab
    fn show_bonds_tab(&mut self, ui: &mut Ui) {
        ui.heading("Bond Visualization");

        // Controls
        if self.show_controls {
            self.show_bond_controls(ui);
            ui.separator();
        }

        // Create demo bond data
        let bond_data = self
            .field_bond_data
            .clone()
            .unwrap_or_else(|| self.create_demo_bond_data());

        // Render bond visualization
        self.bond_renderer.render(ui, &bond_data);

        // Show bond legend
        ui.add_space(10.0);
        self.show_bond_legend(ui);

        // Show bond info
        ui.add_space(10.0);
        ui.label(format!(
            "Archetype Similarity: {:.1}%",
            bond_data.archetype_similarity * 100.0
        ));
        ui.label(format!("Bond Polarity: {:.2}", bond_data.polarity));
        ui.label(format!(
            "Bond Stability: {:.1}%",
            bond_data.stability * 100.0
        ));
    }

    /// Show bond controls
    fn show_bond_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Bond Controls", |ui| {
            ui.checkbox(&mut self.bond_renderer.show_bond_types, "Show Bond Types");
            ui.checkbox(&mut self.bond_renderer.show_bond_order, "Show Bond Order");
            ui.checkbox(&mut self.bond_renderer.show_similarity, "Show Similarity");
            ui.checkbox(
                &mut self.bond_renderer.show_interference,
                "Show Interference",
            );

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.bond_renderer.scale, 0.5..=2.0).text(""));

            ui.label("Animation:");
            ui.add(egui::Slider::new(&mut self.bond_renderer.animation_time, 0.0..=10.0).text(""));
        });
    }

    /// Show bond legend
    fn show_bond_legend(&self, ui: &mut Ui) {
        ui.collapsing("Bond Legend", |ui| {
            let bonds = [
                ("Covalent", MolecularColors::covalent()),
                ("Ionic", MolecularColors::ionic()),
                ("Metallic", MolecularColors::metallic()),
                ("Hydrogen", MolecularColors::hydrogen()),
                ("Van der Waals", MolecularColors::van_der_waals()),
                ("Coordinate", MolecularColors::coordinate()),
                ("Aromatic", MolecularColors::aromatic()),
            ];

            ui.columns(2, |columns| {
                let mut col_idx = 0;
                for (name, color) in bonds.iter() {
                    columns[col_idx].horizontal(|ui| {
                        ui.add_sized(
                            [20.0, 16.0],
                            egui::Label::new(egui::RichText::new("■").color(*color)),
                        );
                        ui.label(*name);
                    });
                    col_idx = (col_idx + 1) % 2;
                }
            });
        });
    }

    /// Show geometry tab
    fn show_geometry_tab(&mut self, ui: &mut Ui) {
        ui.heading("Molecular Geometry (VSEPR)");

        // Controls
        if self.show_controls {
            self.show_geometry_controls(ui);
            ui.separator();
        }

        // Render geometry visualization
        self.geometry_view.render(ui, &self.selected_geometry);

        // Show geometry explanation
        ui.add_space(10.0);
        ui.label("Geometry from Field Interference Minima:");
        ui.label("VSEPR emerges from archetype field interference patterns");
        ui.label("Bond angles determined by interference minima positions");
    }

    /// Show geometry controls
    fn show_geometry_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Geometry Controls", |ui| {
            ui.checkbox(
                &mut self.geometry_view.show_interference_minima,
                "Show Interference Minima",
            );
            ui.checkbox(&mut self.geometry_view.show_bond_angles, "Show Bond Angles");
            ui.checkbox(&mut self.geometry_view.show_shape_name, "Show Shape Name");

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.geometry_view.scale, 0.5..=2.0).text(""));
        });
    }

    /// Show functional groups tab
    fn show_functional_groups_tab(&mut self, ui: &mut Ui) {
        ui.heading("Functional Groups");

        // Controls
        if self.show_controls {
            self.show_functional_group_controls(ui);
            ui.separator();
        }

        // Create demo functional group data
        let fg_data = self
            .field_functional_group
            .clone()
            .unwrap_or_else(|| self.create_demo_functional_group_data());

        // Render functional group visualization
        self.functional_group_visualizer.render(ui, &fg_data);

        // Show functional group info
        ui.add_space(10.0);
        ui.label(format!("Formula: {}", fg_data.formula));
        ui.label(format!("Reactivity: {}", fg_data.reactivity.dominant_type));
    }

    /// Show functional group controls
    fn show_functional_group_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Functional Group Controls", |ui| {
            ui.checkbox(
                &mut self.functional_group_visualizer.show_radar,
                "Show Radar Chart",
            );
            ui.checkbox(
                &mut self.functional_group_visualizer.show_reactivity,
                "Show Reactivity",
            );
            ui.checkbox(
                &mut self.functional_group_visualizer.show_archetype_labels,
                "Show Archetype Labels",
            );

            ui.label("Scale:");
            ui.add(
                egui::Slider::new(&mut self.functional_group_visualizer.scale, 0.5..=2.0).text(""),
            );
        });
    }

    /// Show manifestation tab
    fn show_manifestation_tab(&mut self, ui: &mut Ui) {
        ui.heading("Simultaneous Molecular/Planetary Emergence");

        // Controls
        if self.show_controls {
            self.show_manifestation_controls(ui);
            ui.separator();
        }

        // Create demo manifestation pair
        let pair = self
            .field_pair
            .clone()
            .unwrap_or_else(|| self.create_demo_manifestation_pair());

        // Render manifestation visualization
        self.manifestation_view
            .render_molecule_planet_pair(ui, &pair);

        // Show manifestation explanation
        ui.add_space(10.0);
        ui.label("Molecular-Planetary Correspondence:");
        ui.label("Molecules and planets emerge simultaneously from the same field");
        ui.label("Archetype patterns resonate across scales");
    }

    /// Show manifestation controls
    fn show_manifestation_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Manifestation Controls", |ui| {
            ui.checkbox(&mut self.manifestation_view.show_molecule, "Show Molecule");
            ui.checkbox(&mut self.manifestation_view.show_planet, "Show Planet");
            ui.checkbox(
                &mut self.manifestation_view.show_resonance,
                "Show Resonance",
            );

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.manifestation_view.scale, 0.5..=2.0).text(""));
        });
    }

    /// Create demo bond data
    fn create_demo_bond_data(&self) -> crate::gui::visualization::molecular_viz::BondRenderData {
        use crate::gui::visualization::molecular_viz::BondRenderData;
        use crate::holographic_foundation::atomic_emergence::element_attractor::ElementIdentity;
        use crate::holographic_foundation::molecular_emergence::bond_formation::{
            BondOrder, BondType,
        };

        // Create demo bond based on selected molecule
        let (elem1_atomic, elem2_atomic, bond_type, order) = match self.selected_molecule.as_str() {
            "H2" => (1, 1, BondType::Covalent, BondOrder::Single),
            "O2" => (8, 8, BondType::Covalent, BondOrder::Double),
            "N2" => (7, 7, BondType::Covalent, BondOrder::Triple),
            "CO2" => (6, 8, BondType::Covalent, BondOrder::Double),
            "H2O" => (8, 1, BondType::Covalent, BondOrder::Single),
            "CH4" => (6, 1, BondType::Covalent, BondOrder::Single),
            "NH3" => (7, 1, BondType::Covalent, BondOrder::Single),
            "C2H4" => (6, 6, BondType::Covalent, BondOrder::Double),
            "C6H6" => (6, 6, BondType::Aromatic, BondOrder::Aromatic),
            _ => (6, 1, BondType::Covalent, BondOrder::Single),
        };

        let elem1 = ElementIdentity::from_atomic_number(elem1_atomic);
        let elem2 = ElementIdentity::from_atomic_number(elem2_atomic);

        // Create BondRenderData directly with the data we have
        BondRenderData {
            element1_symbol: elem1.symbol().to_string(),
            element2_symbol: elem2.symbol().to_string(),
            bond_type,
            bond_order: order,
            length: 1.0 + (elem1_atomic as f32 - elem2_atomic as f32).abs() * 0.1,
            energy: match order {
                BondOrder::Single => 350.0,
                BondOrder::Double => 600.0,
                BondOrder::Triple => 800.0,
                _ => 400.0,
            },
            archetype_similarity: 0.7 + (elem1_atomic as f32 * 0.01).min(0.25),
            polarity: (elem1_atomic as f32 - elem2_atomic as f32).abs() * 0.15,
            stability: 0.85,
        }
    }

    /// Create geometry data for a molecule
    fn create_geometry_for_molecule(&self, molecule: &str) -> GeometryRenderData {
        use crate::holographic_foundation::molecular_emergence::molecular_geometry::MolecularShape;

        match molecule {
            "H2O" => GeometryRenderData {
                shape: MolecularShape::Bent,
                bond_angles: vec![104.5],
                steric_number: 4,
                bonding_pairs: 2,
                lone_pairs: 2,
                confidence: 0.95,
                interference_minima_count: 4,
            },
            "CO2" => GeometryRenderData {
                shape: MolecularShape::Linear,
                bond_angles: vec![180.0],
                steric_number: 2,
                bonding_pairs: 2,
                lone_pairs: 0,
                confidence: 0.98,
                interference_minima_count: 2,
            },
            "CH4" => GeometryRenderData {
                shape: MolecularShape::Tetrahedral,
                bond_angles: vec![109.5],
                steric_number: 4,
                bonding_pairs: 4,
                lone_pairs: 0,
                confidence: 0.98,
                interference_minima_count: 4,
            },
            "NH3" => GeometryRenderData {
                shape: MolecularShape::TrigonalPyramidal,
                bond_angles: vec![107.0],
                steric_number: 4,
                bonding_pairs: 3,
                lone_pairs: 1,
                confidence: 0.95,
                interference_minima_count: 4,
            },
            "C2H4" => GeometryRenderData {
                shape: MolecularShape::TrigonalPlanar,
                bond_angles: vec![120.0],
                steric_number: 3,
                bonding_pairs: 3,
                lone_pairs: 0,
                confidence: 0.95,
                interference_minima_count: 3,
            },
            "C6H6" => GeometryRenderData {
                shape: MolecularShape::TrigonalPlanar,
                bond_angles: vec![120.0],
                steric_number: 3,
                bonding_pairs: 3,
                lone_pairs: 0,
                confidence: 0.97,
                interference_minima_count: 6,
            },
            _ => GeometryRenderData {
                shape: MolecularShape::Unknown,
                bond_angles: vec![],
                steric_number: 0,
                bonding_pairs: 0,
                lone_pairs: 0,
                confidence: 0.0,
                interference_minima_count: 0,
            },
        }
    }

    /// Create demo functional group data
    fn create_demo_functional_group_data(
        &self,
    ) -> crate::gui::visualization::molecular_viz::FunctionalGroupRenderData {
        use crate::gui::visualization::molecular_viz::FunctionalGroupRenderData;
        use crate::gui::visualization::molecular_viz::ReactivityRenderData;
        use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;

        // Create a sample archetype pattern
        let mut pattern = [0.0; NUM_ARCHETYPES];
        for (i, val) in pattern.iter_mut().enumerate() {
            *val = ((i as f32 * 0.3).sin().abs() + 0.3).min(1.0);
        }

        let (name, formula) = match self.selected_molecule.as_str() {
            "H2O" => ("Hydroxyl Group", "-OH"),
            "CO2" => ("Carbonyl Group", "C=O"),
            "CH4" => ("Methyl Group", "-CH3"),
            "NH3" => ("Amino Group", "-NH2"),
            "C2H4" => ("Alkene Group", "C=C"),
            "C6H6" => ("Aromatic Ring", "C₆H₆"),
            _ => ("Unknown Group", ""),
        };

        FunctionalGroupRenderData {
            group_name: name.to_string(),
            formula: formula.to_string(),
            archetype_pattern: pattern,
            reactivity: ReactivityRenderData {
                electrophilicity: 0.4,
                nucleophilicity: 0.6,
                acidity: 0.5,
                basicity: 0.3,
                dominant_type: "Nucleophilic".to_string(),
            },
        }
    }

    /// Create demo manifestation pair
    fn create_demo_manifestation_pair(&self) -> crate::holographic_foundation::molecular_emergence::simultaneous_emergence::MolecularPlanetaryPair{
        use crate::holographic_foundation::field_state::Position3D;
        use crate::holographic_foundation::molecular_emergence::simultaneous_emergence::{
            MolecularManifestation, MolecularPlanetaryPair, PlanetType, PlanetaryEmergence,
        };

        // Create position for demo
        let position = Position3D::new(0.0, 0.0, 0.0);

        // Create molecule based on selection (only water and methane have constructors)
        let molecule = match self.selected_molecule.as_str() {
            "H2O" | "CO2" | "NH3" | "O2" | "N2" | "H2" | "C2H4" | "C6H6" => {
                MolecularManifestation::water(position)
            }
            _ => MolecularManifestation::water(position),
        };

        // Create a corresponding planet
        let planet = PlanetaryEmergence::new(PlanetType::Terrestrial, position, 0.75, 0.0);

        // Create the pair
        MolecularPlanetaryPair::new(molecule, planet)
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

    /// Get selected molecule
    pub fn get_selected_molecule(&self) -> &str {
        &self.selected_molecule
    }

    /// Set selected molecule
    pub fn set_selected_molecule(&mut self, molecule: String) {
        self.selected_molecule = molecule;
    }
}

impl Default for MolecularPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molecular_panel_creation() {
        let panel = MolecularPanel::new();
        assert!(!panel.visible);
        assert_eq!(panel.view_mode, MolecularViewMode::Bonds);
    }

    #[test]
    fn test_view_mode_switching() {
        let mut panel = MolecularPanel::new();
        panel.view_mode = MolecularViewMode::Geometry;
        assert_eq!(panel.view_mode, MolecularViewMode::Geometry);

        panel.view_mode = MolecularViewMode::FunctionalGroups;
        assert_eq!(panel.view_mode, MolecularViewMode::FunctionalGroups);

        panel.view_mode = MolecularViewMode::Manifestation;
        assert_eq!(panel.view_mode, MolecularViewMode::Manifestation);
    }

    #[test]
    fn test_visibility_toggle() {
        let mut panel = MolecularPanel::new();
        assert!(!panel.is_visible());

        panel.toggle_visibility();
        assert!(panel.is_visible());

        panel.set_visible(false);
        assert!(!panel.is_visible());
    }

    #[test]
    fn test_animation_update() {
        let mut panel = MolecularPanel::new();
        panel.update(0.1);
        assert!(panel.animation_time > 0.0);
    }

    #[test]
    fn test_molecule_selection() {
        let mut panel = MolecularPanel::new();

        panel.set_selected_molecule("CO2".to_string());
        assert_eq!(panel.get_selected_molecule(), "CO2");

        panel.set_selected_molecule("CH4".to_string());
        assert_eq!(panel.get_selected_molecule(), "CH4");
    }

    #[test]
    fn test_geometry_creation() {
        let panel = MolecularPanel::new();

        let h2o_geometry = panel.create_geometry_for_molecule("H2O");
        assert_eq!(h2o_geometry.bonding_pairs, 2);
        assert_eq!(h2o_geometry.lone_pairs, 2);

        let co2_geometry = panel.create_geometry_for_molecule("CO2");
        assert_eq!(co2_geometry.bonding_pairs, 2);
        assert_eq!(co2_geometry.lone_pairs, 0);
    }
}
