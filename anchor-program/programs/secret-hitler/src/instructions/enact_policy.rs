use anchor_lang::prelude::*;

use crate::{
    constants::{FASCIST_VICTORY_POLICIES, LIBERAL_VICTORY_POLICIES},
    state::GameData,
    GameErrorCode, GameState, PolicyCard,
};

#[derive(Accounts)]
pub struct EnactPolicy<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        // Ensure player is either the current president or chancellor
        constraint = game_data.is_chancellor(player.key)? || game_data.is_president(player.key)? @GameErrorCode::PlayerNotInGovernment,
        // president needs to enact policy in LegistlativePresident state
        constraint = game_data.is_president(player.key)? == (game_data.game_state == GameState::LegislativePresident) @GameErrorCode::PresidentPolicyError,
        // Chancellor needs to enact policy in LegistlativeChancellor state
        constraint = game_data.is_chancellor(player.key)? == (game_data.game_state == GameState::LegislativeChancellor) @GameErrorCode::ChancellorPolicyError,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> EnactPolicy<'info> {
    pub fn enact_policy(&mut self, policy: Option<PolicyCard>) -> Result<()> {
        let game = &mut self.game_data;

        // Check game state for the president or chancellor
        match game.game_state {
            GameState::LegislativePresident => {
                game.next_turn(GameState::LegislativeChancellor)?;
            }
            GameState::LegislativeChancellor => {
                match policy.ok_or(GameErrorCode::ChancellorPolicyError)? {
                    PolicyCard::Fascist => game.fascist_policies_enacted += 1,
                    PolicyCard::Liberal => game.liberal_policies_enacted += 1,
                }

                if game.liberal_policies_enacted == LIBERAL_VICTORY_POLICIES {
                    game.game_state = GameState::LiberalVictoryPolicy;
                    return Ok(());
                }
                if game.fascist_policies_enacted == 0 {
                    game.next_president()?;
                    game.next_turn(GameState::ChancellorNomination)?;
                    return Ok(());
                }
                if game.fascist_policies_enacted == FASCIST_VICTORY_POLICIES {
                    game.game_state = GameState::FascistVictoryPolicy;
                    return Ok(());
                }

                //handle different gameboards for different player counts
                let fascist_board = game.get_fascist_board()?;

                // Determine the presidential power state based on the board and enacted policies
                match game.get_presidential_power_state(fascist_board) {
                    Some(state) => game.next_turn(state)?,
                    None => {
                        game.next_president()?;
                        game.next_turn(GameState::ChancellorNomination)?;
                    }
                }
            }
            _ => return err!(GameErrorCode::InvalidGameState),
        }

        Ok(())
    }
}
