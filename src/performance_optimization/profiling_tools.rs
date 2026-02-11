// ============================================================================
// PROFILING TOOLS
// ============================================================================
// Objective: Provide tools for profiling and analyzing performance.
//
// Tasks:
// - Create profiler for tracking execution time
// - Create hot spot analysis tools
// - Generate profiling reports
// - Identify performance bottlenecks
// ============================================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ============================================================================
// PROFILING STATS
// ============================================================================

/// Statistics for a profiling session
#[derive(Debug, Clone, Default)]
pub struct ProfilingStats {
    /// Name of the profiled function/section
    pub name: String,
    /// Total duration
    pub total_duration: Duration,
    /// Number of calls
    pub call_count: u64,
    /// Average duration per call
    pub avg_duration: Duration,
    /// Minimum duration
    pub min_duration: Duration,
    /// Maximum duration
    pub max_duration: Duration,
    /// Percentage of total time
    pub total_time_percent: f64,
}

impl ProfilingStats {
    /// Update statistics with a new duration
    pub fn update(&mut self, duration: Duration) {
        self.call_count += 1;
        self.total_duration += duration;

        if self.call_count == 1 {
            self.min_duration = duration;
            self.max_duration = duration;
        } else {
            if duration < self.min_duration {
                self.min_duration = duration;
            }
            if duration > self.max_duration {
                self.max_duration = duration;
            }
        }

        self.avg_duration = self.total_duration / self.call_count as u32;
    }

    /// Calculate percentage of total time
    pub fn calculate_total_time_percent(&mut self, total_time: Duration) {
        if total_time.as_nanos() > 0 {
            self.total_time_percent =
                (self.total_duration.as_nanos() as f64 / total_time.as_nanos() as f64) * 100.0;
        }
    }

    /// Get total duration in nanoseconds
    pub fn total_duration_ns(&self) -> u64 {
        self.total_duration.as_nanos() as u64
    }

    /// Get average duration in nanoseconds
    pub fn avg_duration_ns(&self) -> u64 {
        self.avg_duration.as_nanos() as u64
    }
}

// ============================================================================
// PROFILING SESSION
// ============================================================================

/// A profiling session for tracking execution time
#[derive(Debug, Clone)]
pub struct ProfilingSession {
    /// Name of the session
    pub name: String,
    /// Statistics for each profiled function/section
    pub stats: HashMap<String, ProfilingStats>,
    /// Start time of the session
    pub start_time: Instant,
    /// End time of the session
    pub end_time: Option<Instant>,
    /// Total duration of the session
    pub total_duration: Duration,
}

impl ProfilingSession {
    /// Create a new profiling session
    pub fn new(name: String) -> Self {
        Self {
            name,
            stats: HashMap::new(),
            start_time: Instant::now(),
            end_time: None,
            total_duration: Duration::from_nanos(0),
        }
    }

    /// Start profiling a function/section
    pub fn start(&mut self, name: String) -> ProfilingGuard<'_> {
        ProfilingGuard::new(self, name)
    }

    /// Record a duration for a function/section
    pub fn record(&mut self, name: String, duration: Duration) {
        let stats = self
            .stats
            .entry(name.clone())
            .or_insert_with(|| ProfilingStats {
                name,
                ..Default::default()
            });
        stats.update(duration);
    }

    /// End the profiling session
    pub fn end(&mut self) {
        self.end_time = Some(Instant::now());
        self.total_duration = self.start_time.elapsed();

        // Calculate total time percentages
        for stats in self.stats.values_mut() {
            stats.calculate_total_time_percent(self.total_duration);
        }
    }

    /// Get statistics for a specific function/section
    pub fn get_stats(&self, name: &str) -> Option<&ProfilingStats> {
        self.stats.get(name)
    }

    /// Get all statistics
    pub fn get_all_stats(&self) -> &HashMap<String, ProfilingStats> {
        &self.stats
    }

    /// Generate a report
    pub fn generate_report(&self) -> String {
        let mut report = format!(
            "Profiling Session: {}\n\
             Total Duration: {:.2} ms\n\
             Profiled Functions: {}\n\n",
            self.name,
            self.total_duration.as_nanos() as f64 / 1_000_000.0,
            self.stats.len()
        );

        // Sort by total duration (descending)
        let mut sorted_stats: Vec<_> = self.stats.values().collect();
        sorted_stats.sort_by(|a, b| b.total_duration.cmp(&a.total_duration));

        for stats in &sorted_stats {
            report.push_str(&format!(
                "  - {}: {:.2} ms ({} calls, avg: {:.2} μs, {:.1}% of total)\n",
                stats.name,
                stats.total_duration.as_nanos() as f64 / 1_000_000.0,
                stats.call_count,
                stats.avg_duration.as_nanos() as f64 / 1000.0,
                stats.total_time_percent
            ));
        }

        report
    }
}

// ============================================================================
// PROFILING GUARD
// ============================================================================

/// Guard for automatic profiling of a scope
pub struct ProfilingGuard<'a> {
    /// Reference to the profiling session
    session: &'a mut ProfilingSession,
    /// Name of the profiled function/section
    name: String,
    /// Start time
    start_time: Instant,
}

impl<'a> ProfilingGuard<'a> {
    /// Create a new profiling guard
    pub fn new(session: &'a mut ProfilingSession, name: String) -> Self {
        Self {
            session,
            name,
            start_time: Instant::now(),
        }
    }
}

impl<'a> Drop for ProfilingGuard<'a> {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        self.session.record(self.name.clone(), duration);
    }
}

// ============================================================================
// HOT SPOT ANALYSIS
// ============================================================================

/// Analysis of performance hot spots
#[derive(Debug, Clone)]
pub struct HotSpotAnalysis {
    /// Hot spots sorted by total duration
    pub hot_spots: Vec<HotSpot>,
    /// Total profiled time
    pub total_time: Duration,
    /// Number of hot spots identified
    pub hot_spot_count: usize,
}

/// A hot spot in the code
#[derive(Debug, Clone)]
pub struct HotSpot {
    /// Name of the hot spot
    pub name: String,
    /// Total duration
    pub total_duration: Duration,
    /// Number of calls
    pub call_count: u64,
    /// Average duration per call
    pub avg_duration: Duration,
    /// Percentage of total time
    pub total_time_percent: f64,
    /// Severity level
    pub severity: HotSpotSeverity,
}

/// Severity level of a hot spot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotSpotSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl HotSpotAnalysis {
    /// Create a new hot spot analysis from profiling stats
    pub fn from_stats(stats: &HashMap<String, ProfilingStats>, total_time: Duration) -> Self {
        let mut hot_spots: Vec<HotSpot> = stats
            .values()
            .map(|s| {
                let severity = if s.total_time_percent > 50.0 {
                    HotSpotSeverity::Critical
                } else if s.total_time_percent > 20.0 {
                    HotSpotSeverity::High
                } else if s.total_time_percent > 10.0 {
                    HotSpotSeverity::Medium
                } else {
                    HotSpotSeverity::Low
                };

                HotSpot {
                    name: s.name.clone(),
                    total_duration: s.total_duration,
                    call_count: s.call_count,
                    avg_duration: s.avg_duration,
                    total_time_percent: s.total_time_percent,
                    severity,
                }
            })
            .collect();

        // Sort by total duration (descending)
        hot_spots.sort_by(|a, b| b.total_duration.cmp(&a.total_duration));

        let hot_spot_count = hot_spots.len();

        Self {
            hot_spots,
            total_time,
            hot_spot_count,
        }
    }

    /// Get critical hot spots
    pub fn get_critical_hot_spots(&self) -> Vec<&HotSpot> {
        self.hot_spots
            .iter()
            .filter(|hs| hs.severity == HotSpotSeverity::Critical)
            .collect()
    }

    /// Get high severity hot spots
    pub fn get_high_severity_hot_spots(&self) -> Vec<&HotSpot> {
        self.hot_spots
            .iter()
            .filter(|hs| hs.severity == HotSpotSeverity::High)
            .collect()
    }

    /// Get optimization recommendations
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        for hot_spot in &self.hot_spots {
            match hot_spot.severity {
                HotSpotSeverity::Critical => {
                    recommendations.push(format!(
                        "CRITICAL: '{}' takes {:.1}% of total time. Immediate optimization required.",
                        hot_spot.name, hot_spot.total_time_percent
                    ));
                }
                HotSpotSeverity::High => {
                    recommendations.push(format!(
                        "HIGH: '{}' takes {:.1}% of total time. Optimization recommended.",
                        hot_spot.name, hot_spot.total_time_percent
                    ));
                }
                HotSpotSeverity::Medium => {
                    recommendations.push(format!(
                        "MEDIUM: '{}' takes {:.1}% of total time. Consider optimization.",
                        hot_spot.name, hot_spot.total_time_percent
                    ));
                }
                HotSpotSeverity::Low => {
                    // No recommendation for low severity
                }
            }
        }

        recommendations
    }

    /// Generate a report
    pub fn generate_report(&self) -> String {
        let mut report = format!(
            "Hot Spot Analysis\n\
             Total Time: {:.2} ms\n\
             Hot Spots Identified: {}\n\n",
            self.total_time.as_nanos() as f64 / 1_000_000.0,
            self.hot_spot_count
        );

        for (i, hot_spot) in self.hot_spots.iter().enumerate() {
            let severity_str = match hot_spot.severity {
                HotSpotSeverity::Critical => "CRITICAL",
                HotSpotSeverity::High => "HIGH",
                HotSpotSeverity::Medium => "MEDIUM",
                HotSpotSeverity::Low => "LOW",
            };

            report.push_str(&format!(
                "  {}. [{}] {}: {:.2} ms ({} calls, avg: {:.2} μs, {:.1}% of total)\n",
                i + 1,
                severity_str,
                hot_spot.name,
                hot_spot.total_duration.as_nanos() as f64 / 1_000_000.0,
                hot_spot.call_count,
                hot_spot.avg_duration.as_nanos() as f64 / 1000.0,
                hot_spot.total_time_percent
            ));
        }

        // Add recommendations
        let recommendations = self.get_recommendations();
        if !recommendations.is_empty() {
            report.push_str("\nRecommendations:\n");
            for rec in &recommendations {
                report.push_str(&format!("  - {}\n", rec));
            }
        }

        report
    }
}

// ============================================================================
// PROFILER
// ============================================================================

/// Global profiler for tracking performance
pub struct Profiler {
    /// Current profiling session
    session: Option<ProfilingSession>,
    /// Historical profiling sessions
    history: Vec<ProfilingSession>,
}

impl Profiler {
    /// Create a new profiler
    pub fn new() -> Self {
        Self {
            session: None,
            history: Vec::new(),
        }
    }

    /// Start a new profiling session
    pub fn start_session(&mut self, name: String) {
        // End current session if exists
        if let Some(mut session) = self.session.take() {
            session.end();
            self.history.push(session);
        }

        self.session = Some(ProfilingSession::new(name));
    }

    /// End the current profiling session
    pub fn end_session(&mut self) -> Option<ProfilingSession> {
        if let Some(mut session) = self.session.take() {
            session.end();
            self.history.push(session.clone());
            Some(session)
        } else {
            None
        }
    }

    /// Profile a function/section
    pub fn profile(&mut self, name: String) -> Option<ProfilingGuard<'_>> {
        self.session.as_mut().map(|session| session.start(name))
    }

    /// Get the current session
    pub fn current_session(&self) -> Option<&ProfilingSession> {
        self.session.as_ref()
    }

    /// Get historical sessions
    pub fn history(&self) -> &[ProfilingSession] {
        &self.history
    }

    /// Perform hot spot analysis on the current session
    pub fn analyze_hot_spots(&self) -> Option<HotSpotAnalysis> {
        self.session.as_ref().map(|session| {
            HotSpotAnalysis::from_stats(session.get_all_stats(), session.total_duration)
        })
    }

    /// Generate a report for the current session
    pub fn generate_report(&self) -> Option<String> {
        self.session
            .as_ref()
            .map(|session| session.generate_report())
    }

    /// Generate a hot spot analysis report
    pub fn generate_hot_spot_report(&self) -> Option<String> {
        self.analyze_hot_spots()
            .map(|analysis| analysis.generate_report())
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// GLOBAL PROFILER INSTANCE
// ============================================================================

/// Global profiler instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_PROFILER: Arc<Mutex<Profiler>> = Arc::new(Mutex::new(Profiler::new()));
}

/// Convenience function to start a profiling session
pub fn start_profiling_session(name: &str) {
    if let Ok(mut profiler) = GLOBAL_PROFILER.lock() {
        profiler.start_session(name.to_string());
    }
}

/// Convenience function to end a profiling session
pub fn end_profiling_session() -> Option<ProfilingSession> {
    if let Ok(mut profiler) = GLOBAL_PROFILER.lock() {
        profiler.end_session()
    } else {
        None
    }
}

/// Convenience function to profile a function/section
pub fn profile_function(_name: &str) -> Option<ProfilingGuard<'static>> {
    // This is a simplified version - in practice, you'd need more complex lifetime management
    None
}

/// Convenience function to generate a profiling report
pub fn generate_profiling_report() -> Option<String> {
    if let Ok(profiler) = GLOBAL_PROFILER.lock() {
        profiler.generate_report()
    } else {
        None
    }
}

/// Convenience function to generate a hot spot analysis report
pub fn generate_hot_spot_report() -> Option<String> {
    if let Ok(profiler) = GLOBAL_PROFILER.lock() {
        profiler.generate_hot_spot_report()
    } else {
        None
    }
}

// ============================================================================
// PROFILING MACROS
// ============================================================================

/// Macro to profile a block of code
#[macro_export]
macro_rules! profile_block {
    ($name:expr, $block:expr) => {{
        if let Ok(mut profiler) =
            $crate::performance_optimization::profiling_tools::GLOBAL_PROFILER.lock()
        {
            if let Some(mut guard) = profiler.profile($name.to_string()) {
                let result = $block;
                drop(guard);
                result
            } else {
                $block
            }
        } else {
            $block
        }
    }};
}

/// Macro to profile a function
#[macro_export]
macro_rules! profile_function {
    ($name:expr, $block:expr) => {{
        profile_block!($name, $block)
    }};
}

// ============================================================================
// PROFILING TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiling_session_basic() {
        let mut session = ProfilingSession::new("test_session".to_string());

        {
            let _guard = session.start("test_function".to_string());
            std::thread::sleep(Duration::from_millis(10));
        }

        session.end();

        assert!(session.total_duration.as_millis() >= 10);
        assert!(session.stats.contains_key("test_function"));
    }

    #[test]
    fn test_profiling_session_multiple_calls() {
        let mut session = ProfilingSession::new("test_session".to_string());

        for _ in 0..5 {
            let _guard = session.start("test_function".to_string());
            std::thread::sleep(Duration::from_millis(10));
        }

        session.end();

        let stats = session.get_stats("test_function").unwrap();
        assert_eq!(stats.call_count, 5);
    }

    #[test]
    fn test_profiling_guard() {
        let mut session = ProfilingSession::new("test_session".to_string());

        {
            let _guard = session.start("test_function".to_string());
            std::thread::sleep(Duration::from_millis(10));
        }

        let stats = session.get_stats("test_function").unwrap();
        assert!(stats.total_duration.as_millis() >= 10);
    }

    #[test]
    fn test_hot_spot_analysis() {
        let mut stats = HashMap::new();

        let mut stat1 = ProfilingStats {
            name: "function1".to_string(),
            total_duration: Duration::from_nanos(500),
            call_count: 10,
            avg_duration: Duration::from_nanos(50),
            min_duration: Duration::from_nanos(40),
            max_duration: Duration::from_nanos(60),
            total_time_percent: 0.0,
        };

        let mut stat2 = ProfilingStats {
            name: "function2".to_string(),
            total_duration: Duration::from_nanos(300),
            call_count: 5,
            avg_duration: Duration::from_nanos(60),
            min_duration: Duration::from_nanos(50),
            max_duration: Duration::from_nanos(70),
            total_time_percent: 0.0,
        };

        let total_time = Duration::from_nanos(1000);

        stat1.calculate_total_time_percent(total_time);
        stat2.calculate_total_time_percent(total_time);

        stats.insert("function1".to_string(), stat1);
        stats.insert("function2".to_string(), stat2);

        let analysis = HotSpotAnalysis::from_stats(&stats, total_time);
        assert_eq!(analysis.hot_spot_count, 2);
        assert_eq!(analysis.hot_spots[0].name, "function1");
    }

    #[test]
    fn test_hot_spot_severity() {
        let mut stats = HashMap::new();

        let mut stat = ProfilingStats {
            name: "critical_function".to_string(),
            total_duration: Duration::from_nanos(600),
            call_count: 10,
            avg_duration: Duration::from_nanos(60),
            min_duration: Duration::from_nanos(50),
            max_duration: Duration::from_nanos(70),
            total_time_percent: 0.0,
        };

        let total_time = Duration::from_nanos(1000);

        stat.calculate_total_time_percent(total_time);

        stats.insert("critical_function".to_string(), stat);

        let analysis = HotSpotAnalysis::from_stats(&stats, total_time);
        assert_eq!(analysis.hot_spots[0].severity, HotSpotSeverity::Critical);
    }

    #[test]
    fn test_profiler_basic() {
        let mut profiler = Profiler::new();

        profiler.start_session("test_session".to_string());

        {
            let _guard = profiler.profile("test_function".to_string());
            std::thread::sleep(Duration::from_millis(10));
        }

        profiler.end_session();

        assert_eq!(profiler.history().len(), 1);
    }

    #[test]
    fn test_profiler_hot_spot_analysis() {
        let mut profiler = Profiler::new();

        profiler.start_session("test_session".to_string());

        {
            let _guard = profiler.profile("test_function".to_string());
            std::thread::sleep(Duration::from_millis(10));
        }

        profiler.end_session();

        let analysis = profiler.analyze_hot_spots();
        assert!(analysis.is_some());
    }

    #[test]
    fn test_profiling_stats_update() {
        let mut stats = ProfilingStats {
            name: "test".to_string(),
            ..Default::default()
        };

        stats.update(Duration::from_nanos(100));
        stats.update(Duration::from_nanos(200));

        assert_eq!(stats.call_count, 2);
        assert_eq!(stats.total_duration_ns(), 300);
        assert_eq!(stats.avg_duration_ns(), 150);
    }
}
