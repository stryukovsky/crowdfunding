use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::initialize::Campaign;

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
    let campaign = &mut ctx.accounts.campaign;
    let signer = &ctx.accounts.signer;
    if signer.key.key() != campaign.admin {
        return Err(ProgramError::IncorrectProgramId);
    }
    let rent = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
    if **campaign.to_account_info().lamports.borrow() - rent < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
    **signer.to_account_info().try_borrow_mut_lamports()? += amount;
    Ok(())
}
