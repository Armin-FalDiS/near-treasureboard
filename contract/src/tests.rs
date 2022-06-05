use super::*;
use near_sdk::{testing_env, VMContext, Balance, Gas};
use near_sdk::test_utils::{ VMContextBuilder };

/// returns mock context
fn get_context(current: AccountId, predecessor: AccountId, deposit: Balance) -> VMContext {
    let mut builder = VMContextBuilder::new();
    builder.current_account_id(current);
    builder.predecessor_account_id(predecessor);
    builder.account_balance(to_yocto(0));
    builder.attached_deposit(deposit);
    builder.prepaid_gas(Gas::ONE_TERA * 20);

    builder.build()
}

/// returns fake account for test
fn get_account(name: &str) -> AccountId { AccountId::new_unchecked(String::from(name)) }


#[test]
fn newgame() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4)));

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, vec![1, 2, 3, 4]);

    assert_eq!(contract.games()[0].id, 1);
}

#[test]
#[should_panic(expected = "Attached deposit is not sufficient to create a board of this size")]
fn newgame_low_funds() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4)));

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Big, vec![65, 65, 65, 65, 65]);
}

#[test]
fn play() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4)));

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, vec![65, 65, 65, 65, 65]);

    contract.play(1, 0);

    assert_eq!(contract.get_game(1).answers.get(&0).unwrap(), get_account("armin.near"));
}

#[test]
#[should_panic(expected = "No such a game exists")]
fn play_invalid_game() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(1)));

    let mut contract = NearTreasureBoardGame::default();

    contract.play(10000, 1);
}

#[test]
#[should_panic(expected = "That slot has already been taken")]
fn play_duplicate() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4)));

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, vec![65, 65, 65, 65, 65]);

    contract.play(1, 1);
    contract.play(1, 1);
}

#[test]
#[should_panic(expected = "Attached deposit is insufficient")]
fn play_low_fund() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4)));

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, vec![65, 65, 65, 65, 65]);

    testing_env!(get_context(get_account("treasureboard.near"), get_account("ethuil.near"), to_yocto(0)));

    contract.play(1, 1);
}

#[test]
#[should_panic(expected = "This board is closed")]
fn play_closed_game() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4)));

    let mut contract = NearTreasureBoardGame::default();

    contract.new_game(BoardSize::Small, vec![65, 65, 65, 65, 65]);

    contract.play(1, 0);
    contract.play(1, 1);

    contract.play(1, 2);
}

#[test]
fn reveal() {
    testing_env!(get_context(get_account("treasureboard.near"), get_account("armin.near"), to_yocto(4)));
        
    let mut contract = NearTreasureBoardGame::default();

    let solution: Vec<u8> = vec![0, 1, 67, 45, 45, 32, 45, 0];

    contract.new_game(BoardSize::Small, env::sha256(&solution));


    contract.play(1, 2);
    contract.play(1, 3);

    contract.reveal(1, solution);
}