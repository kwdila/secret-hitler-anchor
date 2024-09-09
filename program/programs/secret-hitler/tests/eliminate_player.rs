use anchor_lang::prelude::*;
use secret_hitler::{
    enums::{GameState, PlayerCount},
    id,
    state::GameData,
};
use solana_program_test::*;
use solana_sdk::{
    account::AccountSharedData,
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod utils;
use utils::*;

mod instructions;
use instructions::*;

#[tokio::test]
async fn test_eliminate_player() {
    let mut program_test = ProgramTest::new("secret_hitler", id(), None);
    program_test.set_compute_max_units(200_000);

    let host = Keypair::new();

    airdrop(&mut program_test, host.pubkey(), 10 * LAMPORTS_PER_SOL);

    // Process the instruction
    let mut context = program_test.start_with_context().await;

    // Generate game PDA
    let (game_pubkey, game_bump) = get_game_data_address(host.pubkey());

    // Create initial game state
    let mut all_players = vec![Keypair::new().pubkey(); 4];
    all_players.push(host.pubkey());

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();
    let current_time = clock.unix_timestamp;

    let game_struct: GameData = GameData {
        host: host.pubkey(),
        turn_duration: 100,
        max_players: 5,
        entry_deposit: None,
        bet_amount: None,
        start_player_count: Some(PlayerCount::Five),
        all_starting_players: all_players.clone(),

        active_players: all_players,
        eliminated_players: vec![],
        turn_started_at: Some(current_time),
        game_state: GameState::ChancellorNomination,
        fascist_policies_enacted: 0,
        liberal_policies_enacted: 0,
        failed_elections: 0,
        is_special_election: false,
        current_president_index: 4,        //  0 ???
        previous_president_index: Some(0), //  Some(4)
        current_chancellor_index: None,    //  None
        previous_chancellor_index: None,   //  None
        bump: game_bump,
        deposit_vault_bump: None,
        bet_vault_bump: None,
    };

    let mut game_account_data = vec![];
    game_struct.try_serialize(&mut game_account_data).unwrap();

    let mut account = AccountSharedData::new(u32::MAX as u64, GameData::INIT_SPACE, &id());
    account.set_data_from_slice(&game_account_data);

    context.set_account(&game_pubkey, &account);

    forward_time(&mut context, 150).await;

    // Execute take instruction
    let mut transaction = Transaction::new_with_payer(
        &[eliminate_player(&host.pubkey(), &game_pubkey, None)],
        Some(&context.payer.pubkey()),
    );

    transaction.sign(&[&context.payer, &host], context.last_blockhash);
    let result = context.banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Failed {:?}", result);
}
