use anchor_lang::prelude::*;

declare_id!("2G7wBpPfDCbePwygNkvBVHXK3nkaAje2rKxN2SU9obEp");

#[program]
pub mod favorites_solana_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
