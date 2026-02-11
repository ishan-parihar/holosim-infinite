//! Performance Profiler
//!
//! Real-time performance monitoring and profiling for the GUI.
//! Tracks frame times, GPU metrics, memory usage, and bottlenecks.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Performance profiling tools"

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance profiler for real-time metrics
///
/// Tracks:
/// - Frame timing (CPU and GPU)
/// - Memory usage
/// - Draw call counts
/// - System bottlenecks
/// - Historical performance data
pub struct PerformanceProfiler {
    /// Frame time history (last 120 frames = 2 seconds at 60 FPS)
    frame_times: VecDeque<f32>,

    /// GPU time history
    gpu_times: VecDeque<f32>,

    /// Draw call counts
    draw_calls: VecDeque<u32>,

    /// Entity counts
    entity_counts: VecDeque<usize>,

    /// Memory usage in MB
    memory_usage_mb: f32,

    /// Current FPS
    current_fps: f32,

    /// Average FPS (last 60 frames)
    average_fps: f32,

    /// Minimum FPS recorded
    min_fps: f32,

    /// Maximum FPS recorded
    max_fps: f32,

    /// Frame time percentiles
    percentiles: FramePercentiles,

    /// Active timers
    active_timers: Vec<Timer>,

    /// Timer results
    timer_results: Vec<TimerResult>,

    /// Last update time
    last_update: Instant,

    /// Update interval
    update_interval: Duration,

    /// Target frame time (16.67ms for 60 FPS)
    target_frame_time_ms: f32,

    /// Whether profiling is enabled
    enabled: bool,
}

/// Frame timing percentiles
#[derive(Debug, Clone, Copy, Default)]
pub struct FramePercentiles {
    pub p50: f32, // Median
    pub p90: f32, // 90th percentile
    pub p95: f32, // 95th percentile
    pub p99: f32, // 99th percentile
}

/// Timer for profiling specific operations
#[derive(Debug, Clone)]
pub struct Timer {
    pub name: String,
    pub start_time: Instant,
    pub category: ProfileCategory,
}

/// Timer result
#[derive(Debug, Clone)]
pub struct TimerResult {
    pub name: String,
    pub duration_ms: f32,
    pub category: ProfileCategory,
    pub timestamp: Instant,
}

/// Profile categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileCategory {
    Rendering,
    Simulation,
    UI,
    Input,
    Physics,
    Memory,
    GPU,
    Other,
}

/// Performance snapshot for display
#[derive(Debug, Clone, Default)]
pub struct PerformanceSnapshot {
    pub fps: f32,
    pub average_fps: f32,
    pub frame_time_ms: f32,
    pub gpu_time_ms: f32,
    pub draw_calls: u32,
    pub entity_count: usize,
    pub memory_mb: f32,
    pub percentiles: FramePercentiles,
    pub bottleneck: Option<Bottleneck>,
}

/// Identified bottleneck
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bottleneck {
    CPU,
    GPU,
    Memory,
    Simulation,
    Rendering,
}

impl PerformanceProfiler {
    /// Create new profiler
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(120),
            gpu_times: VecDeque::with_capacity(120),
            draw_calls: VecDeque::with_capacity(120),
            entity_counts: VecDeque::with_capacity(120),
            memory_usage_mb: 0.0,
            current_fps: 0.0,
            average_fps: 0.0,
            min_fps: f32::MAX,
            max_fps: 0.0,
            percentiles: FramePercentiles::default(),
            active_timers: Vec::new(),
            timer_results: Vec::new(),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
            target_frame_time_ms: 1000.0 / 60.0, // 60 FPS
            enabled: true,
        }
    }

    /// Record a frame
    pub fn record_frame(
        &mut self,
        frame_time_ms: f32,
        gpu_time_ms: f32,
        draw_calls: u32,
        entity_count: usize,
    ) {
        if !self.enabled {
            return;
        }

        // Add to history
        self.frame_times.push_back(frame_time_ms);
        self.gpu_times.push_back(gpu_time_ms);
        self.draw_calls.push_back(draw_calls);
        self.entity_counts.push_back(entity_count);

        // Maintain history size
        if self.frame_times.len() > 120 {
            self.frame_times.pop_front();
        }
        if self.gpu_times.len() > 120 {
            self.gpu_times.pop_front();
        }
        if self.draw_calls.len() > 120 {
            self.draw_calls.pop_front();
        }
        if self.entity_counts.len() > 120 {
            self.entity_counts.pop_front();
        }

        // Update current FPS
        self.current_fps = 1000.0 / frame_time_ms.max(0.1);

        // Update min/max
        self.min_fps = self.min_fps.min(self.current_fps);
        self.max_fps = self.max_fps.max(self.current_fps);

        // Update percentiles periodically
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.update_statistics();
            self.last_update = now;
        }
    }

    /// Update statistical calculations
    fn update_statistics(&mut self) {
        if self.frame_times.is_empty() {
            return;
        }

        // Calculate average FPS
        let total_time: f32 = self.frame_times.iter().sum();
        self.average_fps = 1000.0 / (total_time / self.frame_times.len() as f32);

        // Calculate percentiles
        let mut sorted_times: Vec<f32> = self.frame_times.iter().copied().collect();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.percentiles.p50 = Self::calculate_percentile(&sorted_times, 0.50);
        self.percentiles.p90 = Self::calculate_percentile(&sorted_times, 0.90);
        self.percentiles.p95 = Self::calculate_percentile(&sorted_times, 0.95);
        self.percentiles.p99 = Self::calculate_percentile(&sorted_times, 0.99);

        // Update memory usage
        self.update_memory_usage();
    }

    /// Calculate percentile from sorted data
    fn calculate_percentile(sorted_data: &[f32], percentile: f32) -> f32 {
        if sorted_data.is_empty() {
            return 0.0;
        }

        let index = (percentile * (sorted_data.len() - 1) as f32) as usize;
        sorted_data[index.min(sorted_data.len() - 1)]
    }

    /// Update memory usage
    fn update_memory_usage(&mut self) {
        // This is platform-specific
        // On Linux, we could read /proc/self/status
        // For now, use a placeholder
        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = std::fs::read_to_string("/proc/self/status") {
                for line in contents.lines() {
                    if line.starts_with("VmRSS:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(kb) = parts[1].parse::<f32>() {
                                self.memory_usage_mb = kb / 1024.0;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Start a timer
    pub fn start_timer(&mut self, name: &str, category: ProfileCategory) -> usize {
        let timer = Timer {
            name: name.to_string(),
            start_time: Instant::now(),
            category,
        };

        self.active_timers.push(timer);
        self.active_timers.len() - 1
    }

    /// End a timer
    pub fn end_timer(&mut self, timer_id: usize) -> Option<f32> {
        if timer_id >= self.active_timers.len() {
            return None;
        }

        let timer = self.active_timers.swap_remove(timer_id);
        let duration = timer.start_time.elapsed();
        let duration_ms = duration.as_secs_f32() * 1000.0;

        let result = TimerResult {
            name: timer.name,
            duration_ms,
            category: timer.category,
            timestamp: Instant::now(),
        };

        self.timer_results.push(result);

        // Keep only last 1000 results
        if self.timer_results.len() > 1000 {
            self.timer_results.remove(0);
        }

        Some(duration_ms)
    }

    /// Get average time for a category
    pub fn get_average_time(&self, category: ProfileCategory) -> f32 {
        let times: Vec<f32> = self
            .timer_results
            .iter()
            .filter(|r| r.category == category)
            .map(|r| r.duration_ms)
            .collect();

        if times.is_empty() {
            0.0
        } else {
            times.iter().sum::<f32>() / times.len() as f32
        }
    }

    /// Identify current bottleneck
    pub fn identify_bottleneck(&self) -> Option<Bottleneck> {
        if self.frame_times.len() < 10 {
            return None;
        }

        let avg_frame_time = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;

        if avg_frame_time < self.target_frame_time_ms {
            return None; // No bottleneck
        }

        // Check GPU vs CPU
        let avg_gpu_time = if self.gpu_times.len() >= 10 {
            self.gpu_times.iter().sum::<f32>() / self.gpu_times.len() as f32
        } else {
            0.0
        };

        let cpu_time = avg_frame_time - avg_gpu_time;

        if avg_gpu_time > cpu_time {
            Some(Bottleneck::GPU)
        } else if self.memory_usage_mb > 2048.0 {
            Some(Bottleneck::Memory)
        } else {
            // Check simulation vs rendering
            let sim_time = self.get_average_time(ProfileCategory::Simulation);
            let render_time = self.get_average_time(ProfileCategory::Rendering);

            if sim_time > render_time {
                Some(Bottleneck::Simulation)
            } else {
                Some(Bottleneck::Rendering)
            }
        }
    }

    /// Get performance snapshot
    pub fn get_snapshot(&self) -> PerformanceSnapshot {
        PerformanceSnapshot {
            fps: self.current_fps,
            average_fps: self.average_fps,
            frame_time_ms: self.frame_times.back().copied().unwrap_or(0.0),
            gpu_time_ms: self.gpu_times.back().copied().unwrap_or(0.0),
            draw_calls: self.draw_calls.back().copied().unwrap_or(0),
            entity_count: self.entity_counts.back().copied().unwrap_or(0),
            memory_mb: self.memory_usage_mb,
            percentiles: self.percentiles,
            bottleneck: self.identify_bottleneck(),
        }
    }

    /// Get frame time history
    pub fn get_frame_time_history(&self) -> Vec<f32> {
        self.frame_times.iter().copied().collect()
    }

    /// Get FPS history
    pub fn get_fps_history(&self) -> Vec<f32> {
        self.frame_times
            .iter()
            .map(|&t| 1000.0 / t.max(0.1))
            .collect()
    }

    /// Get min/max FPS
    pub fn get_fps_range(&self) -> (f32, f32) {
        (self.min_fps, self.max_fps)
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.gpu_times.clear();
        self.draw_calls.clear();
        self.entity_counts.clear();
        self.min_fps = f32::MAX;
        self.max_fps = 0.0;
        self.timer_results.clear();
        self.active_timers.clear();
    }

    /// Enable/disable profiling
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if profiling is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set target FPS
    pub fn set_target_fps(&mut self, fps: u32) {
        self.target_frame_time_ms = 1000.0 / fps as f32;
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// GPU profiler using WGPU timestamp queries
pub struct GpuProfiler {
    /// Timestamp query set
    query_set: Option<wgpu::QuerySet>,

    /// Query buffer
    query_buffer: Option<wgpu::Buffer>,

    /// Current query index
    query_index: u32,

    /// Whether queries are supported
    supported: bool,
}

impl GpuProfiler {
    /// Create new GPU profiler
    pub fn new(device: &wgpu::Device) -> Self {
        let features = device.features();
        let supported = features.contains(wgpu::Features::TIMESTAMP_QUERY);

        let (query_set, query_buffer) = if supported {
            let query_set = device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("GPU Profiler Queries"),
                count: 128,
                ty: wgpu::QueryType::Timestamp,
            });

            let query_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("GPU Profiler Buffer"),
                size: 128 * 8, // 8 bytes per timestamp
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            (Some(query_set), Some(query_buffer))
        } else {
            (None, None)
        };

        Self {
            query_set,
            query_buffer,
            query_index: 0,
            supported,
        }
    }

    /// Begin GPU timestamp
    pub fn begin_timestamp(&mut self, encoder: &mut wgpu::CommandEncoder) {
        if let Some(ref query_set) = self.query_set {
            encoder.write_timestamp(query_set, self.query_index);
            self.query_index += 1;
        }
    }

    /// End GPU timestamp
    pub fn end_timestamp(&mut self, encoder: &mut wgpu::CommandEncoder) {
        if let Some(ref query_set) = self.query_set {
            encoder.write_timestamp(query_set, self.query_index);
            self.query_index += 1;
        }
    }

    /// Resolve timestamps
    pub fn resolve(&mut self, encoder: &mut wgpu::CommandEncoder) {
        if let (Some(query_set), Some(query_buffer)) = (&self.query_set, &self.query_buffer) {
            encoder.resolve_query_set(query_set, 0..self.query_index, query_buffer, 0);
        }
    }

    /// Check if GPU profiling is supported
    pub fn is_supported(&self) -> bool {
        self.supported
    }
}

/// Memory profiler
pub struct MemoryProfiler {
    /// Peak memory usage in MB
    peak_memory_mb: f32,

    /// Current memory usage in MB
    current_memory_mb: f32,

    /// Memory usage history
    history: VecDeque<f32>,
}

impl MemoryProfiler {
    /// Create new memory profiler
    pub fn new() -> Self {
        Self {
            peak_memory_mb: 0.0,
            current_memory_mb: 0.0,
            history: VecDeque::with_capacity(120),
        }
    }

    /// Update memory usage
    pub fn update(&mut self) {
        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = std::fs::read_to_string("/proc/self/status") {
                for line in contents.lines() {
                    if line.starts_with("VmRSS:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(kb) = parts[1].parse::<f32>() {
                                self.current_memory_mb = kb / 1024.0;
                                self.peak_memory_mb =
                                    self.peak_memory_mb.max(self.current_memory_mb);

                                self.history.push_back(self.current_memory_mb);
                                if self.history.len() > 120 {
                                    self.history.pop_front();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get current memory usage
    pub fn current_mb(&self) -> f32 {
        self.current_memory_mb
    }

    /// Get peak memory usage
    pub fn peak_mb(&self) -> f32 {
        self.peak_memory_mb
    }

    /// Get memory history
    pub fn get_history(&self) -> Vec<f32> {
        self.history.iter().copied().collect()
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_profiler_new() {
        let profiler = PerformanceProfiler::new();
        assert!(profiler.enabled);
        assert_eq!(profiler.current_fps, 0.0);
        assert!(profiler.frame_times.capacity() >= 120);
    }

    #[test]
    fn test_record_frame() {
        let mut profiler = PerformanceProfiler::new();

        profiler.record_frame(16.67, 8.0, 100, 500);
        assert_eq!(profiler.frame_times.len(), 1);

        // Record multiple frames
        for _ in 0..150 {
            profiler.record_frame(16.67, 8.0, 100, 500);
        }

        // Should maintain max history
        assert!(profiler.frame_times.len() <= 120);
    }

    #[test]
    fn test_timer() {
        let mut profiler = PerformanceProfiler::new();

        let timer_id = profiler.start_timer("test_op", ProfileCategory::Rendering);
        std::thread::sleep(Duration::from_millis(10));
        let duration = profiler.end_timer(timer_id);

        assert!(duration.is_some());
        assert!(duration.unwrap() >= 10.0); // At least 10ms
    }

    #[test]
    fn test_calculate_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(PerformanceProfiler::calculate_percentile(&data, 0.0), 1.0);
        assert_eq!(PerformanceProfiler::calculate_percentile(&data, 0.5), 3.0);
        assert_eq!(PerformanceProfiler::calculate_percentile(&data, 1.0), 5.0);
    }

    #[test]
    fn test_performance_snapshot() {
        let profiler = PerformanceProfiler::new();
        let snapshot = profiler.get_snapshot();

        assert_eq!(snapshot.fps, 0.0);
        assert_eq!(snapshot.frame_time_ms, 0.0);
    }

    #[test]
    fn test_memory_profiler() {
        let mut profiler = MemoryProfiler::new();

        assert_eq!(profiler.current_mb(), 0.0);
        assert_eq!(profiler.peak_mb(), 0.0);

        profiler.update();
        // Memory values will be 0 on non-Linux platforms
    }
}
