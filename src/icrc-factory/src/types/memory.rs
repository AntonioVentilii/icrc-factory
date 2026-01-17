use ic_stable_structures::{memory_manager::VirtualMemory, DefaultMemoryImpl, StableCell};

use crate::types::{candid::Candid, config::Config};

pub type VMem = VirtualMemory<DefaultMemoryImpl>;

pub type ConfigCell = StableCell<Option<Candid<Config>>, VMem>;

pub type IcrcLedgerWasmCell = StableCell<Vec<u8>, VMem>;
pub type IcrcIndexWasmCell = StableCell<Vec<u8>, VMem>;
