//! #FIXME
//!
//! # Examples
//! ```
//! // #FIXME
//! ```

#![no_std]

use proc_macro::TokenStream;

#[proc_macro]
pub fn nagme(_item: TokenStream) -> TokenStream {
    concat!("").parse().unwrap()
}
