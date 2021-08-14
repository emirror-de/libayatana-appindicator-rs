# libayatana-appindicator-rs

## libayatana-appindicator-sys

[![Crates.io](https://img.shields.io/crates/v/libayatana-appindicator-sys.svg)](https://crates.io/crates/libayatana-appindicator-sys)

Raw rust bindings for use with libayatana-appindicator.

## libayatana-appindicator

[![Crates.io](https://img.shields.io/crates/v/libayatana-appindicator.svg)](https://crates.io/crates/libayatana-appindicator)

Rust bindings for libayatana-appindicator shared library.

### Dependencies

To remove unnecessary build dependencies, the crate provides a static generated binding file by default. However, if you want to generate the raw bindings on every build, add the following line to your `Cargo.toml` file:
```toml
libayatana-appindicator = { version = "0.1.5", features = ["dynamic"] }
```
When using with the dynamic feature, the [libayatana-appindicator-sys](##libayatana-appindicator-sys) crate is used as build dependency that generates the bindings file for you.

## License

LGPL-3.0

This library follows the license of [libappindicator-rs](https://github.com/qdot/libappindicator-rs), [libappindicator-sys](https://github.com/qdot/libappindicator-sys) and [libayatana-appindicator](https://github.com/AyatanaIndicators/libayatana-appindicator).

## Credits

This library is widely based on [libappindicator-rs](https://github.com/qdot/libappindicator-rs) and [libappindicator-sys](https://github.com/qdot/libappindicator-sys), so credits go to all contributors for their work on it.
