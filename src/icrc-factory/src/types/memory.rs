use ic_stable_structures::{memory_manager::VirtualMemory, DefaultMemoryImpl, StableCell};

pub type VMem = VirtualMemory<DefaultMemoryImpl>;

pub type IcrcLedgerWasmCell = StableCell<Vec<u8>, VMem>;
pub type IcrcIndexWasmCell = StableCell<Vec<u8>, VMem>;
