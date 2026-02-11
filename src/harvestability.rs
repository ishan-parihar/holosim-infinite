// Harvestability Assessment - graduation evaluation system
//
// Knowledge Base Reference: Densities/D4. Fourth Density.json
// "Harvestability"

use crate::types::Polarity;

/// Harvestability Assessment - graduation evaluation
///
/// Knowledge Base Reference: Densities/D4. Fourth Density.json
/// "Harvestability"
#[derive(Debug, Clone, PartialEq)]
pub struct HarvestabilityAssessment {
    /// Violet ray reading (0.0 to 1.0)
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "Violet ray reading"
    pub violet_ray_quality: f64,

    /// Green/Blue/Indigo triad quality (for positive harvest)
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "51% or more STO"
    pub triad_quality: TriadQuality,

    /// Red/Orange/Yellow intensity (for negative harvest)
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "95% or more STS"
    pub lower_intensity: LowerIntensity,

    /// Overall harvestability determination
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "Harvestability"
    pub harvestability: Harvestability,
}

/// Triad Quality - Green/Blue/Indigo centers for positive harvest
///
/// Knowledge Base Reference: Densities/D4. Fourth Density.json
/// "51% or more STO"
#[derive(Debug, Clone, PartialEq)]
pub struct TriadQuality {
    /// Green ray quality (0.0 to 1.0)
    pub green_quality: f64,

    /// Blue ray quality (0.0 to 1.0)
    pub blue_quality: f64,

    /// Indigo ray quality (0.0 to 1.0)
    pub indigo_quality: f64,

    /// Overall triad quality (0.0 to 1.0)
    pub overall_triad_quality: f64,
}

impl TriadQuality {
    /// Create a new TriadQuality
    pub fn new(green: f64, blue: f64, indigo: f64) -> Self {
        let overall = (green + blue + indigo) / 3.0;
        TriadQuality {
            green_quality: green,
            blue_quality: blue,
            indigo_quality: indigo,
            overall_triad_quality: overall,
        }
    }

    /// Check if triad meets positive harvest threshold (51%)
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    /// "51% or more STO"
    pub fn meets_positive_threshold(&self) -> bool {
        self.overall_triad_quality >= 0.51 - 0.001 // Allow small floating point error
    }
}

/// Lower Intensity - Red/Orange/Yellow centers for negative harvest
///
/// Knowledge Base Reference: Densities/D4. Fourth Density.json
/// "95% or more STS"
#[derive(Debug, Clone, PartialEq)]
pub struct LowerIntensity {
    /// Red ray intensity (0.0 to 1.0)
    pub red_intensity: f64,

    /// Orange ray intensity (0.0 to 1.0)
    pub orange_intensity: f64,

    /// Yellow ray intensity (0.0 to 1.0)
    pub yellow_intensity: f64,

    /// Overall lower intensity (0.0 to 1.0)
    pub overall_lower_intensity: f64,
}

impl LowerIntensity {
    /// Create a new LowerIntensity
    pub fn new(red: f64, orange: f64, yellow: f64) -> Self {
        let overall = (red + orange + yellow) / 3.0;
        LowerIntensity {
            red_intensity: red,
            orange_intensity: orange,
            yellow_intensity: yellow,
            overall_lower_intensity: overall,
        }
    }

    /// Check if lower centers meet negative harvest threshold (95%)
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    /// "95% or more STS"
    pub fn meets_negative_threshold(&self) -> bool {
        self.overall_lower_intensity >= 0.95 - 0.001 // Allow small floating point error
    }
}

/// Harvestability - whether entity can graduate
///
/// Knowledge Base Reference: Densities/D4. Fourth Density.json
/// "Harvestability"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Harvestability {
    /// Harvestable for positive polarity (fourth density positive)
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    /// "51% or more STO"
    HarvestablePositive,

    /// Harvestable for negative polarity (fourth density negative)
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    /// "95% or more STS"
    HarvestableNegative,

    /// Not yet harvestable - needs more development
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    /// "Not yet harvestable"
    NotYetHarvestable,
}

impl Harvestability {
    /// Check if entity is harvestable
    pub fn is_harvestable(&self) -> bool {
        matches!(
            self,
            Harvestability::HarvestablePositive | Harvestability::HarvestableNegative
        )
    }

    /// Get description of harvestability
    pub fn description(&self) -> &'static str {
        match self {
            Harvestability::HarvestablePositive => {
                "Harvestable - Ready for fourth density positive"
            }
            Harvestability::HarvestableNegative => {
                "Harvestable - Ready for fourth density negative"
            }
            Harvestability::NotYetHarvestable => "Not yet harvestable - Continue development",
        }
    }
}

impl Harvestability {
    /// Determine harvestability based on polarity and thresholds
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    pub fn determine_harvestability(
        polarity: Polarity,
        sto_percentage: f64,
        sts_percentage: f64,
        _triad_quality: &TriadQuality,
        _lower_intensity: &LowerIntensity,
    ) -> Harvestability {
        match polarity {
            Polarity::Positive => {
                // Positive harvest: 51% or more STO
                //
                // From Densities/D4. Fourth Density.json:
                // "51% or more STO"
                if sto_percentage >= 51.0 {
                    Harvestability::HarvestablePositive
                } else {
                    Harvestability::NotYetHarvestable
                }
            }
            Polarity::Negative => {
                // Negative harvest: 95% or more STS
                //
                // From Densities/D4. Fourth Density.json:
                // "95% or more STS"
                if sts_percentage >= 95.0 {
                    Harvestability::HarvestableNegative
                } else {
                    Harvestability::NotYetHarvestable
                }
            }
            Polarity::Neutral => {
                // Neutral entities are not harvestable
                Harvestability::NotYetHarvestable
            }
        }
    }
}

impl HarvestabilityAssessment {
    /// Create a new HarvestabilityAssessment
    pub fn new(
        violet_ray_quality: f64,
        triad_quality: TriadQuality,
        lower_intensity: LowerIntensity,
        harvestability: Harvestability,
    ) -> Self {
        HarvestabilityAssessment {
            violet_ray_quality,
            triad_quality,
            lower_intensity,
            harvestability,
        }
    }

    /// Get detailed assessment report
    pub fn get_assessment_report(&self) -> AssessmentReport {
        AssessmentReport {
            violet_ray_quality: self.violet_ray_quality,
            triad_quality: self.triad_quality.overall_triad_quality,
            lower_intensity: self.lower_intensity.overall_lower_intensity,
            harvestability: self.harvestability,
            description: self.harvestability.description(),
            is_harvestable: self.harvestability.is_harvestable(),
            needs_work: self.identify_needs(),
        }
    }

    /// Identify areas that need work for harvestability
    fn identify_needs(&self) -> Vec<String> {
        let mut needs = Vec::new();

        // Check violet ray quality
        if self.violet_ray_quality < 0.7 {
            needs.push(String::from("Violet ray quality needs improvement"));
        }

        // Check triad quality for positive harvest
        if !self.triad_quality.meets_positive_threshold() {
            needs.push(format!(
                "Triad quality ({:.1}%) needs to reach 51% for positive harvest",
                self.triad_quality.overall_triad_quality * 100.0
            ));
        }

        // Check lower intensity for negative harvest
        if !self.lower_intensity.meets_negative_threshold() {
            needs.push(format!(
                "Lower intensity ({:.1}%) needs to reach 95% for negative harvest",
                self.lower_intensity.overall_lower_intensity * 100.0
            ));
        }

        if needs.is_empty() {
            needs.push(String::from(
                "No specific needs identified - entity is well-developed",
            ));
        }

        needs
    }
}

/// Assessment Report - detailed harvestability report
#[derive(Debug, Clone)]
pub struct AssessmentReport {
    /// Violet ray quality
    pub violet_ray_quality: f64,

    /// Triad quality
    pub triad_quality: f64,

    /// Lower intensity
    pub lower_intensity: f64,

    /// Harvestability determination
    pub harvestability: Harvestability,

    /// Description
    pub description: &'static str,

    /// Whether entity is harvestable
    pub is_harvestable: bool,

    /// Areas that need work
    pub needs_work: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad_quality_creation() {
        let triad = TriadQuality::new(0.6, 0.7, 0.8);
        assert!((triad.green_quality - 0.6).abs() < 0.001);
        assert!((triad.blue_quality - 0.7).abs() < 0.001);
        assert!((triad.indigo_quality - 0.8).abs() < 0.001);
        assert!((triad.overall_triad_quality - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_triad_quality_positive_threshold() {
        // Meets threshold (51%)
        let triad = TriadQuality::new(0.6, 0.6, 0.6);
        assert!(triad.meets_positive_threshold());

        // Does not meet threshold
        let triad = TriadQuality::new(0.4, 0.4, 0.4);
        assert!(!triad.meets_positive_threshold());

        // Exactly at threshold
        let triad = TriadQuality::new(0.51, 0.51, 0.51);
        assert!(triad.meets_positive_threshold());
    }

    #[test]
    fn test_lower_intensity_creation() {
        let lower = LowerIntensity::new(0.8, 0.9, 1.0);
        assert!((lower.red_intensity - 0.8).abs() < 0.001);
        assert!((lower.orange_intensity - 0.9).abs() < 0.001);
        assert!((lower.yellow_intensity - 1.0).abs() < 0.001);
        assert!((lower.overall_lower_intensity - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_lower_intensity_negative_threshold() {
        // Meets threshold (95%)
        let lower = LowerIntensity::new(0.95, 0.95, 0.95);
        assert!((lower.overall_lower_intensity - 0.95).abs() < 0.001);
        assert!(lower.meets_negative_threshold());

        // Does not meet threshold
        let lower = LowerIntensity::new(0.8, 0.8, 0.8);
        assert!((lower.overall_lower_intensity - 0.8).abs() < 0.001);
        assert!(!lower.meets_negative_threshold());

        // Exactly at threshold
        let lower = LowerIntensity::new(0.95, 0.95, 0.95);
        assert!((lower.overall_lower_intensity - 0.95).abs() < 0.001);
        assert!(lower.meets_negative_threshold());
    }

    #[test]
    fn test_harvestability_positive() {
        let triad = TriadQuality::new(0.6, 0.6, 0.6);
        let lower = LowerIntensity::new(0.4, 0.4, 0.4);

        // Positive polarity with 51% STO
        let harvestability = Harvestability::determine_harvestability(
            Polarity::Positive,
            51.0,
            30.0,
            &triad,
            &lower,
        );
        assert_eq!(harvestability, Harvestability::HarvestablePositive);
        assert!(harvestability.is_harvestable());
    }

    #[test]
    fn test_harvestability_negative() {
        let triad = TriadQuality::new(0.4, 0.4, 0.4);
        let lower = LowerIntensity::new(0.95, 0.95, 0.95);

        // Negative polarity with 95% STS
        let harvestability = Harvestability::determine_harvestability(
            Polarity::Negative,
            30.0,
            95.0,
            &triad,
            &lower,
        );
        assert_eq!(harvestability, Harvestability::HarvestableNegative);
        assert!(harvestability.is_harvestable());
    }

    #[test]
    fn test_harvestability_not_yet_positive() {
        let triad = TriadQuality::new(0.4, 0.4, 0.4);
        let lower = LowerIntensity::new(0.4, 0.4, 0.4);

        // Positive polarity but only 40% STO (below 51% threshold)
        let harvestability = Harvestability::determine_harvestability(
            Polarity::Positive,
            40.0,
            30.0,
            &triad,
            &lower,
        );
        assert_eq!(harvestability, Harvestability::NotYetHarvestable);
        assert!(!harvestability.is_harvestable());
    }

    #[test]
    fn test_harvestability_not_yet_negative() {
        let triad = TriadQuality::new(0.4, 0.4, 0.4);
        let lower = LowerIntensity::new(0.8, 0.8, 0.8);

        // Negative polarity but only 80% STS (below 95% threshold)
        let harvestability = Harvestability::determine_harvestability(
            Polarity::Negative,
            20.0,
            80.0,
            &triad,
            &lower,
        );
        assert_eq!(harvestability, Harvestability::NotYetHarvestable);
        assert!(!harvestability.is_harvestable());
    }

    #[test]
    fn test_harvestability_neutral() {
        let triad = TriadQuality::new(0.5, 0.5, 0.5);
        let lower = LowerIntensity::new(0.5, 0.5, 0.5);

        // Neutral polarity is never harvestable
        let harvestability =
            Harvestability::determine_harvestability(Polarity::Neutral, 50.0, 50.0, &triad, &lower);
        assert_eq!(harvestability, Harvestability::NotYetHarvestable);
        assert!(!harvestability.is_harvestable());
    }

    #[test]
    fn test_harvestability_description() {
        assert_eq!(
            Harvestability::HarvestablePositive.description(),
            "Harvestable - Ready for fourth density positive"
        );
        assert_eq!(
            Harvestability::HarvestableNegative.description(),
            "Harvestable - Ready for fourth density negative"
        );
        assert_eq!(
            Harvestability::NotYetHarvestable.description(),
            "Not yet harvestable - Continue development"
        );
    }

    #[test]
    fn test_harvestability_assessment_creation() {
        let triad = TriadQuality::new(0.6, 0.7, 0.8);
        let lower = LowerIntensity::new(0.3, 0.3, 0.3);
        let assessment =
            HarvestabilityAssessment::new(0.8, triad, lower, Harvestability::HarvestablePositive);

        assert!((assessment.violet_ray_quality - 0.8).abs() < 0.001);
        assert_eq!(
            assessment.harvestability,
            Harvestability::HarvestablePositive
        );
    }

    #[test]
    fn test_assessment_report() {
        let triad = TriadQuality::new(0.3, 0.3, 0.3);
        let lower = LowerIntensity::new(0.3, 0.3, 0.3);
        let assessment =
            HarvestabilityAssessment::new(0.5, triad, lower, Harvestability::NotYetHarvestable);

        let report = assessment.get_assessment_report();
        assert!((report.violet_ray_quality - 0.5).abs() < 0.001);
        assert!(!report.is_harvestable);
        assert!(!report.needs_work.is_empty());
    }
}
