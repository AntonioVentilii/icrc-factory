use candid::CandidType;
use ic_cdk::api::management_canister::provisional::CanisterId;
use serde::{Deserialize, Serialize};

use crate::types::ledger_suite::index::init_args::InitArgs;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum IndexArgs {
    Init(InitArgs),
}

pub fn create_default_index_init_args(ledger_id: CanisterId) -> IndexArgs {
    IndexArgs::Init(InitArgs {
        ledger_id,
        retrieve_blocks_from_ledger_interval_seconds: None,
    })
}
