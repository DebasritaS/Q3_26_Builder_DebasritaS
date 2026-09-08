
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    close_account,
    transfer_checked,
    CloseAccount,
    Mint,
    TokenAccount,
    TokenInterface,
    TransferChecked,
};

use crate::{error::ErrorCode, state::Escrow, ESCROW_SEED};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [
            ESCROW_SEED,
            maker.key().as_ref(),
            escrow.seed.to_le_bytes().as_ref()
        ],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

     #[account(mut)]
    /// CHECK: The maker is validated through the escrow account.
     pub maker: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn take(&mut self) -> Result<()> {
        // Make sure the escrow is still active.
        let current_time = Clock::get()?.unix_timestamp;

        require!(
            current_time <= self.escrow.expiration,
            ErrorCode::EscrowExpired
        );

        let maker_key = self.maker.key();
        let seed_bytes = self.escrow.seed.to_le_bytes();

        let signer_seeds: [&[&[u8]]; 1] = [&[
            ESCROW_SEED,
            maker_key.as_ref(),
            seed_bytes.as_ref(),
            &[self.escrow.bump],
        ]];

        // Transfer token B from the taker to the maker.
        let payment_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let payment_ctx =
            CpiContext::new(self.token_program.key(), payment_accounts);

        transfer_checked(
            payment_ctx,
            self.escrow.receive,
            self.mint_b.decimals,
        )?;

        // Release token A from the escrow vault to the taker.
        let release_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let release_ctx = CpiContext::new_with_signer(
            self.token_program.key(),
            release_accounts,
            &signer_seeds,
        );

        transfer_checked(
            release_ctx,
            self.vault.amount,
            self.mint_a.decimals,
        )?;

        // Close the vault and return its rent to the maker.
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.key(),
            close_accounts,
            &signer_seeds,
        );

        close_account(close_ctx)?;

        Ok(())
    }
}

