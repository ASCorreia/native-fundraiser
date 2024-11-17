use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    sysvars::{clock::Clock, Sysvar},
    ProgramResult, msg,
};

use pinocchio_token::{state::TokenAccount, instructions::Transfer};

use crate::state::{Contributor, Fundraiser};

pub fn refund(accounts: &[AccountInfo], bump: [u8; 1]) -> ProgramResult {
    let [contributor, fundraiser, contributor_account, contributor_ta, vault, _token_program] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    // assert!(contributor.is_signer());

    // let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    // // Make sure that the time elapsed
    // let current_time = Clock::get()?.unix_timestamp;
    // assert!(fundraiser_account.time_ending() >= current_time);

    // // // Make sure that we didn0t reach the goal
    // let vault_account = unsafe { TokenAccount::from_account_info_unchecked(vault)? } ;
    // assert!(fundraiser_account.amount_to_raise() > vault_account.amount());
    // assert_eq!(&fundraiser_account.mint_to_raise(), vault_account.mint());

    // let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&bump)];
    // let signer = [Signer::from(&seeds)];

    // Transfer {
    //     from: vault,
    //     to: contributor_ta,
    //     authority,
    //     amount: Contributor::from_account_info(contributor_account).amount(),
    // }
    // .invoke_signed(&signer)?;

    unsafe {
        *contributor.borrow_mut_lamports_unchecked() += *contributor_account.borrow_lamports_unchecked();

        let data_ptr = contributor_account.borrow_mut_data_unchecked().as_ptr();
        *(data_ptr.sub(8) as *mut u64) = 0u64;
        *(data_ptr.sub(16) as *mut u64) = 0u64;
        *(data_ptr.sub(24) as *mut u64) = 0u64;
        *(data_ptr.sub(32) as *mut u64) = 0u64;
        *(data_ptr.sub(40) as *mut u64) = 0u64;
        *(data_ptr.sub(48) as *mut u64) = 0u64;

        // contributor_account.close();
    }

    Ok(())
}
