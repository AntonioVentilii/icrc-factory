use crate::types::{
    candid::Candid, memory::UserCanisterMap, stored_principal::StoredPrincipal,
    user_canister::UserCanister,
};

const MAX_USER_CANISTER_LIST_LENGTH: usize = 1000;

pub fn upsert_user_canister(
    stored_principal: StoredPrincipal,
    user_canister: &mut UserCanisterMap,
    new_entry: UserCanister,
) {
    let Candid(mut canisters) = user_canister.get(&stored_principal).unwrap_or_default();

    if let Some(existing) = canisters
        .iter_mut()
        .find(|c| c.canister_id == new_entry.canister_id)
    {
        *existing = new_entry;
    } else {
        if canisters.len() == MAX_USER_CANISTER_LIST_LENGTH {
            ic_cdk::trap(&format!(
                "User canister list length should not exceed {MAX_USER_CANISTER_LIST_LENGTH}"
            ));
        }

        canisters.push(new_entry);
    }

    user_canister.insert(stored_principal, Candid(canisters));
}
