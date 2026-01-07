use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::{
    create_canister, install_code, CanisterIdRecord, CanisterInstallMode, CanisterSettings,
    CreateCanisterArgument, InstallCodeArgument,
};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct CreateCanisterWithCmcArgs {
    pub settings: Option<CanisterSettings>,
}

pub async fn create_canister_with_ic_mgmt(
    settings: Option<CanisterSettings>,
    cycles: u128,
) -> Result<Principal, String> {
    let args = CreateCanisterArgument { settings };
    create_canister(args, cycles)
        .await
        .map(|(CanisterIdRecord { canister_id },)| canister_id)
        .map_err(|(code, msg)| format!("Failed to create canister: {:?} - {}", code, msg))
}

pub async fn install_wasm(
    canister_id: Principal,
    wasm_module: Vec<u8>,
    arg: Vec<u8>,
) -> Result<(), String> {
    let args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module,
        arg,
    };
    install_code(args)
        .await
        .map_err(|(code, msg)| format!("Failed to install code: {:?} - {}", code, msg))
}

// Logic inspired by Juno's CMC interaction
pub async fn create_canister_with_cmc(
    cmc_id: Principal,
    settings: Option<CanisterSettings>,
    cycles: u128,
) -> Result<Principal, String> {
    let args = CreateCanisterWithCmcArgs { settings };

    // In Juno, they use notify_top_up or create_canister on CMC.
    // CMC's create_canister usually requires Cycles to be attached.
    ic_cdk::api::call::call_with_payment128::<
        (CreateCanisterWithCmcArgs,),
        (Result<Principal, String>,),
    >(cmc_id, "create_canister", (args,), cycles)
    .await
    .map_err(|(code, msg)| format!("CMC call failed: {:?} - {}", code, msg))?
    .0
}
