pub use crate::errors::GameErrorCode;
pub use anchor_lang::prelude::*;

pub mod constants;
pub mod enums;
pub mod errors;
pub mod helpers;
pub mod instructions;
pub mod state;

pub use enums::*;
pub use instructions::*;

declare_id!("6VYMcKKWTWucWArbJ8soisLHaNqA4PEHPUS5jDo1SyBn");

#[program]
pub mod secret_hitler {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        max_players: u8,
        turn_duration: i64,
        entry_deposit: Option<u64>,
        bet_amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.init_game(
            max_players,
            turn_duration,
            entry_deposit,
            bet_amount,
            ctx.bumps,
        )?;
        Ok(())
    }
    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        ctx.accounts.add_player(ctx.bumps)?;
        Ok(())
    }
    pub fn leave_game(ctx: Context<LeaveGame>) -> Result<()> {
        ctx.accounts.remove_player()?;
        Ok(())
    }
    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        ctx.accounts.start()?;
        Ok(())
    }
    pub fn nominate_chancelor(ctx: Context<NominateChancellor>, player: Pubkey) -> Result<()> {
        ctx.accounts.nominated_chancellor(player, ctx.bumps)?;
        Ok(())
    }
    pub fn vote_chancellor(ctx: Context<VoteChancellor>, vote: PlayerVote) -> Result<()> {
        ctx.accounts.vote(vote)?;
        Ok(())
    }
    pub fn enact_policy(ctx: Context<EnactPolicy>, policy: Option<PolicyCard>) -> Result<()> {
        ctx.accounts.enact_policy(policy)?;
        Ok(())
    }
    pub fn chancellor_initiate_veto(ctx: Context<ChancellorVeto>) -> Result<()> {
        ctx.accounts.initiate_veto()?;
        Ok(())
    }
    pub fn president_answer_veto(ctx: Context<PresidentVeto>, accept_veto: bool) -> Result<()> {
        ctx.accounts.answer_chancellor_veto(accept_veto)?;
        Ok(())
    }
    pub fn eliminate_inactive_player(ctx: Context<EliminatePlayer>) -> Result<()> {
        ctx.accounts.eliminate_player()?;
        Ok(())
    }
}
