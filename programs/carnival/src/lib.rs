use anchor_lang::prelude::*;

declare_id!("51D4Gq8vDCcoRf95RrtcHad5wAVMrUvG1EoDkPp9Jdim");

pub const CARSINO_HOURSE_SEED_PREFIX: &str = "carsino_house";
pub const CARSINO_PLAYER_SEED_PREFIX: &str = "carsino_player";
pub const MAX_PLAYER_COUNT: usize = 6;

#[program]
pub mod carsino {
    use super::*;

    pub fn build_house(ctx: Context<BuildHouse>) -> Result<()> {
        let house = &mut ctx.accounts.house;
        house.authority = ctx.accounts.authority.to_account_info().key();
        house.player_count = 0;
        house.status = HouseStatus::Initialized;
        house.players = vec![];
        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let house = &mut ctx.accounts.house;
        player.key = ctx.accounts.authority.to_account_info().key();
        player.house = house.key();
        house.player_count = house.player_count.checked_add(1).unwrap();
        house.players.push(player.key());
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
            authority.key.as_ref(),
        ],
        bump,
        payer = authority,
    )]
    pub house: Account<'info, House>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPlayer<'info> {
    /// CHECK:
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = house.status == HouseStatus::Initialized,
        constraint = !house.players.iter().any(|x| x == player.to_account_info().key),
        constraint = house.player_count < MAX_PLAYER_COUNT.try_into().unwrap(),
    )]
    pub house: Account<'info, House>,

    #[account(
        init,
        payer = authority,
        space = Player::LEN + 8,
        seeds = [
            CARSINO_PLAYER_SEED_PREFIX.as_bytes(),
            house.key().as_ref(),
            authority.key.as_ref(),
        ],
        bump,
    )]
    pub player: Account<'info, Player>,

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
    pub players: Vec<Pubkey>,
}

impl House {
    pub const LEN: usize = 32 + 1 + 4 + 32 * MAX_PLAYER_COUNT;
}

#[account]
#[derive(Default)]
pub struct Player {
    pub key: Pubkey,
    pub house: Pubkey,
}

impl Player {
    pub const LEN: usize = 32 + 32;
}