mod ledger;
mod mgmt;
mod types;
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
    ledger::create_default_ledger_init_args,
    mgmt::{create_canister_with_ic_mgmt, install_wasm},
    wasm::ledger_wasm::get_stored_ledger_wasm,
};

#[update]
async fn set_ledger_wasm(wasm: Vec<u8>) {
    crate::wasm::ledger_wasm::set_ledger_wasm(wasm);
}

#[update]
async fn set_ledger_wasm_from_url(url: String) -> Result<usize, String> {
    crate::wasm::ledger_wasm::set_ledger_wasm_from_url(url).await
}

#[update]
async fn set_index_wasm(wasm: Vec<u8>) {
    crate::wasm::index_wasm::set_index_wasm(wasm);
}

#[update]
async fn set_index_wasm_from_url(url: String) -> Result<usize, String> {
    crate::wasm::index_wasm::set_index_wasm_from_url(url).await
}

#[update]
async fn create_icrc_ledger() -> Result<Principal, String> {
    let cycles = 1_000_000_000_000u128;

    let caller = caller();

    let ledger_wasm = get_stored_ledger_wasm();
    if ledger_wasm.is_empty() {
        return Err("No WASM stored in factory. Upload or fetch it first.".to_string());
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

    let canister_id = create_canister_with_ic_mgmt(Some(settings), cycles).await?;

    let init_args = create_default_ledger_init_args(caller);
    let arg = Encode!(&init_args).map_err(|e| format!("Failed to encode init args: {}", e))?;

    install_wasm(canister_id, ledger_wasm, arg).await?;

    Ok(canister_id)
}

export_candid!();
