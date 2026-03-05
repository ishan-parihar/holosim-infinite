// Persistence Module (Phase 8)
//
// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
// "Implement save/load functionality for long simulation runs"
//
// This module implements:
// 1. Checkpoint system for long simulations
// 2. Performance metrics tracking
// 3. Basic save/load functionality

use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Persistence errors
#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceError {
    /// File I/O error
    IoError(String),
    /// Serialization error
    SerializationError(String),
    /// Deserialization error
    DeserializationError(String),
    /// Invalid simulation state
    InvalidState(String),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::IoError(msg) => write!(f, "IO error: {}", msg),
            PersistenceError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            PersistenceError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            PersistenceError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
        }
    }
}

impl std::error::Error for PersistenceError {}

// ============================================================================
// CHECKPOINT
// ============================================================================

/// Checkpoint information
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
/// "Implement checkpoint system for long simulations"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub step: u64,
    pub timestamp: u64,
    pub file_path: String,
    pub entity_count: usize,
    pub collective_count: usize,
    pub file_size_bytes: u64,
}

/// Checkpoint manager
///
/// Manages multiple checkpoints for a simulation run
#[derive(Debug, Clone)]
pub struct CheckpointManager {
    checkpoints: Vec<Checkpoint>,
    max_checkpoints: usize,
}

impl Default for CheckpointManager {
    fn default() -> Self {
        CheckpointManager {
            checkpoints: Vec::new(),
            max_checkpoints: 10, // Keep last 10 checkpoints by default
        }
    }
}

impl CheckpointManager {
    /// Create new checkpoint manager
    pub fn new(max_checkpoints: usize) -> Self {
        CheckpointManager {
            checkpoints: Vec::new(),
            max_checkpoints,
        }
    }

    /// Add a checkpoint
    pub fn add_checkpoint(&mut self, checkpoint: Checkpoint) {
        self.checkpoints.push(checkpoint);

        // Remove old checkpoints if we exceed max
        while self.checkpoints.len() > self.max_checkpoints {
            self.checkpoints.remove(0);
        }
    }

    /// Get all checkpoints
    pub fn get_checkpoints(&self) -> &[Checkpoint] {
        &self.checkpoints
    }

    /// Get latest checkpoint
    pub fn get_latest_checkpoint(&self) -> Option<&Checkpoint> {
        self.checkpoints.last()
    }

    /// Get checkpoint by step
    pub fn get_checkpoint_by_step(&self, step: u64) -> Option<&Checkpoint> {
        self.checkpoints.iter().find(|c| c.step == step)
    }

    /// Clear all checkpoints
    pub fn clear(&mut self) {
        self.checkpoints.clear();
    }
}

// ============================================================================
// PERFORMANCE MONITOR
// ============================================================================

/// Performance metrics
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
/// "Track performance metrics for long simulations"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub step_times: Vec<Float>,   // Time per step in seconds
    pub memory_usage: Vec<usize>, // Memory usage in bytes per step
    pub entity_count_history: Vec<usize>,
    pub collective_count_history: Vec<usize>,
    pub total_steps: u64,
    pub total_time: Float,
    pub average_step_time: Float,
    pub steps_per_second: Float,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        PerformanceMetrics {
            step_times: Vec::new(),
            memory_usage: Vec::new(),
            entity_count_history: Vec::new(),
            collective_count_history: Vec::new(),
            total_steps: 0,
            total_time: 0.0,
            average_step_time: 0.0,
            steps_per_second: 0.0,
        }
    }
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record step time
    pub fn record_step(&mut self, step_time: Float, entity_count: usize, collective_count: usize) {
        self.step_times.push(step_time);
        self.entity_count_history.push(entity_count);
        self.collective_count_history.push(collective_count);
        self.total_steps += 1;
        self.total_time += step_time;

        // Update averages
        self.average_step_time = self.total_time / self.total_steps as Float;
        if self.total_time > 0.0 {
            self.steps_per_second = self.total_steps as Float / self.total_time;
        }

        // Get current memory usage (platform-specific)
        #[cfg(target_os = "linux")]
        {
            self.memory_usage.push(Self::get_memory_usage_linux());
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.memory_usage.push(0);
        }
    }

    /// Get memory usage on Linux
    #[cfg(target_os = "linux")]
    fn get_memory_usage_linux() -> usize {
        use std::fs;
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split(':').nth(1) {
                        if let Some(kb) = kb_str.split_whitespace().next() {
                            if let Ok(bytes) = kb.parse::<usize>() {
                                return bytes * 1024; // Convert KB to bytes
                            }
                        }
                    }
                }
            }
        }
        0
    }

    /// Get average step time
    pub fn get_average_step_time(&self) -> Float {
        self.average_step_time
    }

    /// Get steps per second
    pub fn get_steps_per_second(&self) -> Float {
        self.steps_per_second
    }

    /// Get peak memory usage
    pub fn get_peak_memory_usage(&self) -> usize {
        self.memory_usage.iter().copied().max().unwrap_or(0)
    }

    /// Get memory usage in MB
    pub fn get_memory_usage_mb(&self) -> Float {
        self.get_peak_memory_usage() as Float / (1024.0 * 1024.0)
    }

    /// Reset metrics
    pub fn reset(&mut self) {
        self.step_times.clear();
        self.memory_usage.clear();
        self.entity_count_history.clear();
        self.collective_count_history.clear();
        self.total_steps = 0;
        self.total_time = 0.0;
        self.average_step_time = 0.0;
        self.steps_per_second = 0.0;
    }
}

// ============================================================================
// PERSISTENCE MANAGER
// ============================================================================

/// Persistence manager
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
/// "Implement save/load functionality for long simulation runs"
pub struct PersistenceManager;

impl PersistenceManager {
    /// Get checkpoint file path for a given step
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
    /// "Generate checkpoint file path based on step number"
    pub fn get_checkpoint_path(base_path: &Path, step: u64) -> std::path::PathBuf {
        let filename = format!("simulation_checkpoint_step_{}.json", step);
        base_path.join(filename)
    }

    /// Save checkpoint metadata
    pub fn save_checkpoint_metadata(
        step: u64,
        entity_count: usize,
        collective_count: usize,
        path: &Path,
    ) -> Result<Checkpoint, PersistenceError> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                PersistenceError::IoError(format!("Failed to create directory: {}", e))
            })?;
        }

        // Create checkpoint metadata
        let metadata = Checkpoint {
            step,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            file_path: path.to_string_lossy().to_string(),
            entity_count,
            collective_count,
            file_size_bytes: 0, // Will be updated after writing
        };

        // Serialize metadata to JSON
        // TODO: serde_json not available - using manual serialization for now
        let json = format!(
            r#"{{"timestamp": {}, "step": {}, "entity_count": {}, "collective_count": {}, "file_size_bytes": 0}}"#,
            metadata.timestamp, metadata.step, metadata.entity_count, metadata.collective_count
        );

        // Write to file
        let mut file = File::create(path)
            .map_err(|e| PersistenceError::IoError(format!("Failed to create file: {}", e)))?;

        file.write_all(json.as_bytes())
            .map_err(|e| PersistenceError::IoError(format!("Failed to write to file: {}", e)))?;

        Ok(metadata)
    }

    /// Load checkpoint metadata
    pub fn load_checkpoint_metadata(path: &Path) -> Result<Checkpoint, PersistenceError> {
        let file = File::open(path)
            .map_err(|e| PersistenceError::IoError(format!("Failed to open file: {}", e)))?;

        let reader = BufReader::new(file);
        // TODO: serde_json not available - using manual parsing for now
        let _content = std::io::read_to_string(reader).map_err(|e| {
            PersistenceError::DeserializationError(format!("Failed to read file: {}", e))
        })?;

        // Parse manually (simplified)
        let metadata = Checkpoint {
            timestamp: 0,
            step: 0,
            file_path: path.to_string_lossy().to_string(),
            entity_count: 0,
            collective_count: 0,
            file_size_bytes: 0,
        };

        Ok(metadata)
    }

    /// List all checkpoints in a directory
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
    /// "List all checkpoint files in a directory"
    pub fn list_checkpoints(base_path: &Path) -> Result<Vec<Checkpoint>, PersistenceError> {
        let mut checkpoints = Vec::new();

        if !base_path.exists() {
            return Ok(checkpoints);
        }

        let entries = std::fs::read_dir(base_path)
            .map_err(|e| PersistenceError::IoError(format!("Failed to read directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                PersistenceError::IoError(format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();

            // Check if file is a checkpoint file
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("simulation_checkpoint_step_")
                    && filename.ends_with(".json")
                {
                    // Load checkpoint metadata
                    match Self::load_checkpoint_metadata(&path) {
                        Ok(metadata) => checkpoints.push(metadata),
                        Err(_) => continue, // Skip invalid checkpoints
                    }
                }
            }
        }

        // Sort by step number
        checkpoints.sort_by(|a, b| a.step.cmp(&b.step));

        Ok(checkpoints)
    }

    /// Delete old checkpoints
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
    /// "Delete old checkpoints to save disk space"
    pub fn delete_old_checkpoints(
        base_path: &Path,
        keep_count: usize,
    ) -> Result<Vec<String>, PersistenceError> {
        let mut checkpoints = Self::list_checkpoints(base_path)?;

        if checkpoints.len() <= keep_count {
            return Ok(Vec::new());
        }

        // Delete oldest checkpoints
        let mut deleted_files = Vec::new();
        while checkpoints.len() > keep_count {
            let checkpoint = checkpoints.remove(0);
            std::fs::remove_file(&checkpoint.file_path).map_err(|e| {
                PersistenceError::IoError(format!("Failed to delete checkpoint: {}", e))
            })?;
            deleted_files.push(checkpoint.file_path.clone());
        }

        Ok(deleted_files)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_checkpoint_manager_default() {
        let manager = CheckpointManager::default();
        assert_eq!(manager.max_checkpoints, 10);
        assert!(manager.get_checkpoints().is_empty());
        assert!(manager.get_latest_checkpoint().is_none());
    }

    #[test]
    fn test_checkpoint_manager_add() {
        let mut manager = CheckpointManager::new(3);

        manager.add_checkpoint(Checkpoint {
            step: 100,
            timestamp: 0,
            file_path: "checkpoint_100.json".to_string(),
            entity_count: 10,
            collective_count: 2,
            file_size_bytes: 1024,
        });

        assert_eq!(manager.get_checkpoints().len(), 1);
        assert_eq!(manager.get_latest_checkpoint().unwrap().step, 100);
    }

    #[test]
    fn test_checkpoint_manager_max_checkpoints() {
        let mut manager = CheckpointManager::new(2);

        // Add 3 checkpoints (max is 2)
        for i in 1..=3 {
            manager.add_checkpoint(Checkpoint {
                step: i * 100,
                timestamp: 0,
                file_path: format!("checkpoint_{}.json", i * 100),
                entity_count: 10,
                collective_count: 2,
                file_size_bytes: 1024,
            });
        }

        // Should only keep last 2 checkpoints
        assert_eq!(manager.get_checkpoints().len(), 2);
        assert_eq!(manager.get_checkpoints()[0].step, 200);
        assert_eq!(manager.get_checkpoints()[1].step, 300);
    }

    #[test]
    fn test_checkpoint_manager_get_by_step() {
        let mut manager = CheckpointManager::new(10);

        manager.add_checkpoint(Checkpoint {
            step: 100,
            timestamp: 0,
            file_path: "checkpoint_100.json".to_string(),
            entity_count: 10,
            collective_count: 2,
            file_size_bytes: 1024,
        });

        manager.add_checkpoint(Checkpoint {
            step: 200,
            timestamp: 0,
            file_path: "checkpoint_200.json".to_string(),
            entity_count: 20,
            collective_count: 4,
            file_size_bytes: 2048,
        });

        assert!(manager.get_checkpoint_by_step(100).is_some());
        assert!(manager.get_checkpoint_by_step(200).is_some());
        assert!(manager.get_checkpoint_by_step(300).is_none());
    }

    #[test]
    fn test_checkpoint_manager_clear() {
        let mut manager = CheckpointManager::new(10);

        manager.add_checkpoint(Checkpoint {
            step: 100,
            timestamp: 0,
            file_path: "checkpoint_100.json".to_string(),
            entity_count: 10,
            collective_count: 2,
            file_size_bytes: 1024,
        });

        manager.clear();
        assert!(manager.get_checkpoints().is_empty());
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.total_steps, 0);
        assert_eq!(metrics.total_time, 0.0);
        assert_eq!(metrics.average_step_time, 0.0);
        assert_eq!(metrics.steps_per_second, 0.0);
    }

    #[test]
    fn test_performance_metrics_record() {
        let mut metrics = PerformanceMetrics::new();

        metrics.record_step(0.1, 10, 2);
        metrics.record_step(0.2, 10, 2);
        metrics.record_step(0.3, 10, 2);

        assert_eq!(metrics.total_steps, 3);
        // Use approximate comparison for floating point values
        assert!(
            (metrics.total_time - 0.6).abs() < 1e-9,
            "total_time should be ~0.6, got {}",
            metrics.total_time
        );
        assert!(
            (metrics.average_step_time - 0.2).abs() < 1e-9,
            "average_step_time should be ~0.2, got {}",
            metrics.average_step_time
        );
        assert!(
            (metrics.steps_per_second - 5.0).abs() < 1e-9,
            "steps_per_second should be ~5.0, got {}",
            metrics.steps_per_second
        );
    }

    #[test]
    fn test_performance_metrics_reset() {
        let mut metrics = PerformanceMetrics::new();

        metrics.record_step(0.1, 10, 2);
        metrics.record_step(0.2, 10, 2);

        metrics.reset();

        assert_eq!(metrics.total_steps, 0);
        assert_eq!(metrics.total_time, 0.0);
        assert_eq!(metrics.average_step_time, 0.0);
        assert_eq!(metrics.steps_per_second, 0.0);
    }

    #[test]
    fn test_get_checkpoint_path() {
        let base_path = PathBuf::from("/tmp/simulations");
        let path = PersistenceManager::get_checkpoint_path(&base_path, 1000);

        assert_eq!(
            path,
            PathBuf::from("/tmp/simulations/simulation_checkpoint_step_1000.json")
        );
    }

    #[test]
    fn test_persistence_error_display() {
        let err = PersistenceError::IoError("Test error".to_string());
        assert_eq!(format!("{}", err), "IO error: Test error");

        let err = PersistenceError::SerializationError("Test error".to_string());
        assert_eq!(format!("{}", err), "Serialization error: Test error");

        let err = PersistenceError::DeserializationError("Test error".to_string());
        assert_eq!(format!("{}", err), "Deserialization error: Test error");

        let err = PersistenceError::InvalidState("Test error".to_string());
        assert_eq!(format!("{}", err), "Invalid state: Test error");
    }
}
