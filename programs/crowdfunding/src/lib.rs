extern crate core;

use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
mod crowdfunding {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, description: String) -> Result<()> {
        initialize::initialize(ctx, name, description)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        withdraw::withdraw(ctx, amount)
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        donate::donate(ctx, amount)
    }
}

