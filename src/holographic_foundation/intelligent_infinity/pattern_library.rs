//! Pattern Library - Field Templates for Successful Emergence
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Pattern Library = Field templates for successful emergence"
//!
//! # Theory
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The pattern library contains templates for all successful emergence patterns.
//!  These templates are learned from entity experiences and can be applied
//!  to guide new emergence events."
//!
//! # Pattern Categories
//!
//! 1. **Atomic Templates**: Patterns for stable atom formation
//! 2. **Molecular Templates**: Patterns for molecular bonding
//! 3. **Cellular Templates**: Patterns for cell manifestation
//! 4. **Organism Templates**: Patterns for organism development
//! 5. **Collective Templates**: Patterns for collective formation
//! 6. **Density Templates**: Patterns for density transitions
//!
//! # Usage
//!
//! Entities can request patterns from the library to guide their evolution.
//! The library matches entity state with appropriate templates and returns
//! the most resonant patterns.

use crate::types::Float;
use std::collections::HashMap;

/// Pattern Category - categories of emergence patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternCategory {
    Atomic,
    Molecular,
    Cellular,
    Organism,
    Collective,
    Density,
    Consciousness,
    Environment,
    Gateway,
    Source,
}

impl PatternCategory {
    pub fn description(&self) -> &'static str {
        match self {
            PatternCategory::Atomic => "Atomic emergence patterns",
            PatternCategory::Molecular => "Molecular bonding patterns",
            PatternCategory::Cellular => "Cellular manifestation patterns",
            PatternCategory::Organism => "Organism development patterns",
            PatternCategory::Collective => "Collective formation patterns",
            PatternCategory::Density => "Density transition patterns",
            PatternCategory::Consciousness => "Consciousness evolution patterns",
            PatternCategory::Environment => "Environmental field patterns",
            PatternCategory::Gateway => "Gateway access patterns",
            PatternCategory::Source => "Source connection patterns",
        }
    }

    pub fn base_resonance(&self) -> Float {
        match self {
            PatternCategory::Atomic => 0.3,
            PatternCategory::Molecular => 0.35,
            PatternCategory::Cellular => 0.4,
            PatternCategory::Organism => 0.5,
            PatternCategory::Collective => 0.6,
            PatternCategory::Density => 0.7,
            PatternCategory::Consciousness => 0.8,
            PatternCategory::Environment => 0.45,
            PatternCategory::Gateway => 0.85,
            PatternCategory::Source => 0.95,
        }
    }
}

/// Emergence Template - a template for successful emergence
#[derive(Debug, Clone)]
pub struct EmergenceTemplate {
    pub template_id: String,
    pub category: PatternCategory,
    pub field_configuration: Vec<Float>,
    pub archetype_activations: Vec<Float>,
    pub coherence_requirement: Float,
    pub unity_requirement: Float,
    pub success_rate: Float,
    pub applications_count: u32,
    pub source_strength: Float,
    pub metadata: HashMap<String, Float>,
}

impl EmergenceTemplate {
    pub fn new(template_id: &str, category: PatternCategory) -> Self {
        Self {
            template_id: template_id.to_string(),
            category,
            field_configuration: vec![0.5; 8],
            archetype_activations: vec![0.5; 22],
            coherence_requirement: category.base_resonance(),
            unity_requirement: 0.3,
            success_rate: 0.5,
            applications_count: 0,
            source_strength: 0.5,
            metadata: HashMap::new(),
        }
    }

    pub fn with_field_configuration(mut self, config: Vec<Float>) -> Self {
        self.field_configuration = config;
        self
    }

    pub fn with_archetype_activations(mut self, activations: Vec<Float>) -> Self {
        self.archetype_activations = activations;
        self
    }

    pub fn with_requirements(mut self, coherence: Float, unity: Float) -> Self {
        self.coherence_requirement = coherence;
        self.unity_requirement = unity;
        self
    }

    pub fn can_apply(&self, coherence: Float, unity: Float) -> bool {
        coherence >= self.coherence_requirement && unity >= self.unity_requirement
    }

    pub fn calculate_resonance(&self, entity_field: &[Float]) -> Float {
        if entity_field.len() != self.field_configuration.len() {
            return 0.0;
        }

        let dot_product: Float = entity_field
            .iter()
            .zip(self.field_configuration.iter())
            .map(|(a, b)| a * b)
            .sum();

        let entity_norm: Float = entity_field.iter().map(|x| x.powi(2)).sum::<Float>().sqrt();
        let template_norm: Float = self
            .field_configuration
            .iter()
            .map(|x| x.powi(2))
            .sum::<Float>()
            .sqrt();

        if entity_norm < 0.001 || template_norm < 0.001 {
            return 0.0;
        }

        (dot_product / (entity_norm * template_norm))
            .clamp(0.0, 1.0)
    }

    pub fn apply(&mut self) -> Vec<Float> {
        self.applications_count += 1;
        self.field_configuration.clone()
    }

    pub fn record_success(&mut self, success: bool) {
        let weight = 0.1;
        let old_rate = self.success_rate;
        self.success_rate = if success {
            old_rate * (1.0 - weight) + weight
        } else {
            old_rate * (1.0 - weight)
        };
    }

    pub fn update_from_ii(&mut self, ii_strength: Float) {
        self.source_strength = (self.source_strength + ii_strength * 0.1).min(1.0);
    }

    pub fn is_stable(&self) -> bool {
        self.success_rate > 0.7 && self.applications_count >= 10
    }

    pub fn effectiveness(&self) -> Float {
        self.success_rate * self.source_strength * (1.0 + 0.01 * self.applications_count as Float)
    }

    pub fn with_metadata(mut self, key: &str, value: Float) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    pub fn get_metadata(&self, key: &str) -> Float {
        self.metadata.get(key).copied().unwrap_or(0.0)
    }
}

/// Pattern Match - a matched pattern from library search
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub template: EmergenceTemplate,
    pub resonance: Float,
    pub applicability: Float,
    pub recommended_order: u32,
}

impl PatternMatch {
    pub fn new(template: EmergenceTemplate, resonance: Float) -> Self {
        Self {
            applicability: template.effectiveness() * resonance,
            recommended_order: 0,
            template,
            resonance,
        }
    }

    pub fn is_recommended(&self) -> bool {
        self.resonance > 0.5 && self.applicability > 0.3
    }

    pub fn priority_score(&self) -> Float {
        self.resonance * 0.6 + self.applicability * 0.4
    }
}

/// Template Search - parameters for searching the pattern library
#[derive(Debug, Clone)]
pub struct TemplateSearch {
    pub category: Option<PatternCategory>,
    pub min_coherence: Float,
    pub min_unity: Float,
    pub min_resonance: Float,
    pub entity_field: Vec<Float>,
    pub max_results: usize,
}

impl TemplateSearch {
    pub fn new() -> Self {
        Self {
            category: None,
            min_coherence: 0.0,
            min_unity: 0.0,
            min_resonance: 0.3,
            entity_field: vec![],
            max_results: 10,
        }
    }

    pub fn with_category(mut self, category: PatternCategory) -> Self {
        self.category = Some(category);
        self
    }

    pub fn with_requirements(mut self, coherence: Float, unity: Float) -> Self {
        self.min_coherence = coherence;
        self.min_unity = unity;
        self
    }

    pub fn with_entity_field(mut self, field: Vec<Float>) -> Self {
        self.entity_field = field;
        self
    }

    pub fn with_min_resonance(mut self, resonance: Float) -> Self {
        self.min_resonance = resonance;
        self
    }

    pub fn with_max_results(mut self, max: usize) -> Self {
        self.max_results = max;
        self
    }

    pub fn matches_template(&self, template: &EmergenceTemplate) -> bool {
        if let Some(cat) = self.category {
            if template.category != cat {
                return false;
            }
        }

        if !template.can_apply(self.min_coherence, self.min_unity) {
            return false;
        }

        true
    }
}

impl Default for TemplateSearch {
    fn default() -> Self {
        Self::new()
    }
}

/// Pattern Library - the full pattern library system
#[derive(Debug, Clone)]
pub struct PatternLibrary {
    pub templates: HashMap<String, EmergenceTemplate>,
    pub templates_by_category: HashMap<PatternCategory, Vec<String>>,
    pub total_applications: u64,
    pub total_successes: u64,
    pub ii_connection_strength: Float,
    pub last_sync_time: Float,
}

impl PatternLibrary {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            templates_by_category: HashMap::new(),
            total_applications: 0,
            total_successes: 0,
            ii_connection_strength: 0.0,
            last_sync_time: 0.0,
        }
    }

    pub fn with_default_templates(mut self) -> Self {
        self.add_default_templates();
        self
    }

    fn add_default_templates(&mut self) {
        let atomic = EmergenceTemplate::new("hydrogen", PatternCategory::Atomic)
            .with_field_configuration(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
            .with_requirements(0.3, 0.1);
        self.add_template(atomic);

        let molecular = EmergenceTemplate::new("water", PatternCategory::Molecular)
            .with_field_configuration(vec![0.8, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
            .with_requirements(0.35, 0.2);
        self.add_template(molecular);

        let cellular = EmergenceTemplate::new("basic_cell", PatternCategory::Cellular)
            .with_field_configuration(vec![0.6, 0.4, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0])
            .with_requirements(0.4, 0.3);
        self.add_template(cellular);

        let collective = EmergenceTemplate::new("social_memory", PatternCategory::Collective)
            .with_field_configuration(vec![0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5])
            .with_requirements(0.6, 0.5);
        self.add_template(collective);

        let density = EmergenceTemplate::new("density_transition", PatternCategory::Density)
            .with_field_configuration(vec![0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7])
            .with_requirements(0.7, 0.6);
        self.add_template(density);

        let gateway = EmergenceTemplate::new("gateway_opening", PatternCategory::Gateway)
            .with_field_configuration(vec![0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9])
            .with_requirements(0.85, 0.8);
        self.add_template(gateway);
    }

    pub fn add_template(&mut self, template: EmergenceTemplate) {
        let category = template.category;
        let id = template.template_id.clone();

        self.templates.insert(id.clone(), template);

        self.templates_by_category
            .entry(category)
            .or_default()
            .push(id);
    }

    pub fn search(&self, search: &TemplateSearch) -> Vec<PatternMatch> {
        let mut matches: Vec<PatternMatch> = self
            .templates
            .values()
            .filter(|t| search.matches_template(t))
            .filter_map(|t| {
                let resonance = if search.entity_field.is_empty() {
                    t.category.base_resonance()
                } else {
                    t.calculate_resonance(&search.entity_field)
                };

                if resonance >= search.min_resonance {
                    Some(PatternMatch::new(t.clone(), resonance))
                } else {
                    None
                }
            })
            .collect();

        matches.sort_by(|a, b| {
            b.priority_score()
                .partial_cmp(&a.priority_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches.truncate(search.max_results);

        for (i, m) in matches.iter_mut().enumerate() {
            m.recommended_order = i as u32 + 1;
        }

        matches
    }

    pub fn get_template(&self, template_id: &str) -> Option<&EmergenceTemplate> {
        self.templates.get(template_id)
    }

    pub fn get_template_mut(&mut self, template_id: &str) -> Option<&mut EmergenceTemplate> {
        self.templates.get_mut(template_id)
    }

    pub fn apply_template(&mut self, template_id: &str) -> Option<Vec<Float>> {
        let template = self.templates.get_mut(template_id)?;
        self.total_applications += 1;
        Some(template.apply())
    }

    pub fn record_success(&mut self, template_id: &str, success: bool) {
        if let Some(template) = self.templates.get_mut(template_id) {
            template.record_success(success);
            if success {
                self.total_successes += 1;
            }
        }
    }

    pub fn sync_with_ii(&mut self, ii_strength: Float, current_time: Float) {
        self.ii_connection_strength = ii_strength;
        self.last_sync_time = current_time;

        for template in self.templates.values_mut() {
            template.update_from_ii(ii_strength);
        }
    }

    pub fn templates_for_category(&self, category: PatternCategory) -> Vec<&EmergenceTemplate> {
        self.templates_by_category
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.templates.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn overall_success_rate(&self) -> Float {
        if self.total_applications == 0 {
            return 0.0;
        }
        self.total_successes as Float / self.total_applications as Float
    }

    pub fn stable_template_count(&self) -> usize {
        self.templates.values().filter(|t| t.is_stable()).count()
    }

    pub fn average_effectiveness(&self) -> Float {
        if self.templates.is_empty() {
            return 0.0;
        }
        self.templates
            .values()
            .map(|t| t.effectiveness())
            .sum::<Float>()
            / self.templates.len() as Float
    }

    pub fn best_template_for(&self, category: PatternCategory) -> Option<&EmergenceTemplate> {
        self.templates_for_category(category)
            .into_iter()
            .max_by(|a, b| {
                a.effectiveness()
                    .partial_cmp(&b.effectiveness())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

impl Default for PatternLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_category_base_resonance() {
        assert!((PatternCategory::Atomic.base_resonance() - 0.3).abs() < 0.001);
        assert!((PatternCategory::Source.base_resonance() - 0.95).abs() < 0.001);
    }

    #[test]
    fn test_emergence_template_creation() {
        let template = EmergenceTemplate::new("test", PatternCategory::Atomic);
        assert_eq!(template.template_id, "test");
        assert_eq!(template.category, PatternCategory::Atomic);
    }

    #[test]
    fn test_emergence_template_can_apply() {
        let template =
            EmergenceTemplate::new("test", PatternCategory::Atomic).with_requirements(0.5, 0.3);
        assert!(template.can_apply(0.6, 0.4));
        assert!(!template.can_apply(0.4, 0.4));
    }

    #[test]
    fn test_emergence_template_calculate_resonance() {
        let template = EmergenceTemplate::new("test", PatternCategory::Atomic)
            .with_field_configuration(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let resonance = template.calculate_resonance(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((resonance - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_emergence_template_record_success() {
        let mut template = EmergenceTemplate::new("test", PatternCategory::Atomic);
        template.record_success(true);
        assert!(template.success_rate > 0.5);
    }

    #[test]
    fn test_emergence_template_is_stable() {
        let mut template = EmergenceTemplate::new("test", PatternCategory::Atomic);
        template.success_rate = 0.8;
        template.applications_count = 15;
        assert!(template.is_stable());
    }

    #[test]
    fn test_pattern_match_creation() {
        let template = EmergenceTemplate::new("test", PatternCategory::Atomic);
        let match_result = PatternMatch::new(template, 0.8);
        assert!((match_result.resonance - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_pattern_match_is_recommended() {
        let mut template = EmergenceTemplate::new("test", PatternCategory::Atomic);
        template.success_rate = 0.9;
        template.source_strength = 0.9;
        let high_match = PatternMatch::new(template.clone(), 0.8);
        assert!(high_match.is_recommended());

        let low_match = PatternMatch::new(template, 0.3);
        assert!(!low_match.is_recommended());
    }

    #[test]
    fn test_template_search_creation() {
        let search = TemplateSearch::new();
        assert!(search.category.is_none());
        assert_eq!(search.max_results, 10);
    }

    #[test]
    fn test_template_search_matches() {
        let search = TemplateSearch::new()
            .with_category(PatternCategory::Atomic)
            .with_requirements(0.5, 0.3);

        let matching =
            EmergenceTemplate::new("test", PatternCategory::Atomic).with_requirements(0.4, 0.2);
        assert!(search.matches_template(&matching));

        let wrong_category = EmergenceTemplate::new("test", PatternCategory::Molecular);
        assert!(!search.matches_template(&wrong_category));
    }

    #[test]
    fn test_pattern_library_creation() {
        let library = PatternLibrary::new();
        assert!(library.templates.is_empty());
    }

    #[test]
    fn test_pattern_library_add_template() {
        let mut library = PatternLibrary::new();
        let template = EmergenceTemplate::new("test", PatternCategory::Atomic);
        library.add_template(template);
        assert!(library.templates.contains_key("test"));
    }

    #[test]
    fn test_pattern_library_search() {
        let library = PatternLibrary::new().with_default_templates();
        let search = TemplateSearch::new()
            .with_requirements(0.5, 0.5)
            .with_min_resonance(0.1)
            .with_max_results(5);
        let results = library.search(&search);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_pattern_library_apply() {
        let mut library = PatternLibrary::new().with_default_templates();
        let result = library.apply_template("hydrogen");
        assert!(result.is_some());
        assert_eq!(library.total_applications, 1);
    }

    #[test]
    fn test_pattern_library_success_rate() {
        let mut library = PatternLibrary::new().with_default_templates();
        library.apply_template("hydrogen");
        library.record_success("hydrogen", true);
        assert!(library.overall_success_rate() > 0.0);
    }

    #[test]
    fn test_pattern_library_sync_ii() {
        let mut library = PatternLibrary::new().with_default_templates();
        library.sync_with_ii(0.9, 1.0);
        assert!((library.ii_connection_strength - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_pattern_library_best_template() {
        let mut library = PatternLibrary::new().with_default_templates();
        library.get_template_mut("hydrogen").unwrap().success_rate = 0.99;
        library
            .get_template_mut("hydrogen")
            .unwrap()
            .applications_count = 100;
        let best = library.best_template_for(PatternCategory::Atomic);
        assert!(best.is_some());
    }
}
