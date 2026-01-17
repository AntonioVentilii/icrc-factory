use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    /// Payment canister ID. If not provided, the default cycles ledger canister ID will be used.
    pub cycles_ledger: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub enum Args {
    Init(InitArgs),
    Upgrade,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
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
