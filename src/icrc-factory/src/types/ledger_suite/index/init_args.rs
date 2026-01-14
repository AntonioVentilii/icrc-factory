use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::provisional::CanisterId;
use serde::Serialize;

/// https://github.com/dfinity/ic/blob/6dcfafb491092704d374317d9a72a7ad2475d7c9/rs/ledger_suite/icrc1/index/src/lib.rs#L92
#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub struct InitArgs {
    // The Ledger canister id of the Ledger to index.
    pub ledger_id: CanisterId,
}
