use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    state::{GameData, PlayerData},
    GameErrorCode,
};

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        mut,
        close = host,
        seeds=[
            b"secret_hitler",
            host.key().to_bytes().as_ref()
        ],
        bump=game_data.bump,
        // constraint = game_data.game_state == GameState::Setup @GameErrorCode::InvalidGameState,
        constraint = game_data.active_players.len() == 1, //only the last player(host) can end the game
        constraint = game_data.entry_deposit.is_some() == deposit_vault.is_some() @GameErrorCode::DepositNotFound,
        constraint = game_data.bet_amount.is_some() == bet_vault.is_some() @GameErrorCode::BetNotFound,
    )]
    pub game_data: Account<'info, GameData>,
    #[account(
        mut,
        seeds= [
            b"deposit_vault",
            game_data.key().to_bytes().as_ref()
        ],
        bump = game_data.deposit_vault_bump.ok_or(GameErrorCode::BetNotFound)?,
    )]
    pub deposit_vault: Option<SystemAccount<'info>>,
    #[account(
        mut,
        seeds= [
        b"bet_vault",
        game_data.key().to_bytes().as_ref()
    ],
        bump = game_data.bet_vault_bump.ok_or(GameErrorCode::DepositNotFound)?,
    )]
    pub bet_vault: Option<SystemAccount<'info>>,
    #[account(
        mut,
        close = host,
        seeds = [  
            b"player_data",
            game_data.key().to_bytes().as_ref(),
            host.key().to_bytes().as_ref()
        ],
        bump = player_data.bump,
    )]
    pub player_data: Account<'info, PlayerData>,
    pub system_program: Program<'info, System>,
}

impl<'info> EndGame<'info> {
    pub fn refund_host(&mut self) -> Result<()> {
        if let Some(_amount) = self.game_data.entry_deposit {
            let vault = self
                .deposit_vault
                .as_ref()
                .ok_or(GameErrorCode::DepositNotFound)?;
            let accounts = Transfer {
                to: self.host.to_account_info(),
                from: vault.to_account_info(), //this is checked in game_data account constraints
            };

            let game_key = self.game_data.key().to_bytes();
            let seeds = [
                b"deposit_vault",
                game_key.as_ref(),
                &[self
                    .game_data
                    .deposit_vault_bump
                    .ok_or(GameErrorCode::DepositNotFound)?],
            ];
            let signer_seeds = &[&seeds[..]];
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds,
            );
            transfer(ctx, vault.lamports())?
        }

        if let Some(_amount) = self.game_data.bet_amount {
            let vault = self.bet_vault.as_ref().ok_or(GameErrorCode::BetNotFound)?;
            let accounts = Transfer {
                to: self.host.to_account_info(),
                from: vault.to_account_info(), //this is checked in game_data account constraints
            };

            let game_key = self.game_data.key().to_bytes();
            let seeds = [
                b"bet_vault",
                game_key.as_ref(),
                &[self
                    .game_data
                    .bet_vault_bump
                    .ok_or(GameErrorCode::BetNotFound)?],
            ];
            let signer_seeds = &[&seeds[..]];
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds,
            );
            transfer(ctx, vault.lamports())?
        }

        Ok(())
    }
}
