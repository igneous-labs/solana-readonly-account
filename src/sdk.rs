#![cfg(feature = "solana-sdk")]
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use solana_program::{pubkey::Pubkey, stake_history::Epoch};
use solana_sdk::account::Account;

use crate::ReadonlyAccount;

/// Newtype owning reference to account.data in order to work with trait
#[derive(Clone, Copy, Debug, Deref, DerefMut, AsRef, AsMut, From, Into)]
pub struct AccountDataRef<'a>(pub &'a [u8]);

impl ReadonlyAccount for Account {
    type SliceDeref<'d> = &'d [u8] where Self: 'd;
    type DataDeref<'a> = AccountDataRef<'a> where Self: 'a;

    fn lamports(&self) -> u64 {
        self.lamports
    }

    fn data(&self) -> Self::DataDeref<'_> {
        AccountDataRef(&self.data)
    }

    fn owner(&self) -> &Pubkey {
        &self.owner
    }

    fn executable(&self) -> bool {
        self.executable
    }

    fn rent_epoch(&self) -> Epoch {
        self.rent_epoch
    }
}
