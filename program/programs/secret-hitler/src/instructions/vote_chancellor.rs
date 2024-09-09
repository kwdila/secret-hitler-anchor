use anchor_lang::prelude::*;

use crate::state::{GameData, Nomination};
use crate::{GameErrorCode, GameState, PlayerVote};

#[derive(Accounts)]
pub struct VoteChancellor<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds =[
            b"chancellor_nomination",
            game_data.key().to_bytes().as_ref()
            ],
        bump = nomination.bump,
    )]
    pub nomination: Account<'info, Nomination>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref(),
        ],
        bump = game_data.bump,
        constraint = game_data.game_state == GameState::ChancellorVoting @GameErrorCode::InvalidGameState,
        constraint = game_data.active_players.contains(player.key) @GameErrorCode::PlayerNotInGame,
    )]
    pub game_data: Account<'info, GameData>,
}

impl<'info> VoteChancellor<'info> {
    pub fn vote(&mut self, vote: PlayerVote) -> Result<()> {
        let nomination = &mut self.nomination;
        let game = &mut self.game_data;
        let num_players = game.active_players.len() as u8;
        let current_time = Clock::get()?.unix_timestamp;

        require!(
            current_time - game.turn_started_at.ok_or(GameErrorCode::TurnFinished)?
                < game.turn_duration,
            GameErrorCode::TurnFinished
        );

        let total_votes = nomination.ja + nomination.nein;
        require!(total_votes <= num_players, GameErrorCode::MaxVotesReached);

        // get player index from all active players in game
        let player_index = game
            .active_players
            .iter()
            .position(|key| key == self.player.key)
            .ok_or(GameErrorCode::PlayerNotInGame)? as u8;

        // make sure the player has not voted already
        require!(
            !nomination.voters_index.contains(&player_index),
            GameErrorCode::PlayerAlreadyVoted
        );

        match vote {
            PlayerVote::Nein => nomination.nein += 1,
            PlayerVote::Ja => nomination.ja += 1,
        }

        if nomination.nein > (num_players / 2) {
            game.failed_elections += 1;
            game.next_turn(GameState::ChancellorNomination)?;
        }

        if nomination.ja > (num_players / 2) {
            game.previous_chancellor_index = game.current_chancellor_index;
            // this is safe because it is checked earlier
            let index = game.get_player_index(&nomination.nominee).unwrap() as u8;
            game.current_chancellor_index = Some(index);
            game.next_turn(GameState::LegislativePresident)?;
        }

        Ok(())
    }
}
