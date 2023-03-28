use crate::lib::error::fetch_root_key::FetchRootKeyError;

use dfx_core::config::model::network_descriptor::NetworkDescriptor;
use ic_agent::Agent;

pub async fn fetch_root_key_if_needed(
    agent: &Agent,
    network_descriptor: &NetworkDescriptor,
) -> Result<(), FetchRootKeyError> {
    if !network_descriptor.is_ic {
        agent
            .fetch_root_key()
            .await
            .map_err(FetchRootKeyError::ReplicaError)?;
    }
    Ok(())
}

/// Fetches the root key of the local network.
/// Returns an error if attempted to run on the real IC.
pub async fn fetch_root_key_or_anyhow(
    agent: &Agent,
    network_descriptor: &NetworkDescriptor,
) -> Result<(), FetchRootKeyError> {
    if !network_descriptor.is_ic {
        agent
            .fetch_root_key()
            .await
            .map_err(FetchRootKeyError::ReplicaError)
    } else {
        Err(FetchRootKeyError::NotLocal)
    }
}
