# solana-readonly-account

Reimplementation of [ReadableAccount](https://docs.rs/solana-sdk/latest/solana_sdk/account/trait.ReadableAccount.html) to enable code reuse across off-chain clients ([solana-sdk](https://docs.rs/solana-sdk)) and on-chain programs ([solana-program](https://docs.rs/solana-program))

## Why was this crate created?

- You cannot use the original `ReadableAccount` trait from solana-sdk in on-chain programs because the solana-sdk feature flags don't work properly and it won't compile with `build-sbf`
- `Rc<RefCell<>>`s in [AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html) make it incompatible with `&[u8]` for `.data`

## Library

This crate defines the `ReadonlyAccount` trait:

```rust ignore
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
```

And implements it for [solana_program::AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html) and [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html)

## Usage

Importing the crate now enables you to write generic functions that work both on-chain and off-chain

```rust ignore
use solana_program::{
    program_error::ProgramError, program_pack::Pack,
};
use solana_readonly_account::ReadonlyAccount;
use spl_token_2022::state::Account;

pub fn try_deserialize_token_account<A: ReadonlyAccount>(
    acc: A,
) -> Result<Account, ProgramError> {
    Account::unpack(&acc.data())
}
```

By default, this crate only has the trait implemented for `AccountInfo` and is only usable in an on-chain context. To use it in an off-chain context, enable the `solana-sdk` feature, which will implement it for `Account`

## Testing

`cargo test --all-features`
