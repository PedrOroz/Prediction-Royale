use anchor_lang::prelude::*;

declare_id!("5JPjbA41yGiPKSFet9rW4C3zxKss8SEZBEknDG2NJi8D");

#[program]
pub mod six_seven {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
