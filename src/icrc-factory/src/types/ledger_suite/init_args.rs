use candid::{CandidType, Deserialize, Nat, Principal};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue as Value, icrc1::account::Account,
};
use serde::Serialize;

use crate::types::ledger_suite::common::FeatureFlags;

/// https://github.com/dfinity/ic/blob/6dcfafb491092704d374317d9a72a7ad2475d7c9/rs/ledger_suite/common/ledger_canister_core/src/archive.rs#L23
#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub struct ArchiveOptions {
    /// The number of blocks which, when exceeded, will trigger an archiving
    /// operation.
    pub trigger_threshold: usize,
    /// The number of blocks to archive when trigger threshold is exceeded.
    pub num_blocks_to_archive: usize,
    pub node_max_memory_size_bytes: Option<u64>,
    pub max_message_size_bytes: Option<u64>,
    pub controller_id: Principal,
    // More principals to add as controller of the archive.
    #[serde(default)]
    pub more_controller_ids: Option<Vec<Principal>>,
    // cycles to use for the call to create a new archive canister.
    #[serde(default)]
    pub cycles_for_archive_creation: Option<u64>,
    // Max transactions returned by the [get_transactions] endpoint.
    #[serde(default)]
    pub max_transactions_per_response: Option<u64>,
}

/// https://github.com/dfinity/ic/blob/6dcfafb491092704d374317d9a72a7ad2475d7c9/rs/ledger_suite/icrc1/ledger/src/lib.rs#L270
#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub struct InitArgs {
    pub minting_account: Account,
    pub fee_collector_account: Option<Account>,
    pub initial_balances: Vec<(Account, Nat)>,
    pub transfer_fee: Nat,
    pub decimals: Option<u8>,
    pub token_name: String,
    pub token_symbol: String,
    pub metadata: Vec<(String, Value)>,
    pub archive_options: ArchiveOptions,
    pub max_memo_length: Option<u16>,
    pub feature_flags: Option<FeatureFlags>,
    pub index_principal: Option<Principal>,
}
