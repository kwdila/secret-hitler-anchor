use anchor_lang::prelude::*;

use crate::{
    state::{GameData, PlayerData},
    GameErrorCode,
    GameState::*,
};

#[derive(Accounts)]
pub struct ResolveGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        close = player,
        seeds = [
            b"player_data",
            game_data.key().to_bytes().as_ref(),
            player.key().to_bytes().as_ref()
        ],
        bump = player_data.bump
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
        constraint = [FascistVictoryElection,FascistVictoryPolicy,LiberalVictoryExecution,LiberalVictoryPolicy,Setup]
            .contains(&game_data.game_state) @GameErrorCode::InvalidGameState,
        constraint = game_data.is_in_game(player.key) @GameErrorCode::PlayerNotInGame,             
        constraint = game_data.entry_deposit.is_some() == deposit_vault.is_some() @GameErrorCode::DepositNotFound,
        constraint = game_data.bet_amount.is_some() == bet_vault.is_some() @GameErrorCode::BetNotFound,     
    )]
    pub game_data: Account<'info, GameData>,
}
impl<'info> ResolveGame<'info> {}
