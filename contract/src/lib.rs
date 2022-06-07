use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise, log};
use fastrand::Rng;

/// converts the given number to yocto
fn to_yocto(num: u128) -> u128 {
    num * (1e24 as u128)
}

/// converts u128 to byte array
fn to_bytearray(num: u128) -> [u8; 16] {
    let mut arr = [0_u8; 16];

    for i in 0..16 {
        arr[15 - i] = ((num >> (8 * i)) & 0xff) as u8;
    }

    arr
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
    solution_hash: Vector<u8>,
    answers: UnorderedMap<u8, AccountId>,
}

impl TreasureBoard {
    /// return true if the treasure board is closed and not taking any more answers
    fn is_closed(&self) -> bool {
        self.answers.len() == (self.size as u64) / 2
    }
}

/// display struct to return treasure board to the user
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameInfo {
    id: u128,
    size: BoardSize,
    answers: Vec<u8>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearTreasureBoardGame {
    boards: UnorderedMap<u128, TreasureBoard>,
    next_index: u128,
}

impl Default for NearTreasureBoardGame {
    fn default() -> Self {
        Self {
            boards: UnorderedMap::new(b"B"),
            next_index: 1_u128,
        }
    }
}

#[near_bindgen]
impl NearTreasureBoardGame {
    // returns a copy of the treasure board
    fn get_game(&self, id: u128) -> TreasureBoard {
        // check whether a game exists with the given id
        match self.boards.get(&id) {
            None => env::panic_str("No such a game exists"),
            Some(game) => game,
        }
    }

    #[init]
    pub fn new(start_index: u128) -> Self {
        if env::state_exists() {
            env::panic_str("The contract has already been initialized");
        } else {
            Self {
                boards: UnorderedMap::new(b"B"),
                next_index: start_index,
            }
        }
    }

    /// creates a new treasure board of the given size taking NEARs equal to that size
    #[payable]
    pub fn new_game(&mut self, size: BoardSize, solution_hash: Vec<u8>) {
        let creator = env::predecessor_account_id();
        let prize = env::attached_deposit();

        // reject request if the attached funds are insufficient
        if prize < to_yocto(size as u128) {
            env::panic_str("Attached deposit is not sufficient to create a board of this size")
        }

        // init prefixes for borsh serializer
        let mut ans_prefix = to_bytearray(self.next_index).to_vec();
        ans_prefix.extend(b"a");
        let mut sol_prefix = to_bytearray(self.next_index).to_vec();
        sol_prefix.extend(b"s");

        // save solution hash as bytearray
        let mut solution_hash_vector = Vector::new(sol_prefix);
        solution_hash_vector.extend(solution_hash);

        // add new game to state
        self.boards.insert(
            &self.next_index,
            &TreasureBoard {
                creator,
                size,
                solution_hash: solution_hash_vector,
                answers: UnorderedMap::new(ans_prefix),
            },
        );

        // increment index
        self.next_index += 1;

        // closure to get board size as string
        let size_str = |size: BoardSize| {
            match size {
                BoardSize::Small => "small",
                BoardSize::Medium => "medium",
                BoardSize::Big => "big"
            }
        };

        log!("{} created a new {} treasureboard", env::predecessor_account_id(), size_str(size));
    }

    /// return all treasure boards
    pub fn games(&self) -> Vec<GameInfo> {
        let mut games: Vec<GameInfo> = Vec::new();
        for (id, board) in self.boards.iter() {
            games.push(
                GameInfo {
                    id,
                    size: board.size,
                    answers: board.answers.keys_as_vector().to_vec()
                }
            )
        }

        games
    }

    /// reserves a slot on the treasure board for the user
    #[payable]
    pub fn play(&mut self, id: u128, choice: u8) {
        let mut game = self.get_game(id);

        // check whether the game is closed
        if game.is_closed() {
            env::panic_str("This board is closed");
        }

        // check if answer is within the acceptable range
        if choice >= (game.size as u8) {
            env::panic_str("The choice is out of the bounds of this board");
        }

        // reject duplicate choices
        if game.answers.get(&choice) != None {
            env::panic_str("That slot has already been taken")
        }

        let cost = env::attached_deposit();

        // check if enough money is attached
        if cost < to_yocto(1) {
            env::panic_str("Attached deposit is insufficient to play");
        }

        // reserve slot on the board for the player
        game.answers.insert(&choice, &env::predecessor_account_id());

        // update the board
        self.boards.insert(&id, &game);
 
        log!("{} chose {} on treasureboard #{}", env::predecessor_account_id(), choice, id);
    }

    pub fn reveal(&mut self, id: u128, solution: Vec<u8>) {
        // fetch game
        let game = self.get_game(id);

        // only the owner can reveal the solution, others will be rejected
        if game.creator != env::predecessor_account_id() {
            env::panic_str("Only the creator of the board can reveal the solution");
        }

        // only a closed game's solution can be revealed, premature reveal will be prevented
        if !game.is_closed() {
            env::panic_str("This game is still in progress, cannot reveal prematurely");
        }

        // check if solution matches the solution_hash
        if env::sha256(&solution) != game.solution_hash.to_vec() {
            env::panic_str("Provided solution does not match the originally provided hash");
        }

        // get the size of the treasure board
        let tb_size = game.size as usize;

        // check if solution is of valid size
        if solution.len() < tb_size / 2 {
            env::panic_str("Provided solution is invalid for a board of this size");
        }

        // fetch bombs from the solution
        let bombs = solution[0..tb_size / 2].to_vec();

        // keep track of the amount of tokens taken for game creation
        let mut remaining_treasure = to_yocto(tb_size as u128);

        // keep track of payments
        let mut payouts: UnorderedMap<AccountId, u128> = UnorderedMap::new(b"P");

        // generate random seed
        let env_seed = env::random_seed();
        let mut seed: u64 = 0;
        for i in 0..8 {
            seed += env_seed[i] as u64 * (8 ^ i as u64);
        }

        // init RNG
        let rng = Rng::with_seed(seed);

        // spread the tokens used for game creation among free slots
        for i in 0..tb_size {
            // flag indicating whether this slot has a bomb in it
            let is_bombed = bombs.contains(&(i as u8));

            // fetch answer for this slot
            match game.answers.get(&(i as u8)) {
                // no players chose this slot
                None => {
                    if remaining_treasure > 0 {
                        // if this slot is NOT bombed, creator gets the treasure
                        if !is_bombed {
                            // generate random treasure amount
                            let treasure = rng.u128(0..=remaining_treasure);

                            // add tokens to creator's account
                            match payouts.get(&game.creator) {
                                None => {
                                    payouts.insert(&game.creator, &treasure);
                                }
                                Some(creator_balance) => {
                                    payouts.insert(&game.creator, &(creator_balance + treasure));
                                }
                            }

                            // update remaining treasure
                            remaining_treasure -= treasure;
                        }
                    }
                }
                // some player did choose this slot
                Some(player) => {
                    // if this slot was bombed, creator gets player's tokens
                    if is_bombed {
                        match payouts.get(&game.creator) {
                            None => {
                                payouts.insert(&game.creator, &to_yocto(1));
                            }
                            Some(creator_balance) => {
                                payouts.insert(&game.creator, &(creator_balance + to_yocto(1)));
                            }
                        }
                    }
                    // if this slot was NOT bombed, player gets the treasure
                    else {
                        if remaining_treasure > 0 {
                            let treasure = rng.u128(0..=remaining_treasure);
                            // add treasure to player's account (+ the 1 Near user paid to play)
                            match payouts.get(&player) {
                                None => {
                                    payouts.insert(&player, &(treasure + to_yocto(1)));
                                }
                                Some(player_balance) => {
                                    payouts.insert(&player, &(player_balance + treasure + to_yocto(1)));
                                }
                            }

                            // update remaining treasure
                            remaining_treasure -= treasure;
                        }
                    }
                }
            }
        }

        // check if there is still any treasure left
        if remaining_treasure > 0 {
            let lotto_index = rng.u64(0..payouts.len());
            match payouts.to_vec().get(lotto_index as usize) {
                None => env::panic_str("An error occured during lucky donkey lotto"),
                Some(lucky_donkey) => {
                    payouts.insert(&lucky_donkey.0, &(lucky_donkey.1 + remaining_treasure));
                }
            }
        }

        // announce treasure board reveal
        log!("Treasureboard #{} has been revealed. The bombs were place at:", id);

        // log bomb placement on the blockchain
        for b in bombs {
            log!{"{}", b};
        }

        // transfer tokens
        for p in payouts.iter() {
            log!("{} won {} Nears", &p.0, (p.1 / 1e24 as u128) as f32);
            Promise::new(p.0).transfer(p.1);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests;
