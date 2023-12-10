# solana-readonly-account

Reimplementation of [ReadableAccount](https://docs.rs/solana-sdk/latest/solana_sdk/account/trait.ReadableAccount.html) to enable code reuse across off-chain clients ([solana-sdk](https://docs.rs/solana-sdk)) and on-chain programs ([solana-program](https://docs.rs/solana-program))

## Why was this crate created?

- You cannot use the original `ReadableAccount` trait from solana-sdk in on-chain programs because the solana-sdk feature flags don't work properly and it won't compile with `build-sbf`
- `Rc<RefCell<>>`s in [AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html) make it incompatible with `&[u8]` for `.data`

## Library

The 6 main account fields (key, lamports, data, owner, is_executable, rent_epoch) are split into a single getter trait each. This splitting allows for greater trait composability and flexibility.

For example, say you had a function that only requires the account's owner and this is a known static pubkey. Instead of having to fetch the full `Account` just to read its already-known owner field, or creating a dummy `Account`, you can simply define a newtype that only needs to implement `ReadonlyAccountOwner`, while still maintaining the ability to use this function with on-chain `AccountInfo`s.

Since [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html) doesn't have its pubkey field, the following `KeyedReadonlyAccount` struct is defined in `crate::sdk` for off-chain use cases:

```rust ignore
pub struct KeyedReadonlyAccount {
    pub key: Pubkey,
    pub account: Account,
}
```

### KeyedAccount trait

```rust ignore
pub trait KeyedAccount {
    /// Returns the pubkey of this account
    fn key(&self) -> &Pubkey;
}
```

**impl for:**

- [solana_program::AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html)
- `KeyedReadonlyAccount`
- blanket for references

### ReadonlyAccountLamports trait

```rust ignore
pub trait ReadonlyAccountLamports {
    /// Returns the lamports of this account
    fn lamports(&self) -> u64;
}
```

**impl for:**

- [solana_program::AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html)
- [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html)
- `KeyedReadonlyAccount`
- blanket for references

### ReadonlyAccountData trait

```rust ignore
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
```

**impl for:**

- [solana_program::AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html)
- [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html)
- `KeyedReadonlyAccount`
- blanket for references

### ReadonlyAccountOwner trait

```rust ignore
pub trait ReadonlyAccountOwner {
    /// Returns the pubkey of the program owning this account
    fn owner(&self) -> &Pubkey;
}
```

**impl for:**

- [solana_program::AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html)
- [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html)
- `KeyedReadonlyAccount`
- blanket for references

### ReadonlyAccountIsExecutable trait

```rust ignore
pub trait ReadonlyAccountIsExecutable {
    /// Returns true if this is an executable account, false otherwise
    fn executable(&self) -> bool;
}
```

**impl for:**

- [solana_program::AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html)
- [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html)
- `KeyedReadonlyAccount`
- blanket for references

### ReadonlyAccountRentEpoch trait

```rust ignore
pub trait ReadonlyAccountRentEpoch {
    /// Returns the rent epoch of this account
    fn rent_epoch(&self) -> Epoch;
}
```

**impl for:**

- [solana_program::AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html)
- [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html)
- `KeyedReadonlyAccount`
- blanket for references

## Usage

Importing the respective traits from the crate now enables you to write generic functions that work both on-chain and off-chain

```rust ignore
use solana_program::{
    program_error::ProgramError, program_pack::Pack,
};
use solana_readonly_account::ReadonlyAccountData;
use spl_token_2022::state::Account;

pub fn try_deserialize_token_account<A: ReadonlyAccountData>(
    acc: A,
) -> Result<Account, ProgramError> {
    Account::unpack(&acc.data())
}
```

By default, this crate only has the traits implemented for `AccountInfo` and is only usable in an on-chain context. To use it in an off-chain context, enable the `solana-sdk` feature, which will implement them for `Account`

## Testing

`cargo test --all-features`
