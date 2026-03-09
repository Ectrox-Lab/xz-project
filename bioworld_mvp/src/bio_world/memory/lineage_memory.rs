use super::constants::{MAX_DISTILLED_LESSONS, MUTATION_RATE};
use crate::bio_world::engine::world::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct DistilledLesson {
    pub key: String,
    pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineageMemory {
    pub lineage_id: u64,
    pub preferred_strategy: String,
    pub adaptation_bias: f32,
    pub distilled_lessons: Vec<DistilledLesson>,
    pub total_births: u64,
    pub total_deaths: u64,
}

impl LineageMemory {
    pub fn new(lineage_id: u64) -> Self {
        Self {
            lineage_id,
            preferred_strategy: "balanced".to_string(),
            adaptation_bias: 0.5,
            distilled_lessons: Vec::new(),
            total_births: 0,
            total_deaths: 0,
        }
    }

    pub fn mutate(&mut self, rng: &mut Rng) {
        if rng.bool(MUTATION_RATE as f64) {
            self.adaptation_bias = (self.adaptation_bias + rng.normal(0.08) as f32).clamp(0.0, 1.0);
            let roll = rng.f64();
            self.preferred_strategy = if roll < 0.33 {
                "cooperate".into()
            } else if roll < 0.66 {
                "explore".into()
            } else {
                "balanced".into()
            };
        }
    }

    pub fn record_death(&mut self, _cause: &str) {
        self.total_deaths += 1;
    }

    pub fn inherit_from(parent: &LineageMemory, rng: &mut Rng) -> Self {
        let mut child = parent.clone();
        child.total_births += 1;
        child.mutate(rng);
        child
    }

    pub fn push_lesson(&mut self, lesson: DistilledLesson) {
        self.distilled_lessons.push(lesson);
        if self.distilled_lessons.len() > MAX_DISTILLED_LESSONS {
            self.distilled_lessons.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bio_world::engine::world::Rng;

    use super::LineageMemory;

    #[test]
    fn test_max_lessons_enforced() {
        let mut lineage = LineageMemory::new(1);
        for i in 0..20 {
            lineage.push_lesson(super::DistilledLesson {
                key: format!("k{}", i),
                value: i as f32,
            });
        }
        assert_eq!(lineage.distilled_lessons.len(), 5);
    }

    #[test]
    fn test_mutation_rate() {
        let lineage = LineageMemory::new(1);
        let original = lineage.preferred_strategy.clone();

        let mut mutation_count = 0;
        let mut rng = Rng::new(7);
        for _ in 0..1000 {
            let child = LineageMemory::inherit_from(&lineage, &mut rng);
            if child.preferred_strategy != original {
                mutation_count += 1;
            }
        }

        assert!(mutation_count > 30 && mutation_count < 70);
    }
}
