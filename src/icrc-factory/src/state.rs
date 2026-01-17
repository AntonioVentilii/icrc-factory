use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};

use crate::types::memory::IcrcLedgerWasmCell;

const ICRC_LEDGER_WASM_MEMORY_ID: MemoryId = MemoryId::new(0);
const ICRC_INDEX_WASM_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static STATE: RefCell<State> = RefCell::new(
        MEMORY_MANAGER.with(|mm| State {
            icrc_ledger_wasm: IcrcLedgerWasmCell::init(mm.borrow().get(ICRC_LEDGER_WASM_MEMORY_ID), Vec::new()),
            icrc_index_wasm: IcrcLedgerWasmCell::init(mm.borrow().get(ICRC_INDEX_WASM_MEMORY_ID),Vec::new()),
        })
    );
}

pub struct State {
    pub icrc_ledger_wasm: IcrcLedgerWasmCell,
    pub icrc_index_wasm: IcrcLedgerWasmCell,
}

pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

pub fn mutate_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}
