#![cfg_attr(target_os="solana",feature(asm_experimental_arch, asm_const))]

#[cfg(test)]
mod tests;

#[inline(always)]
pub fn based_close(data_ptr: *mut u8) {
    #[cfg(target_os = "solana")]
    unsafe {
        let var = 0u64;
        core::arch::asm!(
            "stxdw [{0}-8], {1}", // data len
            "stxdw [{0}-16], {1}", // lamports
            "stxdw [{0}-24], {1}", // owner[24..32]
            "stxdw [{0}-32], {1}", // owner[16..24]
            "stxdw [{0}-40], {1}", // owner[8..16]
            "stxdw [{0}-48], {1}", // owner[0..8]
            in(reg) data_ptr,
            in(reg) var,
            options(nostack, preserves_flags)
        );
    }
}

mod instructions;
mod state;
use instructions::FundraiserInstruction;
use instructions::{
    checker::checker, contribute::contribute, initialize::initialize, refund::refund,
};

use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::ProgramResult;

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

pub const ID: [u8; 32] =
    five8_const::decode_32_const("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(discriminator)? {
        FundraiserInstruction::Initialize => initialize(accounts, data),
        FundraiserInstruction::Contribute => contribute(accounts, data),
        FundraiserInstruction::Checker => checker(accounts, [data[0]]),
        FundraiserInstruction::Refund => refund(accounts, [data[0]]),
    }
}