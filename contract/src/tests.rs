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

#[test]
#[should_panic(expected = "Attached deposit is not sufficient to create a board of this size")]
fn newgame_low_funds() {
    let context = get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4));

    testing_env!(context);

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Big, String::from("WeiredHash"));
}

#[test]
fn newgame() {
    let context = get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4));

    testing_env!(context);

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, String::from("MyHash"));

    assert_eq!(contract.games()[0].answer_hash, "MyHash");
}

#[test]
#[should_panic(expected = "No such a game exists")]
fn play_invalid_game() {
    let context = get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(1));

    testing_env!(context);

    let mut contract = NearTreasureBoardGame::default();

    contract.play(10000, 1);
}

#[test]
#[should_panic(expected = "That slot has already been taken")]
fn play_duplicate() {
    let context = get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4));

    testing_env!(context);

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, String::from("MyHash"));

    contract.play(1, 1);

    contract.play(1, 1);
}

#[test]
#[should_panic(expected = "Attached deposit is insufficient")]
fn play_low_fund() {
    let context = get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4));

    testing_env!(context);

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, String::from("MyHash"));

    let context = get_context(get_account("treasureboard.near"), get_account("ethuil.near"), to_yocto(0));

    testing_env!(context);

    contract.play(1, 1);
}