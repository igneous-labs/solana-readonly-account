use solana_program::pubkey::Pubkey;

use crate::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountPubkey, ReadonlyAccountRentEpoch,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Keyed<T> {
    pub pubkey: Pubkey,
    pub account: T,
}

impl<T> ReadonlyAccountPubkey for Keyed<T> {
    fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
}

impl<T: ReadonlyAccountData> ReadonlyAccountData for Keyed<T> {
    type SliceDeref<'s> = T::SliceDeref<'s>
    where
        Self: 's;

    type DataDeref<'d> = T::DataDeref<'d>
    where
        Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        self.account.data()
    }
}

impl<T: ReadonlyAccountLamports> ReadonlyAccountLamports for Keyed<T> {
    fn lamports(&self) -> u64 {
        self.account.lamports()
    }
}

impl<T: ReadonlyAccountOwner> ReadonlyAccountOwner for Keyed<T> {
    fn owner(&self) -> &Pubkey {
        self.account.owner()
    }
}

impl<T: ReadonlyAccountIsExecutable> ReadonlyAccountIsExecutable for Keyed<T> {
    fn executable(&self) -> bool {
        self.account.executable()
    }
}

impl<T: ReadonlyAccountRentEpoch> ReadonlyAccountRentEpoch for Keyed<T> {
    fn rent_epoch(&self) -> solana_program::stake_history::Epoch {
        self.account.rent_epoch()
    }
}
