//! Panel Docking System
//!
//! Provides a flexible docking system for organizing UI panels with:
//! - Drag-to-rearrange functionality
//! - Dock areas (Left, Right, Top, Bottom, Floating)
//! - Collapsible panels with animation
//! - Layout persistence and loading
//! - Tab-based panel grouping
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Panel organization and docking system"

use std::collections::HashMap;

/// Unique identifier for a dockable panel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PanelId(pub u64);

impl PanelId {
    /// Create a new panel ID from a raw value
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Docking areas where panels can be placed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DockArea {
    /// Left side panel
    Left,
    /// Right side panel
    #[default]
    Right,
    /// Top panel
    Top,
    /// Bottom panel
    Bottom,
    /// Center/main area
    Center,
    /// Floating panel (can be dragged anywhere)
    Floating,
}

impl DockArea {
    /// Get all available dock areas
    pub fn all() -> &'static [DockArea] {
        &[
            DockArea::Left,
            DockArea::Right,
            DockArea::Top,
            DockArea::Bottom,
            DockArea::Center,
            DockArea::Floating,
        ]
    }

    /// Get the default width/height ratio for this dock area
    pub fn default_ratio(&self) -> f32 {
        match self {
            DockArea::Left | DockArea::Right => 0.25,
            DockArea::Top | DockArea::Bottom => 0.2,
            DockArea::Center => 0.5,
            DockArea::Floating => 0.3,
        }
    }

    /// Check if this is a side panel (Left or Right)
    pub fn is_side(&self) -> bool {
        matches!(self, DockArea::Left | DockArea::Right)
    }

    /// Check if this is a horizontal panel (Top or Bottom)
    pub fn is_horizontal(&self) -> bool {
        matches!(self, DockArea::Top | DockArea::Bottom)
    }
}

/// State of a dockable panel
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PanelState {
    /// Panel is fully visible and expanded
    #[default]
    Expanded,
    /// Panel is collapsed to a tab/edge
    Collapsed,
    /// Panel is hidden (not rendered)
    Hidden,
    /// Panel is minimized to system tray/taskbar
    Minimized,
}

/// Information about a dockable panel
#[derive(Debug, Clone)]
pub struct PanelInfo {
    /// Unique panel identifier
    pub id: PanelId,
    /// Human-readable title
    pub title: String,
    /// Current dock area
    pub dock_area: DockArea,
    /// Current state
    pub state: PanelState,
    /// Width in pixels (for side panels) or percentage
    pub width: f32,
    /// Height in pixels (for horizontal panels) or percentage
    pub height: f32,
    /// Tab order within the dock area (0 = first)
    pub tab_order: usize,
    /// Whether panel can be closed
    pub closable: bool,
    /// Whether panel can be floated
    pub floatable: bool,
    /// Whether panel can be collapsed
    pub collapsible: bool,
    /// Whether panel is currently being dragged
    pub is_dragging: bool,
    /// Panel priority (higher = more important, shown first)
    pub priority: i32,
    /// Icon character (for collapsed state)
    pub icon: Option<char>,
    /// Tooltip shown on hover
    pub tooltip: Option<String>,
}

impl PanelInfo {
    /// Create a new panel info with default settings
    pub fn new(id: PanelId, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            dock_area: DockArea::Right,
            state: PanelState::Expanded,
            width: 300.0,
            height: 200.0,
            tab_order: 0,
            closable: true,
            floatable: true,
            collapsible: true,
            is_dragging: false,
            priority: 0,
            icon: None,
            tooltip: None,
        }
    }

    /// Set the dock area
    pub fn with_dock_area(mut self, area: DockArea) -> Self {
        self.dock_area = area;
        self
    }

    /// Set the panel state
    pub fn with_state(mut self, state: PanelState) -> Self {
        self.state = state;
        self
    }

    /// Set the width
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the height
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set whether the panel can be closed
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set whether the panel can be floated
    pub fn with_floatable(mut self, floatable: bool) -> Self {
        self.floatable = floatable;
        self
    }

    /// Set whether the panel can be collapsed
    pub fn with_collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set the icon
    pub fn with_icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set the tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Check if the panel is currently visible
    pub fn is_visible(&self) -> bool {
        matches!(self.state, PanelState::Expanded | PanelState::Collapsed)
    }

    /// Check if the panel is expanded
    pub fn is_expanded(&self) -> bool {
        self.state == PanelState::Expanded
    }

    /// Toggle between expanded and collapsed states
    pub fn toggle_collapsed(&mut self) {
        self.state = match self.state {
            PanelState::Expanded => PanelState::Collapsed,
            PanelState::Collapsed => PanelState::Expanded,
            other => other,
        };
    }

    /// Get the display title (truncated if too long)
    pub fn display_title(&self, max_len: usize) -> String {
        if self.title.len() > max_len {
            format!("{}...", &self.title[..max_len - 3])
        } else {
            self.title.clone()
        }
    }
}

impl Default for PanelInfo {
    fn default() -> Self {
        Self::new(PanelId::new(0), "Panel")
    }
}

/// Layout configuration for the docking system
#[derive(Debug, Clone)]
pub struct DockLayout {
    /// Panel configurations by ID
    pub panels: HashMap<PanelId, PanelInfo>,
    /// Width of left dock area (pixels)
    pub left_width: f32,
    /// Width of right dock area (pixels)
    pub right_width: f32,
    /// Height of top dock area (pixels)
    pub top_height: f32,
    /// Height of bottom dock area (pixels)
    pub bottom_height: f32,
    /// Minimum panel width
    pub min_panel_width: f32,
    /// Minimum panel height
    pub min_panel_height: f32,
    /// Whether to show panel tabs
    pub show_tabs: bool,
    /// Whether to allow floating panels
    pub allow_floating: bool,
    /// Auto-hide collapsed panels
    pub auto_hide_collapsed: bool,
    /// Tab height in pixels
    pub tab_height: f32,
    /// Collapsed panel width (for side panels)
    pub collapsed_width: f32,
}

impl Default for DockLayout {
    fn default() -> Self {
        Self {
            panels: HashMap::new(),
            left_width: 250.0,
            right_width: 300.0,
            top_height: 150.0,
            bottom_height: 200.0,
            min_panel_width: 150.0,
            min_panel_height: 100.0,
            show_tabs: true,
            allow_floating: true,
            auto_hide_collapsed: false,
            tab_height: 28.0,
            collapsed_width: 32.0,
        }
    }
}

impl DockLayout {
    /// Create a new empty layout
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a panel to the layout
    pub fn add_panel(&mut self, panel: PanelInfo) {
        self.panels.insert(panel.id, panel);
    }

    /// Remove a panel from the layout
    pub fn remove_panel(&mut self, panel_id: PanelId) -> Option<PanelInfo> {
        self.panels.remove(&panel_id)
    }

    /// Get a panel by ID
    pub fn get_panel(&self, panel_id: PanelId) -> Option<&PanelInfo> {
        self.panels.get(&panel_id)
    }

    /// Get a mutable panel by ID
    pub fn get_panel_mut(&mut self, panel_id: PanelId) -> Option<&mut PanelInfo> {
        self.panels.get_mut(&panel_id)
    }

    /// Get all panels in a specific dock area
    pub fn get_panels_in_area(&self, area: DockArea) -> Vec<&PanelInfo> {
        let mut panels: Vec<&PanelInfo> = self
            .panels
            .values()
            .filter(|p| p.dock_area == area && p.is_visible())
            .collect();
        panels.sort_by_key(|p| (p.priority, p.tab_order));
        panels
    }

    /// Get all visible panels sorted by dock area and tab order
    pub fn get_visible_panels(&self) -> Vec<&PanelInfo> {
        let mut panels: Vec<&PanelInfo> = self.panels.values().filter(|p| p.is_visible()).collect();
        panels.sort_by(|a, b| {
            let area_order = |area: DockArea| match area {
                DockArea::Left => 0,
                DockArea::Right => 1,
                DockArea::Top => 2,
                DockArea::Bottom => 3,
                DockArea::Center => 4,
                DockArea::Floating => 5,
            };
            area_order(a.dock_area)
                .cmp(&area_order(b.dock_area))
                .then_with(|| a.priority.cmp(&b.priority))
                .then_with(|| a.tab_order.cmp(&b.tab_order))
        });
        panels
    }

    /// Move a panel to a different dock area
    pub fn move_panel(&mut self, panel_id: PanelId, new_area: DockArea) -> Result<(), String> {
        // Get tab order before mutable borrow
        let tab_order = self.get_panels_in_area(new_area).len();

        if let Some(panel) = self.panels.get_mut(&panel_id) {
            panel.dock_area = new_area;
            // Reset tab order to end
            panel.tab_order = tab_order;
            Ok(())
        } else {
            Err(format!("Panel {:?} not found", panel_id))
        }
    }

    /// Toggle panel collapsed state
    pub fn toggle_panel(&mut self, panel_id: PanelId) -> Result<(), String> {
        if let Some(panel) = self.panels.get_mut(&panel_id) {
            panel.toggle_collapsed();
            Ok(())
        } else {
            Err(format!("Panel {:?} not found", panel_id))
        }
    }

    /// Show a hidden panel
    pub fn show_panel(&mut self, panel_id: PanelId) -> Result<(), String> {
        if let Some(panel) = self.panels.get_mut(&panel_id) {
            panel.state = PanelState::Expanded;
            Ok(())
        } else {
            Err(format!("Panel {:?} not found", panel_id))
        }
    }

    /// Hide a panel
    pub fn hide_panel(&mut self, panel_id: PanelId) -> Result<(), String> {
        if let Some(panel) = self.panels.get_mut(&panel_id) {
            panel.state = PanelState::Hidden;
            Ok(())
        } else {
            Err(format!("Panel {:?} not found", panel_id))
        }
    }

    /// Reorder panels within a dock area
    pub fn reorder_panels(&mut self, area: DockArea, new_order: &[PanelId]) -> Result<(), String> {
        for (idx, panel_id) in new_order.iter().enumerate() {
            if let Some(panel) = self.panels.get_mut(panel_id) {
                if panel.dock_area != area {
                    return Err(format!("Panel {:?} is not in {:?} area", panel_id, area));
                }
                panel.tab_order = idx;
            } else {
                return Err(format!("Panel {:?} not found", panel_id));
            }
        }
        Ok(())
    }

    /// Export layout to a serializable format
    pub fn export(&self) -> LayoutExport {
        LayoutExport {
            left_width: self.left_width,
            right_width: self.right_width,
            top_height: self.top_height,
            bottom_height: self.bottom_height,
            panels: self.panels.values().cloned().collect(),
        }
    }

    /// Import layout from exported format
    pub fn import(export: &LayoutExport) -> Self {
        let mut layout = Self {
            left_width: export.left_width,
            right_width: export.right_width,
            top_height: export.top_height,
            bottom_height: export.bottom_height,
            ..Default::default()
        };

        for panel in &export.panels {
            layout.add_panel(panel.clone());
        }

        layout
    }

    /// Reset to default layout
    pub fn reset_to_default(&mut self) {
        *self = Self::default();
    }

    /// Get available space for center content
    pub fn get_center_rect(&self, window_width: f32, window_height: f32) -> (f32, f32, f32, f32) {
        let left = if self.has_visible_side_panel(DockArea::Left) {
            self.left_width
        } else {
            0.0
        };

        let right = if self.has_visible_side_panel(DockArea::Right) {
            self.right_width
        } else {
            0.0
        };

        let top = if self.has_visible_horizontal_panel(DockArea::Top) {
            self.top_height
        } else {
            0.0
        };

        let bottom = if self.has_visible_horizontal_panel(DockArea::Bottom) {
            self.bottom_height
        } else {
            0.0
        };

        let x = left;
        let y = top;
        let width = window_width - left - right;
        let height = window_height - top - bottom;

        (x, y, width, height)
    }

    fn has_visible_side_panel(&self, area: DockArea) -> bool {
        self.panels
            .values()
            .any(|p| p.dock_area == area && p.is_visible())
    }

    fn has_visible_horizontal_panel(&self, area: DockArea) -> bool {
        self.panels
            .values()
            .any(|p| p.dock_area == area && p.is_visible())
    }
}

/// Serializable layout export format
#[derive(Debug, Clone)]
pub struct LayoutExport {
    pub left_width: f32,
    pub right_width: f32,
    pub top_height: f32,
    pub bottom_height: f32,
    pub panels: Vec<PanelInfo>,
}

/// Manages the docking system state and interactions
#[derive(Debug)]
pub struct DockingManager {
    /// Current layout configuration
    pub layout: DockLayout,
    /// Currently dragged panel (if any)
    pub dragged_panel: Option<PanelId>,
    /// Panel being hovered for drop target
    pub drop_target: Option<DockArea>,
    /// Drag start position
    pub drag_start: Option<(f32, f32)>,
    /// Current drag position
    pub drag_current: Option<(f32, f32)>,
    /// Animation state for smooth transitions
    pub animation_state: AnimationState,
    /// Whether dragging is currently active
    pub is_dragging: bool,
}

impl Default for DockingManager {
    fn default() -> Self {
        Self {
            layout: DockLayout::new(),
            dragged_panel: None,
            drop_target: None,
            drag_start: None,
            drag_current: None,
            animation_state: AnimationState::new(),
            is_dragging: false,
        }
    }
}

impl DockingManager {
    /// Create a new docking manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a docking manager with a predefined layout
    pub fn with_layout(layout: DockLayout) -> Self {
        Self {
            layout,
            ..Default::default()
        }
    }

    /// Start dragging a panel
    pub fn begin_drag(&mut self, panel_id: PanelId, x: f32, y: f32) {
        self.dragged_panel = Some(panel_id);
        self.drag_start = Some((x, y));
        self.drag_current = Some((x, y));
        self.is_dragging = true;

        if let Some(panel) = self.layout.get_panel_mut(panel_id) {
            panel.is_dragging = true;
        }
    }

    /// Update drag position
    pub fn update_drag(&mut self, x: f32, y: f32) -> Option<DockArea> {
        self.drag_current = Some((x, y));

        // Determine drop target based on position
        let target = self.calculate_drop_target(x, y);
        self.drop_target = target;
        target
    }

    /// End dragging and apply the drop
    pub fn end_drag(&mut self) -> Result<(), String> {
        if let (Some(panel_id), Some(target)) = (self.dragged_panel, self.drop_target) {
            self.layout.move_panel(panel_id, target)?;
        }

        // Reset drag state
        if let Some(panel_id) = self.dragged_panel {
            if let Some(panel) = self.layout.get_panel_mut(panel_id) {
                panel.is_dragging = false;
            }
        }

        self.dragged_panel = None;
        self.drop_target = None;
        self.drag_start = None;
        self.drag_current = None;
        self.is_dragging = false;

        Ok(())
    }

    /// Cancel current drag operation
    pub fn cancel_drag(&mut self) {
        if let Some(panel_id) = self.dragged_panel {
            if let Some(panel) = self.layout.get_panel_mut(panel_id) {
                panel.is_dragging = false;
            }
        }

        self.dragged_panel = None;
        self.drop_target = None;
        self.drag_start = None;
        self.drag_current = None;
        self.is_dragging = false;
    }

    /// Calculate drop target based on screen position
    fn calculate_drop_target(&self, _x: f32, _y: f32) -> Option<DockArea> {
        // This is a simplified calculation
        // In the actual EGUI implementation, we'd use the window dimensions
        // For now, return None (no drop target)
        None
    }

    /// Get the current drop target indicator
    pub fn get_drop_indicator(&self) -> Option<DropIndicator> {
        self.drop_target.map(|area| DropIndicator {
            area,
            highlight: true,
        })
    }

    /// Check if a panel is currently being dragged
    pub fn is_panel_dragged(&self, panel_id: PanelId) -> bool {
        self.dragged_panel == Some(panel_id)
    }

    /// Update animation state (call each frame)
    pub fn update_animation(&mut self, delta_time: f32) {
        self.animation_state.update(delta_time);
    }

    /// Get animation progress for a panel collapse/expand
    pub fn get_panel_animation(&self, panel_id: PanelId) -> f32 {
        self.animation_state.get_panel_progress(panel_id)
    }

    /// Save current layout
    pub fn save_layout(&self) -> LayoutExport {
        self.layout.export()
    }

    /// Load a layout
    pub fn load_layout(&mut self, export: &LayoutExport) {
        self.layout = DockLayout::import(export);
    }

    /// Reset to default layout
    pub fn reset_layout(&mut self) {
        self.layout.reset_to_default();
        self.animation_state = AnimationState::new();
    }
}

/// Animation state for smooth panel transitions
#[derive(Debug)]
pub struct AnimationState {
    /// Animation progress for each panel (0.0 = collapsed, 1.0 = expanded)
    panel_animations: HashMap<PanelId, f32>,
    /// Animation speed (units per second)
    #[allow(dead_code)]
    animation_speed: f32,
}

impl AnimationState {
    /// Create new animation state
    pub fn new() -> Self {
        Self {
            panel_animations: HashMap::new(),
            animation_speed: 8.0,
        }
    }

    /// Update animations
    pub fn update(&mut self, delta_time: f32) {
        // In a real implementation, we'd interpolate towards target values
        // For now, this is a placeholder
        let _ = delta_time;
    }

    /// Get animation progress for a panel
    pub fn get_panel_progress(&self, panel_id: PanelId) -> f32 {
        *self.panel_animations.get(&panel_id).unwrap_or(&1.0)
    }

    /// Set target animation state for a panel
    pub fn set_panel_target(&mut self, panel_id: PanelId, target: f32) {
        // In a real implementation, we'd interpolate over time
        self.panel_animations
            .insert(panel_id, target.clamp(0.0, 1.0));
    }
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Visual indicator for drop targets
#[derive(Debug, Clone, Copy)]
pub struct DropIndicator {
    /// Target dock area
    pub area: DockArea,
    /// Whether to show highlight
    pub highlight: bool,
}

/// Builder for creating panel docking configurations
#[derive(Debug, Default)]
pub struct DockingBuilder {
    panels: Vec<PanelInfo>,
    layout: DockLayout,
}

impl DockingBuilder {
    /// Create a new docking builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a panel with default right-side docking
    pub fn with_panel(mut self, panel: PanelInfo) -> Self {
        self.panels.push(panel);
        self
    }

    /// Add a panel to a specific dock area
    pub fn with_panel_in_area(
        mut self,
        id: PanelId,
        title: impl Into<String>,
        area: DockArea,
    ) -> Self {
        let panel = PanelInfo::new(id, title).with_dock_area(area);
        self.panels.push(panel);
        self
    }

    /// Set left panel width
    pub fn with_left_width(mut self, width: f32) -> Self {
        self.layout.left_width = width;
        self
    }

    /// Set right panel width
    pub fn with_right_width(mut self, width: f32) -> Self {
        self.layout.right_width = width;
        self
    }

    /// Set top panel height
    pub fn with_top_height(mut self, height: f32) -> Self {
        self.layout.top_height = height;
        self
    }

    /// Set bottom panel height
    pub fn with_bottom_height(mut self, height: f32) -> Self {
        self.layout.bottom_height = height;
        self
    }

    /// Build the docking manager
    pub fn build(self) -> DockingManager {
        let mut layout = self.layout;
        for panel in self.panels {
            layout.add_panel(panel);
        }
        DockingManager::with_layout(layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_id_creation() {
        let id = PanelId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_dock_area_default_ratio() {
        assert_eq!(DockArea::Left.default_ratio(), 0.25);
        assert_eq!(DockArea::Center.default_ratio(), 0.5);
        assert_eq!(DockArea::Floating.default_ratio(), 0.3);
    }

    #[test]
    fn test_panel_info_builder() {
        let panel = PanelInfo::new(PanelId::new(1), "Test Panel")
            .with_dock_area(DockArea::Left)
            .with_width(250.0)
            .with_closable(false)
            .with_icon('🔧');

        assert_eq!(panel.id.0, 1);
        assert_eq!(panel.title, "Test Panel");
        assert_eq!(panel.dock_area, DockArea::Left);
        assert_eq!(panel.width, 250.0);
        assert!(!panel.closable);
        assert_eq!(panel.icon, Some('🔧'));
    }

    #[test]
    fn test_panel_state_toggle() {
        let mut panel = PanelInfo::new(PanelId::new(1), "Test");
        assert!(panel.is_expanded());

        panel.toggle_collapsed();
        assert!(!panel.is_expanded());
        assert!(matches!(panel.state, PanelState::Collapsed));

        panel.toggle_collapsed();
        assert!(panel.is_expanded());
    }

    #[test]
    fn test_dock_layout_add_remove() {
        let mut layout = DockLayout::new();
        let panel = PanelInfo::new(PanelId::new(1), "Test Panel");

        layout.add_panel(panel);
        assert!(layout.get_panel(PanelId::new(1)).is_some());

        layout.remove_panel(PanelId::new(1));
        assert!(layout.get_panel(PanelId::new(1)).is_none());
    }

    #[test]
    fn test_dock_layout_move_panel() {
        let mut layout = DockLayout::new();
        let panel = PanelInfo::new(PanelId::new(1), "Test Panel").with_dock_area(DockArea::Left);
        layout.add_panel(panel);

        assert!(layout.move_panel(PanelId::new(1), DockArea::Right).is_ok());
        let panel = layout.get_panel(PanelId::new(1)).unwrap();
        assert_eq!(panel.dock_area, DockArea::Right);

        assert!(layout
            .move_panel(PanelId::new(999), DockArea::Left)
            .is_err());
    }

    #[test]
    fn test_dock_layout_export_import() {
        let mut layout = DockLayout::new();
        layout.add_panel(PanelInfo::new(PanelId::new(1), "Panel 1"));
        layout.add_panel(PanelInfo::new(PanelId::new(2), "Panel 2"));
        layout.left_width = 300.0;

        let export = layout.export();
        let imported = DockLayout::import(&export);

        assert_eq!(imported.left_width, 300.0);
        assert_eq!(imported.panels.len(), 2);
    }

    #[test]
    fn test_docking_manager_drag_drop() {
        let mut manager = DockingManager::new();
        manager
            .layout
            .add_panel(PanelInfo::new(PanelId::new(1), "Test"));

        assert!(!manager.is_dragging);

        manager.begin_drag(PanelId::new(1), 100.0, 100.0);
        assert!(manager.is_dragging);
        assert!(manager.is_panel_dragged(PanelId::new(1)));

        manager.cancel_drag();
        assert!(!manager.is_dragging);
        assert!(!manager.is_panel_dragged(PanelId::new(1)));
    }

    #[test]
    fn test_docking_builder() {
        let manager = DockingBuilder::new()
            .with_panel_in_area(PanelId::new(1), "Panel 1", DockArea::Left)
            .with_panel_in_area(PanelId::new(2), "Panel 2", DockArea::Right)
            .with_left_width(300.0)
            .build();

        assert_eq!(manager.layout.panels.len(), 2);
        assert_eq!(manager.layout.left_width, 300.0);

        let left_panels = manager.layout.get_panels_in_area(DockArea::Left);
        assert_eq!(left_panels.len(), 1);
        assert_eq!(left_panels[0].title, "Panel 1");
    }

    #[test]
    fn test_center_rect_calculation() {
        let mut layout = DockLayout::new();
        layout.left_width = 250.0;
        layout.right_width = 300.0;
        layout.top_height = 100.0;
        layout.bottom_height = 150.0;

        // No panels visible - should have full space
        let (x, y, w, h) = layout.get_center_rect(1920.0, 1080.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(w, 1920.0);
        assert_eq!(h, 1080.0);

        // Add panels to all areas
        layout.add_panel(PanelInfo::new(PanelId::new(1), "Left").with_dock_area(DockArea::Left));
        layout.add_panel(PanelInfo::new(PanelId::new(2), "Right").with_dock_area(DockArea::Right));
        layout.add_panel(PanelInfo::new(PanelId::new(3), "Top").with_dock_area(DockArea::Top));
        layout
            .add_panel(PanelInfo::new(PanelId::new(4), "Bottom").with_dock_area(DockArea::Bottom));

        let (x, y, w, h) = layout.get_center_rect(1920.0, 1080.0);
        assert_eq!(x, 250.0); // left_width
        assert_eq!(y, 100.0); // top_height
        assert_eq!(w, 1920.0 - 250.0 - 300.0); // window - left - right
        assert_eq!(h, 1080.0 - 100.0 - 150.0); // window - top - bottom
    }

    #[test]
    fn test_display_title_truncation() {
        let panel = PanelInfo::new(
            PanelId::new(1),
            "Very Long Panel Title That Should Be Truncated",
        );
        assert_eq!(panel.display_title(20), "Very Long Panel T...");
        assert_eq!(
            panel.display_title(50),
            "Very Long Panel Title That Should Be Truncated"
        );
    }
}
