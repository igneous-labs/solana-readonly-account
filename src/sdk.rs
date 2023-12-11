use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use solana_program::{pubkey::Pubkey, stake_history::Epoch};
use solana_sdk::account::Account;

use crate::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountPubkey, ReadonlyAccountRentEpoch,
};

/// Newtype owning reference to account.data in order to work with trait
#[derive(Clone, Copy, Debug, Deref, DerefMut, AsRef, AsMut, From, Into)]
pub struct AccountDataRef<'a>(pub &'a [u8]);

/// `solana_sdk::account::Account` with its pubkey
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyedAccount {
    pub pubkey: Pubkey,
    pub account: Account,
}

impl ReadonlyAccountPubkey for KeyedAccount {
    fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
}

impl ReadonlyAccountLamports for Account {
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountLamports for KeyedAccount {
    fn lamports(&self) -> u64 {
        self.account.lamports
    }
}

impl ReadonlyAccountData for Account {
    type SliceDeref<'s> = &'s [u8] where Self: 's;
    type DataDeref<'d> = AccountDataRef<'d> where Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        AccountDataRef(&self.data)
    }
}

impl ReadonlyAccountData for KeyedAccount {
    type SliceDeref<'s> = <solana_sdk::account::Account as ReadonlyAccountData>::SliceDeref<'s>;
    type DataDeref<'d> = <solana_sdk::account::Account as ReadonlyAccountData>::DataDeref<'d>;

    fn data(&self) -> Self::DataDeref<'_> {
        self.account.data()
    }
}

impl ReadonlyAccountOwner for Account {
    fn owner(&self) -> &Pubkey {
        &self.owner
    }
}

impl ReadonlyAccountOwner for KeyedAccount {
    fn owner(&self) -> &Pubkey {
        self.account.owner()
    }
}

impl ReadonlyAccountIsExecutable for Account {
    fn executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountIsExecutable for KeyedAccount {
    fn executable(&self) -> bool {
        self.account.executable()
    }
}

impl ReadonlyAccountRentEpoch for Account {
    fn rent_epoch(&self) -> Epoch {
        self.rent_epoch
    }
}

impl ReadonlyAccountRentEpoch for KeyedAccount {
    fn rent_epoch(&self) -> Epoch {
        self.account.rent_epoch()
    }
}

#[cfg(test)]
mod tests {
    use solana_program::program_pack::Pack;
    use spl_token_2022::state::Account;

    use crate::test_utils::{gen_test_token_acc, try_deserialize_token_account};

    use super::*;

    #[test]
    fn test_token_acc_serde_roundtrip() {
        let acc = gen_test_token_acc();

        let mut data = vec![0u8; Account::LEN];
        Account::pack(acc, &mut data).unwrap();

        let keyed_account = KeyedAccount {
            pubkey: Pubkey::default(),
            account: solana_sdk::account::Account {
                lamports: 0,
                owner: Pubkey::default(),
                data,
                rent_epoch: 0,
                executable: false,
            },
        };

        // blanket impl for ref
        let ref_deser = try_deserialize_token_account(&keyed_account).unwrap();
        assert_eq!(ref_deser, acc);

        // consume account
        let deser = try_deserialize_token_account(keyed_account).unwrap();
        assert_eq!(deser, acc);
    }
}
