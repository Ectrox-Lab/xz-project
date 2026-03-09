use std::collections::VecDeque;

use crate::bio_world::engine::world::Rng;

use super::constants::{ARCHIVE_SAMPLE_PROBABILITY, MAX_ARCHIVE_WRITE_RATE};
use super::lineage_memory::DistilledLesson;

#[derive(Clone, Debug, PartialEq)]
pub struct CausalArchiveRecord {
    pub generation: u32,
    pub lineage_id: u64,
    pub event_type: String,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArchiveSamplingPolicy {
    pub sample_probability: f32,
    pub samples_per_lifetime: u32,
}

impl Default for ArchiveSamplingPolicy {
    fn default() -> Self {
        Self {
            sample_probability: ARCHIVE_SAMPLE_PROBABILITY,
            samples_per_lifetime: 1,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CausalArchive {
    pub records: Vec<CausalArchiveRecord>,
    pub write_queue: VecDeque<CausalArchiveRecord>,
    pub writes_this_window: u32,
}

impl CausalArchive {
    pub fn queue_record(&mut self, record: CausalArchiveRecord) {
        self.write_queue.push_back(record);
    }

    pub fn process_queue(&mut self) {
        while self.writes_this_window < MAX_ARCHIVE_WRITE_RATE {
            let Some(record) = self.write_queue.pop_front() else {
                break;
            };
            self.records.push(record);
            self.writes_this_window += 1;
        }
    }

    pub fn reset_rate_window(&mut self) {
        self.writes_this_window = 0;
    }

    pub fn compress_old_records(&mut self) {
        const HARD_LIMIT: usize = 5_000;
        if self.records.len() > HARD_LIMIT {
            let drain_count = self.records.len() - HARD_LIMIT;
            self.records.drain(0..drain_count);
        }
    }

    pub fn random_sample<'a>(
        &'a self,
        rng: &mut Rng,
        policy: &ArchiveSamplingPolicy,
    ) -> Option<&'a CausalArchiveRecord> {
        if policy.sample_probability > ARCHIVE_SAMPLE_PROBABILITY {
            return None;
        }
        if self.records.is_empty() || !rng.bool(policy.sample_probability as f64) {
            return None;
        }
        let idx = (rng.next_u64() as usize) % self.records.len();
        self.records.get(idx)
    }

    pub fn compress_to_lesson(record: &CausalArchiveRecord) -> DistilledLesson {
        DistilledLesson {
            key: format!("{}:{}", record.event_type, record.lineage_id),
            value: 1.0,
        }
    }

    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    pub fn write_rate(&self) -> u32 {
        self.writes_this_window
    }
}

#[cfg(test)]
mod tests {
    use super::ArchiveSamplingPolicy;

    #[test]
    fn test_sampling_probability_enforced() {
        let policy = ArchiveSamplingPolicy::default();
        assert!(policy.sample_probability <= 0.01);
    }
}
