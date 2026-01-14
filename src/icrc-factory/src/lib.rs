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
    wasm::{fetch_wasm_from_url, get_stored_wasm, set_wasm},
};

#[update]
async fn create_icrc_ledger(cycles: u128) -> Result<Principal, String> {
    let caller = caller();

    let ledger_wasm = get_stored_wasm();
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

#[update]
async fn upload_wasm(wasm: Vec<u8>) {
    set_wasm(wasm);
}

#[update]
async fn fetch_wasm(url: String) -> Result<usize, String> {
    fetch_wasm_from_url(url).await
}

#[query]
fn transform_wasm_response(args: TransformArgs) -> HttpResponse {
    crate::wasm::transform_wasm_response(args)
}

export_candid!();
