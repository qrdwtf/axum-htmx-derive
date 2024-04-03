#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{parse2, parse_quote, parse_str, ItemFn};

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
    let new_item_fn = transform_fn(layout_fn, &mut source_item_fn);

    quote!(#new_item_fn)
}

fn transform_fn(layout_fn: String, item_fn: &mut ItemFn) -> ItemFn {
    // println!("input code  : {}", quote!(#item_fn));
    // println!("input syntax: {:?}", item_fn);

    // function template
    let template_fn: ItemFn = parse_quote!(
        fn index(HxBoosted(boosted): HxBoosted) {
            if boosted {
                result_boosted
            } else {
                layout_fn(result_with_layout)
            }
        }
    );

    // add HxBoosted input to item_fn
    let hx_boosted_input = template_fn.sig.inputs.first().unwrap().clone();
    item_fn.sig.inputs.push(hx_boosted_input);

    // pop the last statement and wrap it with if-else
    let modify_stmt = item_fn.block.stmts.pop().unwrap();
    let modify_stmt = quote!(#modify_stmt).to_string();
    let new_fn_str = quote!(#template_fn)
        .to_string()
        .replace("layout_fn", layout_fn.as_str())
        .replace("result_boosted", modify_stmt.as_str())
        .replace("result_with_layout", modify_stmt.as_str());

    let new_fn: ItemFn = parse_str(new_fn_str.as_str()).unwrap();
    let new_fn_stmt = new_fn.block.stmts.first().unwrap().clone();

    // push the new statement to item_fn
    item_fn.block.stmts.push(new_fn_stmt);

    item_fn.to_owned()
}
