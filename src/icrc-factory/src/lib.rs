mod ledger;
mod mgmt;
mod wasm;

use candid::{Encode, Principal};
use ic_cdk::api::management_canister::main::CanisterSettings;

use crate::{
    ledger::create_default_ledger_args,
    mgmt::{create_canister_with_ic_mgmt, install_wasm},
    wasm::get_stored_wasm,
};

#[ic_cdk::update]
async fn create_icrc_ledger(wasm_module: Option<Vec<u8>>, cycles: u128) -> Result<Principal, String> {
    let caller = ic_cdk::caller();

    // 1. Get the WASM module (either from args or storage)
    let final_wasm = match wasm_module {
        Some(w) if !w.is_empty() => w,
        _ => {
            let stored = get_stored_wasm();
            if stored.is_empty() {
                return Err("No WASM module provided and no WASM stored in factory.".to_string());
            }
            stored
        }
    };

    // 2. Create the canister
    let settings = CanisterSettings {
        controllers: Some(vec![ic_cdk::id(), caller]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: None,
        log_visibility: None,
        wasm_memory_limit: None,
    };

    let canister_id = create_canister_with_ic_mgmt(Some(settings), cycles).await?;

    // 3. Prepare initialization arguments
    let init_args = create_default_ledger_args(caller);
    let arg = Encode!(&init_args).map_err(|e| format!("Failed to encode init args: {}", e))?;

    // 4. Install the WASM
    install_wasm(canister_id, final_wasm, arg).await?;

    Ok(canister_id)
}

#[ic_cdk::update]
async fn upload_wasm(wasm: Vec<u8>) {
    crate::wasm::set_wasm(wasm);
}

#[ic_cdk::update]
async fn fetch_wasm(url: String) -> Result<usize, String> {
    crate::wasm::fetch_wasm_from_url(url).await
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn transform_wasm_response(args: ic_cdk::api::management_canister::http_request::TransformArgs) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    crate::wasm::transform_wasm_response(args)
}
