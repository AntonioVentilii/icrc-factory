use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::ledger_suite::{
    common::FeatureFlags,
    ledger::{
        init_args::{ArchiveOptions, InitArgs},
        upgrade_args::UpgradeArgs,
    },
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum LedgerArgs {
    Init(InitArgs),
    Upgrade(Option<UpgradeArgs>),
}

pub fn create_default_ledger_init_args(owner: Principal) -> LedgerArgs {
    LedgerArgs::Init(InitArgs {
        token_symbol: "TKN".to_string(),
        token_name: "Test Token".to_string(),
        transfer_fee: Nat::from(10_000u64),
        decimals: Some(8),
        metadata: vec![],
        feature_flags: Some(FeatureFlags { icrc2: true }),
        minting_account: Account {
            owner,
            subaccount: None,
        },
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
