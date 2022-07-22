use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("51D4Gq8vDCcoRf95RrtcHad5wAVMrUvG1EoDkPp9Jdim");

#[program]
pub mod carsino {
    use super::*;

    pub fn build_house(ctx: Context<BuildHouse>) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct BuildHouse {
    
}
