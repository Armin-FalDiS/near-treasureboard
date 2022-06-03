use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};

/// converts the given number to yocto
fn to_yocto(num: u128) -> u128 {
    num * (1e24 as u128)
}

/// treasure boards can be : Small (2 x 2), Medium (4 x 4) or Big (6 x 6)
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum BoardSize {
    Small = 4,
    Medium = 16,
    Big = 36,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TreasureBoard {
    creator: AccountId,
    size: BoardSize,
    answer_hash: String,
    answers: UnorderedMap<AccountId, u8>,
}

impl TreasureBoard {
    /// return true if the treasure board is closed and not taking any more answers
    fn closed(&self) -> bool {
        self.answers.len() == (self.size as u64) / 2
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearTreasureBoardGame {
    boards: UnorderedMap<u128, TreasureBoard>,
    next_index: u128,
}

#[near_bindgen]
impl NearTreasureBoardGame {
    pub fn default() -> Self {
        Self {
            boards: UnorderedMap::new(b"B"),
            next_index: 1_u128
            }
    }

    #[init]
    pub fn new() -> Self {
        if env::state_exists() {
            env::panic_str("The contract has already been initialized");
        } else {
            Self {
                boards: UnorderedMap::new(b"B"),
                next_index: 1_u128,
            }
        }
    }

    /// creates a new treasure board of the given size taking NEARs equal to that size
    #[payable]
    pub fn new_game(&mut self, size: BoardSize, answer_hash: String) {
        let creator = env::predecessor_account_id();
        let prize = env::attached_deposit();

        // reject request if the attached funds are insufficient to cover the game
        if prize < to_yocto(size as u128) {
            env::panic_str("Attached deposit is not sufficient to create a board of this size")
        }

        // add new game to state
        self.boards.insert(
            &self.next_index,
            &TreasureBoard {
                creator,
                size,
                answer_hash,
                answers: UnorderedMap::new(b"A"),
            },
        );

        self.next_index += 1;

        // keep the prize inside contract account
        Promise::new(env::current_account_id()).transfer(prize);
    }

    /// extract all treasure boards and return them
    pub fn games(&self) -> Vec<TreasureBoard> {
        let mut boards: Vec<TreasureBoard> = Vec::new();

        for b in self.boards.iter() {
            boards.push(b.1);
        }

        boards
    }

    /// reserves a slot on the treasure board for the user
    #[payable]
    pub fn play(&mut self, id: u128, choice: u8) {
        let game = self.boards.get(&id);

        match game {
            None => { env::panic_str("No such a game exists"); }
            Some(mut game) => {
                // check if answer is acceptable
                if choice < (game.size as u8) {
                    for a in game.answers.values() {
                        // reject duplicate choice
                        if a == choice {
                            env::panic_str("That slot has already been taken")
                        }
                    }
                }

                let cost = env::attached_deposit();

                // check if enough money is attached
                if cost < to_yocto(1) {
                    env::panic_str("Attached deposit is insufficient to play");
                }

                // reserve slot on the board for user
                game.answers.insert(&env::predecessor_account_id(), &choice);

                // put the deposit into contract account
                Promise::new(env::current_account_id()).transfer(cost);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests;
