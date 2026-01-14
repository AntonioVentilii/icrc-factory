use std::cell::RefCell;

use crate::wasm::wasm::fetch_wasm_from_url;

thread_local! {
    static ICRC_INDEX_WASM_STORAGE: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

pub fn get_stored_index_wasm() -> Vec<u8> {
    ICRC_INDEX_WASM_STORAGE.with(|storage| storage.borrow().clone())
}

pub fn set_index_wasm(wasm: Vec<u8>) {
    ICRC_INDEX_WASM_STORAGE.with(|storage| {
        *storage.borrow_mut() = wasm;
    });
}

pub async fn set_index_wasm_from_url(url: String) -> Result<usize, String> {
    let response = fetch_wasm_from_url(url).await?;

    set_index_wasm(response.body.clone());
    Ok(response.body.len())
}
