use crate::{
    action::ActionWrapper,
    network::{actions::ActionResponse, reducers::send, state::NetworkState},
    state::State,
};
use holochain_core_types::{entry::EntryWithMetaAndHeader, error::HolochainError};
use holochain_net::connection::json_protocol::{
    FetchEntryData, FetchEntryResultData, JsonProtocol,
};

/// Send back to network a HandleFetchEntryResult, no matter what.
/// Will return an empty content field if it actually doesn't have the data.
fn reduce_respond_fetch_data_inner(
    network_state: &mut NetworkState,
    get_dht_data: &FetchEntryData,
    maybe_entry: &Option<EntryWithMetaAndHeader>,
) -> Result<(), HolochainError> {
    network_state.initialized()?;

    send(
        network_state,
        JsonProtocol::HandleFetchEntryResult(FetchEntryResultData {
            request_id: get_dht_data.request_id.clone(),
            requester_agent_id: get_dht_data.requester_agent_id.clone(),
            dna_address: network_state.dna_address.clone().unwrap(),
            provider_agent_id: network_state.agent_id.clone().unwrap(),
            entry_address: get_dht_data.entry_address.clone(),
            entry_content: serde_json::from_str(
                &serde_json::to_string(&maybe_entry.clone()).unwrap(),
            )
            .unwrap(),
        }),
    )
}

pub fn reduce_respond_fetch_data(
    network_state: &mut NetworkState,
    _root_state: &State,
    action_wrapper: &ActionWrapper,
) {
    let action = action_wrapper.action();
    let (get_dht_data, maybe_entry) = unwrap_to!(action => crate::action::Action::RespondFetch);
    let result = reduce_respond_fetch_data_inner(network_state, get_dht_data, maybe_entry);
    network_state.actions.insert(
        action_wrapper.clone(),
        ActionResponse::RespondFetch(match result {
            Ok(_) => Ok(()),
            Err(e) => Err(HolochainError::ErrorGeneric(e.to_string())),
        }),
    );
}
