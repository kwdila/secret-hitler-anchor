use anchor_lang::prelude::*;

use crate::{constants::MIN_PLAYERS, state::GameData, GameErrorCode, GameState, PlayerCount};

#[derive(Accounts)]
pub struct StartGame<'info> {
    pub server: Signer<'info>,
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        mut,
        seeds=[
            b"secret_hitler",
            host.key().to_bytes().as_ref()
        ],
        bump=game_data.bump,
        constraint = game_data.game_state == GameState::Setup @GameErrorCode::InvalidGameState,
        constraint = game_data.active_players.len() >= MIN_PLAYERS as usize, //minimum players to play
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> StartGame<'info> {
    pub fn start(&mut self) -> Result<()> {
        let game = &mut self.game_data;
        game.start_player_count = PlayerCount::from(game.active_players.len() as u8);
        game.all_starting_players = game.active_players.clone();
        game.next_turn(GameState::ChancellorNomination)?;

        Ok(())
    }
}
