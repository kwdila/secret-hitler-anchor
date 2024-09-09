use anchor_lang::prelude::*;

use crate::enums::GameState;
use crate::state::game_data::GameData;
use crate::GameErrorCode;

#[derive(Accounts)]
pub struct PresidentPower<'info> {
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
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> PresidentPower<'info> {
    pub fn activate_president_veto(&mut self, targeted_player: &Pubkey) -> Result<()> {
        let game = &mut self.game_data;
        require!(
            game.active_players.contains(targeted_player),
            GameErrorCode::PlayerNotInGame
        );

        match game.game_state {
            GameState::PresidentialPowerElection => {
                game.special_election(targeted_player)?;
                game.next_turn(GameState::ChancellorNomination)?;
            }
            GameState::PresidentialPowerPeek => {
                //                                                  //
                //              make request to server              //
                //                                                  //
                game.next_president()?;
                game.next_turn(GameState::ChancellorNomination)?;
            }
            GameState::PresidentialPowerInvestigate => {
                //                                                  //
                //              make request to server              //
                //                                                  //
                game.next_president()?;
                game.next_turn(GameState::ChancellorNomination)?;
            }
            GameState::PresidentialPowerExecution => {
                let player = *targeted_player;
                let player_index = game
                    .active_players
                    .iter()
                    .position(|key| key.eq(&player))
                    .unwrap();
                game.eliminated_players.push(player);
                game.active_players.remove(player_index);
                game.next_president()?;
                game.next_turn(GameState::ChancellorNomination)?;
            }
            _ => return err!(GameErrorCode::InvalidGameState),
        }

        Ok(())
    }
}
