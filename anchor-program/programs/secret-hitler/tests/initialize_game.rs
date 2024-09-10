use anchor_lang::prelude::*;
use secret_hitler::id;
use solana_program_test::*;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod utils;
use utils::*;

mod instructions;
use instructions::*;

#[tokio::test]
async fn test_init_game() {
    let mut program_test = ProgramTest::new("secret_hitler", id(), None);
    program_test.set_compute_max_units(200_000);

    let host = Keypair::new();

    airdrop(&mut program_test, host.pubkey(), 10 * LAMPORTS_PER_SOL);

    // Process the instruction
    let mut context = program_test.start_with_context().await;

    // Generate game PDA
    let (game_pubkey, _game_bump) = get_game_data_address(host.pubkey());
    let (player_data_key, _player_bump) = Pubkey::find_program_address(
        &[
            b"player_data",
            game_pubkey.to_bytes().as_ref(),
            host.pubkey().to_bytes().as_ref(),
        ],
        &id(),
    );
    // Execute instruction
    let mut transaction = Transaction::new_with_payer(
        &[init_game(&host.pubkey(), &game_pubkey, &player_data_key)],
        Some(&context.payer.pubkey()),
    );

    transaction.sign(&[&context.payer, &host], context.last_blockhash);
    let result = context.banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Failed {:?}", result);
}
