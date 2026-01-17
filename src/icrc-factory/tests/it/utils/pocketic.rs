//! Utilities for setting up a test environment using `PocketIC`.
pub mod pic_canister;
use std::{env, fs::read, sync::Arc, time::Duration};

use candid::{encode_one, Principal};
use icrc_factory::types::config::{Args::Init, InitArgs};
use pocket_ic::{PocketIc, PocketIcBuilder};

use super::mock::{CONTROLLER, CYCLES_LEDGER_CANISTER_ID};
use crate::utils::mock::CALLER;
pub use crate::utils::pocketic::pic_canister::PicCanisterTrait;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/icrc_factory.wasm";

fn default_init_args() -> InitArgs {
    InitArgs {
        cycles_ledger: Some(cycles_ledger_canister_id()),
    }
}

#[derive(Debug)]
pub struct BackendBuilder {
    canister_id: Option<Principal>,
    cycles: u128,
    wasm_path: String,
    arg: Vec<u8>,
    controllers: Vec<Principal>,
    auto_progress_enabled: bool,
}

impl BackendBuilder {
    pub const DEFAULT_CYCLES: u128 = 2_000_000_000_000;

    pub fn with_arg(mut self, arg: Vec<u8>) -> Self {
        self.arg = arg;
        self
    }

    #[allow(dead_code)]
    pub fn with_init_args(mut self, init: InitArgs) -> Self {
        let encoded = encode_one(Init(init)).expect("encode init args");
        self.arg = encoded;
        self
    }

    pub fn default_wasm_path() -> String {
        env::var("BACKEND_WASM_PATH").unwrap_or_else(|_| BACKEND_WASM.to_string())
    }

    pub fn default_arg() -> Vec<u8> {
        encode_one(Init(default_init_args())).expect("encode default init args")
    }

    pub fn default_controllers() -> Vec<Principal> {
        vec![controller()]
    }

    pub fn default_auto_progress_enabled() -> bool {
        true
    }
}

impl Default for BackendBuilder {
    fn default() -> Self {
        Self {
            canister_id: None,
            cycles: Self::DEFAULT_CYCLES,
            wasm_path: Self::default_wasm_path(),
            arg: Self::default_arg(),
            controllers: Self::default_controllers(),
            auto_progress_enabled: Self::default_auto_progress_enabled(),
        }
    }
}

impl BackendBuilder {
    #[allow(dead_code)]
    pub fn with_controllers(mut self, controllers: Vec<Principal>) -> Self {
        self.controllers = controllers;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasm(mut self, wasm_path: &str) -> Self {
        self.wasm_path = wasm_path.to_string();
        self
    }

    fn wasm_bytes(&self) -> Vec<u8> {
        read(self.wasm_path.clone())
            .unwrap_or_else(|_| panic!("Could not find the backend wasm: {}", self.wasm_path))
    }

    fn canister_id(&mut self, pic: &PocketIc) -> Principal {
        if let Some(canister_id) = self.canister_id {
            canister_id
        } else {
            let fiduciary_subnet_id = pic
                .topology()
                .get_fiduciary()
                .expect("pic should have a fiduciary subnet.");
            let canister_id = pic.create_canister_on_subnet(None, None, fiduciary_subnet_id);
            self.canister_id = Some(canister_id);
            canister_id
        }
    }

    fn add_cycles(&mut self, pic: &PocketIc) {
        if self.cycles > 0 {
            let canister_id = self.canister_id(pic);
            pic.add_cycles(canister_id, self.cycles);
        }
    }

    fn install_backend(&mut self, pic: &PocketIc) {
        let wasm_bytes = self.wasm_bytes();
        let canister_id = self.canister_id(pic);
        let arg = self.arg.clone();
        pic.install_canister(canister_id, wasm_bytes, arg, None);
    }

    fn set_controllers(&mut self, pic: &PocketIc) {
        let canister_id = self.canister_id(pic);
        pic.set_controllers(canister_id, None, self.controllers.clone())
            .expect("Test setup error: Failed to set controllers");
    }

    pub fn deploy_backend(&mut self, pic: &PocketIc) -> Principal {
        let canister_id = self.canister_id(pic);
        self.add_cycles(pic);
        self.install_backend(pic);
        self.set_controllers(pic);
        canister_id
    }

    pub fn deploy_to(&mut self, pic: &PocketIc) -> Principal {
        self.deploy_backend(pic)
    }

    pub fn deploy(&mut self) -> PicBackend {
        let pic = PocketIcBuilder::new()
            .with_ii_subnet()
            .with_fiduciary_subnet()
            .build();
        if self.auto_progress_enabled {
            pic.auto_progress();
        }

        let canister_id = self.deploy_to(&pic);
        PicBackend {
            pic: Arc::new(pic),
            canister_id,
        }
    }
}

#[allow(dead_code)]
pub fn controller() -> Principal {
    Principal::from_text(CONTROLLER)
        .expect("Test setup error: Failed to parse controller principal")
}

pub fn caller() -> Principal {
    Principal::from_text(CALLER).expect("Test setup error: Failed to parse caller principal")
}

pub fn cycles_ledger_canister_id() -> Principal {
    Principal::from_text(CYCLES_LEDGER_CANISTER_ID)
        .expect("Test setup error: Failed to parse cycles ledger canister principal")
}

pub fn setup() -> PicBackend {
    BackendBuilder::default().deploy()
}

pub struct PicBackend {
    pub pic: Arc<PocketIc>,
    pub canister_id: Principal,
}

impl PicCanisterTrait for PicBackend {
    fn pic(&self) -> Arc<PocketIc> {
        self.pic.clone()
    }

    fn canister_id(&self) -> Principal {
        self.canister_id
    }
}

impl PicBackend {
    #[allow(dead_code)]
    pub fn upgrade_latest_wasm(&self, encoded_arg: Option<Vec<u8>>) -> Result<(), String> {
        let backend_wasm_path = BackendBuilder::default_wasm_path();
        self.upgrade_with_wasm(&backend_wasm_path, encoded_arg)
    }

    #[allow(dead_code)]
    pub fn upgrade_with_wasm(
        &self,
        backend_wasm_path: &String,
        encoded_arg: Option<Vec<u8>>,
    ) -> Result<(), String> {
        let wasm_bytes = read(backend_wasm_path.clone())
            .unwrap_or_else(|_| panic!("Could not find the backend wasm: {backend_wasm_path}"));

        let arg = encoded_arg.unwrap_or_else(|| vec![]);

        self.pic.advance_time(Duration::from_secs(100_000));

        self.pic
            .upgrade_canister(self.canister_id, wasm_bytes, arg, Some(controller()))
            .map_err(|e| {
                format!(
                    "Upgrade canister error code: {:?}, message: {}",
                    e.reject_code, e.reject_message
                )
            })
    }
}
