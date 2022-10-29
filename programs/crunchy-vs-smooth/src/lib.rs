use anchor_lang::prelude::*;

declare_id!("99xsRxMDZJFTXr3S8Xu3eU8afc4kYeawscL2NC8PcqTL");

#[program]
pub mod crunchy_vs_smooth {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.crunchy = 0;
        vote_account.smooth = 0;
        Ok(())
    }

    pub fn vote_crunchy(ctx: Context<Vote>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.crunchy += 1;
        Ok(())
    }

    pub fn vote_smooth(ctx: Context<Vote>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.smooth += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[account]
pub struct VoteAccount {
    pub crunchy: u64,
    pub smooth: u64,
}
