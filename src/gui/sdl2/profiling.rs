//! Performance profiling utilities for SDL2 event loop and rendering
//!
//! This module provides tools for measuring and optimizing the performance
//! of SDL2-based applications, including frame time tracking, event processing
//! overhead, and memory usage monitoring.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics tracker for SDL2 applications
#[derive(Debug, Clone)]
pub struct PerformanceTracker {
    /// Frame time samples (in milliseconds)
    frame_times: VecDeque<f64>,

    /// Event processing time samples (in microseconds)
    event_times: VecDeque<f64>,

    /// Render time samples (in microseconds)
    render_times: VecDeque<f64>,

    /// Maximum number of samples to keep
    max_samples: usize,

    /// Current frame start time
    current_frame_start: Option<Instant>,

    /// Current event processing start time
    current_event_start: Option<Instant>,

    /// Current render start time
    current_render_start: Option<Instant>,

    /// Frame count
    frame_count: u64,

    /// Total runtime
    total_runtime: Duration,

    /// Last FPS calculation time
    last_fps_calc: Option<Instant>,

    /// Frames since last FPS calculation
    frames_since_fps_calc: u32,

    /// Current FPS
    current_fps: f64,

    /// Average FPS over tracking period
    average_fps: f64,

    /// Minimum FPS observed
    min_fps: f64,

    /// Maximum FPS observed
    max_fps: f64,
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        Self::new(1000) // Keep last 1000 samples
    }
}

impl PerformanceTracker {
    /// Create a new performance tracker
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(max_samples),
            event_times: VecDeque::with_capacity(max_samples),
            render_times: VecDeque::with_capacity(max_samples),
            max_samples,
            current_frame_start: None,
            current_event_start: None,
            current_render_start: None,
            frame_count: 0,
            total_runtime: Duration::ZERO,
            last_fps_calc: None,
            frames_since_fps_calc: 0,
            current_fps: 0.0,
            average_fps: 0.0,
            min_fps: f64::MAX,
            max_fps: 0.0,
        }
    }

    /// Start tracking a new frame
    pub fn start_frame(&mut self) {
        self.current_frame_start = Some(Instant::now());
    }

    /// End the current frame and record frame time
    pub fn end_frame(&mut self) {
        if let Some(start) = self.current_frame_start.take() {
            let frame_time = start.elapsed().as_secs_f64() * 1000.0; // Convert to ms
            self.frame_times.push_back(frame_time);
            if self.frame_times.len() > self.max_samples {
                self.frame_times.pop_front();
            }

            self.frame_count += 1;
            self.frames_since_fps_calc += 1;

            // Calculate FPS every second
            let now = Instant::now();
            if let Some(last_calc) = self.last_fps_calc {
                if now.duration_since(last_calc) >= Duration::from_secs(1) {
                    self.current_fps = self.frames_since_fps_calc as f64;
                    self.average_fps = self.frame_count as f64 / self.total_runtime.as_secs_f64();
                    self.min_fps = self.min_fps.min(self.current_fps);
                    self.max_fps = self.max_fps.max(self.current_fps);
                    self.frames_since_fps_calc = 0;
                    self.last_fps_calc = Some(now);
                }
            } else {
                self.last_fps_calc = Some(now);
            }
        }
    }

    /// Start tracking event processing
    pub fn start_event_processing(&mut self) {
        self.current_event_start = Some(Instant::now());
    }

    /// End event processing and record time
    pub fn end_event_processing(&mut self) {
        if let Some(start) = self.current_event_start.take() {
            let event_time = start.elapsed().as_secs_f64() * 1000.0; // Convert to microseconds
            self.event_times.push_back(event_time);
            if self.event_times.len() > self.max_samples {
                self.event_times.pop_front();
            }
        }
    }

    /// Start tracking rendering
    pub fn start_rendering(&mut self) {
        self.current_render_start = Some(Instant::now());
    }

    /// End rendering and record time
    pub fn end_rendering(&mut self) {
        if let Some(start) = self.current_render_start.take() {
            let render_time = start.elapsed().as_secs_f64() * 1000.0; // Convert to microseconds
            self.render_times.push_back(render_time);
            if self.render_times.len() > self.max_samples {
                self.render_times.pop_front();
            }
        }
    }

    /// Get average frame time in milliseconds
    pub fn average_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64
    }

    /// Get minimum frame time in milliseconds
    pub fn min_frame_time(&self) -> f64 {
        self.frame_times.iter().cloned().fold(f64::MAX, f64::min)
    }

    /// Get maximum frame time in milliseconds
    pub fn max_frame_time(&self) -> f64 {
        self.frame_times.iter().cloned().fold(0.0, f64::max)
    }

    /// Get average event processing time in microseconds
    pub fn average_event_time(&self) -> f64 {
        if self.event_times.is_empty() {
            return 0.0;
        }
        self.event_times.iter().sum::<f64>() / self.event_times.len() as f64
    }

    /// Get average render time in microseconds
    pub fn average_render_time(&self) -> f64 {
        if self.render_times.is_empty() {
            return 0.0;
        }
        self.render_times.iter().sum::<f64>() / self.render_times.len() as f64
    }

    /// Get current FPS
    pub fn fps(&self) -> f64 {
        self.current_fps
    }

    /// Get average FPS
    pub fn average_fps(&self) -> f64 {
        self.average_fps
    }

    /// Get minimum FPS observed
    pub fn min_fps(&self) -> f64 {
        if self.min_fps == f64::MAX {
            0.0
        } else {
            self.min_fps
        }
    }

    /// Get maximum FPS observed
    pub fn max_fps(&self) -> f64 {
        self.max_fps
    }

    /// Get frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Get total runtime
    pub fn total_runtime(&self) -> Duration {
        self.total_runtime
    }

    /// Get number of samples collected
    pub fn sample_count(&self) -> usize {
        self.frame_times.len()
    }

    /// Get frame time percentile (0-100)
    pub fn frame_time_percentile(&self, percentile: f64) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let mut sorted: Vec<f64> = self.frame_times.iter().cloned().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = ((percentile / 100.0) * sorted.len() as f64) as usize;
        sorted.get(index).cloned().unwrap_or(0.0)
    }

    /// Get frame time median (50th percentile)
    pub fn median_frame_time(&self) -> f64 {
        self.frame_time_percentile(50.0)
    }

    /// Get frame time standard deviation
    pub fn frame_time_stddev(&self) -> f64 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }

        let avg = self.average_frame_time();
        let variance = self
            .frame_times
            .iter()
            .map(|&x| (x - avg).powi(2))
            .sum::<f64>()
            / (self.frame_times.len() - 1) as f64;

        variance.sqrt()
    }

    /// Check if performance is within target (60 FPS = 16.67ms frame time)
    pub fn is_within_target(&self, target_fps: f64) -> bool {
        let target_frame_time = 1000.0 / target_fps;
        self.average_frame_time() <= target_frame_time
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.event_times.clear();
        self.render_times.clear();
        self.frame_count = 0;
        self.total_runtime = Duration::ZERO;
        self.last_fps_calc = None;
        self.frames_since_fps_calc = 0;
        self.current_fps = 0.0;
        self.average_fps = 0.0;
        self.min_fps = f64::MAX;
        self.max_fps = 0.0;
    }

    /// Generate a performance report
    pub fn generate_report(&self) -> PerformanceReport {
        PerformanceReport {
            frame_count: self.frame_count,
            total_runtime_ms: self.total_runtime.as_secs_f64() * 1000.0,
            current_fps: self.current_fps,
            average_fps: self.average_fps,
            min_fps: self.min_fps(),
            max_fps: self.max_fps,
            average_frame_time_ms: self.average_frame_time(),
            min_frame_time_ms: self.min_frame_time(),
            max_frame_time_ms: self.max_frame_time(),
            median_frame_time_ms: self.median_frame_time(),
            frame_time_stddev_ms: self.frame_time_stddev(),
            p95_frame_time_ms: self.frame_time_percentile(95.0),
            p99_frame_time_ms: self.frame_time_percentile(99.0),
            average_event_time_us: self.average_event_time(),
            average_render_time_us: self.average_render_time(),
            sample_count: self.sample_count(),
        }
    }
}

/// Performance report with key metrics
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub frame_count: u64,
    pub total_runtime_ms: f64,
    pub current_fps: f64,
    pub average_fps: f64,
    pub min_fps: f64,
    pub max_fps: f64,
    pub average_frame_time_ms: f64,
    pub min_frame_time_ms: f64,
    pub max_frame_time_ms: f64,
    pub median_frame_time_ms: f64,
    pub frame_time_stddev_ms: f64,
    pub p95_frame_time_ms: f64,
    pub p99_frame_time_ms: f64,
    pub average_event_time_us: f64,
    pub average_render_time_us: f64,
    pub sample_count: usize,
}

impl std::fmt::Display for PerformanceReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Performance Report")?;
        writeln!(f, "==================")?;
        writeln!(f, "Frame Count: {}", self.frame_count)?;
        writeln!(f, "Total Runtime: {:.2} ms", self.total_runtime_ms)?;
        writeln!(f, "Sample Count: {}", self.sample_count)?;
        writeln!(f)?;
        writeln!(f, "FPS Metrics:")?;
        writeln!(f, "  Current: {:.2} FPS", self.current_fps)?;
        writeln!(f, "  Average: {:.2} FPS", self.average_fps)?;
        writeln!(f, "  Min: {:.2} FPS", self.min_fps)?;
        writeln!(f, "  Max: {:.2} FPS", self.max_fps)?;
        writeln!(f)?;
        writeln!(f, "Frame Time Metrics:")?;
        writeln!(f, "  Average: {:.3} ms", self.average_frame_time_ms)?;
        writeln!(f, "  Median: {:.3} ms", self.median_frame_time_ms)?;
        writeln!(f, "  StdDev: {:.3} ms", self.frame_time_stddev_ms)?;
        writeln!(f, "  P95: {:.3} ms", self.p95_frame_time_ms)?;
        writeln!(f, "  P99: {:.3} ms", self.p99_frame_time_ms)?;
        writeln!(f)?;
        writeln!(f, "Processing Time:")?;
        writeln!(f, "  Avg Event: {:.2} μs", self.average_event_time_us)?;
        writeln!(f, "  Avg Render: {:.2} μs", self.average_render_time_us)?;
        Ok(())
    }
}

/// Adaptive vsync controller
#[derive(Debug, Clone)]
pub struct AdaptiveVsync {
    /// Current vsync state
    enabled: bool,

    /// Target FPS
    target_fps: f64,

    /// Frame time history for vsync decisions
    frame_time_history: VecDeque<f64>,

    /// Maximum history size
    max_history: usize,

    /// Threshold for enabling vsync (when average frame time is consistently below target)
    vsync_enable_threshold: f64,

    /// Threshold for disabling vsync (when average frame time exceeds target)
    vsync_disable_threshold: f64,

    /// Number of consecutive frames to check before changing vsync
    stability_window: usize,

    /// Counter for stability window
    stability_counter: usize,
}

impl AdaptiveVsync {
    /// Create a new adaptive vsync controller
    pub fn new(target_fps: f64) -> Self {
        let target_frame_time = 1000.0 / target_fps;
        Self {
            enabled: true, // Start with vsync enabled
            target_fps,
            frame_time_history: VecDeque::with_capacity(60),
            max_history: 60,
            vsync_enable_threshold: target_frame_time * 0.9, // Enable at 90% of target
            vsync_disable_threshold: target_frame_time * 1.1, // Disable at 110% of target
            stability_window: 30,                            // Check over 30 frames
            stability_counter: 0,
        }
    }

    /// Update vsync state based on frame time
    pub fn update(&mut self, frame_time_ms: f64) -> bool {
        // Add frame time to history
        self.frame_time_history.push_back(frame_time_ms);
        if self.frame_time_history.len() > self.max_history {
            self.frame_time_history.pop_front();
        }

        // Need enough samples
        if self.frame_time_history.len() < self.stability_window {
            return self.enabled;
        }

        // Calculate average over stability window
        let recent_avg: f64 = self
            .frame_time_history
            .iter()
            .rev()
            .take(self.stability_window)
            .sum::<f64>()
            / self.stability_window as f64;

        // Check if we should change vsync state
        if self.enabled {
            // Check if we should disable vsync (frame time too high)
            if recent_avg > self.vsync_disable_threshold {
                self.stability_counter += 1;
                if self.stability_counter >= self.stability_window {
                    self.enabled = false;
                    self.stability_counter = 0;
                    return false;
                }
            } else {
                self.stability_counter = 0;
            }
        } else {
            // Check if we should enable vsync (frame time consistently low)
            if recent_avg < self.vsync_enable_threshold {
                self.stability_counter += 1;
                if self.stability_counter >= self.stability_window {
                    self.enabled = true;
                    self.stability_counter = 0;
                    return true;
                }
            } else {
                self.stability_counter = 0;
            }
        }

        self.enabled
    }

    /// Get current vsync state
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set vsync state manually
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.stability_counter = 0;
    }

    /// Get target FPS
    pub fn target_fps(&self) -> f64 {
        self.target_fps
    }

    /// Reset history
    pub fn reset(&mut self) {
        self.frame_time_history.clear();
        self.stability_counter = 0;
    }
}

/// Event polling optimization settings
#[derive(Debug, Clone)]
pub struct EventPollingConfig {
    /// Maximum time to spend polling events per frame (in microseconds)
    pub max_poll_time_us: u64,

    /// Minimum time between polls (in microseconds)
    pub min_poll_interval_us: u64,

    /// Whether to limit the number of events processed per frame
    pub limit_events_per_frame: bool,

    /// Maximum events to process per frame
    pub max_events_per_frame: usize,

    /// Whether to batch event processing
    pub batch_events: bool,

    /// Batch size for event processing
    pub batch_size: usize,
}

impl Default for EventPollingConfig {
    fn default() -> Self {
        Self {
            max_poll_time_us: 100,    // 100 microseconds max
            min_poll_interval_us: 10, // 10 microseconds min
            limit_events_per_frame: false,
            max_events_per_frame: 1000,
            batch_events: true,
            batch_size: 50,
        }
    }
}

impl EventPollingConfig {
    /// Create a new config with custom settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum poll time
    pub fn with_max_poll_time(mut self, time_us: u64) -> Self {
        self.max_poll_time_us = time_us;
        self
    }

    /// Set minimum poll interval
    pub fn with_min_poll_interval(mut self, interval_us: u64) -> Self {
        self.min_poll_interval_us = interval_us;
        self
    }

    /// Enable event limiting
    pub fn with_event_limiting(mut self, limit: bool, max_events: usize) -> Self {
        self.limit_events_per_frame = limit;
        self.max_events_per_frame = max_events;
        self
    }

    /// Enable event batching
    pub fn with_batching(mut self, batch: bool, batch_size: usize) -> Self {
        self.batch_events = batch;
        self.batch_size = batch_size;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_tracker_creation() {
        let tracker = PerformanceTracker::new(100);
        assert_eq!(tracker.max_samples, 100);
        assert_eq!(tracker.frame_count(), 0);
        assert_eq!(tracker.fps(), 0.0);
    }

    #[test]
    fn test_performance_tracker_frame_time() {
        let mut tracker = PerformanceTracker::new(10);

        tracker.start_frame();
        tracker.end_frame();

        assert_eq!(tracker.frame_count(), 1);
        assert!(tracker.average_frame_time() > 0.0);
    }

    #[test]
    fn test_performance_tracker_percentile() {
        let mut tracker = PerformanceTracker::new(10);

        // Add some frame times
        for _ in 0..10 {
            tracker.start_frame();
            tracker.end_frame();
        }

        assert!(tracker.median_frame_time() > 0.0);
        assert!(tracker.p95_frame_time_ms > 0.0);
        assert!(tracker.p99_frame_time_ms > 0.0);
    }

    #[test]
    fn test_performance_tracker_reset() {
        let mut tracker = PerformanceTracker::new(10);

        tracker.start_frame();
        tracker.end_frame();

        assert_eq!(tracker.frame_count(), 1);

        tracker.reset();

        assert_eq!(tracker.frame_count(), 0);
        assert_eq!(tracker.sample_count(), 0);
    }

    #[test]
    fn test_adaptive_vsync_creation() {
        let vsync = AdaptiveVsync::new(60.0);
        assert!(vsync.is_enabled());
        assert_eq!(vsync.target_fps(), 60.0);
    }

    #[test]
    fn test_adaptive_vsync_update() {
        let mut vsync = AdaptiveVsync::new(60.0);

        // Feed low frame times (should keep vsync enabled)
        for _ in 0..60 {
            vsync.update(10.0); // 10ms is well below 16.67ms target
        }

        assert!(vsync.is_enabled());
    }

    #[test]
    fn test_adaptive_vsync_manual_control() {
        let mut vsync = AdaptiveVsync::new(60.0);

        vsync.set_enabled(false);
        assert!(!vsync.is_enabled());

        vsync.set_enabled(true);
        assert!(vsync.is_enabled());
    }

    #[test]
    fn test_event_polling_config_default() {
        let config = EventPollingConfig::default();
        assert_eq!(config.max_poll_time_us, 100);
        assert_eq!(config.min_poll_interval_us, 10);
        assert!(config.batch_events);
    }

    #[test]
    fn test_event_polling_config_builder() {
        let config = EventPollingConfig::new()
            .with_max_poll_time(200)
            .with_min_poll_interval(20)
            .with_event_limiting(true, 500)
            .with_batching(false, 100);

        assert_eq!(config.max_poll_time_us, 200);
        assert_eq!(config.min_poll_interval_us, 20);
        assert!(config.limit_events_per_frame);
        assert_eq!(config.max_events_per_frame, 500);
        assert!(!config.batch_events);
    }
}
