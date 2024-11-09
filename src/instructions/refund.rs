use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::state::{Contributor, Fundraiser};

pub fn refund(accounts: &[AccountInfo], bump: [u8; 1]) -> ProgramResult {
    let [contributor, fundraiser, contributor_account, contributor_ta, vault, _token_program] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    assert!(contributor.is_signer());

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    // Make sure that the time elapsed
    let current_time = Clock::get()?.unix_timestamp;
    assert!(fundraiser_account.time_ending() >= current_time);

    // // Make sure that we didn0t reach the goal
    let vault_account = unsafe { TokenAccount::from_account_info_unchecked(vault)? };
    assert!(fundraiser_account.amount_to_raise() > vault_account.amount());
    assert_eq!(fundraiser_account.mint_to_raise(), vault_account.mint());

    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&bump)];
    let signer = [Signer::from(&seeds)];

    Transfer {
        from: vault,
        to: contributor_ta,
        authority: vault,
        amount: Contributor::from_account_info(contributor_account).amount(),
    }
    .invoke_signed(&signer)?;

    unsafe {
        let lamports = contributor_account.borrow_lamports_unchecked();
        *contributor.borrow_mut_lamports_unchecked() += lamports;
        *contributor_account.borrow_mut_lamports_unchecked() -= lamports;

        // contributor.realloc(0, true)?;
    }

    Ok(())
}
