//! Structure Dashboard
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 3 Week 7:
//! "Create a tree view of composition, statistics per level, interactive exploration"
//!
//! This module provides an EGUI-based dashboard for visualizing physical structure
//! with tree views, statistics, and interactive controls.

use crate::gui::visualization::structure_viz::{
    StructureLevel, StructureMetrics, StructureNode, StructureVisualizer,
};
use egui::{Color32, Ui, Vec2};

/// Structure dashboard configuration
pub struct StructureDashboard {
    /// Show dashboard
    pub show_dashboard: bool,
    /// Selected level
    pub selected_level: Option<StructureLevel>,
    /// Selected node
    pub selected_node: Option<usize>,
    /// Show tree view
    pub show_tree_view: bool,
    /// Show statistics
    pub show_statistics: bool,
    /// Show hierarchy
    pub show_hierarchy: bool,
    /// Show component counts
    pub show_component_counts: bool,
    /// Tree view depth limit
    pub tree_depth_limit: usize,
    /// Show level details
    pub show_level_details: bool,
    /// Search query
    pub search_query: String,
    /// Filter by density
    pub filter_by_density: bool,
    /// Auto-expand nodes
    pub auto_expand: bool,
    /// Show emergence indicators
    pub show_emergence_indicators: bool,
    /// Show health bars
    pub show_health_bars: bool,
}

impl StructureDashboard {
    /// Create new structure dashboard
    pub fn new() -> Self {
        StructureDashboard {
            show_dashboard: true,
            selected_level: None,
            selected_node: None,
            show_tree_view: true,
            show_statistics: true,
            show_hierarchy: true,
            show_component_counts: true,
            tree_depth_limit: 5,
            show_level_details: true,
            search_query: String::new(),
            filter_by_density: false,
            auto_expand: false,
            show_emergence_indicators: true,
            show_health_bars: true,
        }
    }

    /// Render the dashboard
    pub fn render(&mut self, ui: &mut Ui, visualizer: &mut StructureVisualizer) {
        if !self.show_dashboard {
            return;
        }

        egui::Window::new("Structure Dashboard")
            .resizable(true)
            .default_size(Vec2::new(300.0, 400.0))
            .show(ui.ctx(), |ui| {
                // Controls
                self.render_controls(ui);

                ui.separator();

                // Tree view
                if self.show_tree_view {
                    self.render_tree_view(ui, visualizer);
                }

                ui.separator();

                // Statistics
                if self.show_statistics {
                    self.render_statistics(ui, visualizer);
                }
            });
    }

    /// Render controls
    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_tree_view, "Tree");
            ui.checkbox(&mut self.show_statistics, "Stats");
            ui.checkbox(&mut self.show_hierarchy, "Hierarchy");
        });

        ui.horizontal(|ui| {
            ui.label("Depth:");
            ui.add(egui::Slider::new(&mut self.tree_depth_limit, 1..=10).step_by(1.0));
            ui.checkbox(&mut self.auto_expand, "Auto-expand");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_component_counts, "Components");
            ui.checkbox(&mut self.show_emergence_indicators, "Emergence");
            ui.checkbox(&mut self.show_health_bars, "Health");
        });

        ui.separator();

        // Level selector
        ui.label("Select Level:");
        ui.horizontal_wrapped(|ui| {
            for level in [
                StructureLevel::Quantum,
                StructureLevel::Atomic,
                StructureLevel::Molecular,
                StructureLevel::Cellular,
                StructureLevel::Tissue,
                StructureLevel::Organism,
                StructureLevel::Planetary,
                StructureLevel::Stellar,
                StructureLevel::Galactic,
            ] {
                if ui
                    .selectable_label(self.selected_level == Some(level), level.name())
                    .clicked()
                {
                    self.selected_level = Some(level);
                }
            }
        });

        ui.separator();
    }

    /// Render tree view
    fn render_tree_view(&mut self, ui: &mut Ui, visualizer: &mut StructureVisualizer) {
        ui.heading("Structure Hierarchy");

        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for root_id in visualizer.roots.clone() {
                    self.render_node_recursive(ui, visualizer, root_id, 0);
                }
            });
    }

    /// Render node recursively
    fn render_node_recursive(
        &mut self,
        ui: &mut Ui,
        visualizer: &mut StructureVisualizer,
        node_id: usize,
        depth: usize,
    ) {
        if depth > self.tree_depth_limit {
            return;
        }

        let node = match visualizer.get_node(node_id) {
            Some(n) => n,
            None => return,
        };

        // Clone node data to avoid borrow issues
        let node_level = node.level;
        let node_structure_type = node.structure_type;
        let node_component_count = node.component_count;
        let node_simultaneous_emergence = node.simultaneous_emergence;
        let node_health = node.health;
        let node_children = node.children.clone();

        // Filter by search query
        if !self.search_query.is_empty() {
            let search_lower = self.search_query.to_lowercase();
            let node_name = node_structure_type.name().to_lowercase();
            if !node_name.contains(&search_lower) {
                return;
            }
        }

        // Filter by level
        if let Some(selected_level) = self.selected_level {
            if node_level != selected_level {
                return;
            }
        }

        let is_selected = self.selected_node == Some(node_id);
        let is_expanded = visualizer.is_expanded(node_id);
        let has_children = !node_children.is_empty();

        // Indent based on depth
        let indent = depth as f32 * 20.0;

        ui.horizontal(|ui| {
            ui.add_space(indent);

            // Expand/collapse button
            if has_children {
                if ui.button(if is_expanded { "▼" } else { "▶" }).clicked() {
                    visualizer.toggle_expansion(node_id);
                }
            } else {
                ui.add_space(20.0);
            }

            // Node icon
            ui.label(node_structure_type.icon());

            // Node name
            let label = if self.show_component_counts {
                format!("{} ({})", node_structure_type.name(), node_component_count)
            } else {
                node_structure_type.name().to_string()
            };

            if ui.selectable_label(is_selected, label).clicked() {
                self.selected_node = Some(node_id);
                visualizer.select_node(Some(node_id));
            }

            // Emergence indicator
            if self.show_emergence_indicators && node_simultaneous_emergence {
                ui.colored_label(Color32::GOLD, "✨");
            }

            // Health bar
            if self.show_health_bars {
                let health_color = health_to_color(node_health);
                ui.add_sized(
                    [50.0, 10.0],
                    egui::ProgressBar::new(node_health as f32).fill(health_color),
                );
            }
        });

        // Render children if expanded
        if is_expanded {
            for child_id in node_children {
                self.render_node_recursive(ui, visualizer, child_id, depth + 1);
            }
        }
    }

    /// Render statistics
    fn render_statistics(&mut self, ui: &mut Ui, visualizer: &StructureVisualizer) {
        ui.heading("Statistics");

        // Overall statistics
        ui.label(format!("Total Nodes: {}", visualizer.nodes.len()));
        ui.label(format!("Root Nodes: {}", visualizer.roots.len()));

        ui.separator();

        // Statistics per level
        for level in [
            StructureLevel::Quantum,
            StructureLevel::Atomic,
            StructureLevel::Molecular,
            StructureLevel::Cellular,
            StructureLevel::Tissue,
            StructureLevel::Organism,
            StructureLevel::Planetary,
            StructureLevel::Stellar,
            StructureLevel::Galactic,
        ] {
            if let Some(metrics) = visualizer.get_metrics(level) {
                self.render_level_statistics(ui, level, metrics);
            }
        }

        ui.separator();

        // Selected node details
        if let Some(node_id) = self.selected_node {
            if let Some(node) = visualizer.get_node(node_id) {
                self.render_node_details(ui, node);
            }
        }
    }

    /// Render statistics for a level
    fn render_level_statistics(
        &mut self,
        ui: &mut Ui,
        level: StructureLevel,
        metrics: &StructureMetrics,
    ) {
        ui.collapsing(level.name(), |ui| {
            ui.label(format!("Nodes: {}", metrics.total_nodes));
            ui.label(format!("Avg Health: {:.2}", metrics.average_health));
            ui.label(format!("Total Components: {}", metrics.total_components));
            ui.label(format!("Avg Components: {:.1}", metrics.average_components));
            ui.label(format!(
                "Simultaneous Emergence: {}",
                metrics.simultaneous_emergence_count
            ));
            ui.label(format!(
                "Emergence %: {:.1}%",
                metrics.emergence_percentage * 100.0
            ));

            ui.separator();
            ui.label("Density Distribution:");
            for (density, count) in &metrics.density_distribution {
                ui.label(format!("  Density {}: {}", density.as_usize(), count));
            }
        });
    }

    /// Render node details
    fn render_node_details(&mut self, ui: &mut Ui, node: &StructureNode) {
        ui.separator();
        ui.heading("Selected Node");

        ui.label(format!("Type: {}", node.structure_type.name()));
        ui.label(format!("Level: {}", node.level.name()));
        ui.label(format!("Scale: 10^{:.2} m", node.scale.log10()));
        ui.label(format!("Health: {:.2}", node.health));
        ui.label(format!("Density: {}", node.density as usize + 1));
        ui.label(format!("Components: {}", node.component_count));
        ui.label(format!("Children: {}", node.children.len()));

        if node.simultaneous_emergence {
            ui.colored_label(Color32::GOLD, "Simultaneous Emergence");
        }

        // Hierarchy path
        ui.separator();
        ui.label("Hierarchy Path:");
        ui.label(format!("Depth: {}", node.depth()));
    }

    /// Get statistics summary
    pub fn get_statistics_summary(&self, visualizer: &StructureVisualizer) -> String {
        let total_nodes = visualizer.nodes.len();
        let total_roots = visualizer.roots.len();

        let mut total_components = 0;
        let mut total_emergence = 0;

        for node in &visualizer.nodes {
            total_components += node.component_count;
            if node.simultaneous_emergence {
                total_emergence += 1;
            }
        }

        format!(
            "Nodes: {} | Roots: {} | Components: {} | Emergence: {}",
            total_nodes, total_roots, total_components, total_emergence
        )
    }
}

impl Default for StructureDashboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert health value to color
fn health_to_color(health: f64) -> Color32 {
    if health < 0.3 {
        Color32::RED
    } else if health < 0.6 {
        Color32::YELLOW
    } else if health < 0.8 {
        Color32::from_rgb(100, 200, 100)
    } else {
        Color32::GREEN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_dashboard_new() {
        let dashboard = StructureDashboard::new();
        assert!(dashboard.show_dashboard);
        assert!(dashboard.show_tree_view);
        assert!(dashboard.show_statistics);
        assert_eq!(dashboard.tree_depth_limit, 5);
    }

    #[test]
    fn test_health_to_color() {
        assert_eq!(health_to_color(0.2), Color32::RED);
        assert_eq!(health_to_color(0.5), Color32::YELLOW);
        assert_eq!(health_to_color(0.9), Color32::GREEN);
    }

    #[test]
    fn test_get_statistics_summary() {
        let dashboard = StructureDashboard::new();
        let visualizer = StructureVisualizer::new();

        let summary = dashboard.get_statistics_summary(&visualizer);
        assert_eq!(
            summary,
            "Nodes: 0 | Roots: 0 | Components: 0 | Emergence: 0"
        );
    }

    #[test]
    fn test_structure_dashboard_default() {
        let dashboard = StructureDashboard::default();
        assert!(dashboard.show_dashboard);
        assert_eq!(dashboard.tree_depth_limit, 5);
    }
}
