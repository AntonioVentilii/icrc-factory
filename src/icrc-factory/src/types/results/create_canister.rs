use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum CreateCanisterError {
    NoWasmStored,
    CanisterCreationFailed(String),
    InitArgsEncodingFailed(String),
    WasmInstallationFailed(String),
    PaymentError(ic_papi_api::PaymentError),
}

#[derive(CandidType, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum CreateCanisterResult {
    Ok(Principal),
    Err(CreateCanisterError),
}

#[derive(CandidType, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum SetCanisterResult {
    Ok(),
    Err(CreateCanisterError),
}
