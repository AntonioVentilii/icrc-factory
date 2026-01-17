use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

/// <https://github.com/dfinity/ic/blob/e446c64d99a97e38166be23ff2bfade997d15ff7/rs/ledger_suite/icrc1/index-ng/src/lib.rs#L23>
#[derive(Clone, Eq, PartialEq, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct UpgradeArgs {
    pub ledger_id: Option<Principal>,
    pub retrieve_blocks_from_ledger_interval_seconds: Option<u64>,
}
