use anchor_lang::prelude::*;

use crate::{state::GameData, GameErrorCode, GameState};

#[derive(Accounts)]
pub struct ChancellorVeto<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        // Chancellor needs to veto vote in LegistlativeChancellor state
        constraint = game_data.is_chancellor(player.key)? @ GameErrorCode::ChancellorRoleNeeded,
        constraint = game_data.game_state == GameState::LegislativeChancellor @GameErrorCode::InvalidGameState,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> ChancellorVeto<'info> {
    pub fn initiate_veto(&mut self) -> Result<()> {
        self.game_data
            .next_turn(GameState::LegislativePresidentVeto)?;
        Ok(())
    }
}
