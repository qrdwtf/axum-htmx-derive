#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{parse2, ItemFn};

mod boosted_by;
mod boosted_by_async;

pub fn boosted_by(args: TokenStream, input: TokenStream) -> TokenStream {
    // get layout_fn from args
    let layout_fn = args.clone().into_iter().next().map(|x| x.to_string());
    let layout_fn = if let Some(layout_fn) = layout_fn {
        layout_fn
    } else {
        abort!(args, "boosted_by requires layout_fn as argument.");
    };

    // parse input as ItemFn
    let mut source_item_fn = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    // transform ItemFn
    let new_item_fn = boosted_by::transform_fn(layout_fn, &mut source_item_fn);

    quote!(#new_item_fn)
}

pub fn boosted_by_async(args: TokenStream, input: TokenStream) -> TokenStream {
    // get layout_fn from args
    let layout_fn = args.clone().into_iter().next().map(|x| x.to_string());
    let layout_fn = if let Some(layout_fn) = layout_fn {
        layout_fn
    } else {
        abort!(args, "boosted_by requires layout_fn as argument.");
    };

    // parse input as ItemFn
    let mut source_item_fn = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    // transform ItemFn
    let new_item_fn = boosted_by_async::transform_fn(layout_fn, &mut source_item_fn);

    quote!(#new_item_fn)
}
