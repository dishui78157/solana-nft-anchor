use anchor_lang::prelude::*;

declare_id!("DC7xyFp7VkGB4r6ifceY191UrTArRRoA2rgQGertxho4");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
