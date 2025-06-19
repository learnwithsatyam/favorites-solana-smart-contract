use anchor_lang::prelude::*;

// this is program address or id for out smart contract
declare_id!("9bLJCC7xD9oyF4CozMBN3zgPhydqE5BzwqPxVaBZ1AEv"); // In solana playground, we dont need to fill this in. It will be automatically filled when we deploy our contract

// It is something that writtent to every accound on blockchain by anchor program which just specifies the type of account it is. It is used by anchor for some of its checks
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // it means when we deploy on blockchain we will need 8 bytes + the size of whatever we are storing

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
