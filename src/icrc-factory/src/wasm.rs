use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformContext, TransformArgs,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

thread_local! {
    static WASM_STORAGE: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

pub fn get_stored_wasm() -> Vec<u8> {
    WASM_STORAGE.with(|storage| storage.borrow().clone())
}

pub fn set_wasm(wasm: Vec<u8>) {
    WASM_STORAGE.with(|storage| {
        *storage.borrow_mut() = wasm;
    });
}

#[ic_cdk::update]
pub async fn fetch_wasm_from_url(url: String) -> Result<usize, String> {
    let request_headers = vec![
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "IC-Canister".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        max_response_bytes: Some(2_000_000), // 2MB limit
        method: HttpMethod::GET,
        headers: request_headers,
        body: None,
        transform: Some(TransformContext::from_name("transform_wasm_response".to_string(), vec![])),
    };

    // Note: Outcalls require cycles. The amount depends on the size and number of nodes.
    // For simplicity, we assume the canister has enough cycles.
    let (response,) = http_request(request, 50_000_000_000) // 50B cycles placeholder
        .await
        .map_err(|(code, msg)| format!("HTTP request failed: {:?} - {}", code, msg))?;

    if response.status != 200u64 {
        return Err(format!("HTTP error: status {}", response.status));
    }

    set_wasm(response.body.clone());
    Ok(response.body.len())
}

#[ic_cdk::query]
pub fn transform_wasm_response(args: TransformArgs) -> HttpResponse {
    let mut res = args.response;
    // We don't remove headers or change the body for WASM typically, 
    // but some cleansing might be needed if nodes return slightly different headers.
    res.headers = vec![];
    res
}
