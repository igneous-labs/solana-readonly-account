#![doc = include_str!("../README.md")]

use core::ops::Deref;
use solana_program::{pubkey::Pubkey, stake_history::Epoch};

pub mod program;
pub mod sdk;

/// A readonly account that you can read the pubkey of
pub trait KeyedAccount {
    /// Returns the pubkey of this account
    fn key(&self) -> &Pubkey;
}

impl<'a, T> KeyedAccount for &'a T
where
    T: KeyedAccount + ?Sized,
{
    fn key(&self) -> &Pubkey {
        (*self).key()
    }
}

/// A readonly account that you can read the lamports of
pub trait ReadonlyAccountLamports {
    /// Returns the lamports of this account
    fn lamports(&self) -> u64;
}

impl<'a, T> ReadonlyAccountLamports for &'a T
where
    T: ReadonlyAccountLamports + ?Sized,
{
    fn lamports(&self) -> u64 {
        (*self).lamports()
    }
}

/// A readonly account that you can read the data of
///
/// Until [SlicePattern](https://doc.rust-lang.org/core/slice/trait.SlicePattern.html) is merged into rust stable
/// We need to do this double Deref hack for data because
/// Rc<RefCell<&mut [u8]>>::borrow() returns Ref<&mut [u8]>
/// and there's no common trait in stable with method `.as_slice()` that both &mut [u8] and &[u8] impls
/// (that would be SlicePattern)
pub trait ReadonlyAccountData {
    type SliceDeref<'s>: Deref<Target = [u8]>
    where
        Self: 's;
    type DataDeref<'d>: Deref<Target = Self::SliceDeref<'d>>
    where
        Self: 'd;

    /// Returns the data buffer of this account that can be derefed twice into a byte-slice
    fn data(&self) -> Self::DataDeref<'_>;
}

impl<'a, T> ReadonlyAccountData for &'a T
where
    T: ReadonlyAccountData + ?Sized,
{
    type SliceDeref<'s> = T::SliceDeref<'s>
    where
        Self: 's;

    type DataDeref<'d> = T::DataDeref<'d>
    where
        Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        (*self).data()
    }
}

/// A readonly account that you can read the owner program of
pub trait ReadonlyAccountOwner {
    /// Returns the pubkey of the program owning this account
    fn owner(&self) -> &Pubkey;
}

impl<'a, T> ReadonlyAccountOwner for &'a T
where
    T: ReadonlyAccountOwner + ?Sized,
{
    fn owner(&self) -> &Pubkey {
        (*self).owner()
    }
}

/// A readonly account that you can read whether it's executable or not
pub trait ReadonlyAccountIsExecutable {
    /// Returns true if this is an executable account, false otherwise
    fn executable(&self) -> bool;
}

impl<'a, T> ReadonlyAccountIsExecutable for &'a T
where
    T: ReadonlyAccountIsExecutable + ?Sized,
{
    fn executable(&self) -> bool {
        (*self).executable()
    }
}

/// A readonly account that you can read the rent epoch of
pub trait ReadonlyAccountRentEpoch {
    /// Returns the rent epoch of this account
    fn rent_epoch(&self) -> Epoch;
}

impl<'a, T> ReadonlyAccountRentEpoch for &'a T
where
    T: ReadonlyAccountRentEpoch + ?Sized,
{
    fn rent_epoch(&self) -> Epoch {
        (*self).rent_epoch()
    }
}

#[cfg(test)]
pub mod test_utils {
    use solana_program::{
        program_error::ProgramError, program_option::COption, program_pack::Pack,
    };
    use spl_token_2022::state::{Account, AccountState};

    use super::*;

    /// This fn only uses data, but we just add the other traits to make sure
    /// we've implemented them
    pub fn try_deserialize_token_account<
        A: KeyedAccount
            + ReadonlyAccountLamports
            + ReadonlyAccountData
            + ReadonlyAccountOwner
            + ReadonlyAccountIsExecutable
            + ReadonlyAccountRentEpoch,
    >(
        acc: A,
    ) -> Result<Account, ProgramError> {
        Account::unpack(&acc.data())
    }

    pub fn gen_test_token_acc() -> Account {
        let owner = Pubkey::new_unique();
        Account {
            mint: Pubkey::new_unique(),
            owner,
            amount: 123,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(owner),
        }
    }
}
