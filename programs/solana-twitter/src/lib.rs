use anchor_lang::prelude::*;

declare_id!("7HPKTdgddHyyZKQ8RxTkjUa56hjHb87iN8MQ7mcsaKfd");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
