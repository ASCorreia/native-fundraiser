#[cfg(test)]
mod tests {
    use mollusk_svm::{program, result::Check, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };

    use crate::state::{Contributor, Fundraiser};

    #[test]
    #[ignore = "working"]
    fn initialize() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "target/deploy/fundraiser");

        mollusk.add_program(
            &spl_token::ID,
            "src/tests/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_program_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        // Accounts
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

        // Data
        let data = [
            vec![0],
            vec![bump],
            Pubkey::new_unique().to_bytes().to_vec(),
            i64::MAX.to_le_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
        ]
        .concat();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        )
        .unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(fundraiser, true),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (
                    fundraiser,
                    AccountSharedData::new(
                        mollusk
                            .sysvars
                            .rent
                            .minimum_balance(Fundraiser::LEN),
                        Fundraiser::LEN,
                        &program_id,
                    ),
                ),
                (mint, mint_account),
                (
                    vault,
                    AccountSharedData::new(
                        mollusk
                            .sysvars
                            .rent
                            .minimum_balance(spl_token::state::Account::LEN),
                        spl_token::state::Account::LEN,
                        &spl_token::ID,
                    ),
                ),
                (token_program, token_program_account),
            ],
            &[Check::success()],
        );
    }

    #[test]
    #[ignore = "working"]
    fn contribute() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "target/deploy/fundraiser");

        mollusk.add_program(
            &spl_token::ID,
            "src/tests/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_program_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        // Accounts
        let contributor = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

        // Data
        let data = [
            vec![1],
            vec![bump],
            1_000u64.to_le_bytes().to_vec(),
        ]
        .concat();

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: vault,
                amount: 0,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        ).unwrap();

        let mut contributor_ta_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: contributor,
                amount: 1_000_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            contributor_ta_account.data_as_mut_slice(),
        ).unwrap();

        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(Fundraiser::LEN),
            Fundraiser::LEN,
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                Pubkey::default().to_bytes().to_vec(),
                Pubkey::default().to_bytes().to_vec(),
                i64::MAX.to_le_bytes().to_vec(),
                1_000_000u64.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(contributor_account, false),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (contributor, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (fundraiser, fundraiser_account),
                (
                    contributor_account, 
                    AccountSharedData::new(
                        mollusk
                            .sysvars
                            .rent
                            .minimum_balance(Contributor::LEN),
                        Contributor::LEN,
                        &program_id,
                    ),
                ),
                (contributor_ta, contributor_ta_account),
                (vault, vault_account),
                (token_program, token_program_account),
            ],
            &[Check::success()],
        );
    }

    #[test]
    #[ignore = "working"]
    fn checker() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "target/deploy/fundraiser");

        mollusk.add_program(
            &spl_token::ID,
            "src/tests/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_program_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        // Accounts
        let cranker = Pubkey::new_unique();
        let maker = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);
        let maker_ta = Pubkey::new_unique();

        // Data
        let data = [
            vec![2],
            vec![bump],
        ]
        .concat();

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: vault,
                amount: 1_000_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        ).unwrap();

        let mut maker_ta_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: maker,
                amount: 0,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ta_account.data_as_mut_slice(),
        ).unwrap();

        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(Fundraiser::LEN),
            Fundraiser::LEN,
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                Pubkey::default().to_bytes().to_vec(),
                maker.to_bytes().to_vec(),
                i64::MAX.to_le_bytes().to_vec(),
                1_000_000u64.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(cranker, true),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(maker_ta, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (cranker, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (fundraiser, fundraiser_account),
                (vault, vault_account),
                (maker_ta, maker_ta_account),
                (token_program, token_program_account),
            ],
            &[Check::success()],
        );
    }

    #[test]
    fn refund() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "target/deploy/fundraiser");

        mollusk.add_program(
            &spl_token::ID,
            "src/tests/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_program_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        // Accounts
        let contributor = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

        // Data
        let data = [
            vec![3],
            vec![bump],
        ]
        .concat();

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: vault,
                amount: 1_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        ).unwrap();

        let mut contributor_ta_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: contributor,
                amount: 0,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            contributor_ta_account.data_as_mut_slice(),
        ).unwrap();

        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(Fundraiser::LEN),
            Fundraiser::LEN,
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                Pubkey::default().to_bytes().to_vec(),
                Pubkey::default().to_bytes().to_vec(),
                0i64.to_le_bytes().to_vec(),
                1_000_000u64.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        let mut contributor_account_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(Contributor::LEN),
            Contributor::LEN,
            &program_id,
        );
        contributor_account_account.set_data_from_slice(
            &[
                contributor.to_bytes().to_vec(),
                1_000u64.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(contributor_account, false),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (contributor, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (fundraiser, fundraiser_account),
                (contributor_account, contributor_account_account),
                (contributor_ta, contributor_ta_account),
                (vault, vault_account),
                (token_program, token_program_account),
            ],
            &[
                // Check::account(&contributor)
                //     .lamports(1_000_000_000 + 26)
                //     .build(),
                Check::account(&contributor_account)
                    .lamports(0)
                    .build(),
                Check::success(),
            ],
        );
    }

}