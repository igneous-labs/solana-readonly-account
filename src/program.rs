use core::cell::Ref;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey, stake_history::Epoch};

use crate::{KeyedAccount, ReadonlyAccount};

impl ReadonlyAccount for AccountInfo<'_> {
    type SliceDeref<'s> = &'s mut [u8] where Self: 's;
    type DataDeref<'d> = Ref<'d, Self::SliceDeref<'d>> where Self: 'd;

    fn lamports(&self) -> u64 {
        self.lamports()
    }

    /// panics if data is mutably borrowed
    ///
    /// Take note of lifetime of returned Ref;
    /// data cannot be borrow_mut() while it's not dropped
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

impl KeyedAccount for AccountInfo<'_> {
    fn key(&self) -> &Pubkey {
        self.key
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use solana_program::program_pack::Pack;
    use spl_token_2022::state::Account;

    use crate::test_utils::{gen_test_token_acc, try_deserialize_token_account};

    use super::*;

    #[test]
    fn test_token_acc_serde_roundtrip() {
        let acc = gen_test_token_acc();

        let mut data = [0u8; Account::LEN];
        Account::pack(acc, &mut data).unwrap();
        let mut lamports = 0;
        let key = Pubkey::default();
        let owner = Pubkey::default();

        let info = AccountInfo {
            key: &key,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            owner: &owner,
            data: Rc::new(RefCell::new(&mut data)),
            rent_epoch: 0,
            is_signer: false,
            is_writable: false,
            executable: false,
        };

        // blanket impl for ref
        let ref_deser = try_deserialize_token_account(&info).unwrap();
        assert_eq!(ref_deser, acc);

        // consume info
        let deser = try_deserialize_token_account(info).unwrap();
        assert_eq!(deser, acc);
    }
}
