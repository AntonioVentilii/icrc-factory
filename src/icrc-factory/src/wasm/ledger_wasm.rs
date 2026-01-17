use crate::{
    state::{mutate_state, read_state},
    wasm::utils::fetch_wasm_from_url,
};

pub fn get_stored_ledger_wasm() -> Vec<u8> {
    read_state(|s| s.icrc_ledger_wasm.get().to_vec())
}

pub fn set_ledger_wasm(wasm: Vec<u8>) {
    mutate_state(|s| {
        s.icrc_ledger_wasm.set(wasm);
    })
}

pub async fn set_ledger_wasm_from_url(url: String) -> Result<usize, String> {
    let response = fetch_wasm_from_url(url).await?;
    let len = response.body.len();
    set_ledger_wasm(response.body);
    Ok(len)
}
