use anchor_lang::prelude::*;
use crate::Campaign;

pub fn donate(ctx: Context<Donate>, amount: u64) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.sender.key(),
        &ctx.accounts.campaign.key(),
        amount,
    );
    let transfer_result = anchor_lang::solana_program::program::invoke(&transfer_instruction,
         &[
             ctx.accounts.sender.to_account_info(),
             ctx.accounts.campaign.to_account_info(),
         ]);
    if transfer_result.is_err() {
        return Err(ProgramError::InsufficientFunds);
    }
    let campaign = &mut ctx.accounts.campaign;
    campaign.amount_donated += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>
}