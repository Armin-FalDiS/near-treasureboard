use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId};
use near_sdk::collections::{UnorderedMap};
use near_sdk::env::{state_exists, panic_str};

/// treasure boards can be : Small (2 x 2), Medium (4 x 4) or Big (6 x 6)
#[derive(BorshDeserialize, BorshSerialize)]
enum BoardSize {
    Small = 4,
    Medium = 16,
    Big = 36
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TreasureBoard {
    id: u128,
    creator: AccountId,
    size: BoardSize,
    answer_hash: String,
    answers: UnorderedMap<AccountId, u8>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearTreasureBoardGame {
    boards: UnorderedMap<u128, TreasureBoard>,
    next_index: u128
}

#[near_bindgen]
impl NearTreasureBoardGame {
    #[init]
    pub fn new() -> Self {
        if state_exists() {
            panic_str("The contract has already been initialized");
        } else {
            Self {
                boards: UnorderedMap::new(b"A"),
                next_index: 1_u128
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    /// returns mock context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
