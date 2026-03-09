use super::constants::ARCHIVE_SAMPLE_PROBABILITY;

#[derive(Clone, Debug, PartialEq)]
pub enum Accessor {
    Cell(u64),
    Lineage(u64),
    Archive,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    CellMemory(u64),
    CellStrategy(u64),
    LineageMemory(u64),
    Archive,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccessMode {
    Read,
    Write,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccessError {
    Forbidden,
    InvalidSamplingProbability,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccessRequest {
    pub accessor: Accessor,
    pub target: Target,
    pub mode: AccessMode,
    pub sample_probability: Option<f32>,
}

#[derive(Clone, Debug, Default)]
pub struct MemoryAccessGuard {
    pub access_log: Vec<String>,
}

impl MemoryAccessGuard {
    pub fn validate(&mut self, request: AccessRequest) -> Result<(), AccessError> {
        if let Some(prob) = request.sample_probability {
            if prob > ARCHIVE_SAMPLE_PROBABILITY {
                return Err(AccessError::InvalidSamplingProbability);
            }
        }

        match (&request.accessor, &request.target, &request.mode) {
            (Accessor::Cell(_), Target::Archive, _) => Err(AccessError::Forbidden),
            (Accessor::Cell(cell), Target::CellMemory(target), _) if cell != target => {
                Err(AccessError::Forbidden)
            }
            (Accessor::Archive, Target::CellMemory(_), AccessMode::Write) => {
                Err(AccessError::Forbidden)
            }
            (Accessor::Archive, Target::CellStrategy(_), AccessMode::Write) => {
                Err(AccessError::Forbidden)
            }
            _ => {
                self.access_log.push(format!("{:?}", request));
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessError, AccessMode, AccessRequest, Accessor, MemoryAccessGuard, Target};

    #[test]
    fn test_cell_cannot_access_archive() {
        let mut guard = MemoryAccessGuard::default();
        let result = guard.validate(AccessRequest {
            accessor: Accessor::Cell(1),
            target: Target::Archive,
            mode: AccessMode::Read,
            sample_probability: None,
        });

        assert_eq!(result, Err(AccessError::Forbidden));
    }

    #[test]
    fn test_archive_cannot_overwrite_cell() {
        let mut guard = MemoryAccessGuard::default();
        let result = guard.validate(AccessRequest {
            accessor: Accessor::Archive,
            target: Target::CellMemory(1),
            mode: AccessMode::Write,
            sample_probability: None,
        });
        assert_eq!(result, Err(AccessError::Forbidden));
    }

    #[test]
    fn test_archive_cannot_inject_cell_strategy() {
        let mut guard = MemoryAccessGuard::default();
        let result = guard.validate(AccessRequest {
            accessor: Accessor::Archive,
            target: Target::CellStrategy(1),
            mode: AccessMode::Write,
            sample_probability: None,
        });
        assert_eq!(result, Err(AccessError::Forbidden));
    }

    #[test]
    fn test_sampling_probability_enforced() {
        let mut guard = MemoryAccessGuard::default();
        let result = guard.validate(AccessRequest {
            accessor: Accessor::Lineage(1),
            target: Target::Archive,
            mode: AccessMode::Read,
            sample_probability: Some(0.02),
        });

        assert_eq!(result, Err(AccessError::InvalidSamplingProbability));
    }
}
