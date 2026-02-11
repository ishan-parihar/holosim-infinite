pub mod collective_dashboard;
pub mod docking;
pub mod emergence_dashboard;
pub mod entity_inspector;
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
pub use spectrum_dashboard::{SpectrumDashboard, SpectrumEntityData};
pub use structure_dashboard::StructureDashboard;
pub use time_controls::TimeControlsPanel;
