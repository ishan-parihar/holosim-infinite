//! Loading Screen System
//!
//! Provides loading screens with progress indicators, status messages,
//! and smooth transitions for application startup and asset loading.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Loading screens"


/// Loading stage with progress tracking
#[derive(Debug, Clone)]
pub struct LoadingStage {
    /// Stage identifier
    pub id: String,
    /// Stage display name
    pub name: String,
    /// Stage description
    pub description: String,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Whether stage is complete
    pub complete: bool,
    /// Whether stage has started
    pub started: bool,
    /// Estimated duration in seconds
    pub estimated_duration: f32,
    /// Actual start time
    pub start_time: Option<std::time::Instant>,
    /// Actual end time
    pub end_time: Option<std::time::Instant>,
}

impl LoadingStage {
    /// Create a new loading stage
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            progress: 0.0,
            complete: false,
            started: false,
            estimated_duration: 1.0,
            start_time: None,
            end_time: None,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set estimated duration
    pub fn with_duration(mut self, seconds: f32) -> Self {
        self.estimated_duration = seconds;
        self
    }

    /// Start the stage
    pub fn start(&mut self) {
        self.started = true;
        self.start_time = Some(std::time::Instant::now());
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// Complete the stage
    pub fn complete(&mut self) {
        self.progress = 1.0;
        self.complete = true;
        self.end_time = Some(std::time::Instant::now());
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Option<std::time::Duration> {
        self.start_time.map(|start| {
            if let Some(end) = self.end_time {
                end - start
            } else {
                std::time::Instant::now() - start
            }
        })
    }

    /// Get formatted elapsed time
    pub fn elapsed_formatted(&self) -> String {
        match self.elapsed() {
            Some(duration) => {
                let secs = duration.as_secs();
                format!("{}.{:02}s", secs, duration.subsec_millis() / 10)
            }
            None => "--".to_string(),
        }
    }
}

/// Loading screen configuration
#[derive(Debug, Clone)]
pub struct LoadingScreenConfig {
    /// Whether to show loading screen on startup
    pub show_on_startup: bool,
    /// Whether to show progress bar
    pub show_progress_bar: bool,
    /// Whether to show individual stage progress
    pub show_stage_details: bool,
    /// Whether to show elapsed time
    pub show_elapsed_time: bool,
    /// Whether to allow canceling
    pub allow_cancel: bool,
    /// Custom background color
    pub background_color: [f32; 4],
    /// Custom accent color
    pub accent_color: [f32; 4],
    /// Progress bar height
    pub progress_bar_height: f32,
    /// Text size
    pub text_size: f32,
    /// Whether to fade in/out
    pub use_fade: bool,
    /// Fade duration in seconds
    pub fade_duration: f32,
}

impl Default for LoadingScreenConfig {
    fn default() -> Self {
        Self {
            show_on_startup: true,
            show_progress_bar: true,
            show_stage_details: true,
            show_elapsed_time: true,
            allow_cancel: false,
            background_color: [0.08, 0.08, 0.10, 1.0],
            accent_color: [0.40, 0.50, 1.0, 1.0],
            progress_bar_height: 8.0,
            text_size: 16.0,
            use_fade: true,
            fade_duration: 0.5,
        }
    }
}

/// Loading screen state manager
pub struct LoadingScreen {
    /// Configuration
    pub config: LoadingScreenConfig,
    /// Loading stages
    stages: Vec<LoadingStage>,
    /// Current overall progress (0.0 to 1.0)
    overall_progress: f32,
    /// Whether loading is complete
    complete: bool,
    /// Whether loading has started
    started: bool,
    /// Whether visible
    visible: bool,
    /// Current fade opacity (0.0 to 1.0)
    fade_opacity: f32,
    /// Start time
    start_time: Option<std::time::Instant>,
    /// End time
    end_time: Option<std::time::Instant>,
    /// Status message
    status_message: String,
    /// Cancel flag
    cancel_requested: bool,
    /// Completion callbacks
    completion_callbacks: Vec<Box<dyn FnOnce() + Send>>,
    /// Progress callbacks
    progress_callbacks: Vec<Box<dyn Fn(f32) + Send>>,
}

impl Default for LoadingScreen {
    fn default() -> Self {
        Self {
            config: LoadingScreenConfig::default(),
            stages: Self::default_stages(),
            overall_progress: 0.0,
            complete: false,
            started: false,
            visible: false,
            fade_opacity: 0.0,
            start_time: None,
            end_time: None,
            status_message: "Initializing...".to_string(),
            cancel_requested: false,
            completion_callbacks: Vec::new(),
            progress_callbacks: Vec::new(),
        }
    }
}

impl LoadingScreen {
    /// Create a new loading screen
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom configuration
    pub fn with_config(config: LoadingScreenConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Get default loading stages
    fn default_stages() -> Vec<LoadingStage> {
        vec![
            LoadingStage::new("init", "Initializing")
                .with_description("Setting up application core...")
                .with_duration(0.5),
            LoadingStage::new("gpu", "GPU Context")
                .with_description("Initializing graphics subsystem...")
                .with_duration(1.0),
            LoadingStage::new("shaders", "Shaders")
                .with_description("Compiling shader programs...")
                .with_duration(1.5),
            LoadingStage::new("assets", "Assets")
                .with_description("Loading textures and resources...")
                .with_duration(2.0),
            LoadingStage::new("simulation", "Simulation")
                .with_description("Initializing simulation systems...")
                .with_duration(1.5),
            LoadingStage::new("ui", "UI")
                .with_description("Setting up user interface...")
                .with_duration(1.0),
            LoadingStage::new("ready", "Ready")
                .with_description("Preparing to start...")
                .with_duration(0.5),
        ]
    }

    /// Start loading
    pub fn start(&mut self) {
        self.started = true;
        self.visible = true;
        self.start_time = Some(std::time::Instant::now());

        // Start first stage
        if let Some(stage) = self.stages.first_mut() {
            stage.start();
        }

        // Begin fade in
        if self.config.use_fade {
            self.fade_opacity = 0.0;
        } else {
            self.fade_opacity = 1.0;
        }
    }

    /// Update loading progress
    pub fn update(&mut self, delta_time: f32) {
        if !self.started || self.complete {
            return;
        }

        // Update fade
        if self.config.use_fade && !self.complete {
            let fade_speed = 1.0 / self.config.fade_duration;
            if self.fade_opacity < 1.0 {
                self.fade_opacity = (self.fade_opacity + fade_speed * delta_time).min(1.0);
            }
        }

        // Update current stage
        if let Some(current_stage) = self.current_stage_mut() {
            if !current_stage.complete {
                // Estimate progress based on time
                if let Some(elapsed) = current_stage.elapsed() {
                    let progress =
                        (elapsed.as_secs_f32() / current_stage.estimated_duration).min(1.0);
                    current_stage.update_progress(progress);
                }
            }
        }

        // Calculate overall progress
        self.update_overall_progress();

        // Check for completion
        if self.overall_progress >= 1.0 && !self.complete {
            self.complete_loading();
        }

        // Notify callbacks
        for callback in &self.progress_callbacks {
            callback(self.overall_progress);
        }
    }

    /// Complete current stage and move to next
    pub fn complete_stage(&mut self, stage_id: &str) {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.id == stage_id) {
            stage.complete();

            // Find and start next stage
            if let Some(current_index) = self.stages.iter().position(|s| s.id == stage_id) {
                if current_index + 1 < self.stages.len() {
                    self.stages[current_index + 1].start();
                }
            }
        }
    }

    /// Set stage progress directly
    pub fn set_stage_progress(&mut self, stage_id: &str, progress: f32) {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.id == stage_id) {
            stage.update_progress(progress);
        }
    }

    /// Set status message
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = message.into();
    }

    /// Get current stage
    pub fn current_stage(&self) -> Option<&LoadingStage> {
        self.stages.iter().find(|s| s.started && !s.complete)
    }

    fn current_stage_mut(&mut self) -> Option<&mut LoadingStage> {
        self.stages.iter_mut().find(|s| s.started && !s.complete)
    }

    fn update_overall_progress(&mut self) {
        if self.stages.is_empty() {
            self.overall_progress = 1.0;
            return;
        }

        let total_progress: f32 = self.stages.iter().map(|s| s.progress).sum();
        self.overall_progress = total_progress / self.stages.len() as f32;
    }

    fn complete_loading(&mut self) {
        self.complete = true;
        self.end_time = Some(std::time::Instant::now());

        // Begin fade out
        if self.config.use_fade {
            // Fade out happens in update
        } else {
            self.visible = false;
        }

        // Call completion callbacks
        for callback in self.completion_callbacks.drain(..) {
            callback();
        }
    }

    /// Check if loading is complete
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get overall progress
    pub fn get_progress(&self) -> f32 {
        self.overall_progress
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Option<std::time::Duration> {
        self.start_time.map(|start| {
            if let Some(end) = self.end_time {
                end - start
            } else {
                std::time::Instant::now() - start
            }
        })
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible && self.fade_opacity > 0.0
    }

    /// Get fade opacity
    pub fn get_opacity(&self) -> f32 {
        self.fade_opacity
    }

    /// Request cancel
    pub fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    /// Check if cancel requested
    pub fn is_cancel_requested(&self) -> bool {
        self.cancel_requested
    }

    /// Register completion callback
    pub fn on_complete<F>(&mut self, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.completion_callbacks.push(Box::new(callback));
    }

    /// Register progress callback
    pub fn on_progress<F>(&mut self, callback: F)
    where
        F: Fn(f32) + Send + 'static,
    {
        self.progress_callbacks.push(Box::new(callback));
    }

    /// Reset the loading screen
    pub fn reset(&mut self) {
        self.started = false;
        self.complete = false;
        self.visible = false;
        self.overall_progress = 0.0;
        self.fade_opacity = 0.0;
        self.start_time = None;
        self.end_time = None;
        self.cancel_requested = false;

        // Reset stages
        for stage in &mut self.stages {
            stage.progress = 0.0;
            stage.complete = false;
            stage.started = false;
            stage.start_time = None;
            stage.end_time = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_stage_creation() {
        let stage = LoadingStage::new("test", "Test Stage")
            .with_description("A test stage")
            .with_duration(2.5);

        assert_eq!(stage.id, "test");
        assert_eq!(stage.name, "Test Stage");
        assert_eq!(stage.description, "A test stage");
        assert_eq!(stage.estimated_duration, 2.5);
        assert!(!stage.started);
        assert!(!stage.complete);
    }

    #[test]
    fn test_loading_stage_lifecycle() {
        let mut stage = LoadingStage::new("test", "Test");

        // Start
        stage.start();
        assert!(stage.started);
        assert!(stage.start_time.is_some());

        // Update progress
        stage.update_progress(0.5);
        assert_eq!(stage.progress, 0.5);

        // Complete
        stage.complete();
        assert!(stage.complete);
        assert_eq!(stage.progress, 1.0);
        assert!(stage.end_time.is_some());
        assert!(stage.elapsed().is_some());
    }

    #[test]
    fn test_loading_screen_default_stages() {
        let loading = LoadingScreen::new();
        assert!(!loading.stages.is_empty());
        assert!(loading.stages.iter().any(|s| s.id == "init"));
        assert!(loading.stages.iter().any(|s| s.id == "gpu"));
        assert!(loading.stages.iter().any(|s| s.id == "ready"));
    }

    #[test]
    fn test_loading_screen_lifecycle() {
        let mut loading = LoadingScreen::new();

        // Initial state
        assert!(!loading.started);
        assert!(!loading.is_visible());
        assert_eq!(loading.get_progress(), 0.0);

        // Start
        loading.start();
        assert!(loading.started);
        // Note: is_visible() requires fade_opacity > 0, which is 0 initially when fading
        // Check visibility state directly instead
        assert!(loading.visible);
        assert!(loading.start_time.is_some());

        // Update (simulate time passing)
        loading.update(0.016); // ~60fps frame

        // Complete a stage
        loading.complete_stage("init");

        // Check that next stage started
        if let Some(gpu_stage) = loading.stages.iter().find(|s| s.id == "gpu") {
            assert!(gpu_stage.started);
        }
    }

    #[test]
    fn test_loading_screen_progress() {
        let mut loading = LoadingScreen::new();
        loading.start();

        // Initially at 0%
        assert_eq!(loading.get_progress(), 0.0);

        // Complete half the stages
        let stage_count = loading.stages.len();
        for i in 0..stage_count / 2 {
            if let Some(stage) = loading.stages.get_mut(i) {
                stage.complete();
            }
        }

        loading.update_overall_progress();
        assert!(loading.get_progress() > 0.0);
        assert!(loading.get_progress() < 1.0);
    }

    #[test]
    fn test_loading_screen_complete() {
        let mut loading = LoadingScreen::new();
        loading.start();

        // Complete all stages
        for stage in &mut loading.stages {
            stage.complete();
        }

        loading.update_overall_progress();

        // Trigger completion
        loading.update(0.1);

        assert!(loading.is_complete());
        assert!(loading.end_time.is_some());
    }

    #[test]
    fn test_loading_screen_reset() {
        let mut loading = LoadingScreen::new();
        loading.start();
        loading.complete_stage("init");

        loading.reset();

        assert!(!loading.started);
        assert!(!loading.complete);
        assert!(!loading.is_visible());
        assert_eq!(loading.get_progress(), 0.0);

        // All stages should be reset
        for stage in &loading.stages {
            assert!(!stage.started);
            assert!(!stage.complete);
            assert_eq!(stage.progress, 0.0);
        }
    }

    #[test]
    fn test_loading_screen_callbacks() {
        let mut loading = LoadingScreen::new();

        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let progress_called: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let complete_called: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let progress_called_clone = progress_called.clone();
        let complete_called_clone = complete_called.clone();

        loading.on_progress(move |_progress| {
            progress_called_clone.store(true, Ordering::SeqCst);
        });

        loading.on_complete(move || {
            complete_called_clone.store(true, Ordering::SeqCst);
        });

        loading.start();

        // Trigger progress callback
        loading.update(0.016);

        // Complete all stages
        for stage in &mut loading.stages {
            stage.complete();
        }
        loading.update_overall_progress();
        loading.update(0.1);

        assert!(progress_called.load(Ordering::SeqCst));
        assert!(complete_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_loading_screen_cancel() {
        let mut loading = LoadingScreen::new();
        loading.start();

        assert!(!loading.is_cancel_requested());

        loading.request_cancel();

        assert!(loading.is_cancel_requested());
    }

    #[test]
    fn test_loading_screen_elapsed() {
        let mut loading = LoadingScreen::new();
        loading.start();

        // Initially should have some elapsed time
        assert!(loading.elapsed().is_some());

        // Complete and check elapsed is still available
        for stage in &mut loading.stages {
            stage.complete();
        }
        loading.update_overall_progress();
        loading.update(0.1);

        assert!(loading.elapsed().is_some());
    }

    #[test]
    fn test_stage_elapsed_formatted() {
        let mut stage = LoadingStage::new("test", "Test");
        stage.start();

        // Should return formatted time
        let formatted = stage.elapsed_formatted();
        assert!(formatted.contains('s'));

        stage.complete();
        let formatted_after = stage.elapsed_formatted();
        assert!(formatted_after.contains('s'));
    }
}
