mod canister;
mod generic;
mod guards;
mod index;
mod ledger;
pub mod methods;
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
    guards::{caller_is_controller, caller_is_not_anonymous},
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

/// Initializes the canister state on installation.
///
/// # Arguments
/// - `args`: Initial configuration arguments.
///
/// # Behaviour
/// - Accepts `Args::Init` and stores the provided configuration.
/// - Traps if upgrade arguments are provided during installation.
///
/// # Panics
/// - If `Args::Upgrade` is passed during initial installation.
#[init]
pub fn init(args: Args) {
    match args {
        Args::Init(args) => set_config(args),
        Args::Upgrade => ic_cdk::trap("upgrade args in init"),
    }
}

/// Restores or validates state after a canister upgrade.
///
/// # Arguments
/// - `arg`: Optional upgrade arguments.
///
/// # Behaviour
/// - If `Args::Init` is provided, the configuration is overwritten.
/// - Otherwise, the existing configuration is validated.
///
/// # Panics
/// - If the canister is upgraded without an existing configuration, indicating an invalid upgrade
///   (for example, upgrading a freshly installed canister instead of reinstalling).
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

/// Returns the current canister configuration.
///
/// # Access Control
/// - Caller must not be anonymous.
///
/// # Returns
/// - A clone of the stored `Config`.
#[query(guard = "caller_is_not_anonymous")]
pub fn config() -> Config {
    read_config(std::clone::Clone::clone)
}

/// Transforms HTTP responses when fetching WASM binaries.
///
/// # Purpose
/// - Used as an HTTP transform function to sanitise responses from external WASM sources.
///
/// # Arguments
/// - `args`: HTTP transform arguments provided by the IC.
///
/// # Returns
/// - A transformed `HttpResponse`.
#[query]
fn transform_wasm_response(args: TransformArgs) -> HttpResponse {
    crate::wasm::utils::transform_wasm_response(args)
}

/// Stores a new ICRC ledger WASM binary.
///
/// # Access Control
/// - Caller must be a controller.
///
/// # Arguments
/// - `wasm`: Raw WASM bytecode to store.
#[update(guard = "caller_is_controller")]
fn set_ledger_wasm(wasm: Vec<u8>) {
    crate::wasm::ledger_wasm::set_ledger_wasm(wasm);
}

/// Fetches and stores an ICRC ledger WASM binary from a URL.
///
/// # Access Control
/// - Caller must be a controller.
///
/// # Arguments
/// - `url`: URL pointing to the WASM binary.
///
/// # Returns
/// - `SetWasmResult` indicating success or failure.
#[update(guard = "caller_is_controller")]
async fn set_ledger_wasm_from_url(url: String) -> SetWasmResult {
    crate::wasm::ledger_wasm::set_ledger_wasm_from_url(url)
        .await
        .into()
}

/// Stores a new ICRC index WASM binary.
///
/// # Access Control
/// - Caller must be a controller.
///
/// # Arguments
/// - `wasm`: Raw WASM bytecode to store.
#[update(guard = "caller_is_controller")]
fn set_index_wasm(wasm: Vec<u8>) {
    crate::wasm::index_wasm::set_index_wasm(wasm);
}

/// Fetches and stores an ICRC index WASM binary from a URL.
///
/// # Access Control
/// - Caller must be a controller.
///
/// # Arguments
/// - `url`: URL pointing to the WASM binary.
///
/// # Returns
/// - `SetWasmResult` indicating success or failure.
#[update(guard = "caller_is_controller")]
async fn set_index_wasm_from_url(url: String) -> SetWasmResult {
    crate::wasm::index_wasm::set_index_wasm_from_url(url)
        .await
        .into()
}

/// Creates a new ICRC ledger canister.
///
/// # Access Control
/// - Caller must not be anonymous.
///
/// # Arguments
/// - `args`: [`CreateIcrcLedgerArgs`]
///   - All fields inside `args` are optional:
///     - `symbol`: optional token symbol
///     - `name`: optional token name
///     - `transfer_fee`: optional fee (smallest unit)
///     - `decimals`: optional decimals
///     - `minting_account`: optional minting account
///   - Any omitted fields fall back to the ledger’s defaults.
/// - `payment`: Optional [`PaymentType`]
///   - If `None`, defaults to `PaymentType::AttachedCycles`.
///
/// # Returns
/// - `CreateCanisterResult::Ok(Principal)` containing the newly created ledger canister ID.
/// - `CreateCanisterResult::Err(CreateCanisterError)` if payment deduction fails or if canister
///   creation / init-args encoding / WASM installation fails.
#[update(guard = "caller_is_not_anonymous")]
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

/// Creates a new ICRC index canister for an existing ledger.
///
/// # Access Control
/// - Caller must not be anonymous.
///
/// # Arguments
/// - `args`: [`CreateIcrcIndexArgs`]
///   - `ledger_id`: **required** principal of the ledger to index.
/// - `payment`: Optional [`PaymentType`]
///   - If `None`, defaults to `PaymentType::AttachedCycles`.
///
/// # Returns
/// - `CreateCanisterResult::Ok(Principal)` containing the newly created index canister ID.
/// - `CreateCanisterResult::Err(CreateCanisterError)` if payment deduction fails or if canister
///   creation / init-args encoding / WASM installation fails.
#[update(guard = "caller_is_not_anonymous")]
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

/// Associates an index canister with a ledger by upgrading the ledger configuration.
///
/// # Access Control
/// - Caller must not be anonymous.
///
/// # Arguments
/// - `args`: [`SetIndexCanisterArgs`]
///   - `ledger_id`: **required** principal of the ledger to update.
///   - `index_id`: **required** principal of the index canister to associate.
///
/// # Returns
/// - `SetCanisterResult::Ok(())` on success.
/// - `SetCanisterResult::Err(CreateCanisterError)` if the ledger upgrade fails.
#[update(guard = "caller_is_not_anonymous")]
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

/// Updates a ledger’s token symbol by upgrading the ledger configuration.
///
/// # Access Control
/// - Caller must not be anonymous.
///
/// # Arguments
/// - `args`: [`SetSymbolArgs`]
///   - `ledger_id`: **required** principal of the ledger to update.
///   - `symbol`: **required** new token symbol.
///
/// # Returns
/// - `SetCanisterResult::Ok(())` on success.
/// - `SetCanisterResult::Err(CreateCanisterError)` if the ledger upgrade fails.
#[update(guard = "caller_is_not_anonymous")]
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

/// Updates a ledger’s token name by upgrading the ledger configuration.
///
/// # Access Control
/// - Caller must not be anonymous.
///
/// # Arguments
/// - `args`: [`SetNameArgs`]
///   - `ledger_id`: **required** principal of the ledger to update.
///   - `name`: **required** new token name.
///
/// # Returns
/// - `SetCanisterResult::Ok(())` on success.
/// - `SetCanisterResult::Err(CreateCanisterError)` if the ledger upgrade fails.
#[update(guard = "caller_is_not_anonymous")]
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
