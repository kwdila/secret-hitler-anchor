use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{GameData, PlayerData};
use crate::{GameErrorCode, GameState};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        init,
        payer=player,
        space=PlayerData::INIT_SPACE,
        seeds = [
            b"player_data",
            game_data.key().to_bytes().as_ref(),
            player.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(
        mut,
        seeds= [
            b"deposit_vault",
            game_data.key().to_bytes().as_ref()
        ],
        bump = game_data.deposit_vault_bump.ok_or(GameErrorCode::DepositNotFound)?,
    )]
    pub deposit_vault: Option<SystemAccount<'info>>,
    #[account(
        mut,
        seeds= [
            b"bet_vault",
            game_data.key().to_bytes().as_ref()
        ],
        bump = game_data.bet_vault_bump.ok_or(GameErrorCode::BetNotFound)?,
    )]
    pub bet_vault: Option<SystemAccount<'info>>,

    #[account(
        mut,
        seeds = [
            b"secret_hitler",
            game_data.host.to_bytes().as_ref(),
        ],
        bump = game_data.bump,

        constraint = game_data.game_state == GameState::Setup @GameErrorCode::InvalidGameState,
        constraint = !game_data.is_in_game(player.key) @GameErrorCode::PlayerAlreadyJoined, 
        constraint = game_data.active_players.len() < game_data.max_players as usize @GameErrorCode::MaxPlayersReached,
        constraint = game_data.entry_deposit.is_some() == deposit_vault.is_some() @GameErrorCode::DepositNotFound,
        constraint = game_data.bet_amount.is_some() == bet_vault.is_some() @GameErrorCode::BetNotFound,
    )]
    pub game_data: Account<'info, GameData>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinGame<'info> {
    pub fn add_player(&mut self, bumps: JoinGameBumps) -> Result<()> {
        if let Some(amount) = self.game_data.entry_deposit {
            let accounts = Transfer {
                from: self.player.to_account_info(),
                to: self
                    .deposit_vault
                    .as_ref()
                    .ok_or(GameErrorCode::DepositNotFound)?
                    .to_account_info(), // this is checked in game_data account constraints
            };

            let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
            transfer(ctx, amount)?
        }

        if let Some(amount) = self.game_data.bet_amount {
            let accounts = Transfer {
                from: self.player.to_account_info(),
                to: self
                    .bet_vault
                    .as_ref()
                    .ok_or(GameErrorCode::BetNotFound)?
                    .to_account_info(), // this is checked in game_data account constraints
            };

            let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
            transfer(ctx, amount)?
        }

        self.game_data.active_players.push(self.player.key());

        self.player_data.set_inner(PlayerData {
            role: None,
            is_investigated: false,
            bump: bumps.player_data,
        });

        Ok(())
    }
}
