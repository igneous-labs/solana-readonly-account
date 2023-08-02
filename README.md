# solana-readonly-account

Reimplementation of [ReadableAccount](https://docs.rs/solana-sdk/latest/solana_sdk/account/trait.ReadableAccount.html) to enable code reuse across off-chain clients ([solana-sdk](https://docs.rs/solana-sdk)) and on-chain programs ([solana-program](https://docs.rs/solana-program))

## Why was this crate created?

- You cannot use the original `ReadableAccount` trait from solana-sdk in on-chain programs because the solana-sdk feature flags don't work properly and it won't compile with `build-sbf`
- `Rc<RefCell<>>`s in [AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html) make it incompatible with `&[u8]` for `.data`
