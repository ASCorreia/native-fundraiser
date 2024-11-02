use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// # State
/// 
/// Contributor:
/// > Amount: u64

pub struct Contributor(*const u8);

impl Contributor {
    pub const LEN: usize = 8;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn amount(&self) -> u64 {
        unsafe { core::ptr::read_unaligned(self.0.add(64) as *const u64) }
    }
}

/// # State
/// 
/// Fundraiser:
/// > Maker: Pubkey
/// > Mint to Raise: Pubkey
/// > Time Started: i64
/// > Amount to Raise: u64
/// > Current Amount: u64
/// > Duration: u8

pub struct Fundraiser(*const u8);

impl Fundraiser {
    pub const LEN: usize = 88;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn mint_to_raise(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    pub fn time_ending(&self) -> i64 {
        unsafe { core::ptr::read_unaligned(self.0.add(64) as *const i64) }
    }

    pub fn amount_to_raise(&self) -> u64 {
        unsafe { core::ptr::read_unaligned(self.0.add(72) as *const u64) }
    }
}