use anchor_lang::prelude::*;

use crate::enums::GameState;
use crate::state::game_data::GameData;
use crate::state::nomination::Nomination;
use crate::GameErrorCode;

#[derive(Accounts)]
pub struct NominateChancellor<'info> {
    #[account(mut)]
    pub president: Signer<'info>,
    #[account(
        init_if_needed,
        payer=president,
        space= Nomination::INIT_SPACE,
        seeds =[
            b"chancellor_nomination",
            game_data.key().to_bytes().as_ref()
            ],
        bump
    )]
    pub nomination: Account<'info, Nomination>,
    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref()
            ],
        bump = game_data.bump,

        constraint = game_data.is_president(president.key)? @GameErrorCode::PresidentRoleRequired,
        constraint = GameState::ChancellorNomination == game_data.game_state @GameErrorCode::InvalidGameState,
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

impl<'info> NominateChancellor<'info> {
    pub fn nominated_chancellor(
        &mut self,
        nominated_chancellor: Pubkey,
        bumps: NominateChancellorBumps,
    ) -> Result<()> {
        let game = &mut self.game_data;
        require!(
            game.active_players.contains(&nominated_chancellor),
            GameErrorCode::PlayerNotInGame
        );

        let prev_president = game.previous_president_index;

        let eligible = match game.previous_chancellor_index.is_some() {
            //check if nominated_chancellor is eligible
            true => {
                let prev_chancellor_index = game
                    .previous_chancellor_index
                    .ok_or(GameErrorCode::PrevChancellorNotFound)?;

                let prev_chancellor = game.active_players.get(prev_chancellor_index as usize);
                let mut result = true;
                if prev_chancellor.is_some() {
                    result = prev_chancellor.unwrap().ne(&nominated_chancellor);
                    //prev chancellor ineligible
                }

                match game.active_players.len() <= 5 {
                    true => (), // no checks needed, prev president is allowed
                    false => {
                        if prev_president.is_some() {
                            result &= game
                                .active_players
                                .get(prev_president.ok_or(GameErrorCode::PrevPresidentNotFound)?
                                    as usize)
                                .ok_or(GameErrorCode::PlayerNotInGame)?
                                .eq(&nominated_chancellor) //prev president ineligible
                        }
                    }
                }
                result
            }
            false => true, // no checks needed if this is the first chancellor
        };
        require!(eligible, GameErrorCode::IneligibleChancellorNominated);

        game.next_turn(GameState::ChancellorVoting)?;

        self.nomination.voters_index = vec![self.game_data.current_president_index];
        self.nomination.nominee = nominated_chancellor;
        self.nomination.nein = 0;
        self.nomination.ja = 1; // it is assumed the president voted ja by nominating
        self.nomination.bump = bumps.nomination;

        Ok(())
    }
}
