//! GUI Application - Main Entry Point
//!
//! Integrates all GUI subsystems into a unified application.
//! This is the final integration layer for Phase 6.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Full integration of all visualization systems"
//!
//! PHASE 1 UPDATE: Added WGPU initialization and entity rendering
//!
//! PHASE A.3 UPDATE: Added keyboard shortcuts overlay and bookmark system

use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};

// Phase 1: Import WGPU context and entity renderer
use crate::gui::renderer::connection_renderer::ConnectionRenderer;
use crate::gui::renderer::cosmos_renderer::CosmosRenderer;
use crate::gui::renderer::entity_renderer::EntityRenderer;
use crate::gui::renderer::planet_renderer::PlanetRenderer;
use crate::gui::renderer::wgpu_context::WgpuContext;
// Phase B.1: Holographic field rendering
use crate::gui::renderer::{
    FieldVisualBridge, HolographicFieldConfig, HolographicFieldRenderer, VolumeDimensions,
};
use crate::gui::view_system::{ViewSystem, ViewType};
use crate::gui::{Camera2D, CameraControls};
use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::field_state::{
    FieldAmplitude, HolographicFieldState as FoundationFieldState, Position3D,
};
use crate::holographic_foundation::scale_level::ScaleLevel;
// Phase 2: Use SimulationRunnerAdapter instead of IntegratedSystem
use crate::gui::simulation_adapter::SimulationRunnerAdapter;

// Phase A.3: UI panels and docking
use crate::gui::ui::panels::{DockArea, DockingBuilder, PanelId, PanelInfo};
use crate::gui::ui::EguiIntegration;
// Phase B.2: Spectrum Continuum Panel
use crate::gui::ui::panels::SpectrumContinuumPanel;
// Phase B.3: Archetype Panel
use crate::gui::ui::panels::ArchetypePanel;
// Phase C.1: Quantum Panel
use crate::gui::ui::panels::QuantumPanel;
// Phase C.2: Atomic Panel
use crate::gui::ui::panels::AtomicPanel;
// Phase C.3: Molecular Panel
use crate::gui::ui::panels::MolecularPanel;
// Phase C.4: Cellular Panel
use crate::gui::ui::panels::CellularPanel;
// Phase D: Consciousness Panel
use crate::gui::ui::panels::ConsciousnessPanel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualUxMode {
    Legacy,
    SceneFirst,
    FieldEnhanced,
}

impl VisualUxMode {
    fn next(self) -> Self {
        match self {
            Self::Legacy => Self::SceneFirst,
            Self::SceneFirst => Self::FieldEnhanced,
            Self::FieldEnhanced => Self::Legacy,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Legacy => "Legacy",
            Self::SceneFirst => "Scene-First",
            Self::FieldEnhanced => "Field+",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SceneBenchmarkMode {
    Off,
    Clarity,
    Performance,
}

impl SceneBenchmarkMode {
    fn next(self) -> Self {
        match self {
            Self::Off => Self::Clarity,
            Self::Clarity => Self::Performance,
            Self::Performance => Self::Off,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Clarity => "Clarity",
            Self::Performance => "Performance",
        }
    }
}

const LENS_TOGGLE_KEYS_LABEL: &str = "F15 or D";
const LENS_HUD_KEYS_LABEL: &str = "F16 or R";
const LENS_PROFILE_TOGGLE_KEYS_LABEL: &str = "F19 or U";
const LENS_PROFILE_HUD_KEYS_LABEL: &str = "F20 or P";
const AUTO_FOCUS_TOGGLE_KEYS_LABEL: &str = "F17 or [";
const AUTO_FOCUS_HUD_KEYS_LABEL: &str = "F18 or ]";
const CONNECTION_COLOR_TOGGLE_KEYS_LABEL: &str = "F21 or -";
const CONNECTION_COLOR_HUD_KEYS_LABEL: &str = "F22 or =";

/// Main GUI Application
///
/// Orchestrates all subsystems:
/// - WGPU rendering (Phase 1: Now initialized!)
/// - EGUI UI
/// - Camera controls
/// - Input handling
/// - Simulation integration
/// - Visualization systems
pub struct GuiApplication {
    /// Window handle
    window: Arc<Window>,

    /// WGPU Context (Phase 1: Added)
    wgpu_context: Option<WgpuContext>,

    /// Entity Renderer (Phase 1: Added)
    entity_renderer: Option<EntityRenderer>,

    /// Connection Renderer (Phase 4: Hierarchy visualization)
    connection_renderer: Option<ConnectionRenderer>,

    /// Holographic Field Renderer (Phase B.1: Field visualization)
    holographic_field_renderer: Option<HolographicFieldRenderer>,

    /// Field Visual Bridge (Phase B.1: Field data conversion)
    field_visual_bridge: FieldVisualBridge,

    /// Enable holographic field visualization
    show_holographic_field: bool,

    /// Cosmos Renderer (Phase 3: Cosmic structures visualization)
    cosmos_renderer: Option<crate::gui::renderer::CosmosRenderer>,

    /// Enable cosmos visualization
    show_cosmos: bool,

    /// Planet Renderer (Phase 4: Planet surface visualization)
    #[allow(dead_code)]
    planet_renderer: Option<PlanetRenderer>,

    /// Enable planet surface visualization
    #[allow(dead_code)]
    show_planet_surface: bool,

    /// Selected planet ID for detail view
    #[allow(dead_code)]
    selected_planet_id: u64,

    /// Dual-mode benchmark pass for scene clarity/performance A/B checks
    scene_benchmark_mode: SceneBenchmarkMode,

    /// Show benchmark observability HUD
    show_benchmark_hud: bool,

    /// Visual UX mode for non-invasive rendering upgrades
    visual_ux_mode: VisualUxMode,

    /// Enable stage-coded entity morphology in shader
    stage_morphology_enabled: bool,

    /// Enable selection-linked focus halo in scene renderer
    selection_focus_halo: bool,

    /// View system for multi-level visualization
    view_system: ViewSystem,

    /// EGUI integration runtime
    egui_integration: Option<crate::gui::ui::EguiIntegration>,

    /// UI panels
    entity_inspector: crate::gui::ui::panels::EntityInspector,
    collective_dashboard: crate::gui::ui::panels::CollectiveDashboard,
    spectrum_dashboard: crate::gui::ui::panels::SpectrumDashboard,
    emergence_dashboard: crate::gui::ui::panels::EmergenceDashboard,
    time_controls: crate::gui::ui::panels::TimeControlsPanel,

    /// Spectrum Continuum Panel (Phase B.2)
    spectrum_continuum_panel: SpectrumContinuumPanel,

    /// Archetype Panel (Phase B.3)
    archetype_panel: ArchetypePanel,

    /// Quantum Panel (Phase C.1)
    quantum_panel: QuantumPanel,

    /// Atomic Panel (Phase C.2)
    atomic_panel: AtomicPanel,

    /// Molecular Panel (Phase C.3)
    molecular_panel: MolecularPanel,

    /// Cellular Panel (Phase C.4)
    cellular_panel: CellularPanel,

    /// Consciousness Panel (Phase D)
    consciousness_panel: ConsciousnessPanel,

    /// Panel docking state
    docking_manager: crate::gui::ui::panels::DockingManager,
    entity_inspector_panel_id: PanelId,
    collective_dashboard_panel_id: PanelId,
    spectrum_dashboard_panel_id: PanelId,
    emergence_dashboard_panel_id: PanelId,
    time_controls_panel_id: PanelId,
    spectrum_continuum_panel_id: PanelId,
    archetype_panel_id: PanelId,
    quantum_panel_id: PanelId,
    atomic_panel_id: PanelId,
    molecular_panel_id: PanelId,
    cellular_panel_id: PanelId,
    consciousness_panel_id: PanelId,

    /// Visualization backends used by UI panels
    collective_visualizer: crate::gui::visualization::collective_viz::CollectiveVisualizer,
    #[allow(dead_code)]
    emergence_visualizer: crate::gui::visualization::emergence_viz::EmergenceVisualizer,

    /// Camera system
    camera: Camera2D,

    /// Camera controls for user input
    camera_controls: CameraControls,

    /// Raycaster for click-to-inspect entity selection
    raycaster: crate::gui::interaction::Raycaster3D,

    /// Last known cursor position in screen space
    last_cursor_pos: Option<crate::gui::ScreenPosition>,

    /// Bookmark manager for camera positions
    bookmark_manager: crate::gui::interaction::BookmarkManager,

    /// Show keyboard shortcuts overlay
    show_shortcuts_overlay: bool,

    /// Show compact scene guide overlay
    show_scene_guide: bool,

    /// Show selection semantics overlay near viewport
    show_selection_semantics: bool,

    /// Show Legacy vs Scene-First compare overlay
    show_compare_overlay: bool,

    /// Show causal emergence glyph overlay
    show_emergence_glyphs: bool,

    /// Enable adaptive scene emphasis from live holographic metrics
    adaptive_scene_emphasis: bool,

    /// Enable stage-aware connection readability profile in non-legacy modes
    adaptive_connection_readability: bool,

    /// Enable selection-neighborhood focus lens for connection emphasis
    selection_neighborhood_lens: bool,

    /// Show compact neighborhood lens diagnostics HUD
    show_lens_hud: bool,

    /// Enable adaptive selection-neighborhood lens profile tuning
    adaptive_lens_profile: bool,

    /// Show compact adaptive lens profile diagnostics HUD
    show_lens_profile_hud: bool,

    /// Enable guided auto-focus context nudging
    guided_auto_focus: bool,

    /// Show compact auto-focus diagnostics HUD
    show_auto_focus_hud: bool,

    /// Smoothing strength for guided auto-focus transitions
    auto_focus_strength: f32,

    /// Lens radius in normalized render space
    lens_radius: f32,

    /// Base scale for non-focused links under lens
    lens_non_focus_scale: f32,

    /// Additive boost for focused links under lens
    lens_edge_boost: f32,

    /// Show compact link readability HUD overlay
    show_connection_hud: bool,

    /// Enable stage-aware connection color grading profile in non-legacy modes
    adaptive_connection_color_grading: bool,

    /// Show compact connection color grading HUD overlay
    show_connection_color_hud: bool,

    /// Enable camera/context-aware decluttering for connection layers
    adaptive_connection_declutter: bool,

    /// Show compact declutter diagnostics HUD
    show_declutter_hud: bool,

    /// Enable stage-driven holographic field quality profile
    adaptive_field_quality: bool,

    /// Show compact field quality diagnostics HUD
    show_field_quality_hud: bool,

    /// Show compact adaptive emphasis HUD overlay
    show_emphasis_overlay: bool,

    /// Enable guided exploration story mode presets
    guided_story_mode: bool,

    /// Show guided exploration story overlay
    show_story_overlay: bool,

    /// Enable context-sensitive overlay orchestration
    auto_overlay_orchestration: bool,

    /// Show compact overlay policy HUD
    show_overlay_policy_hud: bool,

    /// Enable camera-aware semantic level-of-detail tuning
    semantic_lod_enabled: bool,

    /// Show compact semantic LOD diagnostics HUD
    show_semantic_lod_hud: bool,

    /// Simulation integration (Phase 2: Using SimulationRunnerAdapter)
    simulation: SimulationRunnerAdapter,

    /// Hierarchy navigation: Currently focused entity (what we're "inside")
    /// None means we're at the root/universe level
    hierarchy_focus_id: Option<crate::entity_layer7::EntityId>,

    /// Hierarchy navigation: Stack of parent entities for "back" navigation
    hierarchy_stack: Vec<crate::entity_layer7::EntityId>,

    /// Hierarchy navigation: Show hierarchy path overlay
    show_hierarchy_path: bool,

    /// Application state
    #[allow(dead_code)]
    config: GuiConfig,
    running: bool,
    last_frame_time: Instant,
    frame_count: u64,
    fps: f32,

    /// Performance metrics
    render_stats: RenderStats,
    target_frame_time: Duration,
}

impl GuiApplication {
    /// Create new GUI application
    pub async fn new(event_loop: &EventLoop<()>, config: GuiConfig) -> Result<Self, String> {
        println!("Initializing GUI Application...");

        // Create window
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Holonic Realms - Cosmological Simulation")
                .with_inner_size(PhysicalSize::new(config.window_width, config.window_height))
                .with_resizable(true)
                .build(event_loop)
                .map_err(|e| format!("Failed to create window: {}", e))?,
        );

        println!("Window created successfully");

        // Phase 1: Initialize WGPU context
        println!("Initializing WGPU context...");
        let (
            wgpu_context,
            entity_renderer,
            connection_renderer,
            holographic_field_renderer,
            cosmos_renderer,
            planet_renderer,
            view_system,
            egui_integration,
        ) = match WgpuContext::new(window.clone()).await {
            Ok(ctx) => {
                println!("✓ WGPU context initialized successfully");

                // Phase 1: Create entity renderer
                println!("Creating entity renderer...");
                let renderer = EntityRenderer::new(&ctx.device, &ctx.surface_config);
                println!("✓ Entity renderer created");

                // Phase 4: Create connection renderer
                println!("Creating connection renderer...");
                let conn_renderer = ConnectionRenderer::new(&ctx.device, &ctx.surface_config);
                println!("✓ Connection renderer created");

                // Phase B.1: Initialize holographic field renderer
                println!("Creating holographic field renderer...");
                let field_config = HolographicFieldConfig::default();
                let field_renderer = Some(HolographicFieldRenderer::new(
                    &ctx.device,
                    ctx.surface_config.format,
                    field_config,
                ));
                println!("✓ Holographic field renderer created");

                // Phase 3: Create cosmos renderer
                println!("Creating cosmos renderer...");
                let cosmos_renderer = Some(CosmosRenderer::new(&ctx.device, &ctx.surface_config));
                println!("✓ Cosmos renderer created");

                // Phase 4: Create planet renderer
                println!("Creating planet renderer...");
                let planet_renderer = Some(PlanetRenderer::new(&ctx.device, &ctx.surface_config));
                println!("✓ Planet renderer created");

                println!("Creating view system...");
                let view_system = ViewSystem::new();
                println!("✓ View system created");

                // Initialize EGUI runtime
                let egui_integration =
                    EguiIntegration::new(window.as_ref(), &ctx.device, ctx.surface_config.format);
                println!("✓ EGUI integration initialized");

                (
                    Some(ctx),
                    Some(renderer),
                    Some(conn_renderer),
                    field_renderer,
                    cosmos_renderer,
                    planet_renderer,
                    Some(view_system),
                    Some(egui_integration),
                )
            }
            Err(e) => {
                eprintln!("⚠ Failed to initialize WGPU: {}", e);
                eprintln!("Continuing without GPU rendering...");
                let none_view: Option<ViewSystem> = None;
                (None, None, None, None, None, None, none_view, None)
            }
        };

        // Phase B.1: Initialize field visual bridge (always, even without WGPU)
        let field_visual_bridge = FieldVisualBridge::new(VolumeDimensions::cube(64));

        // Initialize camera
        let camera = Camera2D::new(1920.0 / 1080.0);

        // Initialize camera controls
        let camera_controls = CameraControls::new();

        // Initialize raycaster with current window size and camera
        let mut raycaster =
            crate::gui::interaction::Raycaster3D::new(config.window_width, config.window_height);
        raycaster.set_camera(&camera);

        // Initialize bookmark manager
        let bookmark_manager = crate::gui::interaction::BookmarkManager::new();

        // Initialize simulation (Phase 2: Using SimulationRunnerAdapter)
        println!("Initializing simulation with SimulationRunnerAdapter...");
        let simulation = SimulationRunnerAdapter::new();
        let mut simulation = simulation;
        simulation.initialize();

        // Create panel IDs
        let entity_inspector_panel_id = PanelId::new(1);
        let collective_dashboard_panel_id = PanelId::new(2);
        let spectrum_dashboard_panel_id = PanelId::new(3);
        let emergence_dashboard_panel_id = PanelId::new(4);
        let time_controls_panel_id = PanelId::new(5);
        let spectrum_continuum_panel_id = PanelId::new(6);
        let archetype_panel_id = PanelId::new(7);
        let quantum_panel_id = PanelId::new(8);
        let atomic_panel_id = PanelId::new(9);
        let molecular_panel_id = PanelId::new(10);
        let cellular_panel_id = PanelId::new(11);
        let consciousness_panel_id = PanelId::new(12);

        // Phase B.2: Initialize Spectrum Continuum Panel
        let spectrum_continuum_panel = SpectrumContinuumPanel::new();

        // Phase B.3: Initialize Archetype Panel
        let archetype_panel = ArchetypePanel::new();

        // Phase C.1: Initialize Quantum Panel
        let quantum_panel = QuantumPanel::new();

        // Phase C.2: Initialize Atomic Panel
        let atomic_panel = AtomicPanel::new();

        // Phase C.3: Initialize Molecular Panel
        let molecular_panel = MolecularPanel::new();

        // Phase C.4: Initialize Cellular Panel
        let cellular_panel = CellularPanel::new();

        // Phase D: Initialize Consciousness Panel
        let consciousness_panel = ConsciousnessPanel::new();

        // Build default docking manager
        let docking_manager = Self::build_default_docking_manager(
            entity_inspector_panel_id,
            collective_dashboard_panel_id,
            spectrum_dashboard_panel_id,
            emergence_dashboard_panel_id,
            time_controls_panel_id,
            spectrum_continuum_panel_id,
            archetype_panel_id,
            quantum_panel_id,
            atomic_panel_id,
            molecular_panel_id,
            cellular_panel_id,
            consciousness_panel_id,
        );

        println!("✓ GUI Application initialized successfully");

        Ok(Self {
            window,
            wgpu_context,
            entity_renderer,
            connection_renderer,          // Phase 4: Add connection renderer
            holographic_field_renderer,   // Phase B.1: Holographic field renderer
            field_visual_bridge,          // Phase B.1: Field visual bridge
            show_holographic_field: true, // Scene-first onboarding default
            cosmos_renderer,              // Phase 3: Cosmos renderer
            show_cosmos: true,            // Enable cosmos visualization by default
            planet_renderer,              // Phase 4: Planet surface renderer
            show_planet_surface: false,   // Disabled by default (enable when zoomed to planet)
            selected_planet_id: 0,        // Default planet ID
            scene_benchmark_mode: SceneBenchmarkMode::Off,
            show_benchmark_hud: false,
            visual_ux_mode: VisualUxMode::SceneFirst,
            stage_morphology_enabled: true,
            selection_focus_halo: true,
            egui_integration,
            entity_inspector: crate::gui::ui::panels::EntityInspector::new(),
            collective_dashboard: crate::gui::ui::panels::CollectiveDashboard::new(),
            spectrum_dashboard: crate::gui::ui::panels::SpectrumDashboard::new(),
            emergence_dashboard: crate::gui::ui::panels::EmergenceDashboard::new(),
            time_controls: crate::gui::ui::panels::TimeControlsPanel::default(),
            spectrum_continuum_panel,
            archetype_panel,
            quantum_panel,
            atomic_panel,
            molecular_panel,
            cellular_panel,
            consciousness_panel,
            docking_manager,
            entity_inspector_panel_id,
            collective_dashboard_panel_id,
            spectrum_dashboard_panel_id,
            emergence_dashboard_panel_id,
            time_controls_panel_id,
            spectrum_continuum_panel_id,
            archetype_panel_id,
            quantum_panel_id,
            atomic_panel_id,
            molecular_panel_id,
            cellular_panel_id,
            consciousness_panel_id,
            collective_visualizer:
                crate::gui::visualization::collective_viz::CollectiveVisualizer::new(),
            emergence_visualizer:
                crate::gui::visualization::emergence_viz::EmergenceVisualizer::new(),
            camera,
            camera_controls,
            raycaster,
            last_cursor_pos: None,
            bookmark_manager,
            show_shortcuts_overlay: false,
            show_scene_guide: true,
            show_selection_semantics: true,
            show_compare_overlay: false,
            show_emergence_glyphs: true,
            adaptive_scene_emphasis: true,
            adaptive_connection_readability: true,
            selection_neighborhood_lens: true,
            show_lens_hud: false,
            adaptive_lens_profile: true,
            show_lens_profile_hud: false,
            guided_auto_focus: true,
            show_auto_focus_hud: false,
            auto_focus_strength: 0.12,
            lens_radius: 0.45,
            lens_non_focus_scale: 0.45,
            lens_edge_boost: 0.08,
            show_emphasis_overlay: false,
            show_connection_hud: false,
            adaptive_connection_color_grading: true,
            show_connection_color_hud: false,
            adaptive_connection_declutter: true,
            show_declutter_hud: false,
            adaptive_field_quality: true,
            show_field_quality_hud: false,
            guided_story_mode: true,
            show_story_overlay: true,
            auto_overlay_orchestration: true,
            show_overlay_policy_hud: false,
            semantic_lod_enabled: true,
            show_semantic_lod_hud: false,
            simulation,
            hierarchy_focus_id: None,
            hierarchy_stack: Vec::new(),
            show_hierarchy_path: true,
            config,
            running: false,
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            render_stats: RenderStats::default(),
            target_frame_time: Duration::from_secs_f64(1.0 / 60.0),
            view_system: view_system.unwrap_or_default(),
        })
    }

    /// Run the application event loop
    pub fn run(mut self, event_loop: EventLoop<()>) {
        self.running = true;

        let _ = event_loop.run(move |event, _window_target| {
            match event {
                Event::NewEvents(_) => {
                    // Calculate frame time
                    let now = Instant::now();
                    let frame_time = now.duration_since(self.last_frame_time);
                    self.last_frame_time = now;

                    // Update FPS
                    self.frame_count += 1;
                    if self.frame_count.is_multiple_of(30) {
                        self.fps = 1.0 / frame_time.as_secs_f32();
                    }

                    // Update render stats
                    self.render_stats.frame_time_ms = frame_time.as_secs_f64() * 1000.0;
                    self.render_stats.fps = self.fps;

                    // Update bookmark transitions
                    self.bookmark_manager
                        .update_transition(frame_time.as_secs_f32(), &mut self.camera);
                }

                Event::WindowEvent { event, .. } => {
                    // Handle EGUI events first
                    if let Some(ref mut egui) = self.egui_integration {
                        let _ = egui.handle_event(self.window.as_ref(), &event);
                    }

                    match event {
                        WindowEvent::CloseRequested => {
                            println!("Close requested, shutting down...");
                            _window_target.exit();
                        }

                        WindowEvent::Resized(size) => {
                            println!("Window resized: {}x{}", size.width, size.height);
                            // Phase 1: Resize WGPU surface if available
                            if let Some(ref mut ctx) = self.wgpu_context {
                                ctx.resize(size.width, size.height);
                            }
                            // Update camera aspect ratio
                            let aspect_ratio = size.width as f32 / size.height as f32;
                            self.camera.set_aspect_ratio(aspect_ratio);
                            // Update raycaster
                            self.raycaster.set_screen_size(size.width, size.height);
                        }

                        WindowEvent::CursorMoved { position, .. } => {
                            self.last_cursor_pos = Some(crate::gui::ScreenPosition::new(
                                position.x as f32,
                                position.y as f32,
                            ));
                        }

                        WindowEvent::MouseInput { state, button, .. } => {
                            if state == ElementState::Pressed && button == MouseButton::Left {
                                if let Some(screen_pos) = self.last_cursor_pos {
                                    self.handle_entity_click(screen_pos);
                                }
                            }
                        }

                        WindowEvent::KeyboardInput { event, .. } => {
                            if event.state == ElementState::Pressed {
                                match event.physical_key {
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => {
                                        _window_target.exit();
                                    }
                                    // Phase 5: View switching with keys 1-6
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Digit1) => {
                                        self.switch_view(ViewType::Overview);
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Digit2) => {
                                        self.switch_view(ViewType::Hierarchy);
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Digit3) => {
                                        self.switch_view(ViewType::Realm);
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Digit4) => {
                                        self.switch_view(ViewType::Archetype);
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Digit5) => {
                                        self.switch_view(ViewType::Spectrum);
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Digit6) => {
                                        self.switch_view(ViewType::Evolution);
                                    }
                                    // F1: Show keyboard shortcuts overlay
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F1) => {
                                        self.show_shortcuts_overlay = !self.show_shortcuts_overlay;
                                    }
                                    // F2: Toggle Entity Inspector
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F2) => {
                                        self.toggle_panel(self.entity_inspector_panel_id);
                                    }
                                    // F3: Toggle Collective Dashboard
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F3) => {
                                        self.toggle_panel(self.collective_dashboard_panel_id);
                                    }
                                    // F4: Toggle Spectrum Dashboard
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F4) => {
                                        self.toggle_panel(self.spectrum_dashboard_panel_id);
                                    }
                                    // F5: Toggle Emergence Dashboard
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F5) => {
                                        self.toggle_panel(self.emergence_dashboard_panel_id);
                                    }
                                    // F6: Toggle Time Controls
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F6) => {
                                        self.toggle_panel(self.time_controls_panel_id);
                                    }
                                    // F7: Reset view to home
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F7) => {
                                        self.load_bookmark(0); // Load home bookmark
                                    }
                                    // F8: Toggle adaptive connection declutter
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F8) => {
                                        self.adaptive_connection_declutter =
                                            !self.adaptive_connection_declutter;
                                    }
                                    // F9: Toggle declutter HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F9) => {
                                        self.show_declutter_hud = !self.show_declutter_hud;
                                    }
                                    // F10: Toggle adaptive field quality
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F10) => {
                                        self.adaptive_field_quality = !self.adaptive_field_quality;
                                    }
                                    // F11: Toggle field quality HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F11) => {
                                        self.show_field_quality_hud = !self.show_field_quality_hud;
                                    }
                                    // F12: Cycle benchmark mode
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F12) => {
                                        self.scene_benchmark_mode =
                                            self.scene_benchmark_mode.next();
                                        println!(
                                            "Scene benchmark mode: {}",
                                            self.scene_benchmark_mode.label()
                                        );
                                    }
                                    // F13: Toggle semantic LOD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F13) => {
                                        self.semantic_lod_enabled = !self.semantic_lod_enabled;
                                    }
                                    // F14: Toggle semantic LOD HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F14) => {
                                        self.show_semantic_lod_hud = !self.show_semantic_lod_hud;
                                    }
                                    // F15: Toggle selection neighborhood lens
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F15) => {
                                        self.selection_neighborhood_lens =
                                            !self.selection_neighborhood_lens;
                                    }
                                    // F16: Toggle neighborhood lens HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F16) => {
                                        self.show_lens_hud = !self.show_lens_hud;
                                    }
                                    // F19: Toggle adaptive lens profile
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F19) => {
                                        self.adaptive_lens_profile = !self.adaptive_lens_profile;
                                    }
                                    // F20: Toggle lens profile HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F20) => {
                                        self.show_lens_profile_hud = !self.show_lens_profile_hud;
                                    }
                                    // F21: Toggle adaptive connection color grading
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F21) => {
                                        self.adaptive_connection_color_grading =
                                            !self.adaptive_connection_color_grading;
                                    }
                                    // F22: Toggle connection color grading HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F22) => {
                                        self.show_connection_color_hud =
                                            !self.show_connection_color_hud;
                                    }
                                    // Fallback: E toggles semantic LOD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyE) => {
                                        self.semantic_lod_enabled = !self.semantic_lod_enabled;
                                    }
                                    // Fallback: W toggles semantic LOD HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyW) => {
                                        self.show_semantic_lod_hud = !self.show_semantic_lod_hud;
                                    }
                                    // Fallback: D toggles selection neighborhood lens
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyD) => {
                                        self.selection_neighborhood_lens =
                                            !self.selection_neighborhood_lens;
                                    }
                                    // Fallback: R toggles neighborhood lens HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyR) => {
                                        self.show_lens_hud = !self.show_lens_hud;
                                    }
                                    // F17: Toggle guided auto-focus
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F17) => {
                                        self.guided_auto_focus = !self.guided_auto_focus;
                                    }
                                    // F18: Toggle auto-focus HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::F18) => {
                                        self.show_auto_focus_hud = !self.show_auto_focus_hud;
                                    }
                                    // Fallback: [ toggles guided auto-focus
                                    winit::keyboard::PhysicalKey::Code(KeyCode::BracketLeft) => {
                                        self.guided_auto_focus = !self.guided_auto_focus;
                                    }
                                    // Fallback: ] toggles auto-focus HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::BracketRight) => {
                                        self.show_auto_focus_hud = !self.show_auto_focus_hud;
                                    }
                                    // `: Toggle benchmark HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Backquote) => {
                                        self.show_benchmark_hud = !self.show_benchmark_hud;
                                    }
                                    // Phase B.1: Toggle holographic field visibility
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyF) => {
                                        self.show_holographic_field = !self.show_holographic_field;
                                        println!(
                                            "Holographic field: {}",
                                            if self.show_holographic_field {
                                                "ON"
                                            } else {
                                                "OFF"
                                            }
                                        );
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyV) => {
                                        self.visual_ux_mode = self.visual_ux_mode.next();
                                        println!("Visual UX mode: {}", self.visual_ux_mode.label());
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyG) => {
                                        self.show_scene_guide = !self.show_scene_guide;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyB) => {
                                        self.show_compare_overlay = !self.show_compare_overlay;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyN) => {
                                        self.show_emergence_glyphs = !self.show_emergence_glyphs;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyJ) => {
                                        self.adaptive_scene_emphasis =
                                            !self.adaptive_scene_emphasis;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyK) => {
                                        self.show_emphasis_overlay = !self.show_emphasis_overlay;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyX) => {
                                        self.adaptive_connection_readability =
                                            !self.adaptive_connection_readability;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyT) => {
                                        self.show_connection_hud = !self.show_connection_hud;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyL) => {
                                        self.stage_morphology_enabled =
                                            !self.stage_morphology_enabled;
                                        println!(
                                            "Stage morphology: {}",
                                            if self.stage_morphology_enabled {
                                                "ON"
                                            } else {
                                                "OFF"
                                            }
                                        );
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyY) => {
                                        self.show_selection_semantics =
                                            !self.show_selection_semantics;
                                    }
                                    // Fallback: P toggles lens profile HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyP) => {
                                        self.show_lens_profile_hud = !self.show_lens_profile_hud;
                                    }
                                    // Fallback: - toggles adaptive connection color grading
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Minus) => {
                                        self.adaptive_connection_color_grading =
                                            !self.adaptive_connection_color_grading;
                                    }
                                    // Fallback: = toggles connection color grading HUD
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Equal) => {
                                        self.show_connection_color_hud =
                                            !self.show_connection_color_hud;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyU) => {
                                        self.adaptive_lens_profile = !self.adaptive_lens_profile;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyO) => {
                                        self.auto_overlay_orchestration =
                                            !self.auto_overlay_orchestration;
                                    }
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyI) => {
                                        self.show_overlay_policy_hud =
                                            !self.show_overlay_policy_hud;
                                    }
                                    // Phase B.2: Toggle Spectrum Continuum Panel
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyS) => {
                                        self.toggle_panel(self.spectrum_continuum_panel_id);
                                    }
                                    // Phase B.3: Toggle Archetype Panel
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyA) => {
                                        self.toggle_panel(self.archetype_panel_id);
                                    }
                                    // Phase C.1: Toggle Quantum Panel
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyQ) => {
                                        self.toggle_panel(self.quantum_panel_id);
                                    }
                                    // Phase C.2: Toggle Atomic Panel
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyZ) => {
                                        self.toggle_panel(self.atomic_panel_id);
                                    }
                                    // Phase C.3: Toggle Molecular Panel
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyM) => {
                                        self.toggle_panel(self.molecular_panel_id);
                                    }
                                    // Phase C.4: Toggle Cellular Panel
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyC) => {
                                        self.toggle_panel(self.cellular_panel_id);
                                    }
                                    // Phase D: Toggle Consciousness Panel
                                    winit::keyboard::PhysicalKey::Code(KeyCode::KeyH) => {
                                        self.toggle_panel(self.consciousness_panel_id);
                                    }
                                    // Hierarchy Navigation: Enter = drill down into selected entity
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Enter) => {
                                        self.drill_down_into_selected();
                                    }
                                    // Hierarchy Navigation: Backspace = go back up one level
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Backspace) => {
                                        self.navigate_back_hierarchy();
                                    }
                                    // Hierarchy Navigation: H = toggle hierarchy path overlay
                                    winit::keyboard::PhysicalKey::Code(KeyCode::Slash) => {
                                        self.show_hierarchy_path = !self.show_hierarchy_path;
                                        println!(
                                            "Hierarchy path overlay: {}",
                                            if self.show_hierarchy_path { "ON" } else { "OFF" }
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }

                        WindowEvent::RedrawRequested => {
                            // Phase 1: Render frame using WGPU
                            if let Err(e) = self.render_frame() {
                                eprintln!("Render error: {:?}", e);
                            }

                            // Update render stats
                            self.render_stats.frame_count += 1;

                            // Sleep to maintain target frame rate
                            let elapsed = self.last_frame_time.elapsed();
                            if elapsed < self.target_frame_time {
                                std::thread::sleep(self.target_frame_time - elapsed);
                            }
                        }

                        _ => {
                            // Camera controls
                            self.camera_controls.handle_event(&event, &mut self.camera);
                        }
                    }
                }

                Event::AboutToWait => {
                    if self.running {
                        // Run simulation step
                        if let Err(e) = self.run_step() {
                            eprintln!("Simulation step failed: {:?}", e);
                        }

                        // Request redraw
                        self.window.request_redraw();
                    }
                }

                Event::LoopExiting => {
                    println!("Event loop exiting");
                    self.simulation.shutdown();
                }

                _ => {}
            }
        });
    }

    /// Phase 1: Render a frame using WGPU
    fn render_frame(&mut self) -> Result<(), String> {
        // Check if WGPU context exists, return early if not
        if self.wgpu_context.is_none() {
            return Ok(());
        }

        let holo_stats = self.simulation.get_holo_statistics();

        let normalize_count = |value: usize, cap: usize| -> f32 {
            if cap == 0 {
                return 0.0;
            }
            (value as f32 / cap as f32).clamp(0.0, 1.0)
        };

        let mut stage_scores = [0.0_f32; 6];
        if let Some(stats) = holo_stats.as_ref() {
            stage_scores[0] = 0.5
                * (normalize_count(stats.quantum_regions, 256)
                    + normalize_count(stats.particles, 50_000));
            stage_scores[1] = 0.5
                * (normalize_count(stats.atomic_regions, 256)
                    + normalize_count(stats.atoms, 25_000));
            stage_scores[2] = 0.5
                * (normalize_count(stats.molecular_regions, 256)
                    + normalize_count(stats.molecules, 12_000));
            stage_scores[3] =
                0.5 * (normalize_count(stats.cells, 5_000) + normalize_count(stats.species, 1_000));
            stage_scores[4] = 0.5
                * (normalize_count(stats.planets, 256)
                    + normalize_count(stats.terrain_cells, 20_000));
            stage_scores[5] = 0.5
                * ((stats.integration_coherence as f32).clamp(0.0, 1.0)
                    + (stats.integration_stability as f32).clamp(0.0, 1.0));
        }

        let mut dominant_stage_index = 0_usize;
        for i in 1..stage_scores.len() {
            if stage_scores[i] > stage_scores[dominant_stage_index] {
                dominant_stage_index = i;
            }
        }

        self.render_stats.emphasis_stage = dominant_stage_index as u8;

        let in_non_legacy_mode = matches!(
            self.visual_ux_mode,
            VisualUxMode::SceneFirst | VisualUxMode::FieldEnhanced
        );

        let (base_phase_threshold, field_emphasis, connection_emphasis) =
            if self.adaptive_scene_emphasis && in_non_legacy_mode {
                self.show_holographic_field = true;
                let threshold = match dominant_stage_index {
                    0 => 0.62,
                    1 => 0.68,
                    2 => 0.72,
                    3 => 0.78,
                    4 => 0.84,
                    _ => 0.88,
                };
                let field = match dominant_stage_index {
                    0 => 0.40,
                    1 => 0.48,
                    2 => 0.56,
                    3 => 0.68,
                    4 => 0.80,
                    _ => 0.92,
                };
                let connection = match dominant_stage_index {
                    0 => 0.90,
                    1 => 0.82,
                    2 => 0.74,
                    3 => 0.64,
                    4 => 0.54,
                    _ => 0.44,
                };
                (threshold, field, connection)
            } else {
                (0.70, 0.50, 0.50)
            };

        self.render_stats.phase_threshold_used = base_phase_threshold;
        self.render_stats.field_emphasis = field_emphasis;
        self.render_stats.connection_emphasis = connection_emphasis;

        // Phase 2 Week 3: Update entity renderer from selected visualization path
        let legacy_entities = self.simulation.entities();
        let holo_entities = self.simulation.holo_entities();
        self.render_stats.legacy_entity_count = legacy_entities.len() as u64;
        self.render_stats.holo_entity_count = holo_entities.len() as u64;

        let use_field_derived_entities = matches!(
            self.visual_ux_mode,
            VisualUxMode::SceneFirst | VisualUxMode::FieldEnhanced
        );

        let (base_structural_scale, base_phase_scale, base_intensity_bias) =
            if self.adaptive_connection_readability {
                match dominant_stage_index {
                    0 => (0.78, 1.22, 0.02),
                    1 => (0.88, 1.10, 0.02),
                    2 => (1.00, 1.00, 0.01),
                    3 => (1.08, 0.92, 0.01),
                    4 => (1.16, 0.84, 0.00),
                    _ => (1.22, 0.78, 0.00),
                }
            } else {
                (1.0, 1.0, 0.0)
            };

        let camera_zoom = self.camera.zoom;
        let zoom_range = self.camera.max_zoom - self.camera.min_zoom;
        let zoom_norm = if zoom_range.abs() > f32::EPSILON {
            ((camera_zoom - self.camera.min_zoom) / zoom_range).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let has_selection = self.entity_inspector.selected_entity_id.is_some();
        let focus_weight = if has_selection { 1.0 } else { 0.0 };
        let semantic_lod_level = if self.semantic_lod_enabled && in_non_legacy_mode {
            let overview_score = (1.0 - zoom_norm) * (1.0 - 0.6 * focus_weight);
            if overview_score >= 0.66 {
                2
            } else if overview_score >= 0.33 {
                1
            } else {
                0
            }
        } else {
            0
        };
        let zoomed_out_signal = 1.0 - zoom_norm;

        let (
            mut conn_color_warm_cool_bias,
            mut conn_color_saturation,
            mut conn_color_phase_boost,
            mut conn_color_grade_strength,
        ): (f32, f32, f32, f32) = if self.adaptive_connection_color_grading && in_non_legacy_mode {
            match dominant_stage_index {
                0 => (0.10, 1.18, 0.28, 0.85),
                1 => (0.22, 1.12, 0.22, 0.78),
                2 => (0.40, 1.04, 0.16, 0.68),
                3 => (0.58, 0.98, 0.10, 0.62),
                4 => (0.74, 0.92, 0.06, 0.56),
                _ => (0.86, 0.88, 0.04, 0.50),
            }
        } else {
            (0.50, 1.0, 0.0, 0.0)
        };

        if self.adaptive_connection_color_grading && in_non_legacy_mode {
            conn_color_saturation *= 1.0 - (0.08 * zoomed_out_signal);
        }

        conn_color_warm_cool_bias = conn_color_warm_cool_bias.clamp(0.0, 1.0);
        conn_color_saturation = conn_color_saturation.clamp(0.5, 1.8);
        conn_color_phase_boost = conn_color_phase_boost.clamp(0.0, 0.6);
        conn_color_grade_strength = conn_color_grade_strength.clamp(0.0, 1.0);

        self.render_stats.conn_color_warm_cool_bias = conn_color_warm_cool_bias;
        self.render_stats.conn_color_saturation = conn_color_saturation;
        self.render_stats.conn_color_phase_boost = conn_color_phase_boost;
        self.render_stats.conn_color_grade_strength = conn_color_grade_strength;

        let (field_quality_stage, field_steps_used, field_density_used, field_absorption_used) =
            if in_non_legacy_mode && self.adaptive_field_quality {
                let (base_steps, base_density, base_absorption) = match dominant_stage_index {
                    0 => (96_u32, 1.25_f32, 0.42_f32),
                    1 => (88_u32, 1.18_f32, 0.46_f32),
                    2 => (80_u32, 1.10_f32, 0.50_f32),
                    3 => (72_u32, 1.02_f32, 0.54_f32),
                    4 => (64_u32, 0.96_f32, 0.58_f32),
                    _ => (56_u32, 0.90_f32, 0.62_f32),
                };

                let zoomed_out = 1.0 - zoom_norm;
                let steps_used = base_steps
                    .saturating_sub((24.0 * zoomed_out).max(0.0) as u32)
                    .max(24);
                let density_used = (base_density * (1.0 - 0.15 * zoomed_out)).clamp(0.2, 4.0);
                let absorption_used = (base_absorption + 0.10 * zoomed_out).clamp(0.05, 2.0);
                (
                    dominant_stage_index as u8,
                    steps_used,
                    density_used,
                    absorption_used,
                )
            } else {
                (dominant_stage_index as u8, 64_u32, 1.0_f32, 0.5_f32)
            };

        let active_source_count = if use_field_derived_entities {
            holo_entities.len()
        } else {
            legacy_entities.len()
        };
        let density_signal = (active_source_count as f32 / 300.0).clamp(0.0, 1.0);

        let declutter_factor_candidate =
            (zoomed_out_signal * 0.7 + density_signal * 0.3).clamp(0.0, 1.0);
        let declutter_threshold_candidate =
            (base_phase_threshold + 0.18 * declutter_factor_candidate).clamp(0.5, 0.95);
        let declutter_structural_candidate =
            base_structural_scale * (1.0 - 0.30 * declutter_factor_candidate);
        let declutter_phase_candidate =
            base_phase_scale * (1.0 - 0.40 * declutter_factor_candidate);
        let declutter_bias_candidate =
            base_intensity_bias * (1.0 - 0.50 * declutter_factor_candidate);

        let (
            declutter_factor,
            phase_threshold_effective,
            connection_structural_scale,
            connection_phase_scale,
            connection_intensity_bias,
        ) = if in_non_legacy_mode && self.adaptive_connection_declutter {
            let factor = declutter_factor_candidate;
            let threshold_effective = declutter_threshold_candidate;
            let structural_scale_used = declutter_structural_candidate;
            let phase_scale_used = declutter_phase_candidate;
            let intensity_bias_used = declutter_bias_candidate;
            (
                factor,
                threshold_effective,
                structural_scale_used,
                phase_scale_used,
                intensity_bias_used,
            )
        } else {
            (
                0.0,
                base_phase_threshold,
                base_structural_scale,
                base_phase_scale,
                base_intensity_bias,
            )
        };

        // Apply visual UX mode (Phase 1: non-invasive)
        match self.visual_ux_mode {
            VisualUxMode::Legacy => {}
            VisualUxMode::SceneFirst | VisualUxMode::FieldEnhanced => {
                self.show_holographic_field = true;
            }
        }

        let mut effective_show_holographic_field = self.show_holographic_field;
        let mut effective_stage_morphology = self.stage_morphology_enabled;
        let mut effective_selection_focus_halo = self.selection_focus_halo;
        let mut effective_adaptive_connection_declutter =
            in_non_legacy_mode && self.adaptive_connection_declutter;
        let mut effective_declutter_factor = declutter_factor;
        let mut effective_phase_threshold = phase_threshold_effective;
        let mut effective_conn_structural_scale = connection_structural_scale;
        let mut effective_conn_phase_scale = connection_phase_scale;
        let mut effective_conn_bias = connection_intensity_bias;
        let mut effective_field_steps = field_steps_used;
        let mut effective_field_density = field_density_used;
        let mut effective_field_absorption = field_absorption_used;

        match self.scene_benchmark_mode {
            SceneBenchmarkMode::Off => {}
            SceneBenchmarkMode::Clarity => {
                effective_show_holographic_field = true;
                effective_stage_morphology = true;
                effective_selection_focus_halo = true;
                effective_adaptive_connection_declutter = false;
                effective_declutter_factor = 0.0;

                effective_phase_threshold = (effective_phase_threshold - 0.06)
                    .max(0.55)
                    .clamp(0.5, 0.95);
                effective_conn_structural_scale =
                    (effective_conn_structural_scale * 1.08).clamp(0.10, 3.0);
                effective_conn_phase_scale = (effective_conn_phase_scale * 1.15).clamp(0.10, 3.0);
                effective_conn_bias = (effective_conn_bias + 0.01).clamp(-1.0, 1.0);

                effective_field_steps = (field_steps_used + 16).min(144);
                effective_field_density = (field_density_used * 1.08).clamp(0.2, 4.0);
                effective_field_absorption = (field_absorption_used * 0.95).clamp(0.05, 2.0);
            }
            SceneBenchmarkMode::Performance => {
                effective_show_holographic_field = true;
                effective_stage_morphology = false;
                effective_selection_focus_halo = false;
                effective_adaptive_connection_declutter = true;
                effective_declutter_factor = declutter_factor_candidate;

                effective_phase_threshold = (effective_phase_threshold + 0.12)
                    .min(0.95)
                    .clamp(0.5, 0.95);
                effective_conn_structural_scale =
                    (effective_conn_structural_scale * 0.80).clamp(0.10, 3.0);
                effective_conn_phase_scale = (effective_conn_phase_scale * 0.65).clamp(0.10, 3.0);
                effective_conn_bias = (effective_conn_bias * 0.5).clamp(-1.0, 1.0);

                effective_field_steps = field_steps_used.saturating_sub(24).max(24);
                effective_field_density = (field_density_used * 0.90).clamp(0.2, 4.0);
                effective_field_absorption = (field_absorption_used * 1.08).clamp(0.05, 2.0);
            }
        }

        if in_non_legacy_mode && self.semantic_lod_enabled {
            match semantic_lod_level {
                2 => {
                    effective_phase_threshold = (effective_phase_threshold + 0.10)
                        .min(0.95)
                        .clamp(0.5, 0.95);
                    effective_conn_structural_scale =
                        (effective_conn_structural_scale * 0.80).clamp(0.10, 3.0);
                    effective_conn_phase_scale =
                        (effective_conn_phase_scale * 0.80).clamp(0.10, 3.0);
                    if matches!(self.scene_benchmark_mode, SceneBenchmarkMode::Off) {
                        effective_selection_focus_halo =
                            has_selection && effective_selection_focus_halo;
                    }
                }
                1 => {
                    effective_phase_threshold = (effective_phase_threshold + 0.04)
                        .min(0.95)
                        .clamp(0.5, 0.95);
                    effective_conn_structural_scale =
                        (effective_conn_structural_scale * 0.90).clamp(0.10, 3.0);
                    effective_conn_phase_scale =
                        (effective_conn_phase_scale * 0.90).clamp(0.10, 3.0);
                }
                _ => {}
            }
        }

        self.render_stats.camera_zoom = camera_zoom;
        self.render_stats.semantic_lod_level = semantic_lod_level;
        self.render_stats.semantic_lod_zoom_norm = zoom_norm;
        self.render_stats.semantic_lod_focus_weight = focus_weight;
        self.render_stats.declutter_factor = if effective_adaptive_connection_declutter {
            effective_declutter_factor
        } else {
            0.0
        };
        self.render_stats.phase_threshold_effective = effective_phase_threshold;
        self.render_stats.phase_threshold_used = effective_phase_threshold;
        self.render_stats.connection_structural_scale = effective_conn_structural_scale;
        self.render_stats.connection_phase_scale = effective_conn_phase_scale;
        self.render_stats.connection_intensity_bias = effective_conn_bias;
        self.render_stats.field_quality_stage = field_quality_stage;
        self.render_stats.field_ray_steps = effective_field_steps;
        self.render_stats.field_density_scale = effective_field_density;
        self.render_stats.field_absorption = effective_field_absorption;
        self.render_stats.benchmark_mode = match self.scene_benchmark_mode {
            SceneBenchmarkMode::Off => 0,
            SceneBenchmarkMode::Clarity => 1,
            SceneBenchmarkMode::Performance => 2,
        };
        self.render_stats.benchmark_effective_field_on = effective_show_holographic_field;
        self.render_stats.benchmark_effective_morphology_on = effective_stage_morphology;
        self.render_stats.benchmark_effective_focus_halo_on = effective_selection_focus_halo;
        let phase_threshold_for_network = effective_phase_threshold as f64;

        let cpu_update_start = Instant::now();
        let selected_focus = self.selected_render_focus_position();
        let lens_can_apply =
            self.selection_neighborhood_lens && selected_focus.is_some() && in_non_legacy_mode;

        let lens_profile_stage = self.render_stats.emphasis_stage;
        let mut lens_radius_effective = self.lens_radius;
        let mut lens_non_focus_effective = self.lens_non_focus_scale;
        let mut lens_edge_boost_effective = self.lens_edge_boost;
        let lens_profile_adaptive = self.adaptive_lens_profile && in_non_legacy_mode;

        if lens_profile_adaptive {
            let (stage_radius_factor, stage_non_focus_factor, stage_boost_factor) =
                match lens_profile_stage {
                    0 => (0.80_f32, 0.85_f32, 1.25_f32),
                    1 => (0.88_f32, 0.90_f32, 1.15_f32),
                    2 => (1.00_f32, 1.00_f32, 1.00_f32),
                    3 => (1.10_f32, 1.08_f32, 0.90_f32),
                    4 => (1.18_f32, 1.14_f32, 0.82_f32),
                    _ => (1.26_f32, 1.20_f32, 0.75_f32),
                };

            lens_radius_effective *= stage_radius_factor;
            lens_non_focus_effective *= stage_non_focus_factor;
            lens_edge_boost_effective *= stage_boost_factor;

            let zoomed_out = 1.0 - zoom_norm;
            lens_radius_effective *= 1.0 + 0.20 * zoomed_out;
            lens_non_focus_effective *= 1.0 - 0.10 * zoomed_out;
            lens_edge_boost_effective *= 1.0 - 0.15 * zoomed_out;
        }

        lens_radius_effective = lens_radius_effective.clamp(0.15, 1.8);
        lens_non_focus_effective = lens_non_focus_effective.clamp(0.15, 1.0);
        lens_edge_boost_effective = lens_edge_boost_effective.clamp(0.0, 0.30);

        self.render_stats.lens_active = lens_can_apply;
        self.render_stats.lens_focus_present = selected_focus.is_some();
        self.render_stats.lens_profile_stage = lens_profile_stage;
        self.render_stats.lens_profile_adaptive = lens_profile_adaptive;
        self.render_stats.lens_radius_effective = lens_radius_effective;
        self.render_stats.lens_non_focus_effective = lens_non_focus_effective;
        self.render_stats.lens_edge_boost_effective = lens_edge_boost_effective;
        self.render_stats.lens_radius_used = if lens_can_apply {
            lens_radius_effective
        } else {
            self.lens_radius
        };
        self.render_stats.lens_non_focus_scale_used = if lens_can_apply {
            lens_non_focus_effective
        } else {
            self.lens_non_focus_scale
        };
        self.render_stats.lens_edge_boost_used = if lens_can_apply {
            lens_edge_boost_effective
        } else {
            self.lens_edge_boost
        };

        // Get entity renderer
        let renderer = match &mut self.entity_renderer {
            Some(r) => r,
            None => return Ok(()), // No renderer, skip rendering
        };

        // Phase 2: Pass time for realm ring animations
        // Phase 5: Update view transitions
        let current_time = self.last_frame_time.elapsed();
        if let Some((target_pos, target_zoom, target_rotation)) = self
            .view_system
            .update_transition(Duration::from_secs_f32(0.016), current_time)
        {
            let start_pos = self.view_system.transition_start_position();
            let start_zoom = self.view_system.transition_start_zoom();
            let start_rotation = self.view_system.transition_start_rotation();

            self.camera.smooth_transition_to(
                target_pos,
                target_zoom,
                target_rotation,
                self.view_system.transition_progress(),
                start_pos,
                start_zoom,
                start_rotation,
            );
        }

        let auto_focus_stage = self.render_stats.emphasis_stage;
        let stage_target_zoom = match auto_focus_stage {
            0 => 2.8,
            1 => 2.3,
            2 => 1.8,
            3 => 1.45,
            4 => 1.15,
            _ => 0.95,
        };

        if self.guided_auto_focus && in_non_legacy_mode {
            if let Some(target_pos) = selected_focus {
                let dx = target_pos[0] - self.camera.position.x;
                let dy = target_pos[1] - self.camera.position.y;
                let distance = (dx * dx + dy * dy).sqrt();
                let lens_multiplier = if self.selection_neighborhood_lens {
                    1.15
                } else {
                    1.0
                };
                let alpha = (self.auto_focus_strength * lens_multiplier).clamp(0.02, 0.35);

                self.camera.position.x += dx * alpha;
                self.camera.position.y += dy * alpha;
                self.camera.zoom += (stage_target_zoom - self.camera.zoom) * alpha;
                self.camera.zoom = self
                    .camera
                    .zoom
                    .clamp(self.camera.min_zoom, self.camera.max_zoom);

                self.render_stats.auto_focus_active = true;
                self.render_stats.auto_focus_target_zoom = stage_target_zoom;
                self.render_stats.auto_focus_distance = distance;
                self.render_stats.auto_focus_stage = auto_focus_stage;
            } else {
                self.render_stats.auto_focus_active = false;
                self.render_stats.auto_focus_target_zoom = self.camera.zoom;
                self.render_stats.auto_focus_distance = 0.0;
                self.render_stats.auto_focus_stage = auto_focus_stage;
            }
        } else {
            self.render_stats.auto_focus_active = false;
            self.render_stats.auto_focus_target_zoom = self.camera.zoom;
            self.render_stats.auto_focus_distance = 0.0;
            self.render_stats.auto_focus_stage = auto_focus_stage;
        }

        // Update camera uniforms after transition and auto-focus nudge.
        let view_projection = self.camera.view_projection_matrix();
        let view_projection_array: [[f32; 4]; 4] = view_projection.into();
        self.render_stats.camera_zoom = self.camera.zoom;

        let time = self.last_frame_time.elapsed().as_secs_f32();

        // Get WGPU context reference for queue operations
        let ctx = self.wgpu_context.as_ref().unwrap();
        renderer.update_camera(
            &ctx.queue,
            view_projection_array,
            time,
            effective_stage_morphology,
            selected_focus,
            effective_selection_focus_halo,
        );

        self.render_stats.using_holo_entities = false;
        self.render_stats.using_holo_connections = false;
        self.render_stats.phase_edge_count = 0;
        self.render_stats.active_connection_count = 0;
        self.render_stats.structural_connection_count = 0;
        self.render_stats.phase_connection_count = 0;

        if use_field_derived_entities {
            if !holo_entities.is_empty() {
                renderer.update_holo_entities(&ctx.queue, &holo_entities);
                self.render_stats.entity_count = holo_entities.len() as u64;
                self.render_stats.using_holo_entities = true;
            } else if !legacy_entities.is_empty() {
                renderer.update_entities(&ctx.queue, &legacy_entities);
                self.render_stats.entity_count = legacy_entities.len() as u64;
            } else {
                // Fallback to test instances if selected entity source is empty
                renderer.update_test_instances(&ctx.queue, 50);
                self.render_stats.entity_count = 50;
            }
        } else if !legacy_entities.is_empty() {
            renderer.update_entities(&ctx.queue, &legacy_entities);
            self.render_stats.entity_count = legacy_entities.len() as u64;
        } else {
            // Fallback to test instances if selected entity source is empty
            renderer.update_test_instances(&ctx.queue, 50);
            self.render_stats.entity_count = 50;
        }

        if let Some(conn_renderer) = &mut self.connection_renderer {
            conn_renderer.update_camera(
                &ctx.queue,
                view_projection_array,
                time,
                conn_color_warm_cool_bias,
                conn_color_saturation,
                conn_color_phase_boost,
                conn_color_grade_strength,
            );

            let mut phase_edge_count = 0_u64;
            let mut using_holo_connections = false;

            match self.visual_ux_mode {
                VisualUxMode::Legacy => {
                    if !legacy_entities.is_empty() {
                        let entity_instances: Vec<crate::gui::renderer::EntityInstance> =
                            legacy_entities
                                .iter()
                                .enumerate()
                                .map(|(i, e)| {
                                    crate::gui::renderer::EntityInstance::from_entity(e, i)
                                })
                                .collect();
                        conn_renderer.update_connections(
                            &ctx.queue,
                            &legacy_entities,
                            &entity_instances,
                        );
                    }
                }
                VisualUxMode::SceneFirst | VisualUxMode::FieldEnhanced => {
                    if !holo_entities.is_empty() {
                        using_holo_connections = true;
                        if let Some(field_state) = self.simulation.get_holographic_field_state() {
                            let phase_edges = self
                                .field_visual_bridge
                                .extract_phase_network(field_state, phase_threshold_for_network);
                            phase_edge_count = phase_edges.len() as u64;
                            if lens_can_apply {
                                conn_renderer
                                    .update_holo_connections_with_phase_network_profile_lens(
                                        &ctx.queue,
                                        &holo_entities,
                                        &phase_edges,
                                        effective_conn_structural_scale,
                                        effective_conn_phase_scale,
                                        effective_conn_bias,
                                        selected_focus.unwrap_or([0.0, 0.0, 0.0]),
                                        lens_radius_effective,
                                        lens_non_focus_effective,
                                        lens_edge_boost_effective,
                                    );
                            } else {
                                conn_renderer.update_holo_connections_with_phase_network_profile(
                                    &ctx.queue,
                                    &holo_entities,
                                    &phase_edges,
                                    effective_conn_structural_scale,
                                    effective_conn_phase_scale,
                                    effective_conn_bias,
                                );
                            }
                        } else {
                            conn_renderer.update_holo_connections(&ctx.queue, &holo_entities);
                        }
                    } else if !legacy_entities.is_empty() {
                        let entity_instances: Vec<crate::gui::renderer::EntityInstance> =
                            legacy_entities
                                .iter()
                                .enumerate()
                                .map(|(i, e)| {
                                    crate::gui::renderer::EntityInstance::from_entity(e, i)
                                })
                                .collect();

                        if let Some(field_state) = self.simulation.get_holographic_field_state() {
                            let phase_edges = self
                                .field_visual_bridge
                                .extract_phase_network(field_state, phase_threshold_for_network);
                            phase_edge_count = phase_edges.len() as u64;
                            if lens_can_apply {
                                conn_renderer.update_connections_with_phase_network_profile_lens(
                                    &ctx.queue,
                                    &legacy_entities,
                                    &entity_instances,
                                    &phase_edges,
                                    effective_conn_structural_scale,
                                    effective_conn_phase_scale,
                                    effective_conn_bias,
                                    selected_focus.unwrap_or([0.0, 0.0, 0.0]),
                                    lens_radius_effective,
                                    lens_non_focus_effective,
                                    lens_edge_boost_effective,
                                );
                            } else {
                                conn_renderer.update_connections_with_phase_network_profile(
                                    &ctx.queue,
                                    &legacy_entities,
                                    &entity_instances,
                                    &phase_edges,
                                    effective_conn_structural_scale,
                                    effective_conn_phase_scale,
                                    effective_conn_bias,
                                );
                            }
                        } else {
                            conn_renderer.update_connections(
                                &ctx.queue,
                                &legacy_entities,
                                &entity_instances,
                            );
                        }
                    }
                }
            }

            self.render_stats.using_holo_connections = using_holo_connections;
            self.render_stats.phase_edge_count = phase_edge_count;
            self.render_stats.active_connection_count = conn_renderer.connection_count() as u64;
            self.render_stats.structural_connection_count =
                conn_renderer.structural_connection_count() as u64;
            self.render_stats.phase_connection_count =
                conn_renderer.phase_connection_count() as u64;
        }

        // Phase 3: Update cosmos renderer with cosmic data
        if let Some(cosmos_renderer) = &mut self.cosmos_renderer {
            if self.show_cosmos {
                // Get cosmos data from simulation adapter
                let stars = self.simulation.get_star_vertices();
                let planets = self.simulation.get_planet_vertices();
                let orbits = self.simulation.get_orbit_vertices();
                let filaments = self.simulation.get_filament_vertices();

                // Update cosmos renderer buffers
                cosmos_renderer.update_camera(
                    &ctx.queue,
                    view_projection_array,
                    [self.camera.position.x, self.camera.position.y, self.camera.position.z],
                    time,
                );
                cosmos_renderer.update_stars(&ctx.queue, &stars);
                cosmos_renderer.update_planets(&ctx.queue, &planets);
                cosmos_renderer.update_orbits(&ctx.queue, &orbits);
                cosmos_renderer.update_filaments(&ctx.queue, &filaments);
            }
        }

        // Prepare field data before render pass begin
        let mut prepared_field_data = None;
        if effective_show_holographic_field {
            if let Some(field_state) = self.simulation.get_holographic_field_state() {
                prepared_field_data = Some(self.field_visual_bridge.sample_field(field_state));
            }
        }

        // Get surface texture
        let surface = ctx.surface.as_ref().ok_or("No surface available")?;

        let output = surface
            .get_current_texture()
            .map_err(|e| format!("Failed to get surface texture: {}", e))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create command encoder
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.render_stats.cpu_update_ms = cpu_update_start.elapsed().as_secs_f64() * 1000.0;

        let scene_render_start = Instant::now();

        // Begin render pass with clear color
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05, // Dark blue background
                            g: 0.05,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Phase 4: Render connection lines (behind entities)
            if let Some(conn_renderer) = &self.connection_renderer {
                conn_renderer.render(&mut render_pass);
            }

            // Phase B.1: Render holographic field (before entities, after connections)
            if let Some(ref mut field_renderer) = self.holographic_field_renderer {
                if let Some(ref field_data) = prepared_field_data {
                    field_renderer.set_quality_profile(
                        effective_field_steps,
                        effective_field_density,
                        effective_field_absorption,
                    );
                    field_renderer.update_field(&ctx.queue, field_data);

                    // Update uniforms
                    field_renderer.update_uniforms(
                        &ctx.queue,
                        view_projection_array,
                        [
                            self.camera.position.x,
                            self.camera.position.y,
                            self.camera.position.z,
                        ],
                        time,
                    );

                    // Render field
                    field_renderer.render(&mut render_pass);
                    // Note: If holographic simulation not enabled, field is not rendered
                }
            }

            // Phase 3: Render cosmos structures (stars, planets, orbits, filaments)
            if let Some(cosmos_renderer) = &self.cosmos_renderer {
                if self.show_cosmos {
                    cosmos_renderer.render(&mut render_pass);
                }
            }

            // Phase 1: Render entities
            renderer.render(&mut render_pass);
        }

        self.render_stats.scene_render_ms = scene_render_start.elapsed().as_secs_f64() * 1000.0;

        // Debug output every 60 frames
        if self.frame_count.is_multiple_of(60) && self.render_stats.entity_count > 0 {
            println!(
                "Rendering {} entities at {:.1} FPS",
                self.render_stats.entity_count, self.fps
            );

            // Show holographic simulation stats
            if let Some(stats) = holo_stats.as_ref() {
                println!("  [Holographic] Coherence: {:.3} | Veil: {:.3} | Entities: {} | Collectives: {}",
                    stats.average_coherence, stats.veil_transparency, stats.entity_count, stats.collective_count);
                // Phase B: Show cosmic sequence info
                let layer_names = [
                    "Violet", "Indigo", "Blue", "Green", "Yellow", "Orange", "Red", "L7",
                ];
                let layer_name = layer_names.get(stats.current_layer).unwrap_or(&"?");
                println!(
                    "  [Cosmic] Layer: {} | Attractor: {:.3}",
                    layer_name, stats.attractor_strength
                );
                // Phase C: Show Larson framework info
                let st = if stats.veil_transparency < 0.5 {
                    "ST"
                } else if stats.veil_transparency > 0.7 {
                    "TS"
                } else {
                    "~V~"
                };
                println!(
                    "  [Larson] v=s/t↔v=t/s | Veil: {:.1}% | Mode: {}",
                    stats.veil_transparency * 100.0,
                    st
                );
            }
        }

        // Render EGUI panels (after entities, before submit)
        // Use the same encoder as the 3D scene render
        let ui_render_start = Instant::now();
        self.render_egui_panels(&mut encoder, &view);
        self.render_stats.ui_render_ms = ui_render_start.elapsed().as_secs_f64() * 1000.0;

        // Submit and present
        let ctx = self.wgpu_context.as_ref().unwrap();
        let submit_present_start = Instant::now();
        ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.render_stats.submit_present_ms = submit_present_start.elapsed().as_secs_f64() * 1000.0;

        Ok(())
    }

    /// Run a single simulation step
    /// Phase 5: Switch to a specific view
    fn switch_view(&mut self, view_type: ViewType) {
        println!("Switching to view: {}", view_type.name());

        let (pos, zoom, rotation) = self.camera.get_state();
        self.view_system.switch_view(view_type, pos, zoom, rotation);

        // Update window title to show current view
        self.window.set_title(&format!(
            "Holonic Realms - {} View - FPS: {:.1}",
            view_type.name(),
            self.fps
        ));
    }

    fn project_emergence_field_context(
        source: &crate::hpo::HolographicFieldState,
    ) -> (FoundationFieldState, Position3D) {
        let bounds = source.root.bounds;
        let x_size = (bounds.max[0] - bounds.min[0]).abs();
        let y_size = (bounds.max[1] - bounds.min[1]).abs();
        let z_size = (bounds.max[2] - bounds.min[2]).abs();
        let size = x_size.max(y_size).max(z_size).max(1.0);

        let mut projected = FoundationFieldState::new(size);
        let mut best_focus = Position3D::zero();
        let mut best_score = f64::NEG_INFINITY;

        Self::project_hpo_node_recursive(
            &source.root,
            &mut projected,
            &mut best_focus,
            &mut best_score,
        );

        if source.average_coherence > 0.0 {
            projected.set_amplitude(
                &best_focus,
                ScaleLevel::Biological,
                FieldAmplitude::from_polar(source.average_coherence, source.time),
            );
        }

        (projected, best_focus)
    }

    fn project_hpo_node_recursive(
        node: &crate::hpo::OctreeNode,
        projected: &mut FoundationFieldState,
        best_focus: &mut Position3D,
        best_score: &mut f64,
    ) {
        let center = node.bounds.center();
        let position = Position3D::new(center[0], center[1], center[2]);

        for (band_index, scale) in [
            ScaleLevel::Quantum,
            ScaleLevel::Atomic,
            ScaleLevel::Molecular,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Cosmic,
        ]
        .iter()
        .enumerate()
        {
            let amp = node.field_data.density_amplitudes[band_index];
            projected.set_amplitude(
                &position,
                *scale,
                FieldAmplitude::new(amp.re, amp.im),
            );
        }

        if let Some(target_node) = projected.get_or_create_node_mut(&position) {
            target_node.coherence = node.field_data.coherence.clamp(0.0, 1.0);
            target_node.archetype_vector = Self::derive_archetype_pattern_from_hpo_node(node);
        }

        let node_score = node.field_data.coherence + node.field_data.total_magnitude();
        if node_score > *best_score {
            *best_score = node_score;
            *best_focus = position;
        }

        if let Some(children) = node.children.as_ref() {
            for child in children.iter() {
                Self::project_hpo_node_recursive(child, projected, best_focus, best_score);
            }
        }
    }

    fn derive_archetype_pattern_from_hpo_node(node: &crate::hpo::OctreeNode) -> [f64; NUM_ARCHETYPES] {
        let mut pattern = [0.5; NUM_ARCHETYPES];
        for (i, coeff) in pattern.iter_mut().enumerate() {
            let amp = node.field_data.density_amplitudes[i % 8];
            let magnitude_component = amp.magnitude().tanh().clamp(0.0, 1.0);
            let phase_component = 0.5 * (amp.phase().cos() + 1.0);
            *coeff = (0.6 * node.field_data.coherence + 0.3 * magnitude_component + 0.1 * phase_component)
                .clamp(0.0, 1.0);
        }
        pattern
    }

    fn run_step(&mut self) -> Result<(), String> {
        // Calculate delta time for animations
        let delta_seconds = self.last_frame_time.elapsed().as_secs_f32();

        // Update simulation
        self.simulation.run_step();

        let emergence_context = self
            .simulation
            .get_holographic_field_state()
            .map(Self::project_emergence_field_context);
        let emergence_field = emergence_context.as_ref().map(|(field, _)| field);
        let emergence_focus = emergence_context.as_ref().map(|(_, position)| position);

        // Phase B.2: Update spectrum panel animation
        self.spectrum_continuum_panel.update(delta_seconds);

        // Phase B.3: Update archetype panel animation
        self.archetype_panel.update(delta_seconds);

        // Phase C.1: Update quantum panel animation
        self.quantum_panel
            .update_with_field(delta_seconds, emergence_field);

        // Phase C.2: Update atomic panel animation
        self.atomic_panel
            .update_with_field(delta_seconds, emergence_field, emergence_focus);

        // Phase C.3: Update molecular panel animation
        self.molecular_panel
            .update_with_field(delta_seconds, emergence_field, emergence_focus);

        // Phase C.4: Update cellular panel animation
        self.cellular_panel
            .update_with_field(delta_seconds, emergence_field, emergence_focus);

        // Phase D: Update consciousness panel animation
        self.consciousness_panel.update(delta_seconds);

        Ok(())
    }

    /// Get render statistics
    pub fn render_stats(&self) -> &RenderStats {
        &self.render_stats
    }

    /// Get simulation state
    pub fn simulation_state(&self) -> &crate::integrated_system::SimulationState {
        self.simulation.state()
    }

    /// Check if application is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Check if holographic mode is enabled
    pub fn is_holographic_mode(&self) -> bool {
        self.simulation.is_holographic_mode()
    }

    /// Get holographic simulation statistics
    pub fn get_holo_statistics(&self) -> Option<crate::hpo::SimulationStatistics> {
        self.simulation.get_holo_statistics()
    }

    /// Get field visualization data
    pub fn get_field_visualization(&self) -> Option<crate::hpo::FieldVisualizationData> {
        self.simulation.get_field_visualization()
    }

    // ========== Phase A.3: Keyboard Shortcuts and Bookmark System ==========

    /// Load a bookmark by slot index (for quick navigation)
    fn load_bookmark(&mut self, slot: usize) {
        let bookmark_ids: Vec<&str> = vec![
            "home",
            "scale_quantum",
            "scale_atomic",
            "scale_molecular",
            "scale_galactic",
        ];
        if let Some(&id) = bookmark_ids.get(slot) {
            if self.bookmark_manager.navigate_to(id, &mut self.camera) {
                println!("Navigated to bookmark: {}", id);
            }
        }
    }

    /// Save current camera position to a quick bookmark slot
    #[allow(dead_code)]
    fn save_current_bookmark(&mut self, slot: usize) {
        let bookmark_names = [
            "Quick Slot 1",
            "Quick Slot 2",
            "Quick Slot 3",
            "Quick Slot 4",
            "Quick Slot 5",
        ];
        if let Some(&name) = bookmark_names.get(slot) {
            let id = format!("quick_{}", slot);
            self.bookmark_manager
                .bookmark_current(id, name, &self.camera);
            println!("Saved current position to {}", name);
        }
    }

    /// Render the keyboard shortcuts overlay
    #[allow(dead_code)]
    fn render_shortcuts_overlay(&self, ctx: &egui::Context) {
        egui::Window::new("Keyboard Shortcuts")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_min_width(300.0);
                ui.heading("⌨ Keyboard Shortcuts");
                ui.separator();

                ui.label("F1 - Show/Hide this overlay");
                ui.label("F2 - Toggle Entity Inspector");
                ui.label("F3 - Toggle Collective Dashboard");
                ui.label("F4 - Toggle Spectrum Dashboard");
                ui.label("F5 - Toggle Emergence Dashboard");
                ui.label("F6 - Toggle Time Controls");
                ui.label("F7 - Reset view to Home");
                ui.label("F8 - Toggle Adaptive Declutter");
                ui.label("F9 - Toggle Declutter HUD");
                ui.label("F10 - Toggle Adaptive Field Quality");
                ui.label("F11 - Toggle Field Quality HUD");
                ui.label("F12 - Cycle Benchmark Mode");
                ui.label("F13 or E - Toggle Semantic LOD");
                ui.label("F14 or W - Toggle Semantic LOD HUD");
                ui.label(format!(
                    "{} - Toggle Neighborhood Lens",
                    LENS_TOGGLE_KEYS_LABEL
                ));
                ui.label(format!("{} - Toggle Lens HUD", LENS_HUD_KEYS_LABEL));
                ui.label(format!(
                    "{} - Toggle Guided Auto-Focus",
                    AUTO_FOCUS_TOGGLE_KEYS_LABEL
                ));
                ui.label(format!(
                    "{} - Toggle Auto-Focus HUD",
                    AUTO_FOCUS_HUD_KEYS_LABEL
                ));
                ui.label(format!(
                    "{} - Toggle Adaptive Lens Profile",
                    LENS_PROFILE_TOGGLE_KEYS_LABEL
                ));
                ui.label(format!(
                    "{} - Toggle Lens Profile HUD",
                    LENS_PROFILE_HUD_KEYS_LABEL
                ));
                ui.label("` - Toggle Benchmark HUD");
                ui.label("B - Toggle Compare Overlay");
                ui.label("V - Cycle Visual UX Mode");
                ui.label("F - Toggle Holographic Field (Phase B.1)");
                ui.label("S - Toggle Spectrum Continuum Panel (Phase B.2)");
                ui.label("A - Toggle Archetype Panel (Phase B.3)");
                ui.label("Q - Toggle Quantum Panel (Phase C.1)");
                ui.label("Z - Toggle Atomic Panel (Phase C.2)");
                ui.label("M - Toggle Molecular Panel (Phase C.3)");
                ui.label("C - Toggle Cellular Panel (Phase C.4)");
                ui.label("H - Toggle Consciousness Panel (Phase D)");
                ui.label("N - Toggle Emergence Glyphs");
                ui.label("J - Toggle Adaptive Emphasis");
                ui.label("K - Toggle Emphasis HUD");
                ui.label("X - Toggle Adaptive Link Readability");
                ui.label("T - Toggle Link Readability HUD");
                ui.label(format!(
                    "{} - Toggle Adaptive Link Color",
                    CONNECTION_COLOR_TOGGLE_KEYS_LABEL
                ));
                ui.label(format!(
                    "{} - Toggle Link Color HUD",
                    CONNECTION_COLOR_HUD_KEYS_LABEL
                ));
                ui.label("L - Toggle Stage Morphology");
                ui.label("◎ Focus - Toggle from toolbar");
                ui.label("Y - Toggle Selection Semantics");
                ui.label("O - Toggle Auto Overlay Orchestration");
                ui.label("I - Toggle Overlay Policy HUD");
                ui.separator();

                ui.label("1-6 - Switch views");
                ui.label("Esc - Exit application");
                ui.separator();

                ui.label("Mouse: Click entities to inspect");
                ui.label("Mouse: Drag to pan camera");
                ui.label("Mouse: Scroll to zoom");

                ui.separator();
                ui.small("Press F1 to close this overlay");
            });
    }
    #[allow(clippy::too_many_arguments)]
    fn build_default_docking_manager(
        entity_inspector_id: PanelId,
        collective_dashboard_id: PanelId,
        spectrum_dashboard_id: PanelId,
        emergence_dashboard_id: PanelId,
        time_controls_id: PanelId,
        spectrum_continuum_id: PanelId,
        archetype_id: PanelId,
        quantum_id: PanelId,
        atomic_id: PanelId,
        molecular_id: PanelId,
        cellular_id: PanelId,
        consciousness_id: PanelId,
    ) -> crate::gui::ui::panels::DockingManager {
        DockingBuilder::new()
            .with_panel(
                PanelInfo::new(entity_inspector_id, "Entity Inspector")
                    .with_dock_area(DockArea::Right)
                    .with_width(300.0)
                    .with_icon('🔍'),
            )
            .with_panel(
                PanelInfo::new(collective_dashboard_id, "Collective Dashboard")
                    .with_dock_area(DockArea::Right)
                    .with_width(350.0)
                    .with_icon('👥'),
            )
            .with_panel(
                PanelInfo::new(spectrum_dashboard_id, "Spectrum Dashboard")
                    .with_dock_area(DockArea::Left)
                    .with_width(350.0)
                    .with_icon('📊'),
            )
            .with_panel(
                PanelInfo::new(emergence_dashboard_id, "Emergence Dashboard")
                    .with_dock_area(DockArea::Bottom)
                    .with_height(200.0)
                    .with_icon('✨'),
            )
            .with_panel(
                PanelInfo::new(time_controls_id, "Time Controls")
                    .with_dock_area(DockArea::Floating)
                    .with_width(300.0)
                    .with_icon('⏱'),
            )
            .with_panel(
                PanelInfo::new(spectrum_continuum_id, "Spectrum Continuum")
                    .with_dock_area(DockArea::Floating)
                    .with_width(500.0)
                    .with_height(400.0)
                    .with_icon('🌌'),
            )
            .with_panel(
                PanelInfo::new(archetype_id, "Archetype System")
                    .with_dock_area(DockArea::Floating)
                    .with_width(500.0)
                    .with_height(600.0)
                    .with_icon('🔮'),
            )
            .with_panel(
                PanelInfo::new(quantum_id, "Quantum Visualization")
                    .with_dock_area(DockArea::Right)
                    .with_width(600.0)
                    .with_height(700.0)
                    .with_icon('⚛'),
            )
            .with_panel(
                PanelInfo::new(atomic_id, "Atomic Structure")
                    .with_dock_area(DockArea::Center)
                    .with_width(600.0)
                    .with_height(700.0)
                    .with_icon('⚡'),
            )
            .with_panel(
                PanelInfo::new(molecular_id, "Molecular Dynamics")
                    .with_dock_area(DockArea::Left)
                    .with_width(600.0)
                    .with_height(700.0)
                    .with_icon('🧬'),
            )
            .with_panel(
                PanelInfo::new(cellular_id, "Cellular Systems")
                    .with_dock_area(DockArea::Right)
                    .with_width(600.0)
                    .with_height(700.0)
                    .with_icon('🔬'),
            )
            .with_panel(
                PanelInfo::new(consciousness_id, "Consciousness Systems")
                    .with_dock_area(DockArea::Center)
                    .with_width(600.0)
                    .with_height(700.0)
                    .with_icon('🧠'),
            )
            .with_right_width(320.0)
            .with_left_width(350.0)
            .with_bottom_height(200.0)
            .build()
    }

    /// Toggle a panel visibility
    fn toggle_panel(&mut self, panel_id: PanelId) {
        let _ = self.docking_manager.layout.toggle_panel(panel_id);
    }

    // ========================================================================
    // Hierarchy Navigation Methods
    // ========================================================================

    /// Drill down into the currently selected entity
    /// 
    /// This navigates "inside" the selected entity to view its children
    /// and composition. The entity becomes the new hierarchy focus.
    fn drill_down_into_selected(&mut self) {
        if let Some(ref selected_id) = self.entity_inspector.selected_entity_id.clone() {
            // Get entity info to check if it has children
            let hierarchy_info = self.simulation.get_entity_hierarchy_info_by_id(selected_id);
            
            if hierarchy_info.children_count > 0 || hierarchy_info.composition_count > 0 {
                // Save current focus to stack for "back" navigation
                if let Some(current_focus) = self.hierarchy_focus_id.clone() {
                    self.hierarchy_stack.push(current_focus);
                }
                
                // Set new focus
                self.hierarchy_focus_id = Some(selected_id.clone());
                
                println!(
                    "Drilled down into: {} ({} children, {} composition)",
                    selected_id.uuid,
                    hierarchy_info.children_count,
                    hierarchy_info.composition_count
                );
                
                // Zoom camera to show the entity's contents
                self.zoom_to_hierarchy_level();
            } else {
                println!("Entity {} has no children or composition to drill into", selected_id.uuid);
            }
        } else {
            println!("No entity selected - click an entity first");
        }
    }

    /// Navigate back up one level in the hierarchy
    fn navigate_back_hierarchy(&mut self) {
        if let Some(previous_focus) = self.hierarchy_stack.pop() {
            self.hierarchy_focus_id = Some(previous_focus.clone());
            println!("Navigated back to: {}", previous_focus.uuid);
            self.zoom_to_hierarchy_level();
        } else if self.hierarchy_focus_id.is_some() {
            // At the root level
            self.hierarchy_focus_id = None;
            println!("Navigated to root/universe level");
            self.zoom_to_hierarchy_level();
        } else {
            println!("Already at root level - cannot go back further");
        }
    }

    /// Zoom camera to appropriate level for current hierarchy focus
    fn zoom_to_hierarchy_level(&mut self) {
        // Determine appropriate zoom based on hierarchy level
        let zoom = if let Some(ref focus_id) = self.hierarchy_focus_id {
            // Get the entity type to determine scale
            if let Some(entity) = self.simulation.get_entity(focus_id) {
                match entity.entity_type {
                    crate::entity_layer7::layer7::EntityType::GalacticLogos => 1e21,  // Galactic scale
                    crate::entity_layer7::layer7::EntityType::SolarLogos => 1e12,    // Stellar scale
                    crate::entity_layer7::layer7::EntityType::Environmental => {
                        // Check if it's a galaxy, star, or planet
                        if focus_id.uuid.contains("galaxy") {
                            1e21
                        } else if focus_id.uuid.contains("star") {
                            1e12
                        } else if focus_id.uuid.contains("planet") {
                            1e7   // Planetary scale
                        } else {
                            1e6
                        }
                    }
                    crate::entity_layer7::layer7::EntityType::Individual => {
                        // Check if it's an organism, cell, molecule, atom, or quantum
                        if focus_id.uuid.contains("quantum") {
                            1e-35  // Quantum scale
                        } else if focus_id.uuid.contains("atomic") {
                            1e-10  // Atomic scale
                        } else if focus_id.uuid.contains("molecular") {
                            1e-9   // Molecular scale
                        } else if focus_id.uuid.contains("cellular") {
                            1e-6   // Cellular scale
                        } else if focus_id.uuid.contains("organism") {
                            1e-3   // Organism scale
                        } else if focus_id.uuid.contains("being") {
                            1.0    // Human scale
                        } else {
                            1e-6
                        }
                    }
                    crate::entity_layer7::layer7::EntityType::Collective => 1e3,
                }
            } else {
                1e6  // Default to planetary
            }
        } else {
            1e26  // Universal scale
        };
        
        // Set camera zoom (clamp to valid range)
        self.camera.zoom = (zoom as f32).clamp(self.camera.min_zoom, self.camera.max_zoom);
        
        // Log the scale transition
        let scale_name = self.get_scale_level_name(zoom);
        println!("Zoomed to {} scale ({:.2e}m)", scale_name, zoom);
    }

    /// Get human-readable name for a scale level
    fn get_scale_level_name(&self, scale: f64) -> &'static str {
        if scale >= 1e25 {
            "Universal"
        } else if scale >= 1e20 {
            "Galactic"
        } else if scale >= 1e10 {
            "Stellar"
        } else if scale >= 1e6 {
            "Planetary"
        } else if scale >= 1e2 {
            "Regional"
        } else if scale >= 1.0 {
            "Human"
        } else if scale >= 1e-3 {
            "Organism"
        } else if scale >= 1e-6 {
            "Cellular"
        } else if scale >= 1e-9 {
            "Molecular"
        } else if scale >= 1e-10 {
            "Atomic"
        } else {
            "Quantum"
        }
    }

    /// Get entities visible at the current hierarchy level
    #[allow(dead_code)]
    fn get_visible_entities(&self) -> Vec<crate::entity_layer7::layer7::SubSubLogos> {
        self.simulation.get_children_of_focus(self.hierarchy_focus_id.as_ref())
    }

    fn set_panel_expanded(&mut self, panel_id: PanelId, expanded: bool) {
        let currently_expanded = self
            .docking_manager
            .layout
            .get_panel(panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);

        if currently_expanded != expanded {
            let _ = self.docking_manager.layout.toggle_panel(panel_id);
        }
    }

    fn apply_stage_preset(&mut self, stage: u8) {
        self.visual_ux_mode = VisualUxMode::SceneFirst;
        self.show_holographic_field = true;
        self.show_scene_guide = true;
        self.show_emergence_glyphs = true;

        let view = match stage {
            0 => ViewType::Archetype,
            1 => ViewType::Realm,
            2 => ViewType::Hierarchy,
            3 => ViewType::Evolution,
            4 => ViewType::Spectrum,
            _ => ViewType::Overview,
        };
        self.switch_view(view);

        let open_quantum = stage == 0;
        let open_atomic = stage == 1;
        let open_molecular = stage == 2;
        let open_cellular = stage == 3;
        let open_spectrum = stage == 4;
        let open_consciousness = stage >= 5;

        self.set_panel_expanded(self.quantum_panel_id, open_quantum);
        self.set_panel_expanded(self.atomic_panel_id, open_atomic);
        self.set_panel_expanded(self.molecular_panel_id, open_molecular);
        self.set_panel_expanded(self.cellular_panel_id, open_cellular);
        self.set_panel_expanded(self.spectrum_continuum_panel_id, open_spectrum);
        self.set_panel_expanded(self.consciousness_panel_id, open_consciousness);
    }

    /// Render dock controls toolbar
    #[allow(dead_code)]
    fn render_dock_controls(&mut self, ctx: &egui::Context) {
        // Collect bookmark info first to avoid borrow conflicts
        let bookmark_info: Vec<(String, String)> = self
            .bookmark_manager
            .get_all_bookmarks()
            .iter()
            .map(|b| (b.id.clone(), b.name.clone()))
            .collect();
        let fps = self.fps;
        let mut visual_ux_mode = self.visual_ux_mode;
        let cpu_update_ms = self.render_stats.cpu_update_ms;
        let scene_render_ms = self.render_stats.scene_render_ms;
        let ui_render_ms = self.render_stats.ui_render_ms;
        let submit_present_ms = self.render_stats.submit_present_ms;

        egui::TopBottomPanel::top("dock_toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Panels:");

                if ui
                    .selectable_label(
                        self.docking_manager
                            .layout
                            .get_panel(self.entity_inspector_panel_id)
                            .map(|p| p.is_expanded())
                            .unwrap_or(false),
                        "🔍 Inspector",
                    )
                    .clicked()
                {
                    self.toggle_panel(self.entity_inspector_panel_id);
                }

                if ui
                    .selectable_label(
                        self.docking_manager
                            .layout
                            .get_panel(self.collective_dashboard_panel_id)
                            .map(|p| p.is_expanded())
                            .unwrap_or(false),
                        "👥 Collective",
                    )
                    .clicked()
                {
                    self.toggle_panel(self.collective_dashboard_panel_id);
                }

                if ui
                    .selectable_label(
                        self.docking_manager
                            .layout
                            .get_panel(self.spectrum_dashboard_panel_id)
                            .map(|p| p.is_expanded())
                            .unwrap_or(false),
                        "📊 Spectrum",
                    )
                    .clicked()
                {
                    self.toggle_panel(self.spectrum_dashboard_panel_id);
                }

                if ui
                    .selectable_label(
                        self.docking_manager
                            .layout
                            .get_panel(self.emergence_dashboard_panel_id)
                            .map(|p| p.is_expanded())
                            .unwrap_or(false),
                        "✨ Emergence",
                    )
                    .clicked()
                {
                    self.toggle_panel(self.emergence_dashboard_panel_id);
                }

                if ui
                    .selectable_label(
                        self.docking_manager
                            .layout
                            .get_panel(self.time_controls_panel_id)
                            .map(|p| p.is_expanded())
                            .unwrap_or(false),
                        "⏱ Time",
                    )
                    .clicked()
                {
                    self.toggle_panel(self.time_controls_panel_id);
                }

                ui.separator();

                // Bookmarks menu - use pre-collected info
                ui.menu_button("📍 Bookmarks", |ui| {
                    for (id, name) in &bookmark_info {
                        if ui.button(name).clicked() {
                            self.bookmark_manager.navigate_to(id, &mut self.camera);
                            ui.close_menu();
                        }
                    }
                    ui.separator();
                    if ui.button("Save Current Position").clicked() {
                        self.save_current_bookmark(0);
                        ui.close_menu();
                    }
                });

                ui.separator();
                ui.label(format!("FPS: {:.0}", fps));

                ui.separator();
                ui.label(format!("Mode: {}", visual_ux_mode.label()));
                if ui
                    .selectable_label(visual_ux_mode == VisualUxMode::Legacy, "Legacy")
                    .clicked()
                {
                    visual_ux_mode = VisualUxMode::Legacy;
                }
                if ui
                    .selectable_label(visual_ux_mode == VisualUxMode::SceneFirst, "Scene-First")
                    .clicked()
                {
                    visual_ux_mode = VisualUxMode::SceneFirst;
                }
                if ui
                    .selectable_label(visual_ux_mode == VisualUxMode::FieldEnhanced, "Field+")
                    .clicked()
                {
                    visual_ux_mode = VisualUxMode::FieldEnhanced;
                }

                ui.separator();
                ui.small(format!(
                    "CPU {:.2}ms | Scene {:.2}ms | UI {:.2}ms | Submit {:.2}ms",
                    cpu_update_ms, scene_render_ms, ui_render_ms, submit_present_ms
                ));
            });
        });
        self.visual_ux_mode = visual_ux_mode;
    }

    /// Render all EGUI panels
    fn resolve_selected_entity(&self) -> Option<crate::entity_layer7::layer7::SubSubLogos> {
        let selected_entity_id = self.entity_inspector.selected_entity_id.as_ref()?;
        let legacy_entities = self.simulation.entities();

        if let Some(entity) = legacy_entities
            .iter()
            .find(|entity| entity.entity_id == *selected_entity_id)
        {
            return Some(entity.clone());
        }

        if let Some(index_suffix) = selected_entity_id.uuid.strip_prefix("holo-") {
            if let Ok(index) = index_suffix.parse::<usize>() {
                return legacy_entities.get(index).cloned();
            }
        }

        None
    }

    fn selected_render_focus_position(&self) -> Option<[f32; 3]> {
        let selected_entity_id = self.entity_inspector.selected_entity_id.as_ref()?;
        let legacy_entities = self.simulation.entities();
        let holo_entities = self.simulation.holo_entities();

        if let Some((index, _)) = legacy_entities
            .iter()
            .enumerate()
            .find(|(_, entity)| entity.entity_id == *selected_entity_id)
        {
            if matches!(
                self.visual_ux_mode,
                VisualUxMode::SceneFirst | VisualUxMode::FieldEnhanced
            ) {
                if let Some(holo) = holo_entities.get(index) {
                    return Some([
                        holo.position[0] as f32,
                        holo.position[1] as f32,
                        holo.position[2] as f32,
                    ]);
                }
            }

            let golden_angle = std::f32::consts::PI * 2.0 / 1.618_034;
            let angle = index as f32 * golden_angle;
            let radius = 0.1 + (index as f32 * 0.02);
            return Some([radius * angle.cos(), radius * angle.sin(), 0.0]);
        }

        if let Some(index_suffix) = selected_entity_id.uuid.strip_prefix("holo-") {
            if let Ok(holo_id) = index_suffix.parse::<usize>() {
                if let Some(index) = holo_entities
                    .iter()
                    .position(|holo| holo.entity_id == holo_id)
                {
                    let holo = &holo_entities[index];
                    return Some([
                        holo.position[0] as f32,
                        holo.position[1] as f32,
                        holo.position[2] as f32,
                    ]);
                }
            }
        }

        None
    }

    /// Render all EGUI panels
    fn render_egui_panels(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        // Early return if no EGUI integration or WGPU context
        if self.egui_integration.is_none() || self.wgpu_context.is_none() {
            return;
        }

        // ========== PHASE 1: Collect all data BEFORE any borrows ==========

        // Collect entity inspector data
        let entity_inspector_show = self.entity_inspector.show_inspector;
        let selected_entity = self.resolve_selected_entity();

        // Collect panel visibility states
        let inspector_expanded = self
            .docking_manager
            .layout
            .get_panel(self.entity_inspector_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let collective_expanded = self
            .docking_manager
            .layout
            .get_panel(self.collective_dashboard_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let spectrum_expanded = self
            .docking_manager
            .layout
            .get_panel(self.spectrum_dashboard_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let emergence_expanded = self
            .docking_manager
            .layout
            .get_panel(self.emergence_dashboard_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let time_expanded = self
            .docking_manager
            .layout
            .get_panel(self.time_controls_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let spectrum_continuum_expanded = self
            .docking_manager
            .layout
            .get_panel(self.spectrum_continuum_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let archetype_expanded = self
            .docking_manager
            .layout
            .get_panel(self.archetype_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let quantum_expanded = self
            .docking_manager
            .layout
            .get_panel(self.quantum_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let atomic_expanded = self
            .docking_manager
            .layout
            .get_panel(self.atomic_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let molecular_expanded = self
            .docking_manager
            .layout
            .get_panel(self.molecular_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let cellular_expanded = self
            .docking_manager
            .layout
            .get_panel(self.cellular_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);
        let consciousness_expanded = self
            .docking_manager
            .layout
            .get_panel(self.consciousness_panel_id)
            .map(|p| p.is_expanded())
            .unwrap_or(false);

        // Collect panel IDs (copy, not reference)
        let entity_inspector_panel_id = self.entity_inspector_panel_id;
        let collective_panel_id = self.collective_dashboard_panel_id;
        let spectrum_panel_id = self.spectrum_dashboard_panel_id;
        let emergence_panel_id = self.emergence_dashboard_panel_id;
        let time_panel_id = self.time_controls_panel_id;
        let spectrum_continuum_panel_id = self.spectrum_continuum_panel_id;
        let archetype_panel_id = self.archetype_panel_id;
        let quantum_panel_id = self.quantum_panel_id;
        let atomic_panel_id = self.atomic_panel_id;
        let molecular_panel_id = self.molecular_panel_id;
        let cellular_panel_id = self.cellular_panel_id;
        let consciousness_panel_id = self.consciousness_panel_id;

        // Collect bookmark info
        let bookmark_info: Vec<(String, String)> = self
            .bookmark_manager
            .get_all_bookmarks()
            .iter()
            .map(|b| (b.id.clone(), b.name.clone()))
            .collect();

        // Collect FPS
        let fps = self.fps;

        // Collect shortcuts overlay state
        let show_shortcuts = self.show_shortcuts_overlay;
        let show_scene_guide = self.show_scene_guide;
        let show_compare_overlay = self.show_compare_overlay;
        let show_emergence_glyphs = self.show_emergence_glyphs;
        let show_emphasis_overlay = self.show_emphasis_overlay;
        let show_selection_semantics = self.show_selection_semantics;
        let show_story_overlay = self.show_story_overlay;
        let auto_overlay_orchestration = self.auto_overlay_orchestration;
        let show_overlay_policy_hud = self.show_overlay_policy_hud;
        let semantic_lod_enabled = self.semantic_lod_enabled;
        let show_semantic_lod_hud = self.show_semantic_lod_hud;
        let show_lens_hud = self.show_lens_hud;
        let show_lens_profile_hud = self.show_lens_profile_hud;
        let _adaptive_lens_profile = self.adaptive_lens_profile;
        let guided_auto_focus = self.guided_auto_focus;
        let show_auto_focus_hud = self.show_auto_focus_hud;
        let auto_focus_strength = self.auto_focus_strength;
        let semantic_lod_level = self.render_stats.semantic_lod_level;
        let semantic_lod_zoom_norm = self.render_stats.semantic_lod_zoom_norm;
        let semantic_lod_focus_weight = self.render_stats.semantic_lod_focus_weight;
        let lens_active = self.render_stats.lens_active;
        let lens_focus_present = self.render_stats.lens_focus_present;
        let lens_radius_used = self.render_stats.lens_radius_used;
        let lens_non_focus_scale_used = self.render_stats.lens_non_focus_scale_used;
        let lens_edge_boost_used = self.render_stats.lens_edge_boost_used;
        let lens_profile_stage = self.render_stats.lens_profile_stage;
        let lens_profile_adaptive = self.render_stats.lens_profile_adaptive;
        let lens_radius_effective = self.render_stats.lens_radius_effective;
        let lens_non_focus_effective = self.render_stats.lens_non_focus_effective;
        let lens_edge_boost_effective = self.render_stats.lens_edge_boost_effective;
        let auto_focus_active = self.render_stats.auto_focus_active;
        let auto_focus_target_zoom = self.render_stats.auto_focus_target_zoom;
        let auto_focus_distance = self.render_stats.auto_focus_distance;
        let auto_focus_stage = self.render_stats.auto_focus_stage;
        let show_benchmark_hud = self.show_benchmark_hud;
        let benchmark_mode_label = self.scene_benchmark_mode.label();
        let guided_story_mode = self.guided_story_mode;
        let adaptive_scene_emphasis = self.adaptive_scene_emphasis;
        let adaptive_connection_readability = self.adaptive_connection_readability;
        let adaptive_connection_color_grading = self.adaptive_connection_color_grading;
        let show_connection_hud = self.show_connection_hud;
        let show_connection_color_hud = self.show_connection_color_hud;
        let conn_color_warm_cool_bias = self.render_stats.conn_color_warm_cool_bias;
        let conn_color_saturation = self.render_stats.conn_color_saturation;
        let conn_color_phase_boost = self.render_stats.conn_color_phase_boost;
        let conn_color_grade_strength = self.render_stats.conn_color_grade_strength;
        let holo_stats = self.simulation.get_holo_statistics();

        // Compute context-sensitive overlay policy while preserving raw toggles.
        let has_selection = selected_entity.is_some();
        let story_active = show_story_overlay;
        let compare_active = show_compare_overlay;
        let adaptive_active = adaptive_scene_emphasis;

        let (
            mut effective_scene_guide,
            mut effective_compare,
            mut effective_glyphs,
            mut effective_story,
            mut effective_emphasis,
            mut effective_semantics,
        ) = if auto_overlay_orchestration {
            let scene_guide = show_scene_guide;
            let compare = show_compare_overlay;

            let semantics = if has_selection {
                show_selection_semantics
            } else {
                false
            };

            let glyphs = if has_selection {
                false
            } else {
                show_emergence_glyphs
            };

            let mut story = show_story_overlay && !compare;
            let mut emphasis = show_emphasis_overlay && adaptive_active && !compare;

            if compare {
                story = false;
                emphasis = false;
            }

            (scene_guide, compare, glyphs, story, emphasis, semantics)
        } else {
            (
                show_scene_guide,
                show_compare_overlay,
                show_emergence_glyphs,
                show_story_overlay,
                show_emphasis_overlay,
                show_selection_semantics,
            )
        };

        if semantic_lod_enabled {
            match semantic_lod_level {
                2 => {
                    effective_compare = show_compare_overlay;
                    effective_glyphs = false;
                    effective_story = false;
                    effective_emphasis = false;
                    effective_semantics = has_selection && show_selection_semantics;
                    effective_scene_guide = show_scene_guide;
                }
                1 => {}
                _ => {}
            }
        }

        // Collect scene guide metadata
        let visual_mode_label = self.visual_ux_mode.label();
        let active_entity_source = if self.render_stats.using_holo_entities {
            "Holo"
        } else {
            "Legacy"
        };
        let current_entity_count = self.render_stats.entity_count;
        let legacy_entity_count = self.render_stats.legacy_entity_count;
        let holo_entity_count = self.render_stats.holo_entity_count;
        let active_connection_count = self.render_stats.active_connection_count;
        let structural_connection_count = self.render_stats.structural_connection_count;
        let phase_connection_count = self.render_stats.phase_connection_count;
        let phase_edge_count = self.render_stats.phase_edge_count;
        let using_holo_entities = self.render_stats.using_holo_entities;
        let using_holo_connections = self.render_stats.using_holo_connections;
        let emphasis_stage = self.render_stats.emphasis_stage;
        let phase_threshold_used = self.render_stats.phase_threshold_used;
        let field_emphasis = self.render_stats.field_emphasis;
        let connection_emphasis = self.render_stats.connection_emphasis;
        let connection_structural_scale = self.render_stats.connection_structural_scale;
        let connection_phase_scale = self.render_stats.connection_phase_scale;
        let connection_intensity_bias = self.render_stats.connection_intensity_bias;
        let benchmark_effective_field_on = self.render_stats.benchmark_effective_field_on;
        let benchmark_effective_morphology_on = self.render_stats.benchmark_effective_morphology_on;
        let benchmark_effective_focus_halo_on = self.render_stats.benchmark_effective_focus_halo_on;
        let benchmark_effective_declutter_on = match self.scene_benchmark_mode {
            SceneBenchmarkMode::Clarity => false,
            SceneBenchmarkMode::Performance => true,
            SceneBenchmarkMode::Off => self.adaptive_connection_declutter,
        };
        let benchmark_mode_code = self.render_stats.benchmark_mode;
        let benchmark_field_steps = self.render_stats.field_ray_steps;
        let benchmark_field_density = self.render_stats.field_density_scale;
        let benchmark_field_absorption = self.render_stats.field_absorption;

        // Collect emergence dashboard state
        let show_biological = self.emergence_dashboard.show_biological_graph;
        let show_noospheric = self.emergence_dashboard.show_noospheric_graph;
        let show_gaia = self.emergence_dashboard.show_gaia_graph;

        // Clone visualizer data needed
        let collective_show_dashboard = self.collective_dashboard.show_dashboard;

        // Collect spectrum distribution data based on active render source
        let spectrum_positions: Vec<f32> = match self.visual_ux_mode {
            VisualUxMode::Legacy => self
                .simulation
                .entities()
                .iter()
                .map(|entity| entity.spectrum_position as f32)
                .collect(),
            VisualUxMode::SceneFirst | VisualUxMode::FieldEnhanced => {
                let holo_entities = self.simulation.holo_entities();
                if !holo_entities.is_empty() {
                    holo_entities
                        .iter()
                        .map(|entity| (entity.density_band.index() as f32) / 7.0)
                        .collect()
                } else {
                    self.simulation
                        .entities()
                        .iter()
                        .map(|entity| entity.spectrum_position as f32)
                        .collect()
                }
            }
        };

        // ========== PHASE 2: Begin EGUI frame ==========

        // Get mutable access to EGUI
        let egui = self.egui_integration.as_mut().unwrap();
        egui.begin_frame(self.window.as_ref());
        let egui_ctx = egui.context().clone();

        // ========== PHASE 3: Render panels (using collected data) ==========

        let mut toolbar_focus_dominant = false;
        let mut story_focus_dominant = false;
        let mut story_selected_stage: Option<u8> = None;

        // Render dock controls toolbar
        egui::TopBottomPanel::top("dock_toolbar").show(&egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Panels:");

                if ui
                    .selectable_label(inspector_expanded, "🔍 Inspector")
                    .clicked()
                {
                    let _ = self
                        .docking_manager
                        .layout
                        .toggle_panel(entity_inspector_panel_id);
                }
                if ui
                    .selectable_label(collective_expanded, "👥 Collective")
                    .clicked()
                {
                    let _ = self
                        .docking_manager
                        .layout
                        .toggle_panel(collective_panel_id);
                }
                if ui
                    .selectable_label(spectrum_expanded, "📊 Spectrum")
                    .clicked()
                {
                    let _ = self.docking_manager.layout.toggle_panel(spectrum_panel_id);
                }
                if ui
                    .selectable_label(emergence_expanded, "✨ Emergence")
                    .clicked()
                {
                    let _ = self.docking_manager.layout.toggle_panel(emergence_panel_id);
                }
                if ui.selectable_label(time_expanded, "⏱ Time").clicked() {
                    let _ = self.docking_manager.layout.toggle_panel(time_panel_id);
                }
                if ui
                    .selectable_label(spectrum_continuum_expanded, "🌌 Continuum")
                    .clicked()
                {
                    let _ = self
                        .docking_manager
                        .layout
                        .toggle_panel(spectrum_continuum_panel_id);
                }
                if ui
                    .selectable_label(archetype_expanded, "🔮 Archetype")
                    .clicked()
                {
                    let _ = self.docking_manager.layout.toggle_panel(archetype_panel_id);
                }
                if ui.selectable_label(quantum_expanded, "⚛ Quantum").clicked() {
                    let _ = self.docking_manager.layout.toggle_panel(quantum_panel_id);
                }
                if ui.selectable_label(atomic_expanded, "⚡ Atomic").clicked() {
                    let _ = self.docking_manager.layout.toggle_panel(atomic_panel_id);
                }
                if ui
                    .selectable_label(molecular_expanded, "🧬 Molecular")
                    .clicked()
                {
                    let _ = self.docking_manager.layout.toggle_panel(molecular_panel_id);
                }
                if ui
                    .selectable_label(cellular_expanded, "🔬 Cellular")
                    .clicked()
                {
                    let _ = self.docking_manager.layout.toggle_panel(cellular_panel_id);
                }
                if ui
                    .selectable_label(consciousness_expanded, "🧠 Consciousness")
                    .clicked()
                {
                    let _ = self
                        .docking_manager
                        .layout
                        .toggle_panel(consciousness_panel_id);
                }

                ui.separator();

                // Bookmarks menu
                ui.menu_button("📍 Bookmarks", |ui| {
                    for (id, name) in &bookmark_info {
                        if ui.button(name).clicked() {
                            self.bookmark_manager.navigate_to(id, &mut self.camera);
                            ui.close_menu();
                        }
                    }
                    ui.separator();
                    if ui.button("Save Current Position").clicked() {
                        // Save to quick slot 0
                        let id = format!("quick_{}", 0);
                        self.bookmark_manager
                            .bookmark_current(id, "Quick Slot 1", &self.camera);
                        ui.close_menu();
                    }
                });

                ui.separator();
                ui.label(format!("FPS: {:.0}", fps));

                ui.separator();
                if ui.button("⚖ Bench").clicked() {
                    self.scene_benchmark_mode = self.scene_benchmark_mode.next();
                }
                ui.label(self.scene_benchmark_mode.label().to_string());
                if ui
                    .selectable_label(self.show_benchmark_hud, "🧪 Bench HUD")
                    .clicked()
                {
                    self.show_benchmark_hud = !self.show_benchmark_hud;
                }

                ui.separator();
                if ui
                    .selectable_label(self.show_compare_overlay, "🧪 Compare")
                    .clicked()
                {
                    self.show_compare_overlay = !self.show_compare_overlay;
                }
                if ui
                    .selectable_label(self.show_emergence_glyphs, "🧭 Glyphs")
                    .clicked()
                {
                    self.show_emergence_glyphs = !self.show_emergence_glyphs;
                }
                if ui
                    .selectable_label(self.adaptive_scene_emphasis, "🎛 Emphasis")
                    .clicked()
                {
                    self.adaptive_scene_emphasis = !self.adaptive_scene_emphasis;
                }
                if ui
                    .selectable_label(self.show_emphasis_overlay, "📈 Emphasis HUD")
                    .clicked()
                {
                    self.show_emphasis_overlay = !self.show_emphasis_overlay;
                }
                if ui
                    .selectable_label(self.adaptive_connection_readability, "🕸 Links")
                    .clicked()
                {
                    self.adaptive_connection_readability = !self.adaptive_connection_readability;
                }
                if ui
                    .selectable_label(self.show_connection_hud, "📶 Links HUD")
                    .clicked()
                {
                    self.show_connection_hud = !self.show_connection_hud;
                }
                if ui
                    .selectable_label(self.adaptive_connection_color_grading, "🎨 Link Color")
                    .clicked()
                {
                    self.adaptive_connection_color_grading =
                        !self.adaptive_connection_color_grading;
                }
                if ui
                    .selectable_label(self.show_connection_color_hud, "🖌 Color HUD")
                    .clicked()
                {
                    self.show_connection_color_hud = !self.show_connection_color_hud;
                }
                if ui
                    .selectable_label(self.adaptive_connection_declutter, "🧹 Declutter")
                    .clicked()
                {
                    self.adaptive_connection_declutter = !self.adaptive_connection_declutter;
                }
                if ui
                    .selectable_label(self.show_declutter_hud, "🪟 Declutter HUD")
                    .clicked()
                {
                    self.show_declutter_hud = !self.show_declutter_hud;
                }
                if ui
                    .selectable_label(self.adaptive_field_quality, "🌫 FieldQ")
                    .clicked()
                {
                    self.adaptive_field_quality = !self.adaptive_field_quality;
                }
                if ui
                    .selectable_label(self.show_field_quality_hud, "📊 Field HUD")
                    .clicked()
                {
                    self.show_field_quality_hud = !self.show_field_quality_hud;
                }
                if ui
                    .selectable_label(self.show_story_overlay, "🎬 Story")
                    .clicked()
                {
                    self.show_story_overlay = !self.show_story_overlay;
                }
                if ui
                    .selectable_label(self.auto_overlay_orchestration, "🧩 Auto UI")
                    .clicked()
                {
                    self.auto_overlay_orchestration = !self.auto_overlay_orchestration;
                }
                if ui
                    .selectable_label(self.show_overlay_policy_hud, "📋 Policy")
                    .clicked()
                {
                    self.show_overlay_policy_hud = !self.show_overlay_policy_hud;
                }
                if ui
                    .selectable_label(self.semantic_lod_enabled, "🧠 LOD")
                    .clicked()
                {
                    self.semantic_lod_enabled = !self.semantic_lod_enabled;
                }
                if ui
                    .selectable_label(self.show_semantic_lod_hud, "🪄 LOD HUD")
                    .clicked()
                {
                    self.show_semantic_lod_hud = !self.show_semantic_lod_hud;
                }
                if ui
                    .selectable_label(self.selection_neighborhood_lens, "🔍 Lens")
                    .clicked()
                {
                    self.selection_neighborhood_lens = !self.selection_neighborhood_lens;
                }
                if ui
                    .selectable_label(self.show_lens_hud, "🪞 Lens HUD")
                    .clicked()
                {
                    self.show_lens_hud = !self.show_lens_hud;
                }
                if ui
                    .selectable_label(self.adaptive_lens_profile, "🧬 Lens+")
                    .clicked()
                {
                    self.adaptive_lens_profile = !self.adaptive_lens_profile;
                }
                if ui
                    .selectable_label(self.show_lens_profile_hud, "📐 Lens Profile")
                    .clicked()
                {
                    self.show_lens_profile_hud = !self.show_lens_profile_hud;
                }
                if ui
                    .selectable_label(self.guided_auto_focus, "🧲 AutoFocus")
                    .clicked()
                {
                    self.guided_auto_focus = !self.guided_auto_focus;
                }
                if ui
                    .selectable_label(self.show_auto_focus_hud, "🎯 Focus HUD")
                    .clicked()
                {
                    self.show_auto_focus_hud = !self.show_auto_focus_hud;
                }
                if ui
                    .selectable_label(self.show_selection_semantics, "🏷 Semantics")
                    .clicked()
                {
                    self.show_selection_semantics = !self.show_selection_semantics;
                }
                if ui
                    .selectable_label(self.selection_focus_halo, "◎ Focus")
                    .clicked()
                {
                    self.selection_focus_halo = !self.selection_focus_halo;
                }
                if ui.button("🎯 Focus").clicked() {
                    toolbar_focus_dominant = true;
                }
                if ui
                    .selectable_label(self.stage_morphology_enabled, "◈ Morph")
                    .clicked()
                {
                    self.stage_morphology_enabled = !self.stage_morphology_enabled;
                }
            });
        });

        if toolbar_focus_dominant {
            self.apply_stage_preset(emphasis_stage);
        }

        // Render entity inspector panel
        if entity_inspector_show {
            self.entity_inspector
                .show(&egui_ctx, selected_entity.as_ref());
        }

        // Render collective dashboard
        {
            self.collective_dashboard.show_dashboard = collective_show_dashboard;
            let visualizer = &self.collective_visualizer;
            egui::Window::new("Collective Dynamics")
                .resizable(true)
                .default_size(egui::vec2(400.0, 500.0))
                .show(&egui_ctx, |ui| {
                    ui.label(format!(
                        "Source: {}",
                        if using_holo_entities {
                            "Holo"
                        } else {
                            "Legacy"
                        }
                    ));
                    ui.label(format!(
                        "Entities (legacy/holo): {} / {}",
                        legacy_entity_count, holo_entity_count
                    ));
                    ui.label(format!("Connections active: {}", active_connection_count));
                    ui.label(format!("Phase edges: {}", phase_edge_count));
                    ui.separator();
                    self.collective_dashboard.render(ui, visualizer);
                });
        }

        if effective_compare {
            egui::Window::new("Compare Mode")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_TOP, [12.0, 48.0])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(280.0);
                    ui.heading("Legacy Baseline");
                    ui.label(format!("Entities: {}", legacy_entity_count));
                    ui.small("index-spiral structural topology");
                    ui.separator();

                    ui.heading("Scene-First");
                    ui.label(format!("Entities: {}", holo_entity_count));
                    ui.label(format!(
                        "Entity source active: {}",
                        if using_holo_entities {
                            "Holo stream"
                        } else {
                            "Legacy fallback"
                        }
                    ));
                    ui.label(format!(
                        "Connections active: {} | Phase edges: {}",
                        active_connection_count, phase_edge_count
                    ));
                    ui.separator();

                    ui.small(format!(
                        "Current: entities={} | connections={}",
                        if using_holo_entities {
                            "Holo"
                        } else {
                            "Legacy"
                        },
                        if using_holo_connections {
                            "Holo"
                        } else {
                            "Legacy"
                        }
                    ));
                    ui.small("Use V to switch modes, B to hide compare");
                });
        }

        // Render spectrum dashboard
        self.spectrum_dashboard.show(&egui_ctx);

        // Phase B.2: Spectrum Continuum Panel
        self.spectrum_continuum_panel.show(&egui_ctx);

        // Update entity distribution for spectrum panel
        self.spectrum_continuum_panel
            .update_distribution(&spectrum_positions);

        // Phase B.3: Archetype Panel - update with selected entity
        if let Some(selected_entity) = selected_entity.as_ref() {
            self.archetype_panel.update_from_entity(selected_entity);
        }
        self.archetype_panel.show(&egui_ctx);

        // Phase C.1: Quantum Panel - update with selected entity
        if let Some(selected_entity) = selected_entity.as_ref() {
            self.quantum_panel.update_from_entity(selected_entity);
        }
        self.quantum_panel.show(&egui_ctx);

        // Phase C.2: Atomic Panel - update with selected entity
        if let Some(selected_entity) = selected_entity.as_ref() {
            self.atomic_panel.update_from_entity(selected_entity);
        }
        self.atomic_panel.show(&egui_ctx);

        // Phase C.3: Molecular Panel - update with selected entity
        if let Some(selected_entity) = selected_entity.as_ref() {
            self.molecular_panel.update_from_entity(selected_entity);
        }
        self.molecular_panel.show(&egui_ctx);

        // Phase C.4: Cellular Panel - update with selected entity
        if let Some(selected_entity) = selected_entity.as_ref() {
            self.cellular_panel.update_from_entity(selected_entity);
        }
        self.cellular_panel.show(&egui_ctx);

        // Phase D: Consciousness Panel - update with selected entity
        if let Some(selected_entity) = selected_entity.as_ref() {
            self.consciousness_panel.update_from_entity(selected_entity);
        }
        self.consciousness_panel.show(&egui_ctx);

        // Render emergence dashboard
        {
            egui::Window::new("Emergence Dashboard")
                .resizable(true)
                .default_size(egui::vec2(400.0, 500.0))
                .show(&egui_ctx, |ui| {
                    ui.label(format!("Biological events: {}", show_biological));
                    ui.label(format!("Noospheric events: {}", show_noospheric));
                    ui.label(format!("Gaia events: {}", show_gaia));
                    ui.separator();
                    ui.label("Emergence metrics visualization");
                    ui.label("Use F5 to toggle this panel");
                });
        }

        // Render time controls
        self.time_controls.show(&egui_ctx);

        // Render keyboard shortcuts overlay if enabled
        if show_shortcuts {
            egui::Window::new("Keyboard Shortcuts")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(300.0);
                    ui.heading("⌨ Keyboard Shortcuts");
                    ui.separator();

                    ui.label("F1 - Show/Hide this overlay");
                    ui.label("F2 - Toggle Entity Inspector");
                    ui.label("F3 - Toggle Collective Dashboard");
                    ui.label("F4 - Toggle Spectrum Dashboard");
                    ui.label("F5 - Toggle Emergence Dashboard");
                    ui.label("F6 - Toggle Time Controls");
                    ui.label("F7 - Reset view to Home");
                    ui.label("F8 - Toggle Adaptive Declutter");
                    ui.label("F9 - Toggle Declutter HUD");
                    ui.label("F10 - Toggle Adaptive Field Quality");
                    ui.label("F11 - Toggle Field Quality HUD");
                    ui.label("F12 - Cycle Benchmark Mode");
                    ui.label("F13 or E - Toggle Semantic LOD");
                    ui.label("F14 or W - Toggle Semantic LOD HUD");
                    ui.label(format!(
                        "{} - Toggle Neighborhood Lens",
                        LENS_TOGGLE_KEYS_LABEL
                    ));
                    ui.label(format!("{} - Toggle Lens HUD", LENS_HUD_KEYS_LABEL));
                    ui.label(format!(
                        "{} - Toggle Guided Auto-Focus",
                        AUTO_FOCUS_TOGGLE_KEYS_LABEL
                    ));
                    ui.label(format!(
                        "{} - Toggle Auto-Focus HUD",
                        AUTO_FOCUS_HUD_KEYS_LABEL
                    ));
                    ui.label(format!(
                        "{} - Toggle Adaptive Lens Profile",
                        LENS_PROFILE_TOGGLE_KEYS_LABEL
                    ));
                    ui.label(format!(
                        "{} - Toggle Lens Profile HUD",
                        LENS_PROFILE_HUD_KEYS_LABEL
                    ));
                    ui.label("` - Toggle Benchmark HUD");
                    ui.label("B - Toggle Compare Overlay");
                    ui.label("V - Cycle Visual UX Mode");
                    ui.label("G - Toggle Scene Guide");
                    ui.label("F - Toggle Holographic Field (Phase B.1)");
                    ui.label("S - Toggle Spectrum Continuum Panel (Phase B.2)");
                    ui.label("A - Toggle Archetype Panel (Phase B.3)");
                    ui.label("Q - Toggle Quantum Panel (Phase C.1)");
                    ui.label("Z - Toggle Atomic Panel (Phase C.2)");
                    ui.label("M - Toggle Molecular Panel (Phase C.3)");
                    ui.label("C - Toggle Cellular Panel (Phase C.4)");
                    ui.label("H - Toggle Consciousness Panel (Phase D)");
                    ui.label("N - Toggle Emergence Glyphs");
                    ui.label("J - Toggle Adaptive Emphasis");
                    ui.label("K - Toggle Emphasis HUD");
                    ui.label("X - Toggle Adaptive Link Readability");
                    ui.label("T - Toggle Link Readability HUD");
                    ui.label(format!(
                        "{} - Toggle Adaptive Link Color",
                        CONNECTION_COLOR_TOGGLE_KEYS_LABEL
                    ));
                    ui.label(format!(
                        "{} - Toggle Link Color HUD",
                        CONNECTION_COLOR_HUD_KEYS_LABEL
                    ));
                    ui.label("L - Toggle Stage Morphology");
                    ui.label("◎ Focus - Toggle from toolbar");
                    ui.label("Y - Toggle Selection Semantics");
                    ui.label("O - Toggle Auto Overlay Orchestration");
                    ui.label("I - Toggle Overlay Policy HUD");
                    ui.separator();

                    ui.label("1-6 - Switch views");
                    ui.label("Esc - Exit application");
                    ui.separator();

                    ui.label("Mouse: Click entities to inspect");
                    ui.label("Mouse: Drag to pan camera");
                    ui.label("Mouse: Scroll to zoom");

                    ui.separator();
                    ui.small("Press F1 to close this overlay");
                });
        }

        if effective_scene_guide {
            egui::Window::new("Scene Guide")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!("Mode: {}", visual_mode_label));
                    ui.label(format!("Entity source: {}", active_entity_source));
                    ui.label(format!("Entities: {}", current_entity_count));
                    ui.separator();
                    ui.label("Connection legend:");
                    ui.colored_label(
                        egui::Color32::from_rgb(51, 102, 255),
                        "Blue = Collective/parent-like links",
                    );
                    ui.colored_label(
                        egui::Color32::from_rgb(51, 255, 102),
                        "Green = Same-density structural links",
                    );
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 204, 51),
                        "Yellow = Cross-density/environment links",
                    );
                    ui.colored_label(
                        egui::Color32::from_rgb(89, 242, 255),
                        "Cyan = Phase coherence links",
                    );
                    ui.separator();
                    ui.small("Press V to change mode, G to hide guide");
                });
        }

        if effective_semantics {
            let short_entity_id = |entity_id: &crate::entity_layer7::layer7::EntityId| {
                let full_id = entity_id.to_string();
                if full_id.len() > 24 {
                    format!("{}...{}", &full_id[..12], &full_id[full_id.len() - 8..])
                } else {
                    full_id
                }
            };

            let active_mode_source = format!("{} / {}", visual_mode_label, active_entity_source);

            egui::Window::new("Selection Semantics")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, -12.0])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(330.0);

                    if let Some(entity) = selected_entity.as_ref() {
                        ui.label(format!("Entity ID: {}", short_entity_id(&entity.entity_id)));
                        ui.label(format!(
                            "Density: L{} ({})",
                            entity.current_density.as_u8(),
                            entity.current_density
                        ));
                        ui.label(format!("Consciousness: {:.3}", entity.consciousness_level));
                        ui.label(format!(
                            "Polarity: {} @ {:.1}%",
                            entity.polarization.direction.display_name(),
                            entity.polarization.intensity * 100.0
                        ));
                        ui.label(format!(
                            "Spectrum: {:.3} | Veil: {:.3}",
                            entity.spectrum_position,
                            entity.veil_transparency
                        ));
                        ui.label(format!(
                            "Evolution: clock {:.2} | progress {:.1}%",
                            entity.evolution_clock,
                            entity.learning_progress * 100.0
                        ));
                    } else {
                        ui.strong("Click an entity to inspect semantics");
                    }

                    ui.separator();
                    ui.label(format!("Mode / Source: {}", active_mode_source));
                    ui.label(format!(
                        "Counts: entities {} | connections {} | phase edges {}",
                        current_entity_count,
                        active_connection_count,
                        phase_edge_count
                    ));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(emphasis_stage)
                    ));

                    ui.separator();
                    ui.label("Legend:");
                    ui.small("- Morphologies follow dominant emergence stage (toggle L)");
                    ui.small("- Connections: Blue=parent, Green=same density, Yellow=cross density, Cyan=phase coherence");
                    ui.small("- Story focus controls: use toolbar Focus and Story toggles");
                    ui.small("- Adaptive emphasis controls: J adaptive emphasis, K emphasis HUD");
                });
        }

        if effective_emphasis {
            egui::Window::new("Adaptive Emphasis")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_TOP, [-12.0, 220.0])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Adaptive: {}",
                        if adaptive_scene_emphasis { "ON" } else { "OFF" }
                    ));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(emphasis_stage)
                    ));
                    ui.label(format!("Phase threshold: {:.2}", phase_threshold_used));
                    ui.label(format!("Field emphasis: {:.2}", field_emphasis));
                    ui.label(format!("Connection emphasis: {:.2}", connection_emphasis));
                    ui.separator();
                    ui.small("J toggles adaptive, K toggles HUD");
                });
        }

        if show_connection_hud {
            egui::Window::new("Link Readability")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, -12.0])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Adaptive: {}",
                        if adaptive_connection_readability {
                            "ON"
                        } else {
                            "OFF"
                        }
                    ));
                    ui.label(format!(
                        "Counts total/struct/phase: {} / {} / {}",
                        active_connection_count,
                        structural_connection_count,
                        phase_connection_count
                    ));
                    ui.label(format!(
                        "Profile s/p/b: {:.2} / {:.2} / {:.2}",
                        connection_structural_scale,
                        connection_phase_scale,
                        connection_intensity_bias
                    ));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(emphasis_stage)
                    ));
                    ui.separator();
                    ui.small("X toggles adaptive links, T toggles HUD");
                });
        }

        if show_connection_color_hud {
            let color_hud_anchor_y = if show_connection_hud { -176.0 } else { -12.0 };
            egui::Window::new("Link Color Grade")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, color_hud_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Adaptive: {}",
                        if adaptive_connection_color_grading {
                            "ON"
                        } else {
                            "OFF"
                        }
                    ));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(emphasis_stage)
                    ));
                    ui.label(format!("Warm/Cool bias: {:.2}", conn_color_warm_cool_bias));
                    ui.label(format!("Saturation: {:.2}", conn_color_saturation));
                    ui.label(format!("Phase boost: {:.2}", conn_color_phase_boost));
                    ui.label(format!("Strength: {:.2}", conn_color_grade_strength));
                    ui.separator();
                    ui.small(format!(
                        "{} toggles color grading, {} toggles this HUD",
                        CONNECTION_COLOR_TOGGLE_KEYS_LABEL, CONNECTION_COLOR_HUD_KEYS_LABEL
                    ));
                });
        }

        if self.show_declutter_hud {
            let mut declutter_anchor_y = -12.0;
            if show_connection_hud {
                declutter_anchor_y -= 156.0;
            }
            if show_connection_color_hud {
                declutter_anchor_y -= 168.0;
            }
            egui::Window::new("Declutter")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, declutter_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Adaptive declutter: {}",
                        if self.adaptive_connection_declutter {
                            "ON"
                        } else {
                            "OFF"
                        }
                    ));
                    ui.label(format!(
                        "Camera zoom: {:.6e}",
                        self.render_stats.camera_zoom
                    ));
                    ui.label(format!(
                        "Declutter factor: {:.2}",
                        self.render_stats.declutter_factor
                    ));
                    ui.label(format!(
                        "Effective phase threshold: {:.2}",
                        self.render_stats.phase_threshold_effective
                    ));
                    ui.label(format!(
                        "Profile s/p/b: {:.2} / {:.2} / {:.2}",
                        self.render_stats.connection_structural_scale,
                        self.render_stats.connection_phase_scale,
                        self.render_stats.connection_intensity_bias
                    ));
                    ui.separator();
                    ui.small("F8 toggles declutter, F9 toggles HUD");
                });
        }

        if self.show_field_quality_hud {
            let mut field_quality_anchor_y = -12.0;
            if show_connection_hud {
                field_quality_anchor_y -= 156.0;
            }
            if show_connection_color_hud {
                field_quality_anchor_y -= 168.0;
            }
            if self.show_declutter_hud {
                field_quality_anchor_y -= 156.0;
            }
            egui::Window::new("Field Quality")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, field_quality_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Adaptive: {}",
                        if self.adaptive_field_quality {
                            "ON"
                        } else {
                            "OFF"
                        }
                    ));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(self.render_stats.field_quality_stage)
                    ));
                    ui.label(format!("Ray steps: {}", self.render_stats.field_ray_steps));
                    ui.label(format!(
                        "Density scale: {:.2}",
                        self.render_stats.field_density_scale
                    ));
                    ui.label(format!(
                        "Absorption: {:.2}",
                        self.render_stats.field_absorption
                    ));
                    ui.separator();
                    ui.small("F10 toggles adaptive field quality, F11 toggles HUD");
                });
        }

        if show_benchmark_hud {
            let mut benchmark_anchor_y = -12.0;
            if show_connection_hud {
                benchmark_anchor_y -= 156.0;
            }
            if show_connection_color_hud {
                benchmark_anchor_y -= 168.0;
            }
            if self.show_declutter_hud {
                benchmark_anchor_y -= 156.0;
            }
            if self.show_field_quality_hud {
                benchmark_anchor_y -= 156.0;
            }

            egui::Window::new("Benchmark")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, benchmark_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(280.0);
                    ui.label(format!(
                        "Mode: {} (#{})",
                        benchmark_mode_label, benchmark_mode_code
                    ));
                    ui.label(format!("FPS: {:.1}", fps));
                    ui.separator();
                    ui.label(format!(
                        "Effective toggles: field={} morph={} focus={} declutter={}",
                        benchmark_effective_field_on,
                        benchmark_effective_morphology_on,
                        benchmark_effective_focus_halo_on,
                        benchmark_effective_declutter_on
                    ));
                    ui.label(format!(
                        "Effective phase threshold: {:.2}",
                        phase_threshold_used
                    ));
                    ui.label(format!(
                        "Connection s/p/b: {:.2} / {:.2} / {:.2}",
                        connection_structural_scale,
                        connection_phase_scale,
                        connection_intensity_bias
                    ));
                    ui.label(format!(
                        "Field steps/density/absorption: {} / {:.2} / {:.2}",
                        benchmark_field_steps, benchmark_field_density, benchmark_field_absorption
                    ));
                    ui.separator();
                    ui.small("F12 cycles mode, ` toggles HUD");
                });
        }

        if show_auto_focus_hud {
            let mut auto_focus_anchor_y = -12.0;
            if show_connection_hud {
                auto_focus_anchor_y -= 156.0;
            }
            if show_connection_color_hud {
                auto_focus_anchor_y -= 168.0;
            }
            if self.show_declutter_hud {
                auto_focus_anchor_y -= 156.0;
            }
            if self.show_field_quality_hud {
                auto_focus_anchor_y -= 156.0;
            }
            if show_benchmark_hud {
                auto_focus_anchor_y -= 178.0;
            }

            egui::Window::new("Auto Focus")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, auto_focus_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Autofocus: {}",
                        if guided_auto_focus { "ON" } else { "OFF" }
                    ));
                    ui.label(format!("Active this frame: {}", auto_focus_active));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(auto_focus_stage)
                    ));
                    ui.label(format!("Target zoom: {:.2}", auto_focus_target_zoom));
                    ui.label(format!("Focus distance: {:.4}", auto_focus_distance));
                    ui.label(format!("Strength: {:.2}", auto_focus_strength));
                    ui.separator();
                    ui.small(format!(
                        "{} toggles autofocus, {} toggles this HUD",
                        AUTO_FOCUS_TOGGLE_KEYS_LABEL, AUTO_FOCUS_HUD_KEYS_LABEL
                    ));
                });
        }

        if effective_story {
            let story_anchor_y = if effective_compare { 300.0 } else { 96.0 };
            egui::Window::new("Guided Story")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_TOP, [12.0, story_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Guided mode: {}",
                        if guided_story_mode { "ON" } else { "OFF" }
                    ));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(emphasis_stage)
                    ));
                    ui.separator();

                    if ui.button("Focus Dominant").clicked() {
                        story_focus_dominant = true;
                    }

                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Quantum").clicked() {
                            story_selected_stage = Some(0);
                        }
                        if ui.button("Atomic").clicked() {
                            story_selected_stage = Some(1);
                        }
                        if ui.button("Molecular").clicked() {
                            story_selected_stage = Some(2);
                        }
                        if ui.button("Cellular").clicked() {
                            story_selected_stage = Some(3);
                        }
                        if ui.button("Planetary").clicked() {
                            story_selected_stage = Some(4);
                        }
                        if ui.button("Integration").clicked() {
                            story_selected_stage = Some(5);
                        }
                    });

                    ui.separator();
                    ui.small("Use toolbar Story toggle to hide this overlay");
                });
        }

        if let Some(stage) = story_selected_stage {
            self.apply_stage_preset(stage);
        } else if story_focus_dominant {
            self.apply_stage_preset(emphasis_stage);
        }

        if effective_glyphs {
            let glyph_anchor_y = if effective_compare { 292.0 } else { 48.0 };
            egui::Window::new("Emergence Glyphs")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_TOP, [12.0, glyph_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(340.0);

                    if let Some(stats) = holo_stats.as_ref() {
                        let normalize_count = |value: usize, cap: usize| -> f32 {
                            if cap == 0 {
                                return 0.0;
                            }
                            (value as f32 / cap as f32).clamp(0.0, 1.0)
                        };

                        let render_stage = |
                            ui: &mut egui::Ui,
                            icon: &str,
                            label: &str,
                            progress: f32,
                            metric: String,
                        | {
                            ui.horizontal(|ui| {
                                ui.label(format!("{} {}", icon, label));
                                ui.add(
                                    egui::ProgressBar::new(progress.clamp(0.0, 1.0))
                                        .desired_width(120.0)
                                        .show_percentage(),
                                );
                                ui.small(metric);
                            });
                        };

                        let quantum_progress = 0.5
                            * (normalize_count(stats.quantum_regions, 128)
                                + normalize_count(stats.particles, 20_000));
                        let atomic_progress = 0.5
                            * (normalize_count(stats.atomic_regions, 128)
                                + normalize_count(stats.atoms, 10_000));
                        let molecular_progress = 0.5
                            * (normalize_count(stats.molecular_regions, 128)
                                + normalize_count(stats.molecules, 5_000));
                        let cellular_progress = 0.5
                            * (normalize_count(stats.cells, 2_000)
                                + normalize_count(stats.species, 400));
                        let planetary_progress = 0.5
                            * (normalize_count(stats.planets, 128)
                                + normalize_count(stats.terrain_cells, 8_000));
                        let integration_progress = 0.5
                            * ((stats.integration_coherence as f32).clamp(0.0, 1.0)
                                + (stats.integration_stability as f32).clamp(0.0, 1.0));

                        render_stage(
                            ui,
                            "⚛",
                            "Quantum",
                            quantum_progress,
                            format!("regions {} | particles {}", stats.quantum_regions, stats.particles),
                        );
                        render_stage(
                            ui,
                            "🧱",
                            "Atomic",
                            atomic_progress,
                            format!("regions {} | atoms {}", stats.atomic_regions, stats.atoms),
                        );
                        render_stage(
                            ui,
                            "🧬",
                            "Molecular",
                            molecular_progress,
                            format!("regions {} | molecules {}", stats.molecular_regions, stats.molecules),
                        );
                        render_stage(
                            ui,
                            "🫧",
                            "Cellular",
                            cellular_progress,
                            format!("cells {} | species {}", stats.cells, stats.species),
                        );
                        render_stage(
                            ui,
                            "🪐",
                            "Planetary",
                            planetary_progress,
                            format!("planets {} | terrain {}", stats.planets, stats.terrain_cells),
                        );
                        render_stage(
                            ui,
                            "🧠",
                            "Integration",
                            integration_progress,
                            format!(
                                "coh {:.3} | stab {:.3}",
                                stats.integration_coherence, stats.integration_stability
                            ),
                        );

                        ui.separator();
                        ui.small(
                            "Flow: Quantum → Atomic → Molecular → Cellular → Planetary → Integration",
                        );
                    } else {
                        ui.label("Holographic statistics unavailable.");
                        ui.small("Enable holographic simulation to view live emergence glyphs.");
                    }
                });
        }

        if show_lens_hud {
            let mut lens_anchor_y = -12.0;
            if show_overlay_policy_hud {
                lens_anchor_y -= 178.0;
            }
            if show_semantic_lod_hud {
                lens_anchor_y -= 166.0;
            }

            egui::Window::new("Neighborhood Lens")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_BOTTOM, [12.0, lens_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(250.0);
                    ui.label(format!(
                        "Lens: {}",
                        if self.selection_neighborhood_lens {
                            "ON"
                        } else {
                            "OFF"
                        }
                    ));
                    ui.label(format!("Active this frame: {}", lens_active));
                    ui.label(format!("Focus present: {}", lens_focus_present));
                    ui.label(format!("Radius: {:.2}", lens_radius_used));
                    ui.label(format!("Non-focus scale: {:.2}", lens_non_focus_scale_used));
                    ui.label(format!("Edge boost: {:.2}", lens_edge_boost_used));
                    ui.label(format!("Mode: {}", visual_mode_label));
                    ui.separator();
                    ui.small(format!(
                        "{} toggles lens, {} toggles this HUD",
                        LENS_TOGGLE_KEYS_LABEL, LENS_HUD_KEYS_LABEL
                    ));
                });
        }

        if show_lens_profile_hud {
            let mut lens_profile_anchor_y = -12.0;
            if show_overlay_policy_hud {
                lens_profile_anchor_y -= 178.0;
            }
            if show_semantic_lod_hud {
                lens_profile_anchor_y -= 166.0;
            }
            if show_lens_hud {
                lens_profile_anchor_y -= 170.0;
            }

            egui::Window::new("Lens Profile")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_BOTTOM, [12.0, lens_profile_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(260.0);
                    ui.label(format!(
                        "Adaptive profile: {}",
                        if lens_profile_adaptive { "ON" } else { "OFF" }
                    ));
                    ui.label(format!(
                        "Dominant stage: {}",
                        Self::emphasis_stage_label(lens_profile_stage)
                    ));
                    ui.label(format!(
                        "Effective r/n/b: {:.2} / {:.2} / {:.2}",
                        lens_radius_effective, lens_non_focus_effective, lens_edge_boost_effective
                    ));
                    ui.label(format!(
                        "Base r/n/b: {:.2} / {:.2} / {:.2}",
                        self.lens_radius, self.lens_non_focus_scale, self.lens_edge_boost
                    ));
                    ui.separator();
                    ui.small(format!(
                        "{} toggles profile, {} toggles this HUD",
                        LENS_PROFILE_TOGGLE_KEYS_LABEL, LENS_PROFILE_HUD_KEYS_LABEL
                    ));
                });
        }

        if show_semantic_lod_hud {
            let semantic_lod_anchor_y = if show_overlay_policy_hud {
                -178.0
            } else {
                -12.0
            };
            let semantic_lod_label = match semantic_lod_level {
                0 => "Detail",
                1 => "Balanced",
                2 => "Overview",
                _ => "Unknown",
            };
            egui::Window::new("Semantic LOD")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_BOTTOM, [12.0, semantic_lod_anchor_y])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(240.0);
                    ui.label(format!(
                        "Enabled: {}",
                        if semantic_lod_enabled { "ON" } else { "OFF" }
                    ));
                    ui.label(format!(
                        "Level: {} ({})",
                        semantic_lod_label, semantic_lod_level
                    ));
                    ui.label(format!("Zoom norm: {:.2}", semantic_lod_zoom_norm));
                    ui.label(format!("Focus weight: {:.2}", semantic_lod_focus_weight));
                    ui.separator();
                    ui.small("F13/E toggles LOD, F14/W toggles LOD HUD");
                });
        }

        if show_overlay_policy_hud {
            egui::Window::new("Overlay Policy")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_BOTTOM, [12.0, -12.0])
                .show(&egui_ctx, |ui| {
                    ui.set_min_width(320.0);
                    ui.label(format!(
                        "Auto orchestration: {}",
                        if auto_overlay_orchestration {
                            "ON"
                        } else {
                            "OFF"
                        }
                    ));
                    ui.small(format!(
                        "Context: selection={} story={} compare={} adaptive={}",
                        has_selection, story_active, compare_active, adaptive_active
                    ));
                    ui.separator();
                    ui.small(format!(
                        "Scene Guide: user={} -> effective={}",
                        show_scene_guide, effective_scene_guide
                    ));
                    ui.small(format!(
                        "Compare: user={} -> effective={}",
                        show_compare_overlay, effective_compare
                    ));
                    ui.small(format!(
                        "Glyphs: user={} -> effective={}",
                        show_emergence_glyphs, effective_glyphs
                    ));
                    ui.small(format!(
                        "Story: user={} -> effective={}",
                        show_story_overlay, effective_story
                    ));
                    ui.small(format!(
                        "Emphasis HUD: user={} -> effective={}",
                        show_emphasis_overlay, effective_emphasis
                    ));
                    ui.small(format!(
                        "Semantics: user={} -> effective={}",
                        show_selection_semantics, effective_semantics
                    ));
                    ui.separator();
                    ui.small("O toggles orchestration, I toggles policy HUD");
                });
        }

        // ========== PHASE 4: End EGUI frame and render ===========

        // Get WGPU context (after all rendering is done)
        let ctx = self.wgpu_context.as_ref().unwrap();

        // Get egui reference again
        let egui = self.egui_integration.as_mut().unwrap();
        egui.end_frame_and_draw(self.window.as_ref(), &ctx.device, &ctx.queue, encoder, view);
    }

    /// Handle entity click for selection
    fn handle_entity_click(&mut self, screen_pos: crate::gui::ScreenPosition) {
        // Build entity data for raycasting using the active render source.
        let entities: Vec<(
            crate::entity_layer7::layer7::EntityId,
            crate::gui::Coordinate3D,
            f64,
        )> = match self.visual_ux_mode {
            VisualUxMode::Legacy => self
                .simulation
                .entities()
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    // Use index-based spiral positioning (matching EntityInstance::from_entity)
                    let angle = i as f32 * 0.618_034; // Golden angle
                    let radius = (i as f32).sqrt() * 0.5;
                    let x = radius * angle.cos();
                    let y = radius * angle.sin();

                    let pos = crate::gui::Coordinate3D::new(x as f64, y as f64, 0.0);
                    (e.entity_id.clone(), pos, 1.0) // Default scale
                })
                .collect(),
            VisualUxMode::SceneFirst | VisualUxMode::FieldEnhanced => {
                let legacy_entities = self.simulation.entities();
                self.simulation
                    .holo_entities()
                    .iter()
                    .enumerate()
                    .map(|(i, holo)| {
                        let entity_id = legacy_entities
                            .get(i)
                            .map(|e| e.entity_id.clone())
                            .unwrap_or_else(|| {
                                crate::entity_layer7::layer7::EntityId::new(format!(
                                    "holo-{}",
                                    holo.entity_id
                                ))
                            });

                        let pos = crate::gui::Coordinate3D::new(
                            holo.position[0],
                            holo.position[1],
                            holo.position[2],
                        );
                        let scale = 0.8 + holo.consciousness * 0.6;
                        (entity_id, pos, scale)
                    })
                    .collect()
            }
        };

        // Update raycaster with current camera
        self.raycaster.set_camera(&self.camera);

        // Perform raycast
        let result = self.raycaster.cast_ray(screen_pos, &entities);

        match result {
            crate::gui::interaction::RaycastResult::Entity { entity_id, .. } => {
                println!("Selected entity: {}", entity_id);
                self.entity_inspector.select_entity(entity_id);
            }
            crate::gui::interaction::RaycastResult::EmptySpace { .. } => {
                self.entity_inspector.deselect();
            }
            _ => {}
        }
    }

    fn emphasis_stage_label(stage: u8) -> &'static str {
        match stage {
            0 => "Quantum",
            1 => "Atomic",
            2 => "Molecular",
            3 => "Cellular",
            4 => "Planetary",
            5 => "Integration",
            _ => "Unknown",
        }
    }
}

/// Render statistics
#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    /// Frames per second
    pub fps: f32,

    /// Frame time in milliseconds
    pub frame_time_ms: f64,

    /// Total frame count
    pub frame_count: u64,

    /// Number of entities being rendered
    pub entity_count: u64,

    /// Number of entities available from legacy source
    pub legacy_entity_count: u64,

    /// Number of entities available from holographic source
    pub holo_entity_count: u64,

    /// Number of active rendered connections
    pub active_connection_count: u64,

    /// Number of active structural (non-phase) rendered connections
    pub structural_connection_count: u64,

    /// Number of active phase rendered connections
    pub phase_connection_count: u64,

    /// Number of phase edges used in connection rendering for this frame
    pub phase_edge_count: u64,

    /// Whether active entity renderer path used holographic entities
    pub using_holo_entities: bool,

    /// Whether active connection renderer path used holographic connections
    pub using_holo_connections: bool,

    /// CPU update and preparation stage time in milliseconds
    pub cpu_update_ms: f64,

    /// Scene render pass stage time in milliseconds
    pub scene_render_ms: f64,

    /// UI render stage time in milliseconds
    pub ui_render_ms: f64,

    /// Queue submit and present stage time in milliseconds
    pub submit_present_ms: f64,

    /// Dominant adaptive emphasis stage (0..5)
    pub emphasis_stage: u8,

    /// Phase network threshold used by this frame
    pub phase_threshold_used: f32,

    /// Adaptive field emphasis factor used for UI and tuning
    pub field_emphasis: f32,

    /// Adaptive connection emphasis factor used for UI and tuning
    pub connection_emphasis: f32,

    /// Stage-aware structural connection scale applied this frame
    pub connection_structural_scale: f32,

    /// Stage-aware phase connection scale applied this frame
    pub connection_phase_scale: f32,

    /// Additive intensity bias applied to all profiled links this frame
    pub connection_intensity_bias: f32,

    /// Connection color grading warm/cool bias [0, 1]
    pub conn_color_warm_cool_bias: f32,

    /// Connection color grading saturation [0.5, 1.8]
    pub conn_color_saturation: f32,

    /// Phase link tint boost applied to connection type 3 [0, 0.6]
    pub conn_color_phase_boost: f32,

    /// Blend strength between base and graded connection color [0, 1]
    pub conn_color_grade_strength: f32,

    /// Camera zoom used when rendering this frame
    pub camera_zoom: f32,

    /// Adaptive declutter factor in [0, 1] where higher means stronger declutter
    pub declutter_factor: f32,

    /// Effective phase threshold after declutter adjustment
    pub phase_threshold_effective: f32,

    /// Semantic LOD level: 0 detail, 1 balanced, 2 overview
    pub semantic_lod_level: u8,

    /// Camera zoom normalization used for semantic LOD
    pub semantic_lod_zoom_norm: f32,

    /// Focus signal used for semantic LOD (selection-aware)
    pub semantic_lod_focus_weight: f32,

    /// Stage used to derive field quality profile this frame
    pub field_quality_stage: u8,

    /// Raymarching steps applied for field rendering this frame
    pub field_ray_steps: u32,

    /// Density scale applied for field rendering this frame
    pub field_density_scale: f32,

    /// Absorption applied for field rendering this frame
    pub field_absorption: f32,

    /// Benchmark mode code: 0 off, 1 clarity, 2 performance
    pub benchmark_mode: u8,

    /// Effective field toggle after benchmark overrides
    pub benchmark_effective_field_on: bool,

    /// Effective morphology toggle after benchmark overrides
    pub benchmark_effective_morphology_on: bool,

    /// Effective focus halo toggle after benchmark overrides
    pub benchmark_effective_focus_halo_on: bool,

    /// Whether selection-neighborhood lens was active this frame
    pub lens_active: bool,

    /// Whether a valid selection focus existed this frame
    pub lens_focus_present: bool,

    /// Lens radius parameter used this frame
    pub lens_radius_used: f32,

    /// Lens non-focus scale parameter used this frame
    pub lens_non_focus_scale_used: f32,

    /// Lens edge boost parameter used this frame
    pub lens_edge_boost_used: f32,

    /// Dominant stage used to derive lens profile this frame
    pub lens_profile_stage: u8,

    /// Whether adaptive lens profile was enabled in non-legacy mode this frame
    pub lens_profile_adaptive: bool,

    /// Effective lens radius after stage + zoom tuning
    pub lens_radius_effective: f32,

    /// Effective lens non-focus scale after stage + zoom tuning
    pub lens_non_focus_effective: f32,

    /// Effective lens edge boost after stage + zoom tuning
    pub lens_edge_boost_effective: f32,

    /// Whether guided auto-focus was active this frame
    pub auto_focus_active: bool,

    /// Stage-aware target zoom used by auto-focus this frame
    pub auto_focus_target_zoom: f32,

    /// Planar distance from camera to selected focus target
    pub auto_focus_distance: f32,

    /// Stage used to derive auto-focus target zoom this frame
    pub auto_focus_stage: u8,
}

/// GUI Application Builder
pub struct GuiApplicationBuilder {
    config: GuiConfig,
}

impl GuiApplicationBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            config: GuiConfig::default(),
        }
    }

    /// Set window size
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.config.window_width = width;
        self.config.window_height = height;
        self
    }

    /// Set initial zoom
    pub fn with_initial_zoom(mut self, zoom: f64) -> Self {
        self.config.initial_zoom = zoom;
        self
    }

    /// Set initial time rate
    pub fn with_initial_time_rate(mut self, rate: f64) -> Self {
        self.config.initial_time_rate = rate;
        self
    }

    /// Enable focus-based time dilation
    pub fn with_focus_dilation(mut self, enable: bool) -> Self {
        self.config.enable_focus_dilation = enable;
        self
    }

    /// Build the application
    pub async fn build(self, event_loop: &EventLoop<()>) -> Result<GuiApplication, String> {
        GuiApplication::new(event_loop, self.config).await
    }
}

impl Default for GuiApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// GUI Configuration
#[derive(Debug, Clone)]
pub struct GuiConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub initial_time_rate: f64,
    pub min_time_rate: f64,
    pub max_time_rate: f64,
    pub initial_zoom: f64,
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub enable_focus_dilation: bool,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            window_width: 1920,
            window_height: 1080,
            initial_time_rate: 1.0,
            min_time_rate: 0.1,
            max_time_rate: 1000.0,
            initial_zoom: 1.0,  // Fixed: was 1e-6 making entities invisible
            min_zoom: 1.616255e-35,
            max_zoom: 8.8e26,
            enable_focus_dilation: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_config_default() {
        let config = GuiConfig::default();
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
        assert_eq!(config.initial_time_rate, 1.0);
    }

    #[test]
    fn test_gui_builder() {
        let builder = GuiApplicationBuilder::new()
            .with_window_size(1280, 720)
            .with_initial_zoom(1.0)
            .with_initial_time_rate(2.0)
            .with_focus_dilation(false);

        assert_eq!(builder.config.window_width, 1280);
        assert_eq!(builder.config.window_height, 720);
        assert_eq!(builder.config.initial_zoom, 1.0);
        assert_eq!(builder.config.initial_time_rate, 2.0);
        assert!(!builder.config.enable_focus_dilation);
    }

    #[test]
    fn test_render_stats_default() {
        let stats = RenderStats::default();
        assert_eq!(stats.fps, 0.0);
        assert_eq!(stats.frame_count, 0);
    }
}
