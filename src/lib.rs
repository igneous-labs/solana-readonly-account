use core::ops::Deref;
use solana_program::{pubkey::Pubkey, stake_history::Epoch};

pub mod program;
pub mod sdk;

/// Until [SlicePattern](https://doc.rust-lang.org/core/slice/trait.SlicePattern.html) is merged into rust stable
/// We need to do this double Deref hack for data because
/// Rc<RefCell<&mut [u8]>>::borrow() returns Ref<&mut [u8]>
/// and there's no common trait in stable with method `.as_slice()` that both &mut [u8] and &[u8] impls
/// (that would be SlicePattern)
pub trait ReadonlyAccount {
    type SliceDeref<'s>: Deref<Target = [u8]>
    where
        Self: 's;
    type DataDeref<'d>: Deref<Target = Self::SliceDeref<'d>>
    where
        Self: 'd;

    fn lamports(&self) -> u64;
    fn data(&self) -> Self::DataDeref<'_>;
    fn owner(&self) -> &Pubkey;
    fn executable(&self) -> bool;
    fn rent_epoch(&self) -> Epoch;
}

#[cfg(test)]
pub mod test_utils {
    use solana_program::{
        program_error::ProgramError, program_option::COption, program_pack::Pack,
    };
    use spl_token_2022::state::{Account, AccountState};

    use super::*;

    pub fn try_deserialize_token_account<A: ReadonlyAccount>(
        acc: &A,
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
