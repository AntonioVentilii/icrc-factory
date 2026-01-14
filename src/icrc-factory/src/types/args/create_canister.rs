use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateIcrcLedgerArgs {
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub transfer_fee: Option<u64>,
    pub decimals: Option<u8>,
    pub minting_account: Option<Account>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateIcrcIndexArgs {
    pub ledger_id: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct SetIndexCanisterArgs {
    pub ledger_id: Principal,
    pub index_id: Principal,
}
