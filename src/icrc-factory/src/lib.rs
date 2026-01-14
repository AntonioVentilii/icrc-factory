mod index;
mod ledger;
mod mgmt;
pub mod types;
mod wasm;

use candid::{Encode, Principal};
use ic_cdk::{
    api::management_canister::{
        http_request::{HttpResponse, TransformArgs},
        main::CanisterSettings,
    },
    caller, export_candid, id, query, update,
};

use crate::{
    index::create_default_index_init_args,
    ledger::{create_default_ledger_init_args, LedgerArgs},
    mgmt::{create_canister_with_ic_mgmt, install_wasm, upgrade_wasm},
    types::{
        ledger_suite::ledger::upgrade_args::UpgradeArgs,
        results::{
            create_canister::{CreateCanisterError, CreateCanisterResult, SetIndexCanisterResult},
            set_wasm::SetWasmResult,
        },
    },
    wasm::{index_wasm::get_stored_index_wasm, ledger_wasm::get_stored_ledger_wasm},
};

#[query]
fn transform_wasm_response(args: TransformArgs) -> HttpResponse {
    crate::wasm::utils::transform_wasm_response(args)
}

#[update]
async fn set_ledger_wasm(wasm: Vec<u8>) {
    crate::wasm::ledger_wasm::set_ledger_wasm(wasm);
}

#[update]
async fn set_ledger_wasm_from_url(url: String) -> SetWasmResult {
    crate::wasm::ledger_wasm::set_ledger_wasm_from_url(url)
        .await
        .into()
}

#[update]
async fn set_index_wasm(wasm: Vec<u8>) {
    crate::wasm::index_wasm::set_index_wasm(wasm);
}

#[update]
async fn set_index_wasm_from_url(url: String) -> SetWasmResult {
    crate::wasm::index_wasm::set_index_wasm_from_url(url)
        .await
        .into()
}

#[update]
async fn create_icrc_ledger() -> CreateCanisterResult {
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

    let init_args = create_default_ledger_init_args(caller);
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

#[update]
async fn create_icrc_index(ledger_id: Principal) -> CreateCanisterResult {
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

    let init_args = create_default_index_init_args(ledger_id);
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

#[update]
async fn set_index_canister(ledger_id: Principal, index_id: Principal) -> SetIndexCanisterResult {
    let ledger_wasm = get_stored_ledger_wasm();
    if ledger_wasm.is_empty() {
        return SetIndexCanisterResult::Err(CreateCanisterError::NoWasmStored);
    }

    let upgrade_arg = LedgerArgs::Upgrade(Some(UpgradeArgs {
        index_principal: Some(index_id),
        ..Default::default()
    }));

    let arg = match Encode!(&upgrade_arg) {
        Ok(arg) => arg,
        Err(e) => {
            return SetIndexCanisterResult::Err(CreateCanisterError::InitArgsEncodingFailed(
                format!("Failed to encode upgrade args: {}", e),
            ))
        }
    };

    if let Err(err) = upgrade_wasm(ledger_id, ledger_wasm, arg).await {
        return SetIndexCanisterResult::Err(CreateCanisterError::WasmInstallationFailed(err));
    }

    SetIndexCanisterResult::Ok()
}

export_candid!();
