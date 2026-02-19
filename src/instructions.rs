use anyhow::{Context, Result};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use std::str::FromStr;

use crate::config::PUMPFUN_PROGRAM_ID;

/// Pump.fun instruction discriminators
pub mod discriminators {
    use super::*;
    
    /// Create token instruction discriminator
    pub const CREATE: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
    
    /// Buy instruction discriminator (approximate - verify with IDL)
    pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
}

/// Build a Pump.fun buy instruction
/// 
/// This constructs a buy instruction for the bonding curve.
/// Note: The exact instruction format may vary. This is a simplified version.
/// In production, you should use the Anchor IDL or verify the exact instruction format.
pub fn build_buy_instruction(
    buyer: &Pubkey,
    bonding_curve: &Pubkey,
    associated_bonding_curve: &Pubkey,
    mint: &Pubkey,
    sol_reserves: &Pubkey,
    token_mint: &Pubkey,
    buy_amount_lamports: u64,
    min_tokens_out: u64,
) -> Result<Instruction> {
    let program_id = Pubkey::from_str(PUMPFUN_PROGRAM_ID)
        .context("Failed to parse Pump.fun program ID")?;

    // Build instruction data: discriminator + amount + min_out
    let mut data = Vec::new();
    data.extend_from_slice(&discriminators::BUY);
    data.extend_from_slice(&buy_amount_lamports.to_le_bytes());
    data.extend_from_slice(&min_tokens_out.to_le_bytes());

    // Account metas (order matters - verify with IDL)
    let accounts = vec![
        AccountMeta::new(*buyer, true),                    // Signer
        AccountMeta::new(*bonding_curve, false),            // Bonding curve account
        AccountMeta::new(*associated_bonding_curve, false), // Associated bonding curve
        AccountMeta::new(*mint, false),                     // Token mint
        AccountMeta::new(*sol_reserves, false),             // SOL reserves
        AccountMeta::new(*token_mint, false),               // Token mint (duplicate?)
        AccountMeta::new_readonly(system_program::id(), false), // System program
    ];

    Ok(Instruction {
        program_id,
        accounts,
        data,
    })
}

/// Parse Create instruction from transaction data
/// 
/// Extracts token information from a Pump.fun Create instruction
pub fn parse_create_instruction(data: &[u8]) -> Option<CreateInstructionData> {
    if data.len() < 8 {
        return None;
    }

    // Check discriminator
    let discriminator = &data[0..8];
    if discriminator != discriminators::CREATE {
        return None;
    }

    // Parse instruction data
    // Note: This is a simplified parser. The actual format may vary.
    // You should verify with the Pump.fun IDL or program source.
    if data.len() < 8 + 32 * 4 {
        return None;
    }

    // Extract pubkeys (assuming they're in the instruction data)
    // In reality, these are likely in the account metas
    let mut offset = 8;
    
    // This is a placeholder - actual parsing depends on instruction format
    // You'll need to extract from account metas in the transaction
    Some(CreateInstructionData {
        // These would be extracted from account metas
        mint: None,
        bonding_curve: None,
        creator: None,
    })
}

/// Data extracted from a Create instruction
#[derive(Debug, Clone)]
pub struct CreateInstructionData {
    pub mint: Option<Pubkey>,
    pub bonding_curve: Option<Pubkey>,
    pub creator: Option<Pubkey>,
}

/// Extract token creation data from transaction accounts
/// 
/// This is a helper to extract relevant accounts from a transaction
/// that contains a Create instruction
pub fn extract_create_accounts(
    accounts: &[Pubkey],
    instruction_account_indices: &[u8],
) -> Option<CreateAccounts> {
    // The account order depends on the instruction format
    // This is a simplified version - verify with IDL
    if instruction_account_indices.len() < 4 {
        return None;
    }

    Some(CreateAccounts {
        mint: accounts.get(instruction_account_indices[0] as usize)?.clone(),
        bonding_curve: accounts.get(instruction_account_indices[1] as usize)?.clone(),
        creator: accounts.get(instruction_account_indices[2] as usize)?.clone(),
    })
}

/// Accounts involved in a Create instruction
#[derive(Debug, Clone)]
pub struct CreateAccounts {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub creator: Pubkey,
}
