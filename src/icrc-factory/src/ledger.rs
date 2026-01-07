use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FeatureFlags {
    pub icrc2: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveOptions {
    pub num_blocks_to_archive: u64,
    pub max_transactions_per_response: Option<u64>,
    pub trigger_threshold: u64,
    pub max_message_size_bytes: Option<u64>,
    pub cycles_for_archive_creation: Option<u64>,
    pub node_max_memory_size_bytes: Option<u64>,
    pub controller_id: Principal,
    pub more_controller_ids: Option<Vec<Principal>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArgs {
    pub minting_account: Account,
    pub fee_collector_account: Option<Account>,
    pub transfer_fee: Nat,
    pub decimals: Option<u8>,
    pub max_memo_length: Option<u16>,
    pub token_symbol: String,
    pub token_name: String,
    pub metadata: Vec<(String, MetadataValue)>,
    pub initial_balances: Vec<(Account, Nat)>,
    pub feature_flags: Option<FeatureFlags>,
    pub archive_options: ArchiveOptions,
    pub index_principal: Option<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum MetadataValue {
    Nat(Nat),
    Int(i128),
    Text(String),
    Blob(Vec<u8>),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeArgs {
    pub metadata: Option<Vec<(String, MetadataValue)>>,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub transfer_fee: Option<Nat>,
    pub change_fee_collector: Option<ChangeFeeCollector>,
    pub max_memo_length: Option<u16>,
    pub feature_flags: Option<FeatureFlags>,
    pub change_archive_options: Option<ChangeArchiveOptions>,
    pub index_principal: Option<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ChangeFeeCollector {
    Unset,
    SetTo(Account),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ChangeArchiveOptions {
    pub num_blocks_to_archive: Option<u64>,
    pub max_transactions_per_response: Option<u64>,
    pub trigger_threshold: Option<u64>,
    pub max_message_size_bytes: Option<u64>,
    pub cycles_for_archive_creation: Option<u64>,
    pub node_max_memory_size_bytes: Option<u64>,
    pub controller_id: Option<Principal>,
    pub more_controller_ids: Option<Vec<Principal>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum LedgerArg {
    Init(InitArgs),
    Upgrade(Option<UpgradeArgs>),
}

pub fn create_default_ledger_args(owner: Principal) -> LedgerArg {
    LedgerArg::Init(InitArgs {
        minting_account: Account {
            owner: owner.clone(),
            subaccount: None,
        },
        fee_collector_account: None,
        transfer_fee: Nat::from(10_000u64),
        decimals: Some(8),
        max_memo_length: Some(128),
        token_symbol: "TKN".to_string(),
        token_name: "Test Token".to_string(),
        metadata: vec![],
        initial_balances: vec![(
            Account {
                owner,
                subaccount: None,
            },
            Nat::from(1_000_000_000u64),
        )],
        feature_flags: Some(FeatureFlags { icrc2: true }),
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1000,
            max_transactions_per_response: Some(100),
            trigger_threshold: 500,
            max_message_size_bytes: Some(1024 * 1024),
            cycles_for_archive_creation: Some(10_000_000_000),
            node_max_memory_size_bytes: Some(1024 * 1024 * 1024),
            controller_id: ic_cdk::id(),
            more_controller_ids: None,
        },
        index_principal: None,
    })
}
