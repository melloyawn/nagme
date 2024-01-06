//! super pedantic linting with a good developer experience
//!
//! Easily enable a currated set of lints.
//! Relieve yourself from the burden of maintaining lints across all your crates.
//!
//! # Note: `nagme` uses unstable features
//! `nagme` requires nightly to build
//! (see [rust-lang](https://github.com/rust-lang/rust)
//! [issue #54726](https://github.com/rust-lang/rust/issues/54726))
//!
//! # Examples
//! ```
//! # #![feature(custom_inner_attributes)]
//! # #![feature(prelude_import)]
//! #![nagme::nagme]
//!
//! fn main() {
//!     assert!(true);
//! }
//! ```

#![no_std]
#![feature(proc_macro_quote)]

use proc_macro::{quote, TokenStream};

#[proc_macro_attribute]
pub fn nagme(_attr: TokenStream, items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut lints = quote! {
        #![warn(
            clippy::all,
            clippy::cargo,
            clippy::complexity,
            clippy::nursery,
            clippy::pedantic,
            clippy::perf,
            clippy::style,
            clippy::unwrap_used,
        )]
        #![deny(clippy::suspicious)]
        #![forbid(clippy::correctness)]

        #![warn(
            missing_docs,
            unreachable_pub,
            unused_crate_dependencies,
        )]
    };
    lints.extend(items);
    lints
}
