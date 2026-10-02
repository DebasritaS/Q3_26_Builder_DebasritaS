use anchor_lang::prelude::*;
use mpl_core::{
    fetch_collection_plugin,
    instructions::{AddCollectionPluginV1CpiBuilder, UpdateCollectionPluginV1CpiBuilder},
    types::{Attribute, Attributes, Plugin, PluginAuthority, PluginType},
};

use crate::error::ErrorCode;

pub const TOTAL_STAKED_KEY: &str = "total_staked";

/// Adjust the `total_staked` counter stored in the collection's Attributes
/// plugin. Pass +1 on stake, -1 on unstake/burn.
///
/// A missing plugin (or missing key) is treated as 0 on increment and errors
/// on decrement, so an unstake/burn can never drive the counter negative.
/// Any unrelated collection attributes are preserved.
pub fn update_total_staked<'a>(
    collection: &AccountInfo<'a>,
    update_authority: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    mpl_core_program: &AccountInfo<'a>,
    signer_seeds: &[&[&[u8]]],
    delta: i8,
) -> Result<()> {
    let fetched: Option<Attributes> =
        fetch_collection_plugin::<Attributes>(collection, PluginType::Attributes)
            .ok()
            .map(|(_, attrs, _)| attrs);

    let mut total: u64 = 0;
    let mut found = false;
    let mut attribute_list: Vec<Attribute> = Vec::new();

    if let Some(attrs) = &fetched {
        for attr in &attrs.attribute_list {
            if attr.key == TOTAL_STAKED_KEY {
                found = true;
                total = attr
                    .value
                    .parse::<u64>()
                    .map_err(|_| ErrorCode::InvalidTotalStaked)?;
            } else {
                attribute_list.push(attr.clone());
            }
        }
    }

    if delta >= 0 {
        total = total
            .checked_add(delta as u64)
            .ok_or(ErrorCode::InvalidTotalStaked)?;
    } else {
        require!(found, ErrorCode::InvalidTotalStaked);
        total = total
            .checked_sub(delta.unsigned_abs() as u64)
            .ok_or(ErrorCode::InvalidTotalStaked)?;
    }

    attribute_list.push(Attribute {
        key: TOTAL_STAKED_KEY.to_string(),
        value: total.to_string(),
    });

    let plugin = Plugin::Attributes(Attributes { attribute_list });

    if fetched.is_none() {
        AddCollectionPluginV1CpiBuilder::new(mpl_core_program)
            .collection(collection)
            .payer(payer)
            .authority(Some(update_authority))
            .system_program(system_program)
            .plugin(plugin)
            .init_authority(PluginAuthority::UpdateAuthority)
            .invoke_signed(signer_seeds)?;
    } else {
        UpdateCollectionPluginV1CpiBuilder::new(mpl_core_program)
            .collection(collection)
            .payer(payer)
            .authority(Some(update_authority))
            .system_program(system_program)
            .plugin(plugin)
            .invoke_signed(signer_seeds)?;
    }

    Ok(())
}
