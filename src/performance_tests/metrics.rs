//! Performance Metrics Collection
//!
//! From MASTER_R&D_ROADMAP.md Phase 8: "Performance metrics collection"
//!
//! This module provides specialized metrics for measuring different aspects
//! of system performance:
//! - ThroughputMetric: Operations per second
//! - LatencyMetric: Time per operation
//! - MemoryMetric: Memory usage tracking
//! - CompressionRatioMetric: For MERA compression tests

use std::time::{Duration, Instant};

/// Throughput metric - measures operations per second
///
/// From MASTER_R&D_ROADMAP.md: "ThroughputMetric - operations per second"
#[derive(Debug, Clone)]
pub struct ThroughputMetric {
    /// Number of operations performed
    pub operations: usize,

    /// Total time taken for all operations
    pub duration: Duration,

    /// Start time for the measurement
    start_time: Instant,
}

impl ThroughputMetric {
    /// Create a new throughput metric
    pub fn new() -> Self {
        ThroughputMetric {
            operations: 0,
            duration: Duration::ZERO,
            start_time: Instant::now(),
        }
    }

    /// Start measuring throughput
    pub fn start(&mut self) {
        self.operations = 0;
        self.duration = Duration::ZERO;
        self.start_time = Instant::now();
    }

    /// Record a single operation
    pub fn record_operation(&mut self) {
        self.operations += 1;
    }

    /// Record multiple operations
    pub fn record_operations(&mut self, count: usize) {
        self.operations += count;
    }

    /// Stop measuring and calculate throughput
    pub fn stop(&mut self) {
        self.duration = self.start_time.elapsed();
    }

    /// Get the throughput (operations per second)
    pub fn throughput(&self) -> f64 {
        if self.duration.as_secs_f64() > 0.0 {
            self.operations as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get the average time per operation
    pub fn avg_time_per_operation(&self) -> Duration {
        if self.operations > 0 {
            self.duration / self.operations as u32
        } else {
            Duration::ZERO
        }
    }

    /// Generate a report
    pub fn report(&self) -> String {
        format!(
            "Throughput Metric:\n\
             Operations: {}\n\
             Duration:   {:.3} s\n\
             Throughput: {:.2} ops/sec\n\
             Avg/Op:     {:.6} ms",
            self.operations,
            self.duration.as_secs_f64(),
            self.throughput(),
            self.avg_time_per_operation().as_secs_f64() * 1000.0,
        )
    }
}

impl Default for ThroughputMetric {
    fn default() -> Self {
        Self::new()
    }
}

/// Latency metric - measures time per operation
///
/// From MASTER_R&D_ROADMAP.md: "LatencyMetric - time per operation"
#[derive(Debug, Clone)]
pub struct LatencyMetric {
    /// Individual operation latencies
    latencies: Vec<Duration>,

    /// Minimum latency
    pub min: Duration,

    /// Maximum latency
    pub max: Duration,

    /// Sum of all latencies
    sum: Duration,
}

impl LatencyMetric {
    /// Create a new latency metric
    pub fn new() -> Self {
        LatencyMetric {
            latencies: Vec::new(),
            min: Duration::MAX,
            max: Duration::ZERO,
            sum: Duration::ZERO,
        }
    }

    /// Record a single operation latency
    pub fn record_latency(&mut self, latency: Duration) {
        self.latencies.push(latency);
        self.min = self.min.min(latency);
        self.max = self.max.max(latency);
        self.sum += latency;
    }

    /// Get the number of recorded latencies
    pub fn count(&self) -> usize {
        self.latencies.len()
    }

    /// Get the average latency
    pub fn avg(&self) -> Duration {
        if self.latencies.is_empty() {
            Duration::ZERO
        } else {
            self.sum / self.latencies.len() as u32
        }
    }

    /// Get the median latency
    pub fn median(&self) -> Duration {
        if self.latencies.is_empty() {
            return Duration::ZERO;
        }

        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();

        let count = sorted.len();
        if count.is_multiple_of(2) {
            (sorted[count / 2 - 1] + sorted[count / 2]) / 2
        } else {
            sorted[count / 2]
        }
    }

    /// Get the P95 latency
    pub fn p95(&self) -> Duration {
        if self.latencies.is_empty() {
            return Duration::ZERO;
        }

        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();

        let index = ((self.latencies.len() as f64) * 0.95) as usize;
        sorted[index.min(self.latencies.len() - 1)]
    }

    /// Get the P99 latency
    pub fn p99(&self) -> Duration {
        if self.latencies.is_empty() {
            return Duration::ZERO;
        }

        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();

        let index = ((self.latencies.len() as f64) * 0.99) as usize;
        sorted[index.min(self.latencies.len() - 1)]
    }

    /// Get the standard deviation
    pub fn std_dev(&self) -> Duration {
        if self.latencies.is_empty() {
            return Duration::ZERO;
        }

        let avg_nanos = self.avg().as_nanos() as f64;
        let variance: f64 = self
            .latencies
            .iter()
            .map(|l| {
                let diff = l.as_nanos() as f64 - avg_nanos;
                diff * diff
            })
            .sum::<f64>()
            / self.latencies.len() as f64;

        Duration::from_nanos(variance.sqrt() as u64)
    }

    /// Generate a report
    pub fn report(&self) -> String {
        if self.latencies.is_empty() {
            return "Latency Metric: No data".to_string();
        }

        format!(
            "Latency Metric:\n\
             Count:     {}\n\
             Min:       {:.6} ms\n\
             Max:       {:.6} ms\n\
             Avg:       {:.6} ms\n\
             Median:    {:.6} ms\n\
             P95:       {:.6} ms\n\
             P99:       {:.6} ms\n\
             Std Dev:   {:.6} ms",
            self.count(),
            self.min.as_secs_f64() * 1000.0,
            self.max.as_secs_f64() * 1000.0,
            self.avg().as_secs_f64() * 1000.0,
            self.median().as_secs_f64() * 1000.0,
            self.p95().as_secs_f64() * 1000.0,
            self.p99().as_secs_f64() * 1000.0,
            self.std_dev().as_secs_f64() * 1000.0,
        )
    }
}

impl Default for LatencyMetric {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory metric - tracks memory usage
///
/// From MASTER_R&D_ROADMAP.md: "MemoryMetric - memory usage tracking"
///
/// Note: Rust doesn't provide a built-in way to measure memory usage.
/// This metric provides a framework that can be integrated with platform-specific
/// memory tracking tools or custom allocators.
#[derive(Debug, Clone)]
pub struct MemoryMetric {
    /// Initial memory usage (in bytes)
    pub initial_bytes: usize,

    /// Peak memory usage (in bytes)
    pub peak_bytes: usize,

    /// Final memory usage (in bytes)
    pub final_bytes: usize,

    /// Number of allocations
    pub allocations: usize,

    /// Number of deallocations
    pub deallocations: usize,

    /// Whether tracking is active
    active: bool,
}

impl MemoryMetric {
    /// Create a new memory metric
    pub fn new() -> Self {
        MemoryMetric {
            initial_bytes: 0,
            peak_bytes: 0,
            final_bytes: 0,
            allocations: 0,
            deallocations: 0,
            active: false,
        }
    }

    /// Start tracking memory usage
    ///
    /// Note: This is a placeholder. Actual memory tracking requires
    /// platform-specific code or a custom allocator.
    pub fn start(&mut self) {
        self.active = true;
        self.allocations = 0;
        self.deallocations = 0;

        // Placeholder: Would use platform-specific code to get current memory usage
        // For example, on Linux: read from /proc/self/status
        self.initial_bytes = Self::estimate_current_memory();
        self.peak_bytes = self.initial_bytes;
    }

    /// Record an allocation
    pub fn record_allocation(&mut self, bytes: usize) {
        if !self.active {
            return;
        }

        self.allocations += 1;
        let current = self.initial_bytes + bytes;
        self.peak_bytes = self.peak_bytes.max(current);
    }

    /// Record a deallocation
    pub fn record_deallocation(&mut self, _bytes: usize) {
        if !self.active {
            return;
        }

        self.deallocations += 1;
    }

    /// Stop tracking memory usage
    pub fn stop(&mut self) {
        if !self.active {
            return;
        }

        self.active = false;
        self.final_bytes = Self::estimate_current_memory();
    }

    /// Get the net memory change (final - initial)
    pub fn net_change(&self) -> isize {
        self.final_bytes as isize - self.initial_bytes as isize
    }

    /// Get the peak memory increase
    pub fn peak_increase(&self) -> usize {
        self.peak_bytes.saturating_sub(self.initial_bytes)
    }

    /// Estimate current memory usage
    ///
    /// This is a simplified estimation. For accurate measurements,
    /// use platform-specific APIs or a custom allocator.
    fn estimate_current_memory() -> usize {
        // Placeholder: In a real implementation, this would:
        // - On Linux: parse /proc/self/status (VmRSS field)
        // - On macOS: use task_info()
        // - On Windows: use GetProcessMemoryInfo()
        // - Use a custom allocator that tracks allocations

        // For now, return 0 as a placeholder
        0
    }

    /// Generate a report
    pub fn report(&self) -> String {
        format!(
            "Memory Metric:\n\
             Initial:    {} bytes ({:.2} MB)\n\
             Peak:       {} bytes ({:.2} MB)\n\
             Final:      {} bytes ({:.2} MB)\n\
             Net Change: {} bytes ({:.2} MB)\n\
             Peak Inc:   {} bytes ({:.2} MB)\n\
             Allocations: {}\n\
             Deallocations: {}",
            self.initial_bytes,
            self.initial_bytes as f64 / 1024.0 / 1024.0,
            self.peak_bytes,
            self.peak_bytes as f64 / 1024.0 / 1024.0,
            self.final_bytes,
            self.final_bytes as f64 / 1024.0 / 1024.0,
            self.net_change(),
            self.net_change() as f64 / 1024.0 / 1024.0,
            self.peak_increase(),
            self.peak_increase() as f64 / 1024.0 / 1024.0,
            self.allocations,
            self.deallocations,
        )
    }
}

impl Default for MemoryMetric {
    fn default() -> Self {
        Self::new()
    }
}

/// Compression ratio metric - for MERA compression tests
///
/// From MASTER_R&D_ROADMAP.md: "CompressionRatioMetric - for MERA compression tests"
#[derive(Debug, Clone)]
pub struct CompressionRatioMetric {
    /// Original size (in bytes)
    pub original_size: usize,

    /// Compressed size (in bytes)
    pub compressed_size: usize,

    /// Compression time
    pub compression_time: Duration,

    /// Decompression time
    pub decompression_time: Duration,
}

impl CompressionRatioMetric {
    /// Create a new compression ratio metric
    pub fn new() -> Self {
        CompressionRatioMetric {
            original_size: 0,
            compressed_size: 0,
            compression_time: Duration::ZERO,
            decompression_time: Duration::ZERO,
        }
    }

    /// Set the original size
    pub fn set_original_size(&mut self, size: usize) {
        self.original_size = size;
    }

    /// Set the compressed size
    pub fn set_compressed_size(&mut self, size: usize) {
        self.compressed_size = size;
    }

    /// Set the compression time
    pub fn set_compression_time(&mut self, duration: Duration) {
        self.compression_time = duration;
    }

    /// Set the decompression time
    pub fn set_decompression_time(&mut self, duration: Duration) {
        self.decompression_time = duration;
    }

    /// Get the compression ratio (original / compressed)
    pub fn compression_ratio(&self) -> f64 {
        if self.compressed_size > 0 {
            self.original_size as f64 / self.compressed_size as f64
        } else {
            0.0
        }
    }

    /// Get the space savings percentage
    pub fn space_savings(&self) -> f64 {
        if self.original_size > 0 {
            (1.0 - (self.compressed_size as f64 / self.original_size as f64)) * 100.0
        } else {
            0.0
        }
    }

    /// Get the compression speed (bytes/second)
    pub fn compression_speed(&self) -> f64 {
        if self.compression_time.as_secs_f64() > 0.0 {
            self.original_size as f64 / self.compression_time.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get the decompression speed (bytes/second)
    pub fn decompression_speed(&self) -> f64 {
        if self.decompression_time.as_secs_f64() > 0.0 {
            self.original_size as f64 / self.decompression_time.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Check if the compression meets a target ratio
    pub fn meets_target_ratio(&self, target_ratio: f64) -> bool {
        self.compression_ratio() >= target_ratio
    }

    /// Generate a report
    pub fn report(&self) -> String {
        format!(
            "Compression Ratio Metric:\n\
             Original:    {} bytes ({:.2} MB)\n\
             Compressed:  {} bytes ({:.2} MB)\n\
             Ratio:       {:.2}x\n\
             Savings:     {:.2}%\n\
             Comp Time:   {:.3} ms\n\
             Decomp Time: {:.3} ms\n\
             Comp Speed:  {:.2} MB/s\n\
             Decomp Speed: {:.2} MB/s",
            self.original_size,
            self.original_size as f64 / 1024.0 / 1024.0,
            self.compressed_size,
            self.compressed_size as f64 / 1024.0 / 1024.0,
            self.compression_ratio(),
            self.space_savings(),
            self.compression_time.as_secs_f64() * 1000.0,
            self.decompression_time.as_secs_f64() * 1000.0,
            self.compression_speed() / 1024.0 / 1024.0,
            self.decompression_speed() / 1024.0 / 1024.0,
        )
    }
}

impl Default for CompressionRatioMetric {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throughput_metric() {
        let mut metric = ThroughputMetric::new();
        metric.start();

        for _ in 0..100 {
            metric.record_operation();
        }

        metric.stop();

        assert_eq!(metric.operations, 100);
        assert!(metric.throughput() > 0.0);
    }

    #[test]
    fn test_latency_metric() {
        let mut metric = LatencyMetric::new();

        for i in 1..=100 {
            metric.record_latency(Duration::from_millis(i));
        }

        assert_eq!(metric.count(), 100);
        assert!(metric.avg().as_millis() > 0);
        assert!(metric.min <= metric.avg());
        assert!(metric.max >= metric.avg());
    }

    #[test]
    fn test_compression_ratio_metric() {
        let mut metric = CompressionRatioMetric::new();
        metric.set_original_size(1_000_000);
        metric.set_compressed_size(10_000);
        metric.set_compression_time(Duration::from_millis(100));
        metric.set_decompression_time(Duration::from_millis(50));

        assert_eq!(metric.compression_ratio(), 100.0);
        assert_eq!(metric.space_savings(), 99.0);
        assert!(metric.meets_target_ratio(100.0));
    }
}
