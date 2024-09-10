use anchor_lang::prelude::*;

use crate::enums::GameState;
use crate::state::game_data::GameData;
use crate::GameErrorCode;

#[derive(Accounts)]
pub struct PresidentVeto<'info> {
    #[account(mut)]
    pub president: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        constraint = game_data.is_president(president.key)? @GameErrorCode::PresidentRoleRequired,
        constraint = GameState::LegislativePresidentVeto == game_data.game_state @GameErrorCode::InvalidGameState,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> PresidentVeto<'info> {
    pub fn answer_chancellor_veto(&mut self, accept_veto: bool) -> Result<()> {
        let game = &mut self.game_data;

        match accept_veto {
            true => {
                game.next_president()?;
                game.next_turn(GameState::ChancellorNomination)?;
                game.failed_elections += 1;
            }
            false => game.next_turn(GameState::LegislativeChancellor)?,
        }

        Ok(())
    }
}
