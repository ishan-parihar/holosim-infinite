//! ShaderComposer — Compile-time WGSL shader composition for wgpu 0.19.
//!
//! wgpu 0.19 does not support `#include` natively. This module embeds all
//! WGSL source at compile time via `include_str!` and resolves `#include`
//! directives at runtime by simple text substitution.
//!
//! # Usage
//!
//! ```ignore
//! let composer = ShaderComposer::new();
//! let source = composer.compose("connection_metrics");
//! // `source` is the complete WGSL with all #includes inlined.
//! ```
//!
//! # Include Syntax
//!
//! Inside WGSL files, use:
//! ```wgsl
//! #include "common/types.wgsl"
//! #include "common/math.wgsl"
//! ```
//!
//! Paths are relative to the `shaders/` directory.

use std::collections::HashMap;

// ============================================================================
// Compile-time embedded shader sources
// ============================================================================

const COMMON_TYPES: &str = include_str!("../../shaders/common/types.wgsl");
const COMMON_MATH: &str = include_str!("../../shaders/common/math.wgsl");

const HOLOGRAPHIC_CONNECTION_METRICS: &str =
    include_str!("../../shaders/holographic/connection_metrics.wgsl");
const HOLOGRAPHIC_RESONANCE_MATRIX: &str =
    include_str!("../../shaders/holographic/resonance_matrix.wgsl");

const MERA_MATMUL: &str = include_str!("../../shaders/mera/matmul.wgsl");
const MERA_WAVELET: &str = include_str!("../../shaders/mera/wavelet.wgsl");
const MERA_UPSAMPLE: &str = include_str!("../../shaders/mera/upsample.wgsl");

// ============================================================================
// ShaderComposer
// ============================================================================

/// Resolves `#include` directives in WGSL shaders and returns composed source.
pub struct ShaderComposer {
    /// Map from include path (e.g. "common/types.wgsl") to source text.
    sources: HashMap<&'static str, &'static str>,
}

impl ShaderComposer {
    /// Create a new ShaderComposer with all embedded shader sources.
    pub fn new() -> Self {
        let mut sources = HashMap::new();
        sources.insert("common/types.wgsl", COMMON_TYPES);
        sources.insert("common/math.wgsl", COMMON_MATH);
        sources.insert(
            "holographic/connection_metrics.wgsl",
            HOLOGRAPHIC_CONNECTION_METRICS,
        );
        sources.insert(
            "holographic/resonance_matrix.wgsl",
            HOLOGRAPHIC_RESONANCE_MATRIX,
        );
        sources.insert("mera/matmul.wgsl", MERA_MATMUL);
        sources.insert("mera/wavelet.wgsl", MERA_WAVELET);
        sources.insert("mera/upsample.wgsl", MERA_UPSAMPLE);
        Self { sources }
    }

    /// Compose a shader by resolving all `#include` directives.
    ///
    /// `entry_point` is the shader name without extension (e.g. "connection_metrics").
    /// Returns the fully inlined WGSL source ready for `wgpu::Device::create_shader_module`.
    ///
    /// # Panics
    ///
    /// Panics if the entry point is not found or if an included file is not found.
    pub fn compose(&self, entry_point: &str) -> String {
        // Map entry point to full shader path
        let shader_path = if entry_point.contains('/') {
            format!("{entry_point}.wgsl")
        } else {
            // Try to find the shader by searching all sources for a filename match
            let key = self
                .sources
                .keys()
                .find(|k| k.ends_with(&format!("{entry_point}.wgsl")));
            match key {
                Some(k) => k.to_string(),
                None => panic!("Shader entry point not found: {entry_point}"),
            }
        };

        let raw = self
            .sources
            .get(shader_path.as_str())
            .unwrap_or_else(|| panic!("Shader not found in embedded sources: {shader_path}"));

        self.resolve_includes(raw, &mut std::collections::HashSet::new())
    }

    /// Recursively resolve `#include` directives in shader source.
    ///
    /// Uses a `visited` set to prevent infinite recursion from circular includes.
    fn resolve_includes(
        &self,
        source: &str,
        visited: &mut std::collections::HashSet<String>,
    ) -> String {
        let mut result = String::with_capacity(source.len() * 2);

        for line in source.lines() {
            if let Some(include_path) = Self::parse_include(line) {
                // Prevent circular includes
                if visited.contains(&include_path) {
                    continue;
                }
                visited.insert(include_path.clone());

                let included = self
                    .sources
                    .get(include_path.as_str())
                    .unwrap_or_else(|| panic!("#include not found: \"{include_path}\""));

                // Recursively resolve includes in the included file
                let resolved = self.resolve_includes(included, visited);
                result.push_str(&resolved);
                result.push('\n');
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        result
    }

    /// Parse a `#include "path"` directive from a line.
    /// Returns `Some(path)` if the line is an include directive, `None` otherwise.
    fn parse_include(line: &str) -> Option<String> {
        let trimmed = line.trim();
        if !trimmed.starts_with("#include") {
            return None;
        }

        // Extract the quoted path: #include "path/to/file.wgsl"
        let start = trimmed.find('"')?;
        let end = trimmed.rfind('"')?;
        if start >= end {
            return None;
        }

        Some(trimmed[start + 1..end].to_string())
    }
}

impl Default for ShaderComposer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_include_valid() {
        assert_eq!(
            ShaderComposer::parse_include("#include \"common/types.wgsl\""),
            Some("common/types.wgsl".to_string())
        );
    }

    #[test]
    fn test_parse_include_with_whitespace() {
        assert_eq!(
            ShaderComposer::parse_include("  #include \"common/math.wgsl\"  "),
            Some("common/math.wgsl".to_string())
        );
    }

    #[test]
    fn test_parse_include_not_directive() {
        assert_eq!(ShaderComposer::parse_include("// #include \"fake\""), None);
    }

    #[test]
    fn test_parse_include_not_line() {
        assert_eq!(ShaderComposer::parse_include("fn main() {}"), None);
    }

    #[test]
    fn test_compose_resolves_includes() {
        let composer = ShaderComposer::new();

        // Compose a shader that has #include directives
        let source = composer.compose("connection_metrics");

        // The composed source should contain the content from common/types.wgsl
        assert!(source.contains("struct EntityGPUData"));
        assert!(source.contains("struct ConnectionOutput"));
        assert!(source.contains("struct ResonanceOutput"));
        assert!(source.contains("struct DispatchParams"));

        // Should contain content from common/math.wgsl
        assert!(source.contains("fn clamp01"));
        assert!(source.contains("fn cosine_similarity"));
        assert!(source.contains("fn dot_product_6"));
        assert!(source.contains("fn harmonic_match"));

        // Should NOT contain unresolved #include directives
        assert!(!source.contains("#include"));
    }

    #[test]
    fn test_compose_resonance_matrix() {
        let composer = ShaderComposer::new();
        let source = composer.compose("resonance_matrix");

        assert!(source.contains("struct EntityGPUData"));
        assert!(source.contains("struct ResonanceOutput"));
        assert!(source.contains("fn clamp01"));
        assert!(!source.contains("#include"));
    }

    #[test]
    fn test_compose_mera_shaders() {
        let composer = ShaderComposer::new();

        for shader in &["matmul", "wavelet", "upsample"] {
            let source = composer.compose(shader);
            assert!(
                source.contains("struct EntityGPUData"),
                "{shader} should include types.wgsl"
            );
            assert!(
                source.contains("fn clamp01"),
                "{shader} should include math.wgsl"
            );
            assert!(
                !source.contains("#include"),
                "{shader} should have no unresolved includes"
            );
        }
    }

    #[test]
    #[should_panic(expected = "Shader entry point not found")]
    fn test_compose_unknown_shader_panics() {
        let composer = ShaderComposer::new();
        composer.compose("nonexistent_shader");
    }

    #[test]
    #[should_panic(expected = "#include not found")]
    fn test_compose_invalid_include_panics() {
        let composer = ShaderComposer::new();

        // Manually test with a source that has a bad include
        let bad_source = "#include \"nonexistent/file.wgsl\"\nfn main() {}";
        composer.resolve_includes(bad_source, &mut std::collections::HashSet::new());
    }

    #[test]
    fn test_common_types_embedded() {
        let composer = ShaderComposer::new();
        let types = composer.sources.get("common/types.wgsl").unwrap();

        // Verify the embedded source has the expected struct definitions
        assert!(types.contains("struct EntityGPUData"));
        assert!(types.contains("struct ConnectionOutput"));
        assert!(types.contains("struct ResonanceOutput"));
        assert!(types.contains("struct DispatchParams"));
        assert!(types.contains("entity_id_hash: u32"));
        assert!(types.contains("density_level: f32"));
    }

    #[test]
    fn test_common_math_embedded() {
        let composer = ShaderComposer::new();
        let math = composer.sources.get("common/math.wgsl").unwrap();

        assert!(math.contains("fn clamp01"));
        assert!(math.contains("fn cosine_similarity"));
        assert!(math.contains("fn dot_product_6"));
        assert!(math.contains("fn harmonic_match"));
    }

    #[test]
    fn test_circular_include_prevention() {
        let composer = ShaderComposer::new();

        // Create a mock circular include scenario
        let mut visited = std::collections::HashSet::new();
        visited.insert("common/types.wgsl".to_string());

        // Resolving types.wgsl when it's already visited should skip it
        let result =
            composer.resolve_includes("#include \"common/types.wgsl\"\nfn main() {}", &mut visited);

        // The include should have been skipped (not duplicated)
        assert!(!result.contains("struct EntityGPUData"));
        assert!(result.contains("fn main()"));
    }

    #[test]
    fn test_shader_contains_entry_point() {
        let composer = ShaderComposer::new();

        // Each shader should declare its compute entry point
        let conn = composer.compose("connection_metrics");
        assert!(conn.contains("@compute @workgroup_size(16, 16, 1)"));
        assert!(conn.contains("fn main_connection_metrics"));

        let res = composer.compose("resonance_matrix");
        assert!(res.contains("@compute @workgroup_size(16, 16, 1)"));
        assert!(res.contains("fn main_resonance_matrix"));

        let matmul = composer.compose("matmul");
        assert!(matmul.contains("@compute @workgroup_size(16, 16, 1)"));
        assert!(matmul.contains("fn main_matmul"));

        let wavelet = composer.compose("wavelet");
        assert!(wavelet.contains("@compute @workgroup_size(16, 16, 1)"));
        assert!(wavelet.contains("fn main_wavelet"));

        let upsample = composer.compose("upsample");
        assert!(upsample.contains("@compute @workgroup_size(16, 16, 1)"));
        assert!(upsample.contains("fn main_upsample"));
    }
}
