use anchor_lang::prelude::*;

#[constant]
pub const DECIMALS: u8 = 6;

#[constant]
pub const TRANSFER_FEE_BPS: u16 = 100;

#[constant]
pub const MAXIMUM_FEE: u64 = u64::MAX;

#[constant]
pub const TOKEN_NAME: &str = "Anchor RUST Stablecoin";

#[constant]
pub const TOKEN_SYMBOL: &str = "RUST";

#[constant]
pub const TOKEN_URI: &str = "https://example.com/rust.json";
