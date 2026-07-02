use crate::errors::DomainError;

pub struct MigrateApplyInput;

pub struct MigrateApplyOutput {
    pub applied: usize,
}

pub struct MigrateApplyUseCase;

impl MigrateApplyUseCase {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn execute(&self, _port: MigrateApplyInput) -> Result<MigrateApplyOutput, DomainError> {
        Err(DomainError::NotImplemented(
            "migrate apply requires database connectivity (planned for a future milestone)"
                .to_owned(),
        ))
    }
}
