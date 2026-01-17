use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

/// <https://github.com/dfinity/ic/blob/e446c64d99a97e38166be23ff2bfade997d15ff7/rs/ledger_suite/icrc1/index-ng/src/lib.rs#L17>
#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub struct InitArgs {
    pub ledger_id: Principal,
    pub retrieve_blocks_from_ledger_interval_seconds: Option<u64>,
}
