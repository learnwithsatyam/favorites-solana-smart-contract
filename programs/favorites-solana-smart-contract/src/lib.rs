use anchor_lang::prelude::*;

// this is program address or id for out smart contract
declare_id!("9bLJCC7xD9oyF4CozMBN3zgPhydqE5BzwqPxVaBZ1AEv"); // In solana playground, we dont need to fill this in. It will be automatically filled when we deploy our contract

// It is something that writtent to every accound on blockchain by anchor program which just specifies the type of account it is. It is used by anchor for some of its checks
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // it means when we deploy on blockchain we will need 8 bytes + the size of whatever we are storing

// Best thing about anchor is that you can take a regular rust program and with the help of a macro you can turn it in to an anchor program
// Here is a regular favorites rust module. Now is we use a macro called "Program" then suddenly the "favorites" rust module becomes a full solana program or smart contract.
// This is the power of anchor, it saves you from having to do boring things and also applies a bunch of safe default.
// Becoming a smart contract simply means that the function "setFavorites" is now a solana instruction handler. It will take instructions provided by client and run them and save them in to the blockchain
#[program]
pub mod favorites {
    use super::*;
    pub fn setFavorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id); // you can see this in solscan
        let user_public_key = context.accounts.user.key();

        msg!(
            "user {}'s favorite number is {}, favorite color is {}, and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(()) // Solana programs write info on blockchain rather than returning anything.
    }
}


// we are using "account" macro because we will need to save the instances of structs in an account whenever we store it.
// To let anchor know how big the "Favorites" is we are also going to use "InitSpace". That gives all of our instances of favorites the InitSpace attribute which is the total space used by all the items inside.
// We also need to specify the size of each individual item or property in struct. It is because blockchain does not use heap or async/await. It means everything is stored on stack and needs to be deterministic. Since strings can be of any length we need to specify a max length for strings so that it will allocate that space on stack
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    // 50 bytes
    #[max_len(50)]
    pub color: String,

    // since vector is nested, here we are saying that length of vector array is 5 items only and each individual item ( which is string) can be of 50 bytes
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

// when people are going to call the "setFavorites" function from module, they are going to need to provide a list of accounts they are gonna modify in the blockchain . One of the good things about solana is that if there is a bunch of people on one side  who are running a transaction involing their accounts and then there are a bunch of people on other hand who are running a different transaction involving their accounts, those transactions dont't block each other. Accounts don't overlap and solana can perform or exeute transactions at the same time.
// We will make a struct to store this information. Below is the struct of accounts for out "setFavorites"instruction handler  so the tradition is the call this struct of accounts as the same thing as our setFavorties instruction handler, just in title case.
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    // here we are saying that signer is mutual because the person who signs the insturctions to setFavorites is going to pay to create SetFavorites account on blockchain.
    #[account(mut)]
    pub user: Signer<'info>,

    // also we the person running the setFavorites to specify the SetFavorites account they want to  write to. It does not mean we will let them write to that account , we just need to specify.
    #[account(
        init_if_needed,
        payer=user,
        space=ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
