use candid::Encode;

use crate::{
    get_stored_ledger_wasm, types::args::create_canister::UpgradeLedgerCanisterArgs, upgrade_wasm,
    CreateCanisterError, SetCanisterResult,
};

pub async fn upgrade_ledger_canister(args: UpgradeLedgerCanisterArgs) -> SetCanisterResult {
    let ledger_wasm = get_stored_ledger_wasm();
    if ledger_wasm.is_empty() {
        return SetCanisterResult::Err(CreateCanisterError::NoWasmStored);
    }

    let upgrade_arg = args.args;

    let arg = match Encode!(&upgrade_arg) {
        Ok(arg) => arg,
        Err(e) => {
            return SetCanisterResult::Err(CreateCanisterError::InitArgsEncodingFailed(format!(
                "Failed to encode upgrade args: {e}",
            )))
        }
    };

    if let Err(err) = upgrade_wasm(args.ledger_id, ledger_wasm, arg).await {
        return SetCanisterResult::Err(CreateCanisterError::WasmInstallationFailed(err));
    }

    SetCanisterResult::Ok()
}
