use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub struct InitArgs {
    /// Payment canister ID.
    pub cycles_ledger: Option<Principal>,
}

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub enum Args {
    Init(InitArgs),
    Upgrade,
}

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize, Serialize)]
pub struct Config {
    /// Payment canister ID.
    pub cycles_ledger: Principal,
}

impl From<InitArgs> for Config {
    /// Creates a new `Config` from the provided `InitArgs`.
    fn from(arg: InitArgs) -> Self {
        let InitArgs { cycles_ledger } = arg;
        let cycles_ledger =
            cycles_ledger.unwrap_or_else(ic_papi_api::cycles::cycles_ledger_canister_id);
        Config { cycles_ledger }
    }
}
