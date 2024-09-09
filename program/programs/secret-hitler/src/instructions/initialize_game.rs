use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{MAX_PLAYERS, MINI_TURN_DURATION, MIN_PLAYERS},
    state::{game_data::GameData, player_data::PlayerData},
    GameErrorCode,
};

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub host: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"deposit_vault",
            game_data.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub deposit_vault: Option<SystemAccount<'info>>,

    #[account(
        mut,
        seeds = [
            b"bet_vault",
            game_data.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub bet_vault: Option<SystemAccount<'info>>,

    #[account(
        init,
        payer = host,
        space = PlayerData::INIT_SPACE,
        seeds = [     
            b"player_data",       
            game_data.key().to_bytes().as_ref(),
            host.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub player_data: Account<'info, PlayerData>,

    #[account(
        init,
        payer=host,
        space=GameData::INIT_SPACE,
        seeds = [
            b"secret_hitler",
            host.key().to_bytes().as_ref()
        ],
        bump,
    )]
    pub game_data: Account<'info, GameData>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'info> {
    pub fn init_game(
        &mut self,
        max_players: u8,
        turn_duration: i64,
        entry_deposit: Option<u64>,
        bet_amount: Option<u64>,
        bumps: InitializeGameBumps,
    ) -> Result<()> {
        require!(
            max_players >= MIN_PLAYERS,
            GameErrorCode::MinimumPlayersNotReached
        );
        require!(max_players <= MAX_PLAYERS, GameErrorCode::MaxPlayersReached);
        require!(
            turn_duration >= MINI_TURN_DURATION,
            GameErrorCode::MinimumTurnDurationNotReached
        );

        require!(
            entry_deposit.is_some() == self.deposit_vault.is_some(),
            GameErrorCode::DepositNotFound
        );
        require!(
            bet_amount.is_some() == self.bet_vault.is_some(),
            GameErrorCode::BetNotFound
        );

        self.game_data.init(
            self.host.key(),
            max_players,
            turn_duration,
            entry_deposit,
            bet_amount,
            bumps.game_data,
            bumps.deposit_vault,
            bumps.bet_vault,
        )?;

        match self.game_data.entry_deposit {
            Some(amount) => {
                let accounts = Transfer {
                    from: self.host.to_account_info(),
                    to: self
                        .deposit_vault
                        .as_ref()
                        .ok_or(GameErrorCode::DepositNotFound)?
                        .to_account_info(), //this is checked in game_data account constraints
                };

                let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
                transfer(ctx, amount)?
            }
            None => (),
        }

        match self.game_data.bet_amount {
            Some(amount) => {
                let accounts = Transfer {
                    from: self.host.to_account_info(),
                    to: self
                        .bet_vault
                        .as_ref()
                        .ok_or(GameErrorCode::BetNotFound)?
                        .to_account_info(), //this is checked in game_data account constraints
                };

                let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
                transfer(ctx, amount)?
            }
            None => (),
        }

        self.player_data.set_inner(PlayerData {
            role: None,
            is_investigated: false,
            bump: bumps.player_data,
        });

        Ok(())
    }
}
