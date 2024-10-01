#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("8p4oZrABQ8WHFLiiqMmVrmdTfqqgpbVdg6UVL1ipXNi7");

#[program]
pub mod app {
    use super::*;

  pub fn close(_ctx: Context<CloseApp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.app.count = ctx.accounts.app.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.app.count = ctx.accounts.app.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeApp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.app.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeApp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + App::INIT_SPACE,
  payer = payer
  )]
  pub app: Account<'info, App>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseApp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub app: Account<'info, App>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub app: Account<'info, App>,
}

#[account]
#[derive(InitSpace)]
pub struct App {
  count: u8,
}
