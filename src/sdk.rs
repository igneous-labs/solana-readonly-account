#![cfg(feature = "solana-sdk")]

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use solana_program::{pubkey::Pubkey, stake_history::Epoch};
use solana_sdk::account::Account;

use crate::{
    KeyedAccount, ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountRentEpoch,
};

/// Newtype owning reference to account.data in order to work with trait
#[derive(Clone, Copy, Debug, Deref, DerefMut, AsRef, AsMut, From, Into)]
pub struct AccountDataRef<'a>(pub &'a [u8]);

/// `solana_sdk::account::Account` with its pubkey
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyedReadonlyAccount {
    pub key: Pubkey,
    pub account: Account,
}

impl KeyedAccount for KeyedReadonlyAccount {
    fn key(&self) -> &Pubkey {
        &self.key
    }
}

impl ReadonlyAccountLamports for Account {
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountLamports for KeyedReadonlyAccount {
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

impl ReadonlyAccountData for KeyedReadonlyAccount {
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

impl ReadonlyAccountOwner for KeyedReadonlyAccount {
    fn owner(&self) -> &Pubkey {
        self.account.owner()
    }
}

impl ReadonlyAccountIsExecutable for Account {
    fn executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountIsExecutable for KeyedReadonlyAccount {
    fn executable(&self) -> bool {
        self.account.executable()
    }
}

impl ReadonlyAccountRentEpoch for Account {
    fn rent_epoch(&self) -> Epoch {
        self.rent_epoch
    }
}

impl ReadonlyAccountRentEpoch for KeyedReadonlyAccount {
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

        let keyed_account = KeyedReadonlyAccount {
            key: Pubkey::default(),
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
