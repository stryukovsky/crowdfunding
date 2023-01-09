use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 100)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, name: String, description: String) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    campaign.name = name;
    campaign.description = description;
    campaign.amount_donated = 0;
    campaign.admin = *ctx.accounts.owner.key;
    Ok(())
}


#[account]
pub struct Campaign {
    pub name: String,
    pub description: String,
    pub amount_donated: u64,
    pub admin: Pubkey,
}
