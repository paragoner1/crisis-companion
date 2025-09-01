// src/onchain/emergency_rewards.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Emerg1234567890AbcdefGhijklMnopQrstUv");

#[program]
pub mod emergency_rewards {
    use super::*;

    pub fn award_emergency_tokens(ctx: Context<AwardTokens>, amount: u64, emergency_type: u8) -> Result<()> {
        require!(amount <= 1000, ErrorCode::InvalidAmount);

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer_token_account.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            amount,
        )?;

        emit!(RewardAwarded {
            recipient: ctx.accounts.recipient_token_account.owner,
            amount,
            emergency_type,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AwardTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct RewardAwarded {
    pub recipient: Pubkey,
    pub amount: u64,
    pub emergency_type: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid reward amount")]
    InvalidAmount,
}
