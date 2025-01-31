use anchor_lang::prelude::*;

declare_id!("Eq9V6SLJKvpALMrFEqhhGVxWubauhCcuN4SQp5yZZoyZ");

// Anchor programs always use
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// What we will put inside the Favorites PDA
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,
}

// When people call the set_favorites instruction, they will need to provide the accounts that will
// be modified. This keeps Solana fast!
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // все инпуты метода setFavorites в одной структуре
    #[account(
        init, // инициализация нового аккаунта, при вызове метода setFavorites
        payer = user, // кто платит за создание аккаунта
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, // максимальный размер аккаунта
        seeds = [b"favorites", user.key().as_ref()], // ключ аккаунта, по которому его можно будет найти. (маппинг всего аккаунта - на основе слова "favorites" и ключа user)
        bump, // для исключения видимости приватного ключа (что бы не прописывать вручную)
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

// Our Solana program!
#[program]
pub mod favorites {
    use super::*;

    // Our instruction handler! It sets the user's favorite number and color
    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );

        context
            .accounts
            .favorites
            .set_inner(Favorites { number, color });
        Ok(())
    }

    // We can also add a get_favorites instruction to get the user's favorite number and color
}



// #[program]
// pub mod favorites {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}
