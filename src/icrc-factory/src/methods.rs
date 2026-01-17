use crate::generic::MIN_CYCLES_FOR_CANISTER_CREATION;

pub enum SignerMethods {
    CreateIcrcLedger,
    CreateIcrcIndex,
}

impl SignerMethods {
    /// The cost, in cycles, of every paid API method.
    #[must_use]
    #[allow(clippy::match_same_arms)] // Entries are sorted by method, as this makes them easier to manage.
    pub fn fee(&self) -> u64 {
        // Note: Fees are determined with the aid of scripts/check-pricing
        match self {
            SignerMethods::CreateIcrcLedger => MIN_CYCLES_FOR_CANISTER_CREATION + 400_000_000_000,
            SignerMethods::CreateIcrcIndex => MIN_CYCLES_FOR_CANISTER_CREATION + 400_000_000_000,
        }
    }
}
