use anchor_lang::prelude::*;

declare_id!("51D4Gq8vDCcoRf95RrtcHad5wAVMrUvG1EoDkPp9Jdim");

pub const CARSINO_HOURSE_SEED_PREFIX: &str = "carsino_house";
#[program]
pub mod carsino {
    use super::*;

    pub fn build_house(ctx: Context<BuildHouse>) -> Result<()> {
        let house = &mut ctx.accounts.house;
        house.player_count = 0;
        house.status = HouseStatus::Initialized;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct BuildHouse<'info> {
    /// CHECK:
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        space = House::LEN + 8,
        seeds = [
            CARSINO_HOURSE_SEED_PREFIX.as_bytes(),
            authority.to_account_info().key.as_ref(),
        ],
        bump,
        payer = authority,
    )]
    pub house: Account<'info, House>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum HouseStatus {
    None,
    Initialized
}

impl Default for HouseStatus {
    fn default() -> Self {
        Self::None
    }
}


#[account]
#[derive(Default)]
pub struct House {
    pub authority: Pubkey,
    pub status: HouseStatus,
    pub player_count: u32,
}

impl House {
    pub const LEN: usize = 32 + 1 + 4;
}