mod canister;
mod generic;
mod index;
mod ledger;
mod methods;
mod mgmt;
mod state;
pub mod types;
mod wasm;

use ic_cdk::{
    api::management_canister::http_request::{HttpResponse, TransformArgs},
    export_candid, init, post_upgrade, query, update,
};
use ic_papi_api::PaymentType;

use crate::{
    canister::upgrade_ledger_canister,
    ledger::LedgerArgs,
    methods::SignerMethods,
    mgmt::upgrade_wasm,
    state::{read_config, read_state, set_config, PAYMENT_GUARD},
    types::{
        args::create_canister::{
            CreateIcrcIndexArgs, CreateIcrcLedgerArgs, SetIndexCanisterArgs, SetNameArgs,
            SetSymbolArgs, UpgradeLedgerCanisterArgs,
        },
        config::{Args, Config},
        ledger_suite::ledger::upgrade_args::UpgradeArgs,
        results::{
            create_canister::{CreateCanisterError, CreateCanisterResult, SetCanisterResult},
            set_wasm::SetWasmResult,
        },
    },
    wasm::ledger_wasm::get_stored_ledger_wasm,
};

/// Initializes state on canister creation
#[init]
pub fn init(args: Args) {
    match args {
        Args::Init(args) => set_config(args),
        Args::Upgrade => ic_cdk::trap("upgrade args in init"),
    }
}

/// Updates state after canister upgrade
///
/// # Panics
/// - If there is an attempt to upgrade a canister without existing state.  This is most likely an
///   attempt to upgrade a new canister when an installation was intended.
#[post_upgrade]
pub fn post_upgrade(arg: Option<Args>) {
    match arg {
        Some(Args::Init(arg)) => set_config(arg),
        _ => {
            read_state(|s| {
                let _ = s.config.get().as_ref().expect(
                    "config is not initialized: reinstall the canister instead of upgrading",
                );
            });
        }
    }
}

#[query]
pub fn config() -> Config {
    read_config(std::clone::Clone::clone)
}

#[query]
fn transform_wasm_response(args: TransformArgs) -> HttpResponse {
    crate::wasm::utils::transform_wasm_response(args)
}

#[update]
async fn set_ledger_wasm(wasm: Vec<u8>) {
    crate::wasm::ledger_wasm::set_ledger_wasm(wasm);
}

#[update]
async fn set_ledger_wasm_from_url(url: String) -> SetWasmResult {
    crate::wasm::ledger_wasm::set_ledger_wasm_from_url(url)
        .await
        .into()
}

#[update]
async fn set_index_wasm(wasm: Vec<u8>) {
    crate::wasm::index_wasm::set_index_wasm(wasm);
}

#[update]
async fn set_index_wasm_from_url(url: String) -> SetWasmResult {
    crate::wasm::index_wasm::set_index_wasm_from_url(url)
        .await
        .into()
}

#[update]
async fn create_icrc_ledger(
    args: CreateIcrcLedgerArgs,
    payment: Option<PaymentType>,
) -> CreateCanisterResult {
    if let Err(err) = PAYMENT_GUARD
        .deduct(
            payment.unwrap_or(PaymentType::AttachedCycles),
            SignerMethods::CreateIcrcLedger.fee(),
        )
        .await
    {
        return CreateCanisterResult::Err(CreateCanisterError::PaymentError(err));
    }

    generic::create_icrc_ledger(args).await
}

#[update]
async fn create_icrc_index(
    args: CreateIcrcIndexArgs,
    payment: Option<PaymentType>,
) -> CreateCanisterResult {
    if let Err(err) = PAYMENT_GUARD
        .deduct(
            payment.unwrap_or(PaymentType::AttachedCycles),
            SignerMethods::CreateIcrcIndex.fee(),
        )
        .await
    {
        return CreateCanisterResult::Err(CreateCanisterError::PaymentError(err));
    }

    generic::create_icrc_index(args).await
}

#[update]
async fn set_index_canister(args: SetIndexCanisterArgs) -> SetCanisterResult {
    let upgrade_arg = LedgerArgs::Upgrade(Some(UpgradeArgs {
        index_principal: Some(args.index_id),
        ..Default::default()
    }));

    upgrade_ledger_canister(UpgradeLedgerCanisterArgs {
        ledger_id: args.ledger_id,
        args: upgrade_arg,
    })
    .await
}

#[update]
async fn set_symbol(args: SetSymbolArgs) -> SetCanisterResult {
    let upgrade_arg = LedgerArgs::Upgrade(Some(UpgradeArgs {
        token_symbol: Some(args.symbol),
        ..Default::default()
    }));

    upgrade_ledger_canister(UpgradeLedgerCanisterArgs {
        ledger_id: args.ledger_id,
        args: upgrade_arg,
    })
    .await
}

#[update]
async fn set_name(args: SetNameArgs) -> SetCanisterResult {
    let upgrade_arg = LedgerArgs::Upgrade(Some(UpgradeArgs {
        token_name: Some(args.name),
        ..Default::default()
    }));

    upgrade_ledger_canister(UpgradeLedgerCanisterArgs {
        ledger_id: args.ledger_id,
        args: upgrade_arg,
    })
    .await
}

export_candid!();
