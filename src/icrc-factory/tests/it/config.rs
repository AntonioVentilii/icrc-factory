use candid::{encode_one, Principal};
use icrc_factory::types::config::{Args, Args::Upgrade, Config, InitArgs};

use crate::utils::pocketic::{
    caller, controller, cycles_ledger_canister_id, setup, BackendBuilder, PicCanisterTrait,
};

mod init {
    use super::*;

    #[test]
    fn test_init_success() {
        let pic_setup = setup();

        let cycles_ledger_canister = cycles_ledger_canister_id();

        let config: Config = pic_setup
            .query(controller(), "config", ())
            .expect("Failed to query get_config");

        assert_eq!(config.cycles_ledger, cycles_ledger_canister);
    }

    #[test]
    fn test_init_with_upgrade_args_traps() {
        let result = std::panic::catch_unwind(|| {
            let bad_arg = encode_one(Upgrade).expect("encode upgrade args");

            let _ = BackendBuilder::default().with_arg(bad_arg).deploy();
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_config_query_with_generic_caller_succeds() {
        let pic_setup = setup();

        let cycles_ledger_canister = cycles_ledger_canister_id();

        let config: Config = pic_setup
            .query(caller(), "config", ())
            .expect("Failed to query config as generic caller");

        assert_eq!(config.cycles_ledger, cycles_ledger_canister);
    }

    #[test]
    fn test_config_query_with_anonymous_fails() {
        let pic_setup = setup();

        let result: Result<Config, _> = pic_setup.query(Principal::anonymous(), "config", ());

        assert!(result.is_err());
    }
}

mod upgrade {
    use super::*;

    #[test]
    #[ignore] // upgrade tests are failing for rate limitation reasons
    fn test_upgrade_without_args_keeps_config() {
        let pic_setup = setup();

        let before: Config = pic_setup
            .query(controller(), "config", ())
            .expect("Failed to query config (before upgrade)");

        let no_args = encode_one(Option::<Args>::None).expect("encode None upgrade args");

        pic_setup
            .upgrade_latest_wasm(Some(no_args))
            .expect("Failed to upgrade");

        let after: Config = pic_setup
            .query(controller(), "config", ())
            .expect("Failed to query config (after upgrade)");

        assert_eq!(after, before);
    }

    #[test]
    #[ignore] // upgrade tests are failing for rate limitation reasons
    fn test_upgrade_with_init_args_updates_config() {
        let pic_setup = setup();

        let new_ledger = Principal::anonymous();

        let init_args = encode_one(Some(Args::Init(InitArgs {
            cycles_ledger: Some(new_ledger),
        })))
        .expect("encode Some(Args::Init)");

        pic_setup
            .upgrade_latest_wasm(Some(init_args))
            .expect("Failed to upgrade");

        let after: Config = pic_setup
            .query(Principal::anonymous(), "config", ())
            .expect("Failed to query config (after upgrade)");

        assert_eq!(after.cycles_ledger, new_ledger);
    }

    #[test]
    #[ignore] // upgrade tests are failing for rate limitation reasons
    fn test_upgrade_without_args_keeps_config_for_generic_caller() {
        let pic_setup = setup();

        let before: Config = pic_setup
            .query(caller(), "config", ())
            .expect("Failed to query config as generic caller (before upgrade)");

        let no_args = encode_one(Option::<Args>::None).expect("encode None upgrade args");

        pic_setup
            .upgrade_latest_wasm(Some(no_args))
            .expect("Failed to upgrade");

        let after: Config = pic_setup
            .query(caller(), "config", ())
            .expect("Failed to query config as generic caller (after upgrade)");

        assert_eq!(after, before);
    }

    #[test]
    #[ignore] // upgrade tests are failing for rate limitation reasons
    fn test_upgrade_config_query_with_anonymous_fails_after_upgrade() {
        let pic_setup = setup();

        let no_args = encode_one(Option::<Args>::None).expect("encode None upgrade args");

        pic_setup
            .upgrade_latest_wasm(Some(no_args))
            .expect("Failed to upgrade");

        let result: Result<Config, _> = pic_setup.query(Principal::anonymous(), "config", ());

        assert!(result.is_err());
    }
}
