use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum UserCanisterKind {
    IcrcLedger,
    IcrcIndex,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct UserCanister {
    pub canister_id: Principal,
    pub kind: UserCanisterKind,
    pub installed: bool,
}
