use std::{cell::RefCell, sync::LazyLock};

use candid::Principal;
use ic_papi_guard::guards::any::{PaymentGuard, VendorPaymentConfig};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};

use crate::types::{
    candid::Candid,
    config::{Config, InitArgs},
    memory::{ConfigCell, IcrcLedgerWasmCell},
};

const CONFIG_MEMORY_ID: MemoryId = MemoryId::new(0);
const ICRC_LEDGER_WASM_MEMORY_ID: MemoryId = MemoryId::new(2);
const ICRC_INDEX_WASM_MEMORY_ID: MemoryId = MemoryId::new(3);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static STATE: RefCell<State> = RefCell::new(
        MEMORY_MANAGER.with(|mm| State {
             config: ConfigCell::init(mm.borrow().get(CONFIG_MEMORY_ID), None),
            icrc_ledger_wasm: IcrcLedgerWasmCell::init(mm.borrow().get(ICRC_LEDGER_WASM_MEMORY_ID), Vec::new()),
            icrc_index_wasm: IcrcLedgerWasmCell::init(mm.borrow().get(ICRC_INDEX_WASM_MEMORY_ID),Vec::new()),
        })
    );
}

pub struct State {
    pub config: ConfigCell,
    pub icrc_ledger_wasm: IcrcLedgerWasmCell,
    pub icrc_index_wasm: IcrcLedgerWasmCell,
}

pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

pub fn mutate_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

/// Reads the internal canister configuration, normally set at canister install or upgrade.
///
/// # Panics
/// - If the `STATE.config` is not initialized.
pub fn read_config<R>(f: impl FnOnce(&Config) -> R) -> R {
    read_state(|state| {
        f(state
            .config
            .get()
            .as_ref()
            .expect("config is not initialized"))
    })
}

pub fn set_config(arg: InitArgs) {
    let config = Config::from(arg);
    mutate_state(|state| {
        state.config.set(Some(Candid(config)));
    });
}

pub static PAYMENT_GUARD: LazyLock<PaymentGuard<5>> = LazyLock::new(|| PaymentGuard {
    supported: [
        VendorPaymentConfig::AttachedCycles,
        VendorPaymentConfig::CallerPaysIcrc2Cycles,
        VendorPaymentConfig::PatronPaysIcrc2Cycles,
        VendorPaymentConfig::CallerPaysIcrc2Tokens {
            ledger: payment_ledger(),
        },
        VendorPaymentConfig::PatronPaysIcrc2Tokens {
            ledger: payment_ledger(),
        },
    ],
});

/// Provides the canister id of the ledger used for payments.
pub fn payment_ledger() -> Principal {
    read_config(|config| config.cycles_ledger)
}
