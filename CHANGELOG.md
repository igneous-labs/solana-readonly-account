# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0]

### Added

- implementations for `solana_sdk::account::AccountSharedData`

## [1.1.0]

### Changed

- `KeyedAccount` to a type alias that uses new pub generic struct `Keyed<T>` so that other crates can use `Keyed<T>`. `Keyed<T>` has blanket implementations for the other traits.

### Fixes

- Documentation for `KeyedAccount` in README

## [1.0.0] - 2023-12-11

Initial release
