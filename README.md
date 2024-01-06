# nagme
[![Crates.io](https://img.shields.io/crates/v/nagme.svg)](https://crates.io/crates/nagme)
[![Crates.io](https://img.shields.io/crates/d/nagme.svg)](https://crates.io/crates/nagme)
[![Documentation](https://img.shields.io/docsrs/nagme?logo=docs.rs)](https://docs.rs/nagme)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

super pedantic, highly maintainable, linting

Relieve yourself from the mental fatigue of deciding which lints to enabled for each and every crate.

Just use `nagme`. All crates will have a curated, uniform set of very pedantic lints.

# Note: `nagme` uses unstable features
`nagme` requires nightly to build
(see [rust-lang](https://github.com/rust-lang/rust)
[issue #54726](https://github.com/rust-lang/rust/issues/54726))

## Usage
1. Add the `nagme` attribute macro to your crate:
   ```rust
   #![nagme::nagme]
   ```

Until this crate can exist on stable rust, these additional macros will be required:
   ```rust
   #![feature(custom_inner_attributes)]
   #![feature(prelude_import)]
   ```

## Documentation
For more information, refer to:
- [docs.rs](https://docs.rs/nagme)

## Contributing
Before doing anything else: **open an issue**.
