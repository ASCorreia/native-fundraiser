use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    ProgramResult,
};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::state::Fundraiser;

pub fn checker(accounts: &[AccountInfo], bump: [u8; 1]) -> ProgramResult {
    let [_cranker, fundraiser, vault, maker_ta, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    // Check that the owner of the vault is the maker
    assert_eq!(&fundraiser_account.maker(), unsafe {
        &TokenAccount::from_account_info_unchecked(maker_ta)?.authority()
    });

    let vault_account = unsafe { TokenAccount::from_account_info_unchecked(vault)? };
    // Save the current amount of the vault
    let amount = vault_account.amount();

    // Check that the mint of the vault is the mint to raise
    assert_eq!(&vault_account.mint(), &fundraiser_account.mint_to_raise());

    // Check that the amount of the vault is more than the amount to raise
    assert!(amount >= fundraiser_account.amount_to_raise());

    // Derive the signer
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    // Transfer the amount to the fundraiser
    Transfer {
        from: vault,
        to: maker_ta,
        authority: vault,
        amount,
    }
    .invoke_signed(&signer)?;

    Ok(())
}
