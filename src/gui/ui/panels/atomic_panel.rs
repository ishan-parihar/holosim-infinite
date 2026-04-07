//! Atomic Panel - Atomic Level Visualization Panel
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.2:
//! "Render atomic attractor fields as holographic topologies,
//!  Show orbital shell emergence from attractor basins,
//!  Visualize atomic mass and charge as field geometry,
//!  Implement electron probability distributions as field ripples,
//!  Create periodic table landscape from attractor field relationships"
//!
//! This module provides:
//! - Tabbed interface for different atomic views
//! - Orbital shell visualization from attractor basins
//! - Attractor field holographic topology display
//! - Mass and charge derivation from field geometry
//! - Particle signature view with probability distributions
//! - Periodic table landscape from field relationships
//! - Entity integration for atomic property display

use egui::{Color32, Context, Ui};
use std::collections::HashMap;

use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::visualization::atomic_viz::{
    AttractorFieldVisualizer, MassChargeDerivation, MassChargeDerivationData, OrbitalRenderer,
    ParticleSignatureData, ParticleSignatureView, PeriodicTableLandscape,
};
use crate::holographic_foundation::atomic_emergence::atomic_manifestation::AtomicManifestation;
use crate::holographic_foundation::atomic_emergence::attractor_field::AttractorField;
use crate::holographic_foundation::atomic_emergence::particle_derivation::{
    DerivationFactors, ParticleArchetypePattern, ParticleProperties, ParticleType,
};
use crate::holographic_foundation::atomic_emergence::ElementAttractorField;
use crate::holographic_foundation::field_state::{HolographicFieldState, Position3D};
use crate::holographic_foundation::quantum_consciousness::quantum_numbers::{
    QuantumNumberSet, Spin,
};

/// View mode for atomic panel tabs
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AtomicViewMode {
    /// Orbital shell visualization
    #[default]
    Orbitals,
    /// Attractor field topology
    AttractorFields,
    /// Particle signatures
    Particles,
    /// Mass/charge derivation
    Derivation,
    /// Periodic table landscape
    PeriodicTable,
}

/// Atomic Panel for Phase C.2
///
/// Provides integrated atomic visualization with tabbed interface for:
/// - Orbitals: Shell emergence from attractor basins
/// - Attractor Fields: Holographic topologies
/// - Particles: Electron probability distributions
/// - Derivation: Mass and charge from field geometry
/// - Periodic Table: Landscape from field relationships
pub struct AtomicPanel {
    /// Panel visibility toggle
    pub visible: bool,

    /// Current tab/view mode
    pub view_mode: AtomicViewMode,

    /// Orbital renderer
    orbital_renderer: OrbitalRenderer,

    /// Attractor field visualizer
    attractor_visualizer: AttractorFieldVisualizer,

    /// Mass/charge derivation display
    mass_charge_derivation: MassChargeDerivation,

    /// Particle signature view
    particle_signature_view: ParticleSignatureView,

    /// Periodic table landscape
    periodic_table_landscape: PeriodicTableLandscape,

    /// Selected atomic number (1-118)
    selected_atomic_number: u32,

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

    /// Field-derived element for primary emergence rendering path
    field_element: Option<ElementAttractorField>,

    /// Field-derived orbital quantum numbers
    field_quantum_numbers: Option<QuantumNumberSet>,

    /// Field-derived particle signature
    field_particle_signature: Option<ParticleSignatureData>,

    /// Field-derived mass/charge derivation data
    field_derivation_data: Option<MassChargeDerivationData>,

    /// Field-derived particle type for derivation display
    field_particle_type: Option<ParticleType>,
}

impl AtomicPanel {
    /// Create a new atomic panel
    pub fn new() -> Self {
        Self {
            visible: true,
            view_mode: AtomicViewMode::Orbitals,
            orbital_renderer: OrbitalRenderer::new(),
            attractor_visualizer: AttractorFieldVisualizer::new(),
            mass_charge_derivation: MassChargeDerivation::new(),
            particle_signature_view: ParticleSignatureView::new(),
            periodic_table_landscape: PeriodicTableLandscape::new(),
            selected_atomic_number: 1, // Hydrogen by default
            animation_time: 0.0,
            show_controls: true,
            current_entity: None,
            entity_cache: HashMap::new(),
            selected_entity_id: None,
            field_element: None,
            field_quantum_numbers: None,
            field_particle_signature: None,
            field_derivation_data: None,
            field_particle_type: None,
        }
    }

    /// Update from entity - extract atomic-relevant data
    pub fn update_from_entity(&mut self, entity: &SubSubLogos) {
        self.current_entity = Some(entity.clone());
        self.selected_entity_id = Some(entity.entity_id.as_u64());

        // Cache entity
        self.entity_cache
            .insert(entity.entity_id.as_u64(), entity.clone());

        // Map entity state to atomic number (for visualization)
        self.selected_atomic_number = self.map_entity_to_atomic_number(entity);
    }

    /// Map entity state to an atomic number for visualization
    fn map_entity_to_atomic_number(&self, entity: &SubSubLogos) -> u32 {
        // Use the entity's archetype activations to determine a representative atomic number
        // This is a heuristic mapping - in practice, the mapping would be more sophisticated
        let archetype_sum: f64 = entity.archetype_activations.iter().sum();

        // Map to 1-118 range
        let normalized = archetype_sum / 22.0; // Average activation
        let atomic_num = ((normalized * 10.0).floor() as u32) % 118;

        atomic_num.clamp(1, 118)
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
        self.orbital_renderer.update(delta_time);
        self.attractor_visualizer.update(delta_time);
        self.mass_charge_derivation.update(delta_time);
        self.particle_signature_view.update(delta_time);
        self.periodic_table_landscape.update(delta_time);
    }

    fn refresh_from_field(&mut self, field_state: &HolographicFieldState, focus: &Position3D) {
        let manifestation = AtomicManifestation::from_field(field_state, *focus);
        let (element, quantum_numbers) = if let Some(manifestation) = manifestation {
            (
                manifestation.element().clone(),
                *manifestation.element().quantum_numbers(),
            )
        } else if let Some(attractor) = AttractorField::from_field_state(field_state, focus) {
            let qn = *attractor.quantum_numbers();
            let estimated_atomic_number =
                ((qn.n as f64 * attractor.configuration().coherence * 10.0) as u32).clamp(1, 118);
            (
                ElementAttractorField::from_atomic_number(estimated_atomic_number),
                qn,
            )
        } else {
            return;
        };

        self.selected_atomic_number = element.atomic_number();
        self.field_quantum_numbers = Some(quantum_numbers);
        self.field_element = Some(element.clone());

        let archetype = element.configuration().archetype_vector;
        let mind_dominance = archetype[0..7].iter().sum::<f64>() / 7.0;
        let body_balance = archetype[7..14].iter().sum::<f64>() / 7.0;
        let spirit_dominance = archetype[14..21].iter().sum::<f64>() / 7.0;
        let charge = element.charge().net_charge;

        let particle_type = if charge > 0.05 {
            ParticleType::Proton
        } else if charge < -0.05 {
            ParticleType::Electron
        } else {
            ParticleType::Neutron
        };

        let derivation_factors = DerivationFactors {
            mind_dominance,
            spirit_dominance,
            body_balance,
            coherence_depth: element.configuration().coherence,
            charge_polarity: charge.clamp(-1.0, 1.0),
        };

        self.field_derivation_data =
            Some(MassChargeDerivationData::from_factors(&derivation_factors));
        self.field_particle_type = Some(particle_type);

        let particle_pattern = ParticleArchetypePattern::pattern_for_particle(particle_type);
        let properties = ParticleProperties::derive_from_archetype(particle_type, particle_pattern);
        self.field_particle_signature = Some(ParticleSignatureData::from_properties(&properties));
    }

    /// Show the panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("⚛️ Atomic Level")
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

        // Atomic number selector
        self.show_atomic_number_selector(ui);
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
            AtomicViewMode::Orbitals => {
                self.show_orbitals_tab(ui);
            }
            AtomicViewMode::AttractorFields => {
                self.show_attractor_fields_tab(ui);
            }
            AtomicViewMode::Particles => {
                self.show_particles_tab(ui);
            }
            AtomicViewMode::Derivation => {
                self.show_derivation_tab(ui);
            }
            AtomicViewMode::PeriodicTable => {
                self.show_periodic_table_tab(ui);
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

    /// Show atomic number selector
    fn show_atomic_number_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Atomic:");
            ui.add(egui::Slider::new(&mut self.selected_atomic_number, 1..=118));

            // Show element symbol
            if let Some(symbol) = self.get_element_symbol(self.selected_atomic_number) {
                ui.label(format!("({})", symbol));
            }
        });

        // Show element name
        if let Some(name) = self.get_element_name(self.selected_atomic_number) {
            ui.label(name);
        }
    }

    /// Show tab navigation
    fn show_tab_navigation(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.view_mode, AtomicViewMode::Orbitals, "🌐 Orbitals");
            ui.selectable_value(
                &mut self.view_mode,
                AtomicViewMode::AttractorFields,
                "🌀 Attractor",
            );
            ui.selectable_value(
                &mut self.view_mode,
                AtomicViewMode::Particles,
                "✨ Particles",
            );
            ui.selectable_value(
                &mut self.view_mode,
                AtomicViewMode::Derivation,
                "📊 Derivation",
            );
            ui.selectable_value(
                &mut self.view_mode,
                AtomicViewMode::PeriodicTable,
                "📋 Periodic Table",
            );
        });
    }

    /// Show orbitals tab
    fn show_orbitals_tab(&mut self, ui: &mut Ui) {
        ui.heading("Orbital Shell Emergence");

        // Controls
        if self.show_controls {
            self.show_orbital_controls(ui);
            ui.separator();
        }

        // Create demo quantum numbers based on atomic number
        let quantum_numbers = self
            .field_quantum_numbers
            .unwrap_or_else(|| self.generate_quantum_numbers(self.selected_atomic_number));

        // Render orbital visualization
        self.orbital_renderer.render(ui, &quantum_numbers);

        // Show orbital legend
        ui.add_space(10.0);
        self.show_orbital_legend(ui);

        // Show quantum state info
        ui.add_space(10.0);
        ui.label(format!(
            "Current State: {}",
            quantum_numbers.to_spectroscopic_notation()
        ));
    }

    /// Show orbital controls
    fn show_orbital_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Orbital Controls", |ui| {
            ui.checkbox(&mut self.orbital_renderer.show_lobes, "Show Orbital Lobes");
            ui.checkbox(
                &mut self.orbital_renderer.show_probability_cloud,
                "Probability Cloud",
            );
            ui.checkbox(
                &mut self.orbital_renderer.show_quantum_numbers,
                "Show Quantum Numbers",
            );

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.orbital_renderer.scale, 0.5..=2.0).text(""));

            ui.label("Rotation:");
            ui.add(
                egui::Slider::new(
                    &mut self.orbital_renderer.rotation,
                    0.0..=std::f32::consts::TAU,
                )
                .text(""),
            );
        });
    }

    /// Show attractor fields tab
    fn show_attractor_fields_tab(&mut self, ui: &mut Ui) {
        ui.heading("Attractor Field Topology");

        // Controls
        if self.show_controls {
            self.show_attractor_controls(ui);
            ui.separator();
        }

        // Create demo attractor field
        let attractor_field = self
            .field_element
            .clone()
            .unwrap_or_else(|| self.create_demo_attractor_field());

        // Render attractor field visualization
        self.attractor_visualizer.render(ui, &attractor_field);

        // Show field properties (element symbol and number)
        ui.add_space(10.0);
        if let Some(symbol) = self.get_element_symbol(self.selected_atomic_number) {
            ui.label(format!(
                "Element: {} ({})",
                symbol, self.selected_atomic_number
            ));
        }
    }

    /// Show attractor controls
    fn show_attractor_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Attractor Controls", |ui| {
            ui.checkbox(
                &mut self.attractor_visualizer.show_depth,
                "Show Basin Depth",
            );
            ui.checkbox(
                &mut self.attractor_visualizer.show_stability,
                "Show Stability",
            );
            ui.checkbox(
                &mut self.attractor_visualizer.show_field_lines,
                "Field Lines",
            );

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.attractor_visualizer.scale, 0.5..=2.0).text(""));
        });
    }

    /// Show particles tab
    fn show_particles_tab(&mut self, ui: &mut Ui) {
        ui.heading("Particle Signatures");

        // Controls
        if self.show_controls {
            self.show_particle_controls(ui);
            ui.separator();
        }

        // Create demo particle signature data
        let particle_signature = self
            .field_particle_signature
            .clone()
            .unwrap_or_else(|| self.create_demo_particle_signature_data());

        // Render particle signature visualization
        self.particle_signature_view.render(ui, &particle_signature);

        // Show particle statistics
        ui.add_space(10.0);
        ui.label("Particle Statistics:");
        ui.label(format!("  Particle Type: {}", particle_signature.name()));
        ui.label(format!("  Mass: {:.3} u", particle_signature.mass));
        ui.label(format!("  Charge: {:.1}", particle_signature.charge));
        ui.label(format!(
            "  Mind Dominance: {:.3}",
            particle_signature.mind_dominance
        ));
        ui.label(format!(
            "  Spirit Dominance: {:.3}",
            particle_signature.spirit_dominance
        ));
    }

    /// Show particle controls
    fn show_particle_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Particle Controls", |ui| {
            ui.checkbox(
                &mut self.particle_signature_view.show_complex_breakdown,
                "Show Complex Breakdown",
            );
            ui.checkbox(&mut self.particle_signature_view.show_charge, "Show Charge");
            ui.checkbox(&mut self.particle_signature_view.show_mass, "Show Mass");

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.particle_signature_view.scale, 0.5..=2.0).text(""));
        });
    }

    /// Show derivation tab
    fn show_derivation_tab(&mut self, ui: &mut Ui) {
        ui.heading("Mass & Charge Derivation");

        // Controls
        if self.show_controls {
            self.show_derivation_controls(ui);
            ui.separator();
        }

        // Create demo derivation data (proton for demo)
        let derivation_data = self
            .field_derivation_data
            .clone()
            .unwrap_or_else(|| self.create_demo_derivation_data());
        let particle_type = self.field_particle_type.unwrap_or(ParticleType::Proton);

        // Render mass/charge derivation visualization
        self.mass_charge_derivation
            .render(ui, &derivation_data, particle_type);

        // Show derivation explanation
        ui.add_space(10.0);
        ui.label("Derivation From Field Geometry:");
        ui.label("Mass emerges from field curvature (basin depth)");
        ui.label("Charge emerges from field gradient (flow direction)");
        ui.label("These are intrinsic field properties, not 'added' attributes");
    }

    /// Show derivation controls
    fn show_derivation_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Derivation Controls", |ui| {
            ui.checkbox(
                &mut self.mass_charge_derivation.show_flow,
                "Show Flow Arrows",
            );

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.mass_charge_derivation.scale, 0.5..=2.0).text(""));

            ui.label(format!(
                "Step: {}/4",
                self.mass_charge_derivation.current_step + 1
            ));
            ui.label(format!(
                "Progress: {:.0}%",
                self.mass_charge_derivation.animation_progress * 100.0
            ));

            if ui.button("Reset Animation").clicked() {
                self.mass_charge_derivation.reset();
            }
        });
    }

    /// Show periodic table tab
    fn show_periodic_table_tab(&mut self, ui: &mut Ui) {
        ui.heading("Periodic Table Landscape");

        // Controls
        if self.show_controls {
            self.show_periodic_table_controls(ui);
            ui.separator();
        }

        // Create demo periodic table
        let periodic_table = self.create_demo_periodic_table();

        // Render periodic table landscape
        self.periodic_table_landscape.render(ui, &periodic_table);

        // Show selected element info
        ui.add_space(10.0);
        ui.separator();
        self.show_element_info(ui);
    }

    /// Show periodic table controls
    fn show_periodic_table_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Periodic Table Controls", |ui| {
            ui.checkbox(
                &mut self.periodic_table_landscape.show_basins,
                "Show Attractor Basins",
            );
            ui.checkbox(
                &mut self.periodic_table_landscape.show_properties,
                "Show Properties",
            );
            ui.checkbox(
                &mut self.periodic_table_landscape.color_by_category,
                "Color by Category",
            );

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.periodic_table_landscape.scale, 0.5..=2.0).text(""));
        });
    }

    /// Show element information
    fn show_element_info(&self, ui: &mut Ui) {
        if let Some(symbol) = self.get_element_symbol(self.selected_atomic_number) {
            ui.label(format!(
                "Element: {} ({})",
                self.get_element_name(self.selected_atomic_number)
                    .unwrap_or("Unknown"),
                symbol
            ));
            ui.label(format!("Atomic Number: {}", self.selected_atomic_number));

            // Derive approximate mass and charge
            let mass = self.derive_atomic_mass(self.selected_atomic_number);
            let charge = self.derive_atomic_charge(self.selected_atomic_number);

            ui.label(format!("Atomic Mass: {:.3} u", mass));
            ui.label(format!("Nuclear Charge: +{}", charge));

            // Electronic configuration
            let config = self.electronic_configuration(self.selected_atomic_number);
            ui.label(format!("Configuration: {}", config));
        }
    }

    /// Show orbital legend
    fn show_orbital_legend(&self, ui: &mut Ui) {
        ui.collapsing("Orbital Legend", |ui| {
            let orbitals = [
                ("1s", Color32::from_rgb(255, 100, 100)),
                ("2s", Color32::from_rgb(100, 255, 100)),
                ("2p", Color32::from_rgb(100, 100, 255)),
                ("3s", Color32::from_rgb(255, 255, 100)),
                ("3p", Color32::from_rgb(255, 100, 255)),
                ("3d", Color32::from_rgb(100, 255, 255)),
                ("4s", Color32::from_rgb(255, 150, 50)),
                ("4p", Color32::from_rgb(150, 255, 150)),
                ("4d", Color32::from_rgb(150, 150, 255)),
                ("4f", Color32::from_rgb(255, 150, 255)),
            ];

            ui.columns(2, |columns| {
                let mut col_idx = 0;
                for (orbital, color) in orbitals.iter() {
                    columns[col_idx].horizontal(|ui| {
                        ui.add_sized(
                            [20.0, 16.0],
                            egui::Label::new(egui::RichText::new("■").color(*color)),
                        );
                        ui.label(*orbital);
                    });
                    col_idx = (col_idx + 1) % 2;
                }
            });
        });
    }

    /// Get element symbol
    fn get_element_symbol(&self, atomic_number: u32) -> Option<&'static str> {
        const ELEMENTS: [&str; 119] = [
            "", "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P",
            "S", "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn",
            "Ga", "Ge", "As", "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru",
            "Rh", "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce",
            "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf",
            "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn",
            "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm",
            "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh", "Fl",
            "Mc", "Lv", "Ts", "Og",
        ];

        ELEMENTS.get(atomic_number as usize).copied()
    }

    /// Get element name
    fn get_element_name(&self, atomic_number: u32) -> Option<&'static str> {
        const NAMES: [&str; 119] = [
            "",
            "Hydrogen",
            "Helium",
            "Lithium",
            "Beryllium",
            "Boron",
            "Carbon",
            "Nitrogen",
            "Oxygen",
            "Fluorine",
            "Neon",
            "Sodium",
            "Magnesium",
            "Aluminum",
            "Silicon",
            "Phosphorus",
            "Sulfur",
            "Chlorine",
            "Argon",
            "Potassium",
            "Calcium",
            "Scandium",
            "Titanium",
            "Vanadium",
            "Chromium",
            "Manganese",
            "Iron",
            "Cobalt",
            "Nickel",
            "Copper",
            "Zinc",
            "Gallium",
            "Germanium",
            "Arsenic",
            "Selenium",
            "Bromine",
            "Krypton",
            "Rubidium",
            "Strontium",
            "Yttrium",
            "Zirconium",
            "Niobium",
            "Molybdenum",
            "Technetium",
            "Ruthenium",
            "Rhodium",
            "Palladium",
            "Silver",
            "Cadmium",
            "Indium",
            "Tin",
            "Antimony",
            "Tellurium",
            "Iodine",
            "Xenon",
            "Cesium",
            "Barium",
            "Lanthanum",
            "Cerium",
            "Praseodymium",
            "Neodymium",
            "Promethium",
            "Samarium",
            "Europium",
            "Gadolinium",
            "Terbium",
            "Dysprosium",
            "Holmium",
            "Erbium",
            "Thulium",
            "Ytterbium",
            "Lutetium",
            "Hafnium",
            "Tantalum",
            "Tungsten",
            "Rhenium",
            "Osmium",
            "Iridium",
            "Platinum",
            "Gold",
            "Mercury",
            "Thallium",
            "Lead",
            "Bismuth",
            "Polonium",
            "Astatine",
            "Radon",
            "Francium",
            "Radium",
            "Actinium",
            "Thorium",
            "Protactinium",
            "Uranium",
            "Neptunium",
            "Plutonium",
            "Americium",
            "Curium",
            "Berkelium",
            "Californium",
            "Einsteinium",
            "Fermium",
            "Mendelevium",
            "Nobelium",
            "Lawrencium",
            "Rutherfordium",
            "Dubnium",
            "Seaborgium",
            "Bohrium",
            "Hassium",
            "Meitnerium",
            "Darmstadtium",
            "Roentgenium",
            "Copernicium",
            "Nihonium",
            "Flerovium",
            "Moscovium",
            "Livermorium",
            "Tennessine",
            "Oganesson",
        ];

        NAMES.get(atomic_number as usize).copied()
    }

    /// Generate quantum numbers for a given atomic number
    fn generate_quantum_numbers(&self, atomic_number: u32) -> QuantumNumberSet {
        // Determine principal quantum number based on atomic number
        let n = match atomic_number {
            1..=2 => 1,
            3..=10 => 2,
            11..=18 => 3,
            19..=36 => 4,
            37..=54 => 5,
            55..=86 => 6,
            _ => 7,
        };

        // Determine azimuthal quantum number
        let l = (atomic_number - 1) % (n * (n + 1) / 2);

        // Magnetic quantum number
        let m = ((atomic_number - 1) % (2 * l + 1)) as i32 - l as i32;

        // Spin
        let spin = if atomic_number.is_multiple_of(2) {
            Spin::Down
        } else {
            Spin::Up
        };

        QuantumNumberSet::new(n, l, m, spin)
    }

    /// Derive atomic mass from field geometry
    fn derive_atomic_mass(&self, atomic_number: u32) -> f64 {
        // Simplified: mass is approximately atomic number
        atomic_number as f64
    }

    /// Derive atomic charge from field gradient
    fn derive_atomic_charge(&self, atomic_number: u32) -> i32 {
        atomic_number as i32
    }

    /// Get electronic configuration
    fn electronic_configuration(&self, atomic_number: u32) -> String {
        // Simplified Aufbau principle
        let mut electrons = atomic_number;
        let mut config = Vec::new();

        let orbitals = [
            (1, "s", 2u32),
            (2, "s", 2),
            (2, "p", 6),
            (3, "s", 2),
            (3, "p", 6),
            (3, "d", 10),
            (4, "s", 2),
            (4, "p", 6),
            (4, "d", 10),
            (4, "f", 14),
            (5, "s", 2),
            (5, "p", 6),
            (5, "d", 10),
            (5, "f", 14),
            (6, "s", 2),
            (6, "p", 6),
            (6, "d", 10),
            (7, "s", 2),
        ];

        for (n, subshell, capacity) in orbitals {
            if electrons == 0 {
                break;
            }
            let count = electrons.min(capacity);
            if count > 0 {
                config.push(format!("{}{}{}", n, subshell, count));
                electrons -= count;
            }
        }

        config.join(" ")
    }
    /// Create demo attractor field
    fn create_demo_attractor_field(&self) -> ElementAttractorField {
        // Create an element field based on the selected atomic number
        ElementAttractorField::from_atomic_number(self.selected_atomic_number)
    }

    /// Create demo particle signature data
    fn create_demo_particle_signature_data(
        &self,
    ) -> crate::gui::visualization::atomic_viz::ParticleSignatureData {
        use crate::holographic_foundation::atomic_emergence::particle_derivation::{
            ParticleArchetypePattern, ParticleProperties, ParticleType,
        };

        // For demo, show proton signature
        let pattern = ParticleArchetypePattern::proton_pattern();
        let props = ParticleProperties::derive_from_archetype(ParticleType::Proton, pattern);

        crate::gui::visualization::atomic_viz::ParticleSignatureData::from_properties(&props)
    }

    /// Create demo derivation data
    fn create_demo_derivation_data(
        &self,
    ) -> crate::gui::visualization::atomic_viz::MassChargeDerivationData {
        use crate::holographic_foundation::atomic_emergence::particle_derivation::DerivationFactors;

        // Create demo derivation factors for a proton
        let factors = DerivationFactors {
            mind_dominance: 0.8,
            spirit_dominance: 0.2,
            body_balance: 0.5,
            coherence_depth: 0.7,
            charge_polarity: 1.0,
        };

        crate::gui::visualization::atomic_viz::MassChargeDerivationData::from_factors(&factors)
    }

    /// Create demo periodic table
    fn create_demo_periodic_table(&self) -> crate::holographic_foundation::atomic_emergence::periodic_table_attractors::PeriodicTableAttractors{
        crate::holographic_foundation::atomic_emergence::periodic_table_attractors::PeriodicTableAttractors::default()
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

    /// Get selected atomic number
    pub fn get_selected_atomic_number(&self) -> u32 {
        self.selected_atomic_number
    }

    /// Set selected atomic number
    pub fn set_selected_atomic_number(&mut self, atomic_number: u32) {
        self.selected_atomic_number = atomic_number.clamp(1, 118);
    }
}

impl Default for AtomicPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_panel_creation() {
        let panel = AtomicPanel::new();
        assert!(!panel.visible);
        assert_eq!(panel.view_mode, AtomicViewMode::Orbitals);
    }

    #[test]
    fn test_view_mode_switching() {
        let mut panel = AtomicPanel::new();
        panel.view_mode = AtomicViewMode::AttractorFields;
        assert_eq!(panel.view_mode, AtomicViewMode::AttractorFields);

        panel.view_mode = AtomicViewMode::PeriodicTable;
        assert_eq!(panel.view_mode, AtomicViewMode::PeriodicTable);
    }

    #[test]
    fn test_visibility_toggle() {
        let mut panel = AtomicPanel::new();
        assert!(!panel.is_visible());

        panel.toggle_visibility();
        assert!(panel.is_visible());

        panel.set_visible(false);
        assert!(!panel.is_visible());
    }

    #[test]
    fn test_animation_update() {
        let mut panel = AtomicPanel::new();
        panel.update(0.1);
        assert!(panel.animation_time > 0.0);
    }

    #[test]
    fn test_atomic_number_bounds() {
        let mut panel = AtomicPanel::new();

        // Test upper bound
        panel.set_selected_atomic_number(200);
        assert_eq!(panel.selected_atomic_number, 118);

        // Test lower bound
        panel.set_selected_atomic_number(0);
        assert_eq!(panel.selected_atomic_number, 1);
    }

    #[test]
    fn test_element_lookup() {
        let panel = AtomicPanel::new();

        assert_eq!(panel.get_element_symbol(1), Some("H"));
        assert_eq!(panel.get_element_name(1), Some("Hydrogen"));

        assert_eq!(panel.get_element_symbol(6), Some("C"));
        assert_eq!(panel.get_element_name(6), Some("Carbon"));

        assert_eq!(panel.get_element_symbol(118), Some("Og"));
    }

    #[test]
    fn test_quantum_number_generation() {
        let panel = AtomicPanel::new();

        let qn = panel.generate_quantum_numbers(1);
        assert_eq!(qn.n, 1);

        let qn = panel.generate_quantum_numbers(6);
        assert_eq!(qn.n, 2);

        let qn = panel.generate_quantum_numbers(26);
        assert_eq!(qn.n, 4);
    }
}
