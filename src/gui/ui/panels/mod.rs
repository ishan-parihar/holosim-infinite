//! UI Panels - Dashboard and Inspection Panels
//!
//! Phase 6: Interactive Inspection
//!
//! Provides various dashboard and inspection panels:
//! - Entity Inspector (basic and enhanced)
//! - Collective Dynamics Dashboard
//! - Spectrum Dashboard
//! - Emergence Dashboard
//! - Structure Dashboard
//! - Time Controls Panel
//! - Docking system

pub mod collective_dashboard;
pub mod docking;
pub mod emergence_dashboard;
pub mod entity_inspector;
pub mod entity_inspector_enhanced;
pub mod spectrum_dashboard;
pub mod structure_dashboard;
pub mod time_controls;

pub use collective_dashboard::CollectiveDashboard;
pub use docking::{
    DockArea, DockLayout, DockingBuilder, DockingManager, PanelId, PanelInfo, PanelState,
};
pub use emergence_dashboard::{
    EmergenceDashboard, EmergenceSummary, EventStatistics, GraphData, GraphDataPoint,
};
pub use entity_inspector::EntityInspector;
pub use entity_inspector_enhanced::EntityInspectorEnhanced;
pub use spectrum_dashboard::{SpectrumDashboard, SpectrumEntityData};
pub use structure_dashboard::StructureDashboard;
pub use time_controls::TimeControlsPanel;
