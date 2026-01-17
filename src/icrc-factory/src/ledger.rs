use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::ledger_suite::{
    common::FeatureFlags,
    ledger::{
        init_args::{ArchiveOptions, InitArgs},
        upgrade_args::UpgradeArgs,
    },
};

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub enum LedgerArgs {
    Init(InitArgs),
    Upgrade(Option<UpgradeArgs>),
}

pub fn create_default_ledger_init_args(
    symbol: String,
    name: String,
    transfer_fee: u64,
    decimals: u8,
    minting_account: Account,
) -> LedgerArgs {
    LedgerArgs::Init(InitArgs {
        token_symbol: symbol,
        token_name: name,
        transfer_fee: Nat::from(transfer_fee),
        decimals: Some(decimals),
        metadata: vec![],
        feature_flags: Some(FeatureFlags { icrc2: true }),
        minting_account,
        initial_balances: vec![],
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1_000,
            trigger_threshold: 2_000,
            controller_id: ic_cdk::id(),
            cycles_for_archive_creation: Some(10_000_000_000_000),
            max_transactions_per_response: None,
            max_message_size_bytes: None,
            node_max_memory_size_bytes: None,
            more_controller_ids: None,
        },
        index_principal: None,
        fee_collector_account: None,
        max_memo_length: None,
    })
}
