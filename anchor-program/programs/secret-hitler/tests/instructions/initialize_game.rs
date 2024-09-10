use anchor_lang::prelude::{Pubkey, ToAccountMetas};
use secret_hitler::id;
use solana_sdk::instruction::Instruction;

#[allow(dead_code)]
pub fn init_game(host: &Pubkey, game_data: &Pubkey, player_data: &Pubkey) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: ToAccountMetas::to_account_metas(
            &secret_hitler::accounts::InitializeGame {
                host: *host,
                player_data: *player_data,
                game_data: *game_data,
                deposit_vault: None,
                bet_vault: None,
                system_program: solana_sdk::system_program::ID,
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&secret_hitler::instruction::InitializeGame {
            max_players: 5,
            turn_duration: 120,
            entry_deposit: None,
            bet_amount: None,
        }),
    }
}
