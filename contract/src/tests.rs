use super::*;
use near_sdk::{testing_env, Balance, VMContext};
use near_sdk::test_utils::{ VMContextBuilder };

/// returns mock context
fn get_context(current: AccountId, predecessor: AccountId, deposit: Balance) -> VMContext {
    let mut builder = VMContextBuilder::new();
    builder.current_account_id(current);
    builder.predecessor_account_id(predecessor);
    builder.attached_deposit(deposit);

    builder.build()
}

/// returns fake account for test
fn get_account(name: &str) -> AccountId { AccountId::new_unchecked(String::from(name)) }

/// returns default contract
fn get_contract() -> NearTreasureBoardGame {
    NearTreasureBoardGame {
    boards: UnorderedMap::new(b"B"),
    next_index: 1_u128
    }
}

#[test]
#[should_panic(expected = "Attached deposit is not sufficient for a game of this size")]
fn newgame_low_funds() {
    let context = get_context(get_account("armin.near"), get_account("faldis.near"), to_yocto(4));

    testing_env!(context);

    let mut contract = get_contract();

    contract.new_game(BoardSize::Big, String::from("WeiredHash"));
}

#[test]
fn newgame() {
    let context = get_context(get_account("armin.near"), get_account("faldis.near"), to_yocto(4));

    testing_env!(context);

    let mut contract = get_contract();

    contract.new_game(BoardSize::Small, String::from("MyHash"));

    assert_eq!(contract.games()[0].answer_hash, "MyHash");
}