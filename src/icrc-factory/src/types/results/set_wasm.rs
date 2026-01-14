use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum SetWasmResult {
    Ok(usize),
    Err(String),
}
impl From<Result<usize, String>> for SetWasmResult {
    fn from(result: Result<usize, String>) -> Self {
        match result {
            Ok(res) => SetWasmResult::Ok(res),
            Err(err) => SetWasmResult::Err(err),
        }
    }
}
