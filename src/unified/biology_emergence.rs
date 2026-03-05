//! Biology Emergence - Life from Field

use super::physics_emergence::EmergentMolecule;
use super::unified_field::UnifiedField;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BiologyEmergenceConfig {
    pub dna_threshold: f64,
    pub cell_threshold: f64,
    pub min_cells_for_organism: usize,
}

impl Default for BiologyEmergenceConfig {
    fn default() -> Self {
        Self {
            dna_threshold: 0.8,
            cell_threshold: 0.85,
            min_cells_for_organism: 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmergentDNA {
    pub id: String,
    pub sequence: Vec<u8>,
    pub pattern: [f64; 22],
    pub coherence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Prokaryote,
    Eukaryote,
    Plant,
    Animal,
}

#[derive(Debug, Clone)]
pub struct LivingCell {
    pub id: String,
    pub dna: EmergentDNA,
    pub cell_type: CellType,
    pub energy: f64,
    pub alive: bool,
}

#[derive(Debug, Clone)]
pub struct BiologicalOrganism {
    pub id: String,
    pub cells: Vec<LivingCell>,
    pub species: String,
    pub complexity: f64,
}

pub struct BiologyEmergence {
    #[allow(dead_code)]
    field: Arc<UnifiedField>,
    config: BiologyEmergenceConfig,
    dna_patterns: Vec<EmergentDNA>,
    cells: Vec<LivingCell>,
}

impl BiologyEmergence {
    pub fn new(field: Arc<UnifiedField>) -> Self {
        Self {
            field,
            config: BiologyEmergenceConfig::default(),
            dna_patterns: Vec::new(),
            cells: Vec::new(),
        }
    }

    pub fn derive_dna(&mut self, molecules: &[EmergentMolecule]) -> Vec<EmergentDNA> {
        self.dna_patterns.clear();

        if molecules.is_empty() {
            return Vec::new();
        }

        let mut pattern = [0.0; 22];
        for mol in molecules {
            for (i, _) in mol.atoms.iter().enumerate().take(22) {
                pattern[i] += mol.bond_energy;
            }
        }

        let count = molecules.len() as f64;
        for p in &mut pattern {
            *p = (*p / count).min(1.0);
        }

        let avg_pattern: f64 = pattern.iter().sum::<f64>() / 22.0;
        if avg_pattern > self.config.dna_threshold {
            self.dna_patterns.push(EmergentDNA {
                id: "dna-0".to_string(),
                sequence: vec![0; 100],
                pattern,
                coherence: avg_pattern,
            });
        }

        self.dna_patterns.clone()
    }

    pub fn derive_cells(&mut self, dna_list: &[EmergentDNA]) -> Vec<LivingCell> {
        self.cells.clear();

        for (i, dna) in dna_list.iter().enumerate() {
            if dna.coherence > self.config.cell_threshold {
                self.cells.push(LivingCell {
                    id: format!("cell-{}", i),
                    dna: dna.clone(),
                    cell_type: CellType::Prokaryote,
                    energy: dna.coherence,
                    alive: true,
                });
            }
        }

        self.cells.clone()
    }

    pub fn derive_organisms(&self) -> Vec<BiologicalOrganism> {
        if self.cells.len() >= self.config.min_cells_for_organism {
            vec![BiologicalOrganism {
                id: "organism-0".to_string(),
                cells: self.cells.clone(),
                species: "emergent".to_string(),
                complexity: self.cells.len() as f64,
            }]
        } else {
            Vec::new()
        }
    }

    pub fn emerge_all(&mut self, molecules: &[EmergentMolecule]) -> BiologyEmergenceResult {
        let dna = self.derive_dna(molecules);
        let cells = self.derive_cells(&dna);
        let organisms = self.derive_organisms();

        BiologyEmergenceResult {
            dna_patterns: dna,
            cells,
            organisms,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BiologyEmergenceResult {
    pub dna_patterns: Vec<EmergentDNA>,
    pub cells: Vec<LivingCell>,
    pub organisms: Vec<BiologicalOrganism>,
}

pub struct BiologyPhysicsBridge {
    physics: super::physics_emergence::PhysicsEmergence,
    biology: BiologyEmergence,
}

impl BiologyPhysicsBridge {
    pub fn new(field: Arc<UnifiedField>) -> Self {
        Self {
            physics: super::physics_emergence::PhysicsEmergence::new(field.clone()),
            biology: BiologyEmergence::new(field),
        }
    }

    pub fn process(
        &mut self,
    ) -> (
        super::physics_emergence::PhysicsEmergenceResult,
        BiologyEmergenceResult,
    ) {
        let physics_result = self.physics.emerge_all();
        let biology_result = self.biology.emerge_all(&physics_result.molecules);

        (physics_result, biology_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biology_emergence() {
        let field = Arc::new(UnifiedField::new("test".to_string()));
        let biology = BiologyEmergence::new(field);
        assert!(biology.dna_patterns.is_empty());
    }
}
