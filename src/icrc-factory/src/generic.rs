use candid::Encode;
use ic_cdk::{api::management_canister::main::CanisterSettings, caller, id};
use icrc_ledger_types::icrc1::account::Account;

use crate::{
    index::create_default_index_init_args,
    ledger::create_default_ledger_init_args,
    mgmt::{create_canister_with_ic_mgmt, install_wasm},
    types::{
        args::create_canister::{CreateIcrcIndexArgs, CreateIcrcLedgerArgs},
        results::create_canister::{CreateCanisterError, CreateCanisterResult},
    },
    wasm::{index_wasm::get_stored_index_wasm, ledger_wasm::get_stored_ledger_wasm},
};

pub async fn create_icrc_ledger(args: CreateIcrcLedgerArgs) -> CreateCanisterResult {
    let cycles = 1_000_000_000_000u128;

    let caller = caller();

    let ledger_wasm = get_stored_ledger_wasm();
    if ledger_wasm.is_empty() {
        return CreateCanisterResult::Err(CreateCanisterError::NoWasmStored);
    }

    let settings = CanisterSettings {
        controllers: Some(vec![id(), caller]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: None,
        log_visibility: None,
        wasm_memory_limit: None,
    };

    let canister_id = match create_canister_with_ic_mgmt(Some(settings), cycles).await {
        Ok(id) => id,
        Err(err) => {
            return CreateCanisterResult::Err(CreateCanisterError::CanisterCreationFailed(err))
        }
    };

    let symbol = args.symbol.unwrap_or_else(|| "TKN".to_string());
    let name = args.name.unwrap_or_else(|| "ICRC Token".to_string());

    let transfer_fee = args.transfer_fee.unwrap_or(10_000u64);
    let decimals = args.decimals.unwrap_or(8);

    let minting_account = args.minting_account.unwrap_or(Account {
        owner: caller,
        subaccount: None,
    });

    let init_args =
        create_default_ledger_init_args(symbol, name, transfer_fee, decimals, minting_account);
    let arg = match Encode!(&init_args) {
        Ok(arg) => arg,
        Err(e) => {
            return CreateCanisterResult::Err(CreateCanisterError::InitArgsEncodingFailed(format!(
                "Failed to encode init args: {}",
                e
            )))
        }
    };

    if let Err(err) = install_wasm(canister_id, ledger_wasm, arg).await {
        return CreateCanisterResult::Err(CreateCanisterError::WasmInstallationFailed(err));
    }

    CreateCanisterResult::Ok(canister_id)
}

pub async fn create_icrc_index(args: CreateIcrcIndexArgs) -> CreateCanisterResult {
    let cycles = 1_000_000_000_000u128;

    let caller = caller();

    let index_wasm = get_stored_index_wasm();
    if index_wasm.is_empty() {
        return CreateCanisterResult::Err(CreateCanisterError::NoWasmStored);
    }

    let settings = CanisterSettings {
        controllers: Some(vec![id(), caller]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: None,
        log_visibility: None,
        wasm_memory_limit: None,
    };

    let canister_id = match create_canister_with_ic_mgmt(Some(settings), cycles).await {
        Ok(id) => id,
        Err(err) => {
            return CreateCanisterResult::Err(CreateCanisterError::CanisterCreationFailed(err))
        }
    };

    let init_args = create_default_index_init_args(args.ledger_id);
    let arg = match Encode!(&init_args) {
        Ok(arg) => arg,
        Err(e) => {
            return CreateCanisterResult::Err(CreateCanisterError::InitArgsEncodingFailed(format!(
                "Failed to encode init args: {}",
                e
            )))
        }
    };

    if let Err(err) = install_wasm(canister_id, index_wasm, arg).await {
        return CreateCanisterResult::Err(CreateCanisterError::WasmInstallationFailed(err));
    }

    CreateCanisterResult::Ok(canister_id)
}
