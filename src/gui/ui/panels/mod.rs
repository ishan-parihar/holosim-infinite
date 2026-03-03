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
//! - Spectrum Continuum Panel (Phase B.2)
//! - Archetype Panel (Phase B.3)
//! - Quantum Panel (Phase C.1)
//! - Atomic Panel (Phase C.2)
//! - Molecular Panel (Phase C.3)
//! - Cellular Panel (Phase C.4)
//! - Consciousness Panel (Phase D.1)

pub mod archetype_panel;
pub mod atomic_panel;
pub mod cellular_panel;
pub mod collective_dashboard;
pub mod consciousness_panel;
pub mod docking;
pub mod emergence_dashboard;
pub mod entity_inspector;
pub mod entity_inspector_enhanced;
pub mod molecular_panel;
pub mod quantum_panel;
pub mod spectrum_continuum_panel;
pub mod spectrum_dashboard;
pub mod structure_dashboard;
pub mod time_controls;

pub use archetype_panel::{ArchetypePanel, ArchetypeViewMode};
pub use atomic_panel::{AtomicPanel, AtomicViewMode};
pub use cellular_panel::{CellularPanel, CellularViewMode};
pub use collective_dashboard::CollectiveDashboard;
pub use consciousness_panel::{ConsciousnessPanel, ConsciousnessViewMode};
pub use docking::{
    DockArea, DockLayout, DockingBuilder, DockingManager, PanelId, PanelInfo, PanelState,
};
pub use emergence_dashboard::{
    EmergenceDashboard, EmergenceSummary, EventStatistics, GraphData, GraphDataPoint,
};
pub use entity_inspector::EntityInspector;
pub use entity_inspector_enhanced::EntityInspectorEnhanced;
pub use molecular_panel::{MolecularPanel, MolecularViewMode};
pub use quantum_panel::{QuantumPanel, QuantumViewMode};
pub use spectrum_continuum_panel::{SpectrumContinuumPanel, SpectrumRegion, SpectrumStatistics};
pub use spectrum_dashboard::{SpectrumDashboard, SpectrumEntityData};
pub use structure_dashboard::StructureDashboard;
pub use time_controls::TimeControlsPanel;
