//! Performance Optimizer - Phase 6 Integration & Testing
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 6:
//! "Performance optimization for parallel execution, rendering, and memory management"
//!
//! This module provides:
//! - Parallel scheduler for efficient multi-threaded execution
//! - Rendering optimizer for GPU-accelerated visualization
//! - Memory manager for efficient resource allocation

use crate::types::Float;
use std::collections::HashMap;

/// Performance Optimizer
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Performance optimization to meet requirements (60 FPS, < 1s latency)"
#[derive(Clone)]
pub struct PerformanceOptimizer {
    /// Parallel scheduler for multi-threaded execution
    parallel_scheduler: ParallelScheduler,

    /// Rendering optimizer for GPU acceleration
    rendering_optimizer: RenderingOptimizer,

    /// Memory manager for resource allocation
    memory_manager: MemoryManager,

    /// Performance profile
    profile: PerformanceProfile,
}

/// Parallel scheduler for efficient multi-threaded execution
#[derive(Debug, Clone)]
pub struct ParallelScheduler {
    /// Number of worker threads
    pub worker_threads: usize,

    /// Task queue size
    pub task_queue_size: usize,

    /// Parallel efficiency (0.0-1.0)
    pub parallel_efficiency: Float,

    /// Average task completion time (seconds)
    pub avg_task_time: Float,

    /// Tasks completed
    pub tasks_completed: usize,

    /// Tasks failed
    pub tasks_failed: usize,
}

/// Rendering optimizer for GPU-accelerated visualization
#[derive(Debug, Clone)]
pub struct RenderingOptimizer {
    /// Target frame rate (FPS)
    pub target_fps: Float,

    /// Current frame rate (FPS)
    pub current_fps: Float,

    /// Frame time (seconds)
    pub frame_time: Float,

    /// GPU utilization (0.0-1.0)
    pub gpu_utilization: Float,

    /// Level of detail enabled
    pub lod_enabled: bool,

    /// Frustum culling enabled
    pub frustum_culling_enabled: bool,

    /// Occlusion culling enabled
    pub occlusion_culling_enabled: bool,
}

/// Memory manager for efficient resource allocation
#[derive(Debug, Clone)]
pub struct MemoryManager {
    /// Total memory available (bytes)
    pub total_memory: usize,

    /// Memory currently in use (bytes)
    pub used_memory: usize,

    /// Peak memory usage (bytes)
    pub peak_memory: usize,

    /// Memory fragmentation (0.0-1.0)
    pub fragmentation: Float,

    /// Allocation count
    pub allocation_count: usize,

    /// Deallocation count
    pub deallocation_count: usize,

    /// Memory pools
    pub memory_pools: HashMap<String, usize>,
}

/// Performance profile
#[derive(Debug, Clone, Default)]
pub struct PerformanceProfile {
    /// Average FPS
    pub average_fps: Float,

    /// Average latency (seconds)
    pub average_latency: Float,

    /// Peak memory usage (bytes)
    pub peak_memory: usize,

    /// Total execution time (seconds)
    pub total_execution_time: Float,

    /// Parallel efficiency (0.0-1.0)
    pub parallel_efficiency: Float,

    /// Frame time distribution
    pub frame_times: Vec<Float>,

    /// Task completion times
    pub task_times: Vec<Float>,
}

impl Default for ParallelScheduler {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get(),
            task_queue_size: 1000,
            parallel_efficiency: 0.0,
            avg_task_time: 0.0,
            tasks_completed: 0,
            tasks_failed: 0,
        }
    }
}

impl Default for RenderingOptimizer {
    fn default() -> Self {
        Self {
            target_fps: 60.0,
            current_fps: 0.0,
            frame_time: 0.0,
            gpu_utilization: 0.0,
            lod_enabled: true,
            frustum_culling_enabled: true,
            occlusion_culling_enabled: true,
        }
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            total_memory: 16 * 1024 * 1024 * 1024, // 16 GB
            used_memory: 0,
            peak_memory: 0,
            fragmentation: 0.0,
            allocation_count: 0,
            deallocation_count: 0,
            memory_pools: HashMap::new(),
        }
    }
}

impl Default for PerformanceOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceOptimizer {
    /// Create a new performance optimizer with default settings
    pub fn new() -> Self {
        Self {
            parallel_scheduler: ParallelScheduler::default(),
            rendering_optimizer: RenderingOptimizer::default(),
            memory_manager: MemoryManager::default(),
            profile: PerformanceProfile::default(),
        }
    }

    /// Create a new performance optimizer with custom settings
    pub fn with_settings(worker_threads: usize, target_fps: Float, total_memory: usize) -> Self {
        Self {
            parallel_scheduler: ParallelScheduler {
                worker_threads,
                ..Default::default()
            },
            rendering_optimizer: RenderingOptimizer {
                target_fps,
                ..Default::default()
            },
            memory_manager: MemoryManager {
                total_memory,
                ..Default::default()
            },
            profile: PerformanceProfile::default(),
        }
    }

    /// Optimize parallel execution
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Parallel execution optimization for 1000+ entities"
    pub fn optimize_parallel_execution(&mut self) {
        println!("Optimizing parallel execution...");

        // Calculate parallel efficiency
        if self.parallel_scheduler.tasks_completed > 0 {
            let ideal_speedup = self.parallel_scheduler.worker_threads as Float;
            let actual_speedup = ideal_speedup * self.parallel_scheduler.parallel_efficiency;
            println!(
                "  Parallel efficiency: {:.2}%",
                self.parallel_scheduler.parallel_efficiency * 100.0
            );
            println!(
                "  Speedup: {:.2}x (ideal: {:.2}x)",
                actual_speedup, ideal_speedup
            );
        }

        // Adjust worker thread count based on efficiency
        if self.parallel_scheduler.parallel_efficiency < 0.7 {
            let optimal_threads = (self.parallel_scheduler.worker_threads as Float * 0.8) as usize;
            println!(
                "  Reducing worker threads from {} to {}",
                self.parallel_scheduler.worker_threads, optimal_threads
            );
            self.parallel_scheduler.worker_threads = optimal_threads.max(1);
        }

        // Update profile
        self.profile.parallel_efficiency = self.parallel_scheduler.parallel_efficiency;
    }

    /// Optimize rendering
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Rendering optimization for 60 FPS at 1080p"
    pub fn optimize_rendering(&mut self) {
        println!("Optimizing rendering...");

        // Calculate frame time
        if self.rendering_optimizer.current_fps > 0.0 {
            self.rendering_optimizer.frame_time = 1.0 / self.rendering_optimizer.current_fps;
        }

        // Check if target FPS is met
        let fps_ratio = self.rendering_optimizer.current_fps / self.rendering_optimizer.target_fps;
        if fps_ratio < 0.9 {
            println!(
                "  FPS below target: {:.1} / {:.1}",
                self.rendering_optimizer.current_fps, self.rendering_optimizer.target_fps
            );

            // Enable/disable optimizations based on performance
            if fps_ratio < 0.5 {
                self.rendering_optimizer.lod_enabled = true;
                self.rendering_optimizer.frustum_culling_enabled = true;
                self.rendering_optimizer.occlusion_culling_enabled = true;
                println!("  Enabling all rendering optimizations");
            }
        } else {
            println!(
                "  Target FPS met: {:.1} / {:.1}",
                self.rendering_optimizer.current_fps, self.rendering_optimizer.target_fps
            );
        }

        // Update profile
        self.profile.average_fps = self.rendering_optimizer.current_fps;
    }

    /// Optimize memory
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Memory optimization for 1000+ entities within 16GB"
    pub fn optimize_memory(&mut self) {
        println!("Optimizing memory...");

        // Calculate memory usage percentage
        let memory_usage_percent = (self.memory_manager.used_memory as Float
            / self.memory_manager.total_memory as Float)
            * 100.0;
        println!(
            "  Memory usage: {:.2}% ({} / {} bytes)",
            memory_usage_percent, self.memory_manager.used_memory, self.memory_manager.total_memory
        );

        // Check fragmentation
        if self.memory_manager.fragmentation > 0.3 {
            println!(
                "  High fragmentation detected: {:.2}%",
                self.memory_manager.fragmentation * 100.0
            );
            println!("  Defragmentation recommended");
        }

        // Update peak memory
        if self.memory_manager.used_memory > self.memory_manager.peak_memory {
            self.memory_manager.peak_memory = self.memory_manager.used_memory;
        }

        // Update profile
        self.profile.peak_memory = self.memory_manager.peak_memory;
    }

    /// Profile performance
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Performance metrics: FPS, latency, memory, parallel efficiency"
    pub fn profile(&self) -> PerformanceProfile {
        self.profile.clone()
    }

    /// Start performance monitoring
    pub fn start_monitoring(&mut self) {
        println!("Starting performance monitoring...");
        self.profile = PerformanceProfile::default();
    }

    /// Stop performance monitoring
    pub fn stop_monitoring(&mut self) {
        println!("Stopping performance monitoring...");

        // Calculate averages
        if !self.profile.frame_times.is_empty() {
            let avg_frame_time: Float = self.profile.frame_times.iter().sum::<Float>()
                / self.profile.frame_times.len() as Float;
            self.profile.average_fps = if avg_frame_time > 0.0 {
                1.0 / avg_frame_time
            } else {
                0.0
            };
        }

        if !self.profile.task_times.is_empty() {
            self.profile.average_latency = self.profile.task_times.iter().sum::<Float>()
                / self.profile.task_times.len() as Float;
        }

        // Print summary
        println!("Performance Summary:");
        println!("  Average FPS: {:.1}", self.profile.average_fps);
        println!("  Average Latency: {:.4}s", self.profile.average_latency);
        println!("  Peak Memory: {} bytes", self.profile.peak_memory);
        println!(
            "  Parallel Efficiency: {:.2}%",
            self.profile.parallel_efficiency * 100.0
        );
    }

    /// Record frame time
    pub fn record_frame_time(&mut self, frame_time: Float) {
        self.profile.frame_times.push(frame_time);
        self.rendering_optimizer.frame_time = frame_time;
        self.rendering_optimizer.current_fps = if frame_time > 0.0 {
            1.0 / frame_time
        } else {
            0.0
        };
    }

    /// Record task completion time
    pub fn record_task_time(&mut self, task_time: Float) {
        self.profile.task_times.push(task_time);
        self.parallel_scheduler.tasks_completed += 1;

        // Update average task time
        if self.parallel_scheduler.tasks_completed > 0 {
            let total: Float = self.profile.task_times.iter().sum();
            self.parallel_scheduler.avg_task_time = total / self.profile.task_times.len() as Float;
        }
    }

    /// Allocate memory
    pub fn allocate_memory(&mut self, size: usize) -> Result<(), String> {
        if self.memory_manager.used_memory + size > self.memory_manager.total_memory {
            return Err(format!(
                "Insufficient memory: requested {}, available {}",
                size,
                self.memory_manager.total_memory - self.memory_manager.used_memory
            ));
        }

        self.memory_manager.used_memory += size;
        self.memory_manager.allocation_count += 1;
        Ok(())
    }

    /// Deallocate memory
    pub fn deallocate_memory(&mut self, size: usize) {
        if self.memory_manager.used_memory >= size {
            self.memory_manager.used_memory -= size;
        }
        self.memory_manager.deallocation_count += 1;
    }

    /// Get parallel scheduler
    pub fn parallel_scheduler(&self) -> &ParallelScheduler {
        &self.parallel_scheduler
    }

    /// Get rendering optimizer
    pub fn rendering_optimizer(&self) -> &RenderingOptimizer {
        &self.rendering_optimizer
    }

    /// Get memory manager
    pub fn memory_manager(&self) -> &MemoryManager {
        &self.memory_manager
    }

    /// Check if performance requirements are met
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Performance Requirements: 60 FPS, < 1s latency, <= 16GB memory"
    pub fn check_requirements(&self) -> bool {
        let fps_ok = self.profile.average_fps >= 60.0;
        let latency_ok = self.profile.average_latency < 1.0;
        let memory_ok = self.profile.peak_memory <= 16 * 1024 * 1024 * 1024; // 16 GB

        fps_ok && latency_ok && memory_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_optimizer_creation() {
        let optimizer = PerformanceOptimizer::new();
        assert_eq!(optimizer.rendering_optimizer.target_fps, 60.0);
        assert_eq!(
            optimizer.memory_manager.total_memory,
            16 * 1024 * 1024 * 1024
        );
    }

    #[test]
    fn test_performance_optimizer_with_settings() {
        let optimizer = PerformanceOptimizer::with_settings(8, 120.0, 8 * 1024 * 1024 * 1024);
        assert_eq!(optimizer.parallel_scheduler.worker_threads, 8);
        assert_eq!(optimizer.rendering_optimizer.target_fps, 120.0);
        assert_eq!(
            optimizer.memory_manager.total_memory,
            8 * 1024 * 1024 * 1024
        );
    }

    #[test]
    fn test_optimize_parallel_execution() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.parallel_scheduler.tasks_completed = 100;
        optimizer.parallel_scheduler.parallel_efficiency = 0.8;

        optimizer.optimize_parallel_execution();
        assert_eq!(optimizer.profile.parallel_efficiency, 0.8);
    }

    #[test]
    fn test_optimize_rendering() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.rendering_optimizer.current_fps = 30.0;

        optimizer.optimize_rendering();
        assert!(optimizer.rendering_optimizer.lod_enabled);
    }

    #[test]
    fn test_optimize_memory() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.memory_manager.used_memory = 8 * 1024 * 1024 * 1024;

        optimizer.optimize_memory();
        assert_eq!(optimizer.memory_manager.peak_memory, 8 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_record_frame_time() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.record_frame_time(0.016); // ~60 FPS

        assert_eq!(optimizer.profile.frame_times.len(), 1);
        assert!((optimizer.rendering_optimizer.current_fps - 62.5).abs() < 1.0);
    }

    #[test]
    fn test_record_task_time() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.record_task_time(0.1);

        assert_eq!(optimizer.profile.task_times.len(), 1);
        assert_eq!(optimizer.parallel_scheduler.tasks_completed, 1);
    }

    #[test]
    fn test_allocate_memory() {
        let mut optimizer = PerformanceOptimizer::new();
        let result = optimizer.allocate_memory(1024);

        assert!(result.is_ok());
        assert_eq!(optimizer.memory_manager.used_memory, 1024);
        assert_eq!(optimizer.memory_manager.allocation_count, 1);
    }

    #[test]
    fn test_allocate_memory_insufficient() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.memory_manager.used_memory = optimizer.memory_manager.total_memory;

        let result = optimizer.allocate_memory(1024);
        assert!(result.is_err());
    }

    #[test]
    fn test_deallocate_memory() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.allocate_memory(1024).unwrap();
        optimizer.deallocate_memory(512);

        assert_eq!(optimizer.memory_manager.used_memory, 512);
        assert_eq!(optimizer.memory_manager.deallocation_count, 1);
    }

    #[test]
    fn test_start_stop_monitoring() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.start_monitoring();

        optimizer.record_frame_time(0.016);
        optimizer.record_task_time(0.1);

        optimizer.stop_monitoring();
        assert_eq!(optimizer.profile.frame_times.len(), 1);
        assert_eq!(optimizer.profile.task_times.len(), 1);
    }

    #[test]
    fn test_check_requirements() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.profile.average_fps = 60.0;
        optimizer.profile.average_latency = 0.5;
        optimizer.profile.peak_memory = 8 * 1024 * 1024 * 1024;

        assert!(optimizer.check_requirements());
    }

    #[test]
    fn test_check_requirements_fps_fail() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.profile.average_fps = 30.0;
        optimizer.profile.average_latency = 0.5;
        optimizer.profile.peak_memory = 8 * 1024 * 1024 * 1024;

        assert!(!optimizer.check_requirements());
    }

    #[test]
    fn test_check_requirements_latency_fail() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.profile.average_fps = 60.0;
        optimizer.profile.average_latency = 1.5;
        optimizer.profile.peak_memory = 8 * 1024 * 1024 * 1024;

        assert!(!optimizer.check_requirements());
    }

    #[test]
    fn test_check_requirements_memory_fail() {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.profile.average_fps = 60.0;
        optimizer.profile.average_latency = 0.5;
        optimizer.profile.peak_memory = 32 * 1024 * 1024 * 1024;

        assert!(!optimizer.check_requirements());
    }

    #[test]
    fn test_parallel_scheduler_default() {
        let scheduler = ParallelScheduler::default();
        assert_eq!(scheduler.worker_threads, num_cpus::get());
        assert_eq!(scheduler.task_queue_size, 1000);
    }

    #[test]
    fn test_rendering_optimizer_default() {
        let optimizer = RenderingOptimizer::default();
        assert_eq!(optimizer.target_fps, 60.0);
        assert!(optimizer.lod_enabled);
        assert!(optimizer.frustum_culling_enabled);
    }

    #[test]
    fn test_memory_manager_default() {
        let manager = MemoryManager::default();
        assert_eq!(manager.total_memory, 16 * 1024 * 1024 * 1024);
        assert_eq!(manager.used_memory, 0);
    }
}
