use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum CreateCanisterError {
    NoWasmStored,
    CanisterCreationFailed(String),
    InitArgsEncodingFailed(String),
    WasmInstallationFailed(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum CreateCanisterResult {
    Ok(Principal),
    Err(CreateCanisterError),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum SetIndexCanisterResult {
    Ok(),
    Err(CreateCanisterError),
}
