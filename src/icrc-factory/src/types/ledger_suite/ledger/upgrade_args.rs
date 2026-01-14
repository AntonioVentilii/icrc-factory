use candid::{types::number::Nat, CandidType, Deserialize, Principal};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue as Value, icrc1::account::Account,
};
use serde::Serialize;

use crate::types::ledger_suite::common::FeatureFlags;

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub enum ChangeFeeCollector {
    Unset,
    SetTo(Account),
}

#[derive(Clone, Eq, PartialEq, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct ChangeArchiveOptions {
    pub trigger_threshold: Option<usize>,
    pub num_blocks_to_archive: Option<usize>,
    pub node_max_memory_size_bytes: Option<u64>,
    pub max_message_size_bytes: Option<u64>,
    pub controller_id: Option<Principal>,
    pub more_controller_ids: Option<Vec<Principal>>,
    pub cycles_for_archive_creation: Option<u64>,
    pub max_transactions_per_response: Option<u64>,
}

/// https://github.com/dfinity/ic/blob/e446c64d99a97e38166be23ff2bfade997d15ff7/rs/ledger_suite/icrc1/ledger/src/lib.rs#L342
#[derive(Clone, Eq, PartialEq, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct UpgradeArgs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<(String, Value)>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_fee: Option<Nat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_fee_collector: Option<ChangeFeeCollector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_memo_length: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<FeatureFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_archive_options: Option<ChangeArchiveOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_principal: Option<Principal>,
}
