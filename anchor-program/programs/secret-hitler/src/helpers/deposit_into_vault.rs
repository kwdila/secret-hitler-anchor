use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

pub fn deposit_into_vault<'info>(
    amount: u64,
    from: &Signer<'info>,
    vault: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let accounts = Transfer {
        from: from.to_account_info(),
        to: vault.to_account_info(), //this is checked in game_data account constraints
    };

    let ctx = CpiContext::new(system_program.to_account_info(), accounts);
    transfer(ctx, amount)?;
    Ok(())
}
