use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use solana_program::{pubkey::Pubkey, stake_history::Epoch};
use solana_sdk::account::Account;

use crate::{
    keyed::Keyed, ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountRentEpoch,
};

/// Newtype owning reference to account.data in order to work with trait
#[derive(Clone, Copy, Debug, Deref, DerefMut, AsRef, AsMut, From, Into)]
pub struct AccountDataRef<'a>(pub &'a [u8]);

pub type KeyedAccount = Keyed<Account>;

impl ReadonlyAccountLamports for Account {
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountData for Account {
    type SliceDeref<'s> = &'s [u8] where Self: 's;
    type DataDeref<'d> = AccountDataRef<'d> where Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        AccountDataRef(&self.data)
    }
}

impl ReadonlyAccountOwner for Account {
    fn owner(&self) -> &Pubkey {
        &self.owner
    }
}

impl ReadonlyAccountIsExecutable for Account {
    fn executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountRentEpoch for Account {
    fn rent_epoch(&self) -> Epoch {
        self.rent_epoch
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
