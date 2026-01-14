use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};

pub async fn fetch_wasm_from_url(url: String) -> Result<HttpResponse, String> {
    let request_headers = vec![HttpHeader {
        name: "User-Agent".to_string(),
        value: "IC-Canister".to_string(),
    }];

    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        max_response_bytes: Some(2_000_000), // 2MB limit
        method: HttpMethod::GET,
        headers: request_headers,
        body: None,
        transform: Some(TransformContext::from_name(
            "transform_wasm_response".to_string(),
            vec![],
        )),
    };

    // Note: Outcalls require cycles. The amount depends on the size and number of nodes.
    // For simplicity, we assume the canister has enough cycles.
    let (response,) = http_request(request, 50_000_000_000) // 50B cycles placeholder
        .await
        .map_err(|(code, msg)| format!("HTTP request failed: {:?} - {}", code, msg))?;

    if response.status != 200u64 {
        return Err(format!(
            "HTTP error: status {} - message {:?}",
            response.status, response.body
        ));
    }

    Ok(response)
}
