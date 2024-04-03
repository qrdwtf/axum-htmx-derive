#![doc = include_str!("../README.md")]

use axum_htmx_derive_core::boosted_by;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn hx_boosted_by(args: TokenStream, input: TokenStream) -> TokenStream {
    boosted_by(args.into(), input.into()).into()
}
