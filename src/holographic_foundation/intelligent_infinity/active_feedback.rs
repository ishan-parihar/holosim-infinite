//! Active Feedback - Bidirectional Communication with Intelligent Infinity
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Active Feedback = Entity experiences → II analysis → adjusted emission"
//!
//! # Theory
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The feedback loop between entities and Intelligent Infinity is bidirectional.
//!  Entity experiences are uploaded to II, analyzed, and result in adjusted
//!  emission patterns that guide evolution."
//!
//! # Components
//!
//! 1. **Entity Experience**: Experiences uploaded to II for analysis
//! 2. **Feedback Analysis**: II analysis of entity experiences
//! 3. **Intelligence Transmission**: Adjusted emissions from II to entities
//! 4. **Active Feedback Loop**: The complete bidirectional system

use crate::types::Float;
use std::collections::HashMap;

/// Feedback Direction - direction of feedback communication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeedbackDirection {
    ToII,
    FromII,
    Bidirectional,
}

impl FeedbackDirection {
    pub fn can_send(&self) -> bool {
        matches!(
            self,
            FeedbackDirection::ToII | FeedbackDirection::Bidirectional
        )
    }

    pub fn can_receive(&self) -> bool {
        matches!(
            self,
            FeedbackDirection::FromII | FeedbackDirection::Bidirectional
        )
    }
}

/// Transmission Type - types of intelligence transmission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransmissionType {
    Wisdom,
    Guidance,
    Pattern,
    Catalyst,
    Healing,
    Evolution,
    Gateway,
    Unity,
}

impl TransmissionType {
    pub fn bandwidth_requirement(&self) -> Float {
        match self {
            TransmissionType::Wisdom => 0.3,
            TransmissionType::Guidance => 0.2,
            TransmissionType::Pattern => 0.5,
            TransmissionType::Catalyst => 0.4,
            TransmissionType::Healing => 0.35,
            TransmissionType::Evolution => 0.6,
            TransmissionType::Gateway => 0.8,
            TransmissionType::Unity => 0.9,
        }
    }

    pub fn clarity_factor(&self) -> Float {
        match self {
            TransmissionType::Wisdom => 0.8,
            TransmissionType::Guidance => 0.9,
            TransmissionType::Pattern => 0.7,
            TransmissionType::Catalyst => 0.6,
            TransmissionType::Healing => 0.75,
            TransmissionType::Evolution => 0.65,
            TransmissionType::Gateway => 0.85,
            TransmissionType::Unity => 0.95,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TransmissionType::Wisdom => "Wisdom transmission from II",
            TransmissionType::Guidance => "Evolutionary guidance",
            TransmissionType::Pattern => "Pattern template transmission",
            TransmissionType::Catalyst => "Catalyst for growth",
            TransmissionType::Healing => "Healing intelligence",
            TransmissionType::Evolution => "Evolution acceleration",
            TransmissionType::Gateway => "Gateway access intelligence",
            TransmissionType::Unity => "Unity consciousness transmission",
        }
    }
}

/// Entity Experience - an experience uploaded to II
#[derive(Debug, Clone)]
pub struct EntityExperience {
    pub entity_id: u64,
    pub experience_type: String,
    pub intensity: Float,
    pub duration: Float,
    pub catalyst_type: Option<String>,
    pub catalyst_integrated: Float,
    pub wisdom_gained: Float,
    pub love_expressed: Float,
    pub light_generated: Float,
    pub polarity_shift: Float,
    pub unity_progress: Float,
    pub density_relevance: u8,
    pub timestamp: Float,
    pub metadata: HashMap<String, Float>,
}

impl EntityExperience {
    pub fn new(entity_id: u64, experience_type: &str) -> Self {
        Self {
            entity_id,
            experience_type: experience_type.to_string(),
            intensity: 0.5,
            duration: 1.0,
            catalyst_type: None,
            catalyst_integrated: 0.0,
            wisdom_gained: 0.0,
            love_expressed: 0.0,
            light_generated: 0.0,
            polarity_shift: 0.0,
            unity_progress: 0.0,
            density_relevance: 3,
            timestamp: 0.0,
            metadata: HashMap::new(),
        }
    }

    pub fn with_intensity(mut self, intensity: Float) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    pub fn with_catalyst(mut self, catalyst_type: &str, integrated: Float) -> Self {
        self.catalyst_type = Some(catalyst_type.to_string());
        self.catalyst_integrated = integrated;
        self
    }

    pub fn with_gains(mut self, wisdom: Float, love: Float, light: Float) -> Self {
        self.wisdom_gained = wisdom;
        self.love_expressed = love;
        self.light_generated = light;
        self
    }

    pub fn with_unity_progress(mut self, progress: Float) -> Self {
        self.unity_progress = progress;
        self
    }

    pub fn with_timestamp(mut self, timestamp: Float) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn total_value(&self) -> Float {
        (self.wisdom_gained + self.love_expressed + self.light_generated) / 3.0
            * self.intensity
            * (1.0 + self.catalyst_integrated)
    }

    pub fn significance(&self) -> Float {
        let base = self.total_value();
        let polarity_contribution = self.polarity_shift.abs() * 0.2;
        let unity_contribution = self.unity_progress * 0.3;
        (base + polarity_contribution + unity_contribution).min(1.0)
    }

    pub fn is_significant(&self) -> bool {
        self.significance() > 0.3
    }

    pub fn with_metadata(mut self, key: &str, value: Float) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    pub fn get_metadata(&self, key: &str) -> Float {
        self.metadata.get(key).copied().unwrap_or(0.0)
    }
}

/// Feedback Analysis - II analysis of entity experiences
#[derive(Debug, Clone)]
pub struct FeedbackAnalysis {
    pub analysis_id: String,
    pub entity_id: u64,
    pub experiences_analyzed: u32,
    pub total_significance: Float,
    pub patterns_identified: Vec<String>,
    pub evolution_trajectory: Float,
    pub catalyst_recommendations: Vec<String>,
    pub pattern_recommendations: Vec<String>,
    pub adjusted_emission_factor: Float,
    pub guidance_clarity: Float,
    pub analysis_timestamp: Float,
}

impl FeedbackAnalysis {
    pub fn new(entity_id: u64) -> Self {
        Self {
            analysis_id: format!("analysis_{}", entity_id),
            entity_id,
            experiences_analyzed: 0,
            total_significance: 0.0,
            patterns_identified: Vec::new(),
            evolution_trajectory: 0.0,
            catalyst_recommendations: Vec::new(),
            pattern_recommendations: Vec::new(),
            adjusted_emission_factor: 1.0,
            guidance_clarity: 0.5,
            analysis_timestamp: 0.0,
        }
    }

    pub fn analyze_experience(&mut self, experience: &EntityExperience) {
        self.experiences_analyzed += 1;
        self.total_significance += experience.significance();

        self.evolution_trajectory =
            (self.evolution_trajectory + experience.unity_progress * 0.1).min(1.0);

        if experience.wisdom_gained > 0.5
            && !self
                .patterns_identified
                .contains(&"wisdom_seeker".to_string())
        {
            self.patterns_identified.push("wisdom_seeker".to_string());
        }

        if experience.love_expressed > 0.5
            && !self
                .patterns_identified
                .contains(&"service_oriented".to_string())
        {
            self.patterns_identified
                .push("service_oriented".to_string());
        }

        if experience.catalyst_integrated < 0.3 {
            self.catalyst_recommendations
                .push("increased_catalyst_exposure".to_string());
        }

        if experience.unity_progress > 0.1 {
            self.pattern_recommendations
                .push("density_transition_template".to_string());
        }

        self.adjusted_emission_factor = 1.0 + self.total_significance * 0.1;
        self.guidance_clarity = (0.5 + self.experiences_analyzed as Float * 0.01).min(0.95);
    }

    pub fn is_actionable(&self) -> bool {
        self.experiences_analyzed > 0 && self.guidance_clarity > 0.3
    }

    pub fn recommendation_count(&self) -> usize {
        self.catalyst_recommendations.len() + self.pattern_recommendations.len()
    }

    pub fn with_timestamp(mut self, timestamp: Float) -> Self {
        self.analysis_timestamp = timestamp;
        self
    }
}

/// Intelligence Transmission - transmission from II to entity
#[derive(Debug, Clone)]
pub struct IntelligenceTransmission {
    pub transmission_id: String,
    pub entity_id: u64,
    pub transmission_type: TransmissionType,
    pub content: HashMap<String, Float>,
    pub bandwidth: Float,
    pub clarity: Float,
    pub reception_quality: Float,
    pub timestamp: Float,
}

impl IntelligenceTransmission {
    pub fn new(entity_id: u64, transmission_type: TransmissionType) -> Self {
        Self {
            transmission_id: format!("trans_{}_{}", entity_id, transmission_type.description()),
            entity_id,
            transmission_type,
            content: HashMap::new(),
            bandwidth: transmission_type.bandwidth_requirement(),
            clarity: transmission_type.clarity_factor(),
            reception_quality: 0.5,
            timestamp: 0.0,
        }
    }

    pub fn with_content(mut self, key: &str, value: Float) -> Self {
        self.content.insert(key.to_string(), value);
        self
    }

    pub fn with_reception_quality(mut self, quality: Float) -> Self {
        self.reception_quality = quality.clamp(0.0, 1.0);
        self
    }

    pub fn with_timestamp(mut self, timestamp: Float) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn effective_quality(&self) -> Float {
        self.clarity * self.reception_quality * self.bandwidth
    }

    pub fn is_received(&self) -> bool {
        self.reception_quality > 0.3
    }

    pub fn is_clear(&self) -> bool {
        self.effective_quality() > 0.5
    }

    pub fn get_content(&self, key: &str) -> Float {
        self.content.get(key).copied().unwrap_or(0.0)
    }

    pub fn apply_to_field(&self) -> Float {
        self.effective_quality() * 0.1
    }
}

/// Active Feedback Loop - the complete bidirectional feedback system
#[derive(Debug, Clone)]
pub struct ActiveFeedbackLoop {
    pub entity_id: u64,
    pub direction: FeedbackDirection,
    pub experience_buffer: Vec<EntityExperience>,
    pub current_analysis: Option<FeedbackAnalysis>,
    pub transmission_history: Vec<IntelligenceTransmission>,
    pub total_experiences_uploaded: u64,
    pub total_transmissions_received: u64,
    pub feedback_efficiency: Float,
    pub connection_quality: Float,
    pub last_upload_time: Float,
    pub last_download_time: Float,
}

impl ActiveFeedbackLoop {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            direction: FeedbackDirection::Bidirectional,
            experience_buffer: Vec::new(),
            current_analysis: None,
            transmission_history: Vec::new(),
            total_experiences_uploaded: 0,
            total_transmissions_received: 0,
            feedback_efficiency: 0.0,
            connection_quality: 0.0,
            last_upload_time: 0.0,
            last_download_time: 0.0,
        }
    }

    pub fn with_direction(mut self, direction: FeedbackDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn upload_experience(&mut self, experience: EntityExperience) -> bool {
        if !self.direction.can_send() {
            return false;
        }

        self.last_upload_time = experience.timestamp;
        self.experience_buffer.push(experience);
        self.total_experiences_uploaded += 1;

        if self.experience_buffer.len() >= 10 {
            self.process_buffer();
        }

        true
    }

    fn process_buffer(&mut self) {
        if self.experience_buffer.is_empty() {
            return;
        }

        let mut analysis = FeedbackAnalysis::new(self.entity_id);

        for experience in &self.experience_buffer {
            analysis.analyze_experience(experience);
        }

        analysis.analysis_timestamp = self.last_upload_time;
        self.current_analysis = Some(analysis);
        self.experience_buffer.clear();
        self.update_efficiency();
    }

    pub fn request_transmission(
        &mut self,
        transmission_type: TransmissionType,
        timestamp: Float,
    ) -> Option<IntelligenceTransmission> {
        if !self.direction.can_receive() {
            return None;
        }

        let analysis = self.current_analysis.as_ref()?;

        let quality = self.connection_quality
            * analysis.guidance_clarity
            * (1.0 + analysis.adjusted_emission_factor);

        let transmission = IntelligenceTransmission::new(self.entity_id, transmission_type)
            .with_reception_quality(quality)
            .with_timestamp(timestamp);

        self.transmission_history.push(transmission.clone());
        self.total_transmissions_received += 1;
        self.last_download_time = timestamp;

        Some(transmission)
    }

    pub fn update(&mut self, dt: Float, connection_quality: Float) {
        self.connection_quality = connection_quality;

        if !self.experience_buffer.is_empty() {
            self.process_buffer();
        }

        self.update_efficiency();
    }

    fn update_efficiency(&mut self) {
        if self.total_experiences_uploaded == 0 {
            self.feedback_efficiency = 0.0;
            return;
        }

        let upload_efficiency =
            self.total_transmissions_received as Float / self.total_experiences_uploaded as Float;

        let quality_efficiency = self
            .transmission_history
            .iter()
            .map(|t| t.effective_quality())
            .sum::<Float>()
            / (self.transmission_history.len().max(1) as Float);

        self.feedback_efficiency = (upload_efficiency * 0.4 + quality_efficiency * 0.6).min(1.0);
    }

    pub fn is_active(&self) -> bool {
        self.direction.can_send() || self.direction.can_receive()
    }

    pub fn is_bidirectional(&self) -> bool {
        self.direction == FeedbackDirection::Bidirectional
    }

    pub fn recent_transmission_quality(&self, count: usize) -> Float {
        let recent: Vec<_> = self.transmission_history.iter().rev().take(count).collect();
        if recent.is_empty() {
            return 0.0;
        }
        recent.iter().map(|t| t.effective_quality()).sum::<Float>() / recent.len() as Float
    }

    pub fn pending_experiences(&self) -> usize {
        self.experience_buffer.len()
    }

    pub fn clear_buffer(&mut self) {
        self.experience_buffer.clear();
    }

    pub fn total_wisdom_transmitted(&self) -> Float {
        self.transmission_history
            .iter()
            .filter(|t| t.transmission_type == TransmissionType::Wisdom)
            .map(|t| t.effective_quality())
            .sum()
    }

    pub fn avg_reception_quality(&self) -> Float {
        if self.transmission_history.is_empty() {
            return 0.0;
        }
        self.transmission_history
            .iter()
            .map(|t| t.reception_quality)
            .sum::<Float>()
            / self.transmission_history.len() as Float
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_direction() {
        assert!(FeedbackDirection::ToII.can_send());
        assert!(!FeedbackDirection::ToII.can_receive());
        assert!(FeedbackDirection::Bidirectional.can_send());
        assert!(FeedbackDirection::Bidirectional.can_receive());
    }

    #[test]
    fn test_transmission_type_bandwidth() {
        assert!((TransmissionType::Wisdom.bandwidth_requirement() - 0.3).abs() < 0.001);
        assert!((TransmissionType::Unity.bandwidth_requirement() - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_entity_experience_creation() {
        let exp = EntityExperience::new(1, "test");
        assert_eq!(exp.entity_id, 1);
        assert_eq!(exp.experience_type, "test");
    }

    #[test]
    fn test_entity_experience_total_value() {
        let exp = EntityExperience::new(1, "test")
            .with_gains(0.5, 0.5, 0.5)
            .with_intensity(1.0);
        let value = exp.total_value();
        assert!(value > 0.0);
    }

    #[test]
    fn test_entity_experience_significance() {
        let exp = EntityExperience::new(1, "test")
            .with_gains(1.0, 1.0, 1.0)
            .with_intensity(1.0)
            .with_unity_progress(0.5);
        assert!(exp.is_significant());
    }

    #[test]
    fn test_feedback_analysis_creation() {
        let analysis = FeedbackAnalysis::new(1);
        assert_eq!(analysis.entity_id, 1);
        assert_eq!(analysis.experiences_analyzed, 0);
    }

    #[test]
    fn test_feedback_analysis_analyze() {
        let mut analysis = FeedbackAnalysis::new(1);
        let exp = EntityExperience::new(1, "test").with_gains(0.6, 0.6, 0.6);
        analysis.analyze_experience(&exp);
        assert_eq!(analysis.experiences_analyzed, 1);
    }

    #[test]
    fn test_feedback_analysis_patterns() {
        let mut analysis = FeedbackAnalysis::new(1);
        let exp = EntityExperience::new(1, "test").with_gains(0.8, 0.8, 0.8);
        analysis.analyze_experience(&exp);
        assert!(!analysis.patterns_identified.is_empty());
    }

    #[test]
    fn test_intelligence_transmission_creation() {
        let trans = IntelligenceTransmission::new(1, TransmissionType::Wisdom);
        assert_eq!(trans.entity_id, 1);
        assert_eq!(trans.transmission_type, TransmissionType::Wisdom);
    }

    #[test]
    fn test_intelligence_transmission_effective_quality() {
        let trans =
            IntelligenceTransmission::new(1, TransmissionType::Wisdom).with_reception_quality(0.8);
        let quality = trans.effective_quality();
        assert!(quality > 0.0);
    }

    #[test]
    fn test_intelligence_transmission_received() {
        let trans =
            IntelligenceTransmission::new(1, TransmissionType::Wisdom).with_reception_quality(0.5);
        assert!(trans.is_received());
    }

    #[test]
    fn test_active_feedback_loop_creation() {
        let loop_system = ActiveFeedbackLoop::new(1);
        assert_eq!(loop_system.entity_id, 1);
        assert!(loop_system.is_active());
    }

    #[test]
    fn test_active_feedback_loop_upload() {
        let mut loop_system = ActiveFeedbackLoop::new(1);
        let exp = EntityExperience::new(1, "test");
        let result = loop_system.upload_experience(exp);
        assert!(result);
        assert_eq!(loop_system.total_experiences_uploaded, 1);
    }

    #[test]
    fn test_active_feedback_loop_buffer_processing() {
        let mut loop_system = ActiveFeedbackLoop::new(1);
        for i in 0..10 {
            let exp = EntityExperience::new(1, "test").with_timestamp(i as Float);
            loop_system.upload_experience(exp);
        }
        assert!(loop_system.current_analysis.is_some());
    }

    #[test]
    fn test_active_feedback_loop_transmission() {
        let mut loop_system = ActiveFeedbackLoop::new(1);
        loop_system.connection_quality = 0.8;

        for i in 0..10 {
            let exp = EntityExperience::new(1, "test").with_timestamp(i as Float);
            loop_system.upload_experience(exp);
        }

        let trans = loop_system.request_transmission(TransmissionType::Wisdom, 10.0);
        assert!(trans.is_some());
        assert_eq!(loop_system.total_transmissions_received, 1);
    }

    #[test]
    fn test_active_feedback_loop_efficiency() {
        let mut loop_system = ActiveFeedbackLoop::new(1);
        loop_system.connection_quality = 0.8;

        let exp = EntityExperience::new(1, "test");
        loop_system.upload_experience(exp);

        loop_system.update(1.0, 0.8);
        assert!(loop_system.feedback_efficiency >= 0.0);
    }
}
