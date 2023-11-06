use thiserror::Error;

pub trait NewId<Id> {
    fn new(&self) -> Result<Id, NewIdError>;
}

#[derive(Debug, Error)]
#[error("unable to generate a new ID")]
pub struct NewIdError;
