use crate::error::encryption::EncryptionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateIdentityConfigError {
    #[error("Failed to generate a fresh encryption configuration: {0}")]
    GenerateFreshEncryptionConfigurationFailed(EncryptionError),
}
