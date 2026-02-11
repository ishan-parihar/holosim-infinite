//! Camera Bookmarks - Save and Load Camera Positions
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 5 Week 10:
//! "Camera bookmarks - Save camera positions, Quick navigation to views, Named bookmarks"
//!
//! This module provides:
//! - Named camera bookmarks
//! - Quick navigation with smooth transitions
//! - Bookmark management (save, load, delete, rename)
//! - Persistence support

use crate::gui::Camera2D as Camera3D;
use nalgebra_glm::Vec3;
use std::collections::HashMap;
use std::time::Instant;

/// A camera bookmark with position, orientation, and metadata
#[derive(Debug, Clone, PartialEq)]
pub struct CameraBookmark {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Camera position
    pub position: Vec3,
    /// Zoom level
    pub zoom: f32,
    /// Scale level (for multi-scale context)
    pub scale_level: ScaleLevel,
    /// Creation timestamp
    pub created_at: Instant,
    /// Description/notes
    pub description: Option<String>,
    /// Category/tag
    pub category: Option<String>,
    /// Thumbnail data (optional)
    pub thumbnail: Option<Vec<u8>>,
    /// Custom color for visualization
    pub color: Option<[f32; 4]>,
}

impl CameraBookmark {
    /// Create a new bookmark from current camera state
    pub fn from_camera(id: String, name: String, camera: &Camera3D) -> Self {
        CameraBookmark {
            id,
            name,
            position: camera.position,
            zoom: camera.zoom,
            scale_level: ScaleLevel::from_zoom(camera.zoom),
            created_at: Instant::now(),
            description: None,
            category: None,
            thumbnail: None,
            color: None,
        }
    }

    /// Create a bookmark with custom position
    pub fn new(id: String, name: String, position: Vec3, zoom: f32) -> Self {
        CameraBookmark {
            id,
            name,
            position,
            zoom,
            scale_level: ScaleLevel::from_zoom(zoom),
            created_at: Instant::now(),
            description: None,
            category: None,
            thumbnail: None,
            color: None,
        }
    }

    /// Apply this bookmark to a camera
    pub fn apply_to_camera(&self, camera: &mut Camera3D) {
        camera.position = self.position;
        camera.zoom = self.zoom;
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set color
    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = Some(color);
        self
    }

    /// Get age of bookmark
    pub fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }

    /// Get a summary string
    pub fn summary(&self) -> String {
        format!("{}: {} - Scale: {:?}", self.id, self.name, self.scale_level)
    }
}

impl Default for CameraBookmark {
    fn default() -> Self {
        CameraBookmark {
            id: "default".to_string(),
            name: "Default View".to_string(),
            position: Vec3::new(0.0, 0.0, 100.0),
            zoom: 1.0,
            scale_level: ScaleLevel::Molecular,
            created_at: Instant::now(),
            description: None,
            category: None,
            thumbnail: None,
            color: Some([1.0, 1.0, 1.0, 1.0]),
        }
    }
}

/// Scale level for bookmarks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaleLevel {
    Quantum,   // 10^-35 to 10^-15 m
    Atomic,    // 10^-15 to 10^-10 m
    Molecular, // 10^-10 to 10^-6 m
    Cellular,  // 10^-6 to 10^-3 m
    Organism,  // 10^-3 to 10^0 m
    Planetary, // 10^0 to 10^9 m
    Stellar,   // 10^9 to 10^15 m
    Galactic,  // 10^15 to 10^26 m
}

impl ScaleLevel {
    /// Get scale level from zoom factor
    pub fn from_zoom(zoom: f32) -> Self {
        // These are approximate ranges based on log scale
        match zoom {
            z if z < 0.001 => ScaleLevel::Quantum,
            z if z < 0.01 => ScaleLevel::Atomic,
            z if z < 0.1 => ScaleLevel::Molecular,
            z if z < 1.0 => ScaleLevel::Cellular,
            z if z < 10.0 => ScaleLevel::Organism,
            z if z < 100.0 => ScaleLevel::Planetary,
            z if z < 1000.0 => ScaleLevel::Stellar,
            _ => ScaleLevel::Galactic,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Quantum Scale",
            ScaleLevel::Atomic => "Atomic Scale",
            ScaleLevel::Molecular => "Molecular Scale",
            ScaleLevel::Cellular => "Cellular Scale",
            ScaleLevel::Organism => "Organism Scale",
            ScaleLevel::Planetary => "Planetary Scale",
            ScaleLevel::Stellar => "Stellar Scale",
            ScaleLevel::Galactic => "Galactic Scale",
        }
    }

    /// Get approximate size range in meters
    pub fn size_range(&self) -> (f64, f64) {
        match self {
            ScaleLevel::Quantum => (1e-35, 1e-15),
            ScaleLevel::Atomic => (1e-15, 1e-10),
            ScaleLevel::Molecular => (1e-10, 1e-6),
            ScaleLevel::Cellular => (1e-6, 1e-3),
            ScaleLevel::Organism => (1e-3, 1e0),
            ScaleLevel::Planetary => (1e0, 1e9),
            ScaleLevel::Stellar => (1e9, 1e15),
            ScaleLevel::Galactic => (1e15, 1e26),
        }
    }

    /// Get typical zoom value
    pub fn typical_zoom(&self) -> f32 {
        match self {
            ScaleLevel::Quantum => 0.0001,
            ScaleLevel::Atomic => 0.001,
            ScaleLevel::Molecular => 0.01,
            ScaleLevel::Cellular => 0.1,
            ScaleLevel::Organism => 1.0,
            ScaleLevel::Planetary => 10.0,
            ScaleLevel::Stellar => 100.0,
            ScaleLevel::Galactic => 1000.0,
        }
    }

    /// Get color for this scale level
    pub fn color(&self) -> [f32; 4] {
        match self {
            ScaleLevel::Quantum => [1.0, 0.5, 0.5, 1.0],   // Red
            ScaleLevel::Atomic => [1.0, 0.7, 0.3, 1.0],    // Orange
            ScaleLevel::Molecular => [1.0, 0.9, 0.3, 1.0], // Yellow
            ScaleLevel::Cellular => [0.3, 1.0, 0.3, 1.0],  // Green
            ScaleLevel::Organism => [0.3, 1.0, 1.0, 1.0],  // Cyan
            ScaleLevel::Planetary => [0.3, 0.5, 1.0, 1.0], // Blue
            ScaleLevel::Stellar => [0.5, 0.3, 1.0, 1.0],   // Purple
            ScaleLevel::Galactic => [1.0, 1.0, 1.0, 1.0],  // White
        }
    }
}

/// Camera transition animation
#[derive(Debug, Clone)]
pub struct CameraTransition {
    /// Starting bookmark
    pub from: CameraBookmark,
    /// Target bookmark
    pub to: CameraBookmark,
    /// Transition progress (0.0 to 1.0)
    pub progress: f32,
    /// Total duration in seconds
    pub duration: f32,
    /// Elapsed time
    pub elapsed: f32,
    /// Easing function type
    pub easing: EasingFunction,
    /// Is transition active
    pub active: bool,
}

impl CameraTransition {
    /// Create a new transition
    pub fn new(from: CameraBookmark, to: CameraBookmark, duration: f32) -> Self {
        CameraTransition {
            from,
            to,
            progress: 0.0,
            duration,
            elapsed: 0.0,
            easing: EasingFunction::EaseInOutCubic,
            active: false,
        }
    }

    /// Start the transition
    pub fn start(&mut self) {
        self.active = true;
        self.progress = 0.0;
        self.elapsed = 0.0;
    }

    /// Update transition progress
    pub fn update(&mut self, delta_time: f32) -> bool {
        if !self.active {
            return false;
        }

        self.elapsed += delta_time;
        self.progress = (self.elapsed / self.duration).min(1.0);

        let eased_progress = self.easing.apply(self.progress);

        // Interpolate position and zoom
        let current_pos = Self::lerp_vec3(&self.from.position, &self.to.position, eased_progress);
        let current_zoom = Self::lerp_f32(self.from.zoom, self.to.zoom, eased_progress);

        // Check if complete
        if self.progress >= 1.0 {
            self.active = false;
            true // Transition complete
        } else {
            false // Still transitioning
        }
    }

    /// Get current interpolated bookmark
    pub fn current_state(&self) -> CameraBookmark {
        let eased_progress = self.easing.apply(self.progress);

        CameraBookmark {
            id: "transition".to_string(),
            name: "Transition".to_string(),
            position: Self::lerp_vec3(&self.from.position, &self.to.position, eased_progress),
            zoom: Self::lerp_f32(self.from.zoom, self.to.zoom, eased_progress),
            scale_level: ScaleLevel::from_zoom(Self::lerp_f32(
                self.from.zoom,
                self.to.zoom,
                eased_progress,
            )),
            created_at: Instant::now(),
            description: None,
            category: None,
            thumbnail: None,
            color: None,
        }
    }

    /// Linear interpolation for Vec3
    fn lerp_vec3(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
        Vec3::new(
            a.x + (b.x - a.x) * t,
            a.y + (b.y - a.y) * t,
            a.z + (b.z - a.z) * t,
        )
    }

    /// Linear interpolation for f32
    fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    /// Cancel the transition
    pub fn cancel(&mut self) {
        self.active = false;
        self.progress = 0.0;
        self.elapsed = 0.0;
    }

    /// Set easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
}

impl Default for CameraTransition {
    fn default() -> Self {
        CameraTransition::new(CameraBookmark::default(), CameraBookmark::default(), 1.0)
    }
}

/// Easing functions for smooth transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
}

impl EasingFunction {
    /// Apply easing to a value t in [0, 1]
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseInQuad => t * t,
            EasingFunction::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingFunction::EaseInCubic => t * t * t,
            EasingFunction::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            EasingFunction::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            EasingFunction::EaseInQuart => t * t * t * t,
            EasingFunction::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            EasingFunction::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            EasingFunction::Linear => "Linear",
            EasingFunction::EaseInQuad => "Ease In Quad",
            EasingFunction::EaseOutQuad => "Ease Out Quad",
            EasingFunction::EaseInOutQuad => "Ease In-Out Quad",
            EasingFunction::EaseInCubic => "Ease In Cubic",
            EasingFunction::EaseOutCubic => "Ease Out Cubic",
            EasingFunction::EaseInOutCubic => "Ease In-Out Cubic",
            EasingFunction::EaseInQuart => "Ease In Quart",
            EasingFunction::EaseOutQuart => "Ease Out Quart",
            EasingFunction::EaseInOutQuart => "Ease In-Out Quart",
        }
    }
}

/// Bookmark manager for saving, loading, and navigating bookmarks
pub struct BookmarkManager {
    /// All saved bookmarks
    bookmarks: HashMap<String, CameraBookmark>,
    /// Currently active transition
    current_transition: Option<CameraTransition>,
    /// Default transition duration in seconds
    default_transition_duration: f32,
    /// Maximum number of bookmarks
    max_bookmarks: usize,
    /// Recent bookmarks (for quick access)
    recent_bookmarks: Vec<String>,
    /// Maximum recent bookmarks
    max_recent: usize,
    /// Categories/tags
    categories: Vec<String>,
    /// Enable smooth transitions
    smooth_transitions: bool,
    /// Last accessed bookmark
    last_accessed: Option<String>,
}

impl BookmarkManager {
    /// Create a new bookmark manager
    pub fn new() -> Self {
        let mut manager = BookmarkManager {
            bookmarks: HashMap::new(),
            current_transition: None,
            default_transition_duration: 1.0,
            max_bookmarks: 50,
            recent_bookmarks: Vec::new(),
            max_recent: 10,
            categories: vec![
                "Default".to_string(),
                "Favorites".to_string(),
                "Scale Views".to_string(),
                "Entity Views".to_string(),
            ],
            smooth_transitions: true,
            last_accessed: None,
        };

        // Create default bookmarks
        manager.create_default_bookmarks();
        manager
    }

    /// Create default system bookmarks
    fn create_default_bookmarks(&mut self) {
        // Home bookmark (overview)
        let home = CameraBookmark::new(
            "home".to_string(),
            "Home View".to_string(),
            Vec3::new(0.0, 0.0, 100.0),
            1.0,
        )
        .with_description("Default overview position")
        .with_category("Default");

        // Scale-level bookmarks
        let quantum = CameraBookmark::new(
            "scale_quantum".to_string(),
            "Quantum View".to_string(),
            Vec3::new(0.0, 0.0, 0.001),
            0.0001,
        )
        .with_description("Quantum scale perspective")
        .with_category("Scale Views");

        let atomic = CameraBookmark::new(
            "scale_atomic".to_string(),
            "Atomic View".to_string(),
            Vec3::new(0.0, 0.0, 0.01),
            0.001,
        )
        .with_description("Atomic scale perspective")
        .with_category("Scale Views");

        let molecular = CameraBookmark::new(
            "scale_molecular".to_string(),
            "Molecular View".to_string(),
            Vec3::new(0.0, 0.0, 0.1),
            0.01,
        )
        .with_description("Molecular scale perspective")
        .with_category("Scale Views");

        let galactic = CameraBookmark::new(
            "scale_galactic".to_string(),
            "Galactic View".to_string(),
            Vec3::new(0.0, 0.0, 1000.0),
            1000.0,
        )
        .with_description("Galactic scale perspective")
        .with_category("Scale Views");

        // Save default bookmarks
        self.save_bookmark(home);
        self.save_bookmark(quantum);
        self.save_bookmark(atomic);
        self.save_bookmark(molecular);
        self.save_bookmark(galactic);
    }

    /// Save a bookmark
    pub fn save_bookmark(&mut self, bookmark: CameraBookmark) {
        // Check if we're at capacity
        if self.bookmarks.len() >= self.max_bookmarks && !self.bookmarks.contains_key(&bookmark.id)
        {
            // Remove oldest bookmark (not in recent)
            let to_remove = self
                .bookmarks
                .keys()
                .find(|id| !self.recent_bookmarks.contains(id))
                .cloned();
            if let Some(id) = to_remove {
                self.bookmarks.remove(&id);
            }
        }

        self.bookmarks.insert(bookmark.id.clone(), bookmark);
    }

    /// Create bookmark from current camera
    pub fn bookmark_current(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        camera: &Camera3D,
    ) -> CameraBookmark {
        let bookmark = CameraBookmark::from_camera(id.into(), name.into(), camera);
        self.save_bookmark(bookmark.clone());
        bookmark
    }

    /// Load a bookmark by ID
    pub fn load_bookmark(&self, id: &str) -> Option<&CameraBookmark> {
        self.bookmarks.get(id)
    }

    /// Navigate to a bookmark
    pub fn navigate_to(&mut self, id: &str, camera: &mut Camera3D) -> bool {
        if let Some(bookmark) = self.bookmarks.get(id).cloned() {
            if self.smooth_transitions {
                // Create transition from current state
                let from = CameraBookmark::from_camera(
                    "current".to_string(),
                    "Current".to_string(),
                    camera,
                );
                let mut transition =
                    CameraTransition::new(from, bookmark.clone(), self.default_transition_duration);
                transition.start();
                self.current_transition = Some(transition);
            } else {
                // Apply immediately
                bookmark.apply_to_camera(camera);
            }

            // Update recent list
            self.update_recent(id);
            self.last_accessed = Some(id.to_string());

            true
        } else {
            false
        }
    }

    /// Update recent bookmarks list
    fn update_recent(&mut self, id: &str) {
        // Remove if already exists
        self.recent_bookmarks.retain(|x| x != id);
        // Add to front
        self.recent_bookmarks.insert(0, id.to_string());
        // Trim if needed
        if self.recent_bookmarks.len() > self.max_recent {
            self.recent_bookmarks.pop();
        }
    }

    /// Delete a bookmark
    pub fn delete_bookmark(&mut self, id: &str) -> bool {
        self.recent_bookmarks.retain(|x| x != id);
        self.bookmarks.remove(id).is_some()
    }

    /// Rename a bookmark
    pub fn rename_bookmark(&mut self, id: &str, new_name: impl Into<String>) -> bool {
        if let Some(bookmark) = self.bookmarks.get_mut(id) {
            bookmark.name = new_name.into();
            true
        } else {
            false
        }
    }

    /// Get all bookmarks
    pub fn get_all_bookmarks(&self) -> Vec<&CameraBookmark> {
        self.bookmarks.values().collect()
    }

    /// Get bookmarks by category
    pub fn get_bookmarks_by_category(&self, category: &str) -> Vec<&CameraBookmark> {
        self.bookmarks
            .values()
            .filter(|b| b.category.as_ref() == Some(&category.to_string()))
            .collect()
    }

    /// Get recent bookmarks
    pub fn get_recent_bookmarks(&self) -> Vec<&CameraBookmark> {
        self.recent_bookmarks
            .iter()
            .filter_map(|id| self.bookmarks.get(id))
            .collect()
    }

    /// Get bookmark by ID
    pub fn get_bookmark(&self, id: &str) -> Option<&CameraBookmark> {
        self.bookmarks.get(id)
    }

    /// Check if bookmark exists
    pub fn has_bookmark(&self, id: &str) -> bool {
        self.bookmarks.contains_key(id)
    }

    /// Get all categories
    pub fn get_categories(&self) -> &[String] {
        &self.categories
    }

    /// Add a category
    pub fn add_category(&mut self, category: impl Into<String>) {
        let cat = category.into();
        if !self.categories.contains(&cat) {
            self.categories.push(cat);
        }
    }

    /// Update transition animation
    pub fn update_transition(&mut self, delta_time: f32, camera: &mut Camera3D) -> bool {
        if let Some(ref mut transition) = self.current_transition {
            if transition.update(delta_time) {
                // Transition complete
                transition.to.apply_to_camera(camera);
                self.current_transition = None;
                true
            } else {
                // Apply current interpolated state
                let current = transition.current_state();
                current.apply_to_camera(camera);
                false
            }
        } else {
            true // No transition, so "complete"
        }
    }

    /// Check if transitioning
    pub fn is_transitioning(&self) -> bool {
        self.current_transition
            .as_ref()
            .map(|t| t.active)
            .unwrap_or(false)
    }

    /// Cancel current transition
    pub fn cancel_transition(&mut self) {
        if let Some(ref mut transition) = self.current_transition {
            transition.cancel();
        }
        self.current_transition = None;
    }

    /// Set default transition duration
    pub fn set_transition_duration(&mut self, duration: f32) {
        self.default_transition_duration = duration.max(0.1).min(5.0);
    }

    /// Enable/disable smooth transitions
    pub fn set_smooth_transitions(&mut self, enabled: bool) {
        self.smooth_transitions = enabled;
    }

    /// Get number of bookmarks
    pub fn bookmark_count(&self) -> usize {
        self.bookmarks.len()
    }

    /// Get last accessed bookmark ID
    pub fn last_accessed(&self) -> Option<&str> {
        self.last_accessed.as_deref()
    }

    /// Clear all bookmarks
    pub fn clear_all(&mut self) {
        self.bookmarks.clear();
        self.recent_bookmarks.clear();
        self.current_transition = None;
        self.last_accessed = None;
    }

    /// Export bookmarks to string (for persistence)
    pub fn export_bookmarks(&self) -> String {
        // Simple text format: id|name|px|py|pz|zoom|category
        let mut lines = Vec::new();
        for bookmark in self.bookmarks.values() {
            let line = format!(
                "{}|{}|{:.6}|{:.6}|{:.6}|{:.6}|{:?}",
                bookmark.id,
                bookmark.name,
                bookmark.position.x,
                bookmark.position.y,
                bookmark.position.z,
                bookmark.zoom,
                bookmark.category.as_deref().unwrap_or("Default"),
            );
            lines.push(line);
        }
        lines.join("\n")
    }

    /// Import bookmarks from string
    pub fn import_bookmarks(&mut self, data: &str) -> usize {
        let mut count = 0;
        for line in data.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 7 {
                if let (Ok(px), Ok(py), Ok(pz), Ok(zoom)) = (
                    parts[2].parse::<f32>(),
                    parts[3].parse::<f32>(),
                    parts[4].parse::<f32>(),
                    parts[5].parse::<f32>(),
                ) {
                    let bookmark = CameraBookmark::new(
                        parts[0].to_string(),
                        parts[1].to_string(),
                        Vec3::new(px, py, pz),
                        zoom,
                    )
                    .with_category(*parts.get(6).unwrap_or(&"Default"));
                    self.save_bookmark(bookmark);
                    count += 1;
                }
            }
        }
        count
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_bookmark_new() {
        let bookmark = CameraBookmark::new(
            "test".to_string(),
            "Test View".to_string(),
            Vec3::new(1.0, 2.0, 3.0),
            2.0,
        );

        assert_eq!(bookmark.id, "test");
        assert_eq!(bookmark.name, "Test View");
        assert_eq!(bookmark.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(bookmark.zoom, 2.0);
    }

    #[test]
    fn test_camera_bookmark_from_camera() {
        let camera = Camera3D::default();
        let bookmark =
            CameraBookmark::from_camera("cam".to_string(), "Camera View".to_string(), &camera);

        assert_eq!(bookmark.id, "cam");
        assert_eq!(bookmark.position, camera.position);
    }

    #[test]
    fn test_scale_level_from_zoom() {
        assert_eq!(ScaleLevel::from_zoom(0.0001), ScaleLevel::Quantum);
        assert_eq!(ScaleLevel::from_zoom(0.001), ScaleLevel::Atomic);
        assert_eq!(ScaleLevel::from_zoom(0.01), ScaleLevel::Molecular);
        assert_eq!(ScaleLevel::from_zoom(1.0), ScaleLevel::Organism);
        assert_eq!(ScaleLevel::from_zoom(1000.0), ScaleLevel::Galactic);
    }

    #[test]
    fn test_scale_level_display_name() {
        assert_eq!(ScaleLevel::Quantum.display_name(), "Quantum Scale");
        assert_eq!(ScaleLevel::Galactic.display_name(), "Galactic Scale");
    }

    #[test]
    fn test_easing_function() {
        assert!((EasingFunction::Linear.apply(0.5) - 0.5).abs() < 0.001);
        assert!(EasingFunction::EaseInQuad.apply(0.5) < 0.5);
        assert!(EasingFunction::EaseOutQuad.apply(0.5) > 0.5);
    }

    #[test]
    fn test_bookmark_manager_new() {
        let manager = BookmarkManager::new();
        assert!(manager.bookmark_count() > 0); // Should have default bookmarks
        assert!(manager.smooth_transitions);
    }

    #[test]
    fn test_bookmark_manager_save_and_load() {
        let mut manager = BookmarkManager::new();
        let bookmark = CameraBookmark::new(
            "test".to_string(),
            "Test".to_string(),
            Vec3::new(1.0, 2.0, 3.0),
            1.0,
        );

        manager.save_bookmark(bookmark);
        assert!(manager.has_bookmark("test"));

        let loaded = manager.load_bookmark("test");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "Test");
    }

    #[test]
    fn test_bookmark_manager_rename() {
        let mut manager = BookmarkManager::new();
        let bookmark = CameraBookmark::new(
            "test".to_string(),
            "Old Name".to_string(),
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
        );

        manager.save_bookmark(bookmark);
        assert!(manager.rename_bookmark("test", "New Name"));

        let loaded = manager.load_bookmark("test").unwrap();
        assert_eq!(loaded.name, "New Name");
    }

    #[test]
    fn test_bookmark_manager_delete() {
        let mut manager = BookmarkManager::new();
        let bookmark = CameraBookmark::new(
            "test".to_string(),
            "Test".to_string(),
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
        );

        manager.save_bookmark(bookmark);
        assert!(manager.delete_bookmark("test"));
        assert!(!manager.has_bookmark("test"));
    }

    #[test]
    fn test_bookmark_manager_export_import() {
        let mut manager1 = BookmarkManager::new();
        let bookmark = CameraBookmark::new(
            "test".to_string(),
            "Test View".to_string(),
            Vec3::new(1.0, 2.0, 3.0),
            2.5,
        )
        .with_category("Test Category");

        manager1.save_bookmark(bookmark);
        let exported = manager1.export_bookmarks();

        let mut manager2 = BookmarkManager::new();
        let count = manager2.import_bookmarks(&exported);
        // Count includes 5 default bookmarks + 1 test bookmark = 6
        assert_eq!(count, 6);

        let loaded = manager2.load_bookmark("test").unwrap();
        assert_eq!(loaded.name, "Test View");
        assert!((loaded.position.x - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_camera_transition() {
        let from = CameraBookmark::default();
        let to = CameraBookmark::new(
            "target".to_string(),
            "Target".to_string(),
            Vec3::new(10.0, 0.0, 0.0),
            5.0,
        );

        let mut transition = CameraTransition::new(from, to, 1.0);
        assert!(!transition.active);

        transition.start();
        assert!(transition.active);
        assert_eq!(transition.progress, 0.0);

        // Update halfway
        transition.update(0.5);
        assert!(transition.active);
        assert!(transition.progress > 0.4 && transition.progress < 0.6);

        // Complete
        transition.update(0.5);
        assert!(!transition.active);
        assert_eq!(transition.progress, 1.0);
    }
}
