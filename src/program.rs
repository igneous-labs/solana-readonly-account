use core::cell::Ref;

use solana_program::{account_info::AccountInfo, pubkey::Pubkey, stake_history::Epoch};

use crate::ReadonlyAccount;

impl ReadonlyAccount for AccountInfo<'_> {
    type SliceDeref<'d> = &'d mut [u8] where Self: 'd;
    type DataDeref<'a> = Ref<'a, Self::SliceDeref<'a>> where Self: 'a;

    fn lamports(&self) -> u64 {
        self.lamports()
    }

    /// panics if data is mutably borrowed
    fn data(&self) -> Self::DataDeref<'_> {
        self.data.borrow()
    }

    fn owner(&self) -> &Pubkey {
        self.owner
    }

    fn executable(&self) -> bool {
        self.executable
    }

    fn rent_epoch(&self) -> Epoch {
        self.rent_epoch
    }
}
