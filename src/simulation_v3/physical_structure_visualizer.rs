// Physical Structure Visualizer - Phase 3: Physical Manifestation System
//
// This module provides visualization for all physical structures that manifest
// at each density sub-level. It shows the hierarchical composition and
// simultaneous emergence of individual and collective structures.
//
// Visualizations include:
// 1. Physical structure tree (hierarchical composition)
// 2. Density/sub-level distribution
// 3. Simultaneous emergence timeline
// 4. Structure composition breakdown
// 5. Individual vs collective ratio

use crate::physical_manifestation::{
    HierarchicalCompositionManager, PhysicalStructure, PhysicalStructureManager,
    PhysicalStructureType, SimultaneousEmergenceManager,
};
use crate::types::Float;

// ============================================================================
// PHYSICAL STRUCTURE VISUALIZER
// ============================================================================

/// Visualizes physical structures and their relationships
pub struct PhysicalStructureVisualizer {
    /// Physical structure manager
    pub structure_manager: PhysicalStructureManager,

    /// Hierarchical composition manager
    pub composition_manager: HierarchicalCompositionManager,

    /// Simultaneous emergence manager
    pub emergence_manager: SimultaneousEmergenceManager,
}

impl PhysicalStructureVisualizer {
    /// Create a new physical structure visualizer
    pub fn new(
        structure_manager: PhysicalStructureManager,
        composition_manager: HierarchicalCompositionManager,
        emergence_manager: SimultaneousEmergenceManager,
    ) -> Self {
        PhysicalStructureVisualizer {
            structure_manager,
            composition_manager,
            emergence_manager,
        }
    }

    /// Visualize the physical structure tree
    pub fn visualize_structure_tree(&self, root_id: u64, max_depth: usize) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║          PHYSICAL STRUCTURE HIERARCHY TREE                    ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        if let Some(root) = self.structure_manager.get_structure(root_id) {
            self.visualize_node_recursive(&mut output, root, 0, max_depth, true);
        } else {
            output.push_str("No root structure found.\n");
        }

        output
    }

    /// Recursively visualize a node in the tree
    fn visualize_node_recursive(
        &self,
        output: &mut String,
        structure: &PhysicalStructure,
        depth: usize,
        max_depth: usize,
        is_last: bool,
    ) {
        if depth > max_depth {
            return;
        }

        let indent = "  ".repeat(depth);
        let prefix = if depth == 0 {
            "".to_string()
        } else if is_last {
            "└── ".to_string()
        } else {
            "├── ".to_string()
        };

        let structure_symbol = match structure.structure_type {
            PhysicalStructureType::QuantumParticle => "●",
            PhysicalStructureType::Atom => "○",
            PhysicalStructureType::Star => "★",
            PhysicalStructureType::Molecule => "◊",
            PhysicalStructureType::Planet => "◉",
            PhysicalStructureType::SolarSystem => "☉",
            PhysicalStructureType::Cell => "▪",
            PhysicalStructureType::GaiaSystem => "⬢",
            PhysicalStructureType::SimpleOrganism => "◆",
            PhysicalStructureType::Community => "⬡",
            PhysicalStructureType::ComplexOrganism => "⬣",
            PhysicalStructureType::Ecosystem => "⬥",
            PhysicalStructureType::ConsciousOrganism => "◈",
            PhysicalStructureType::Society => "⬦",
            PhysicalStructureType::LightBody => "✦",
            PhysicalStructureType::SocialMemoryComplex => "✧",
            PhysicalStructureType::PlanetaryLogos => "⬘",
            PhysicalStructureType::SolarLogos => "⬙",
            PhysicalStructureType::GalacticLogos => "⬚",
        };

        output.push_str(&format!(
            "{}{}{} {} (ID: {})\n",
            indent, prefix, structure_symbol, structure.name, structure.id
        ));

        output.push_str(&format!(
            "{}    Type: {:?} | Density: {} | Sub-Level: {} | Components: {}\n",
            indent,
            structure.structure_type,
            structure.density_level,
            structure.sub_level,
            structure.component_count()
        ));

        // Visualize children
        if let Some(children) = self.composition_manager.get_children(structure.id) {
            for (i, &child_id) in children.iter().enumerate() {
                if let Some(child) = self.structure_manager.get_structure(child_id) {
                    let is_last_child = i == children.len() - 1;
                    self.visualize_node_recursive(
                        output,
                        child,
                        depth + 1,
                        max_depth,
                        is_last_child,
                    );
                }
            }
        }
    }

    /// Visualize density and sub-level distribution
    pub fn visualize_density_distribution(&self) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║            DENSITY AND SUB-LEVEL DISTRIBUTION                  ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        let stats = self.structure_manager.get_statistics();

        // 1st Density
        output.push_str("1st Density (Material Substrate):\n");
        output.push_str(&format!(
            "  Quantum Realm: {} quantum particles\n",
            stats.quantum_particle_count
        ));
        output.push_str(&format!(
            "  Atomic Realm: {} atoms, {} stars\n",
            stats.atom_count, stats.star_count
        ));
        output.push_str(&format!(
            "  Molecular Realm: {} molecules, {} planets\n",
            stats.molecule_count, stats.planet_count
        ));
        output.push_str(&format!(
            "  Planetary Realm: {} solar systems\n",
            stats.solar_system_count
        ));
        output.push_str(&format!(
            "  Total: {} structures\n\n",
            stats.quantum_particle_count
                + stats.atom_count
                + stats.star_count
                + stats.molecule_count
                + stats.planet_count
                + stats.solar_system_count
        ));

        // 2nd Density
        output.push_str("2nd Density (Growth and Awareness):\n");
        output.push_str(&format!(
            "  Cellular Realm: {} cells, {} Gaia systems\n",
            stats.cell_count, stats.gaia_system_count
        ));
        output.push_str(&format!(
            "  Simple Life Realm: {} simple organisms, {} communities\n",
            stats.simple_organism_count, stats.community_count
        ));
        output.push_str(&format!(
            "  Complex Life Realm: {} complex organisms, {} ecosystems\n",
            stats.complex_organism_count, stats.ecosystem_count
        ));
        output.push_str(&format!(
            "  Total: {} structures\n\n",
            stats.cell_count
                + stats.gaia_system_count
                + stats.simple_organism_count
                + stats.community_count
                + stats.complex_organism_count
                + stats.ecosystem_count
        ));

        // 3rd Density
        output.push_str("3rd Density (Self-Awareness):\n");
        output.push_str(&format!(
            "  Conscious Life Realm: {} conscious organisms, {} societies\n",
            stats.conscious_organism_count, stats.society_count
        ));
        output.push_str(&format!(
            "  Total: {} structures\n\n",
            stats.conscious_organism_count + stats.society_count
        ));

        // Higher densities
        output.push_str("Higher Densities (Light-based):\n");
        output.push_str(&format!("  Light Bodies: {}\n", stats.light_body_count));
        output.push_str(&format!(
            "  Social Memory Complexes: {}\n",
            stats.social_memory_complex_count
        ));
        output.push_str(&format!(
            "  Planetary Logoi: {}\n",
            stats.planetary_logos_count
        ));
        output.push_str(&format!("  Solar Logoi: {}\n", stats.solar_logos_count));
        output.push_str(&format!(
            "  Galactic Logoi: {}\n",
            stats.galactic_logos_count
        ));
        output.push_str(&format!(
            "  Total: {} structures\n\n",
            stats.light_body_count
                + stats.social_memory_complex_count
                + stats.planetary_logos_count
                + stats.solar_logos_count
                + stats.galactic_logos_count
        ));

        // Grand total
        output.push_str(&format!(
            "TOTAL STRUCTURES: {}\n",
            stats.total_structure_count
        ));
        output.push_str(&format!("TOTAL MASS: {:.2} kg\n\n", stats.total_mass));

        output
    }

    /// Visualize simultaneous emergence timeline
    pub fn visualize_emergence_timeline(&self) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║          SIMULTANEOUS EMERGENCE TIMELINE                       ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        let emergence_stats = self.emergence_manager.get_emergence_statistics();

        output.push_str("Emergence Events:\n\n");

        for (i, event) in self.emergence_manager.emergence_events.iter().enumerate() {
            output.push_str(&format!(
                "{} Event {}: {} Sub-Level\n",
                i + 1,
                i + 1,
                event.sub_level
            ));
            output.push_str(&format!("  Type: {:?}\n", event.emergence_type));
            output.push_str(&format!(
                "  Individual Structures: {} created\n",
                event.individual_structures.len()
            ));
            output.push_str(&format!(
                "  Collective Structures: {} created\n",
                event.collective_structures.len()
            ));

            // Show individual structures
            if !event.individual_structures.is_empty() {
                output.push_str("  Individual: ");
                for (j, &id) in event.individual_structures.iter().take(5).enumerate() {
                    if j > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&format!("ID={}", id));
                }
                if event.individual_structures.len() > 5 {
                    output.push_str(&format!(
                        " (+{} more)",
                        event.individual_structures.len() - 5
                    ));
                }
                output.push('\n');
            }

            // Show collective structures
            if !event.collective_structures.is_empty() {
                output.push_str("  Collective: ");
                for (j, &id) in event.collective_structures.iter().take(5).enumerate() {
                    if j > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&format!("ID={}", id));
                }
                if event.collective_structures.len() > 5 {
                    output.push_str(&format!(
                        " (+{} more)",
                        event.collective_structures.len() - 5
                    ));
                }
                output.push('\n');
            }

            output.push('\n');
        }

        output.push_str("Summary:\n");
        output.push_str(&format!(
            "  Total Emergence Events: {}\n",
            emergence_stats.total_emergence_events
        ));
        output.push_str(&format!(
            "  Total Individual Structures Created: {}\n",
            emergence_stats.total_individual_structures
        ));
        output.push_str(&format!(
            "  Total Collective Structures Created: {}\n",
            emergence_stats.total_collective_structures
        ));

        output
    }

    /// Visualize individual vs collective ratio
    pub fn visualize_individual_collective_ratio(&self) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║          INDIVIDUAL VS COLLECTIVE RATIO                       ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        let stats = self.structure_manager.get_statistics();

        // Individual structures
        let individual_count = stats.quantum_particle_count
            + stats.atom_count
            + stats.molecule_count
            + stats.cell_count
            + stats.simple_organism_count
            + stats.complex_organism_count
            + stats.conscious_organism_count;

        // Collective structures
        let collective_count = stats.star_count
            + stats.planet_count
            + stats.solar_system_count
            + stats.gaia_system_count
            + stats.community_count
            + stats.ecosystem_count
            + stats.society_count
            + stats.galactic_logos_count
            + stats.solar_logos_count
            + stats.planetary_logos_count
            + stats.social_memory_complex_count;

        let total = individual_count + collective_count;
        let individual_ratio = if total > 0 {
            individual_count as Float / total as Float
        } else {
            0.0
        };
        let collective_ratio = if total > 0 {
            collective_count as Float / total as Float
        } else {
            0.0
        };

        output.push_str(&format!(
            "Individual Structures: {} ({:.1}%)\n",
            individual_count,
            individual_ratio * 100.0
        ));
        output.push_str(&format!(
            "Collective Structures: {} ({:.1}%)\n",
            collective_count,
            collective_ratio * 100.0
        ));
        output.push_str(&format!("Total Structures: {}\n\n", total));

        // Visual bar
        let bar_width = 60;
        let individual_bar_width = (individual_ratio * bar_width as Float) as usize;
        let collective_bar_width = (collective_ratio * bar_width as Float) as usize;

        output.push_str("Individual: ");
        output.push_str(&"█".repeat(individual_bar_width));
        output.push_str(&" ".repeat(bar_width - individual_bar_width));
        output.push_str(&format!(" {:.1}%\n", individual_ratio * 100.0));

        output.push_str("Collective: ");
        output.push_str(&"█".repeat(collective_bar_width));
        output.push_str(&" ".repeat(bar_width - collective_bar_width));
        output.push_str(&format!(" {:.1}%\n\n", collective_ratio * 100.0));

        // Breakdown by density
        output.push_str("Breakdown by Density:\n\n");

        output.push_str("1st Density:\n");
        output.push_str(&format!(
            "  Individual: {} (quantum particles, atoms, molecules)\n",
            stats.quantum_particle_count + stats.atom_count + stats.molecule_count
        ));
        output.push_str(&format!(
            "  Collective: {} (stars, planets, solar systems)\n",
            stats.star_count + stats.planet_count + stats.solar_system_count
        ));
        output.push_str(&format!(
            "  Ratio: 1:{:.1}\n\n",
            (stats.star_count + stats.planet_count + stats.solar_system_count) as Float
                / (stats.quantum_particle_count + stats.atom_count + stats.molecule_count) as Float
        ));

        output.push_str("2nd Density:\n");
        output.push_str(&format!(
            "  Individual: {} (cells, simple organisms, complex organisms)\n",
            stats.cell_count + stats.simple_organism_count + stats.complex_organism_count
        ));
        output.push_str(&format!(
            "  Collective: {} (Gaia systems, communities, ecosystems)\n",
            stats.gaia_system_count + stats.community_count + stats.ecosystem_count
        ));
        output.push_str(&format!(
            "  Ratio: 1:{:.1}\n\n",
            (stats.gaia_system_count + stats.community_count + stats.ecosystem_count) as Float
                / (stats.cell_count + stats.simple_organism_count + stats.complex_organism_count)
                    as Float
        ));

        output.push_str("3rd Density:\n");
        output.push_str(&format!(
            "  Individual: {} (conscious organisms)\n",
            stats.conscious_organism_count
        ));
        output.push_str(&format!(
            "  Collective: {} (societies)\n",
            stats.society_count
        ));
        output.push_str(&format!(
            "  Ratio: 1:{:.1}\n\n",
            stats.society_count as Float / stats.conscious_organism_count as Float
        ));

        output
    }

    /// Visualize structure composition breakdown
    pub fn visualize_composition_breakdown(&self, structure_id: u64) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║          STRUCTURE COMPOSITION BREAKDOWN                       ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        if let Some(structure) = self.structure_manager.get_structure(structure_id) {
            output.push_str(&format!(
                "Structure: {} (ID: {})\n",
                structure.name, structure.id
            ));
            output.push_str(&format!("Type: {:?}\n", structure.structure_type));
            output.push_str(&format!(
                "Density: {} | Sub-Level: {}\n",
                structure.density_level, structure.sub_level
            ));
            output.push_str(&format!("Components: {}\n\n", structure.component_count()));

            if structure.component_count() == 0 {
                output.push_str("This structure has no components (it is a leaf structure).\n");
            } else {
                output.push_str("Components:\n");

                for (i, &component_id) in structure.composition.iter().enumerate() {
                    if let Some(component) = self.structure_manager.get_structure(component_id) {
                        output.push_str(&format!(
                            "  {}. {} (ID: {}, Type: {:?})\n",
                            i + 1,
                            component.name,
                            component.id,
                            component.structure_type
                        ));
                        output.push_str(&format!(
                            "     Density: {} | Sub-Level: {}\n",
                            component.density_level, component.sub_level
                        ));
                        output.push_str(&format!(
                            "     Mass: {:.2e} kg | Components: {}\n",
                            component.mass,
                            component.component_count()
                        ));
                    }
                }

                // Show hierarchy depth
                let depth = self.composition_manager.get_hierarchy_depth(structure_id);
                output.push_str(&format!("\nHierarchy Depth: {} levels\n", depth));

                // Show descendants
                let descendants = self.composition_manager.get_descendants(structure_id);
                output.push_str(&format!(
                    "Total Descendants: {} structures\n",
                    descendants.len()
                ));
            }
        } else {
            output.push_str(&format!("Structure with ID {} not found.\n", structure_id));
        }

        output
    }

    /// Generate comprehensive physical structure report
    pub fn generate_comprehensive_report(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.visualize_density_distribution());
        output.push('\n');

        output.push_str(&self.visualize_emergence_timeline());
        output.push('\n');

        output.push_str(&self.visualize_individual_collective_ratio());
        output.push('\n');

        // Show structure trees for each density
        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║          REPRESENTATIVE STRUCTURE TREES                        ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        // Show a representative from each density
        let densities = vec![1, 2, 3];
        for density in densities {
            let structures = self.structure_manager.get_structures_by_density(density);
            if !structures.is_empty() {
                output.push_str(&format!("{} Density Representative:\n\n", density));
                let root_id = structures[0].id;
                output.push_str(&self.visualize_structure_tree(root_id, 3));
                output.push('\n');
            }
        }

        output
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Create a physical structure visualizer from managers
pub fn create_visualizer(
    structure_manager: PhysicalStructureManager,
    composition_manager: HierarchicalCompositionManager,
    emergence_manager: SimultaneousEmergenceManager,
) -> PhysicalStructureVisualizer {
    PhysicalStructureVisualizer::new(structure_manager, composition_manager, emergence_manager)
}
