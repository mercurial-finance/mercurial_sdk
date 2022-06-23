use crate::strategy_handler::base::StrategyHandler;
use anchor_client::Program;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
pub struct FranciumHandler {}

impl StrategyHandler for FranciumHandler {
    fn withdraw_directly_from_strategy(
        &self,
        program_client: &Program,
        strategy: Pubkey,
        token_mint: Pubkey,
        base: Pubkey,
        amount: u64,
    ) -> Result<()> {
        panic!("Not implemented yet");
    }
}
