use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("the crate '{0}' does not exist")]
    CrateDoesNotExist(String),
}
