use quote::quote;
use syn::{parse_quote, parse_str, ItemFn};

pub fn transform_fn(item_fn: &mut ItemFn, layout_fn: String, args: Vec<String>) -> ItemFn {
    // println!("input code  : {}", quote!(#item_fn));
    // println!("input syntax: {:?}", item_fn);

    // function template
    let template_fn: ItemFn = parse_quote!(
        fn index(axum_htmx::HxBoosted(boosted): axum_htmx::HxBoosted) {
            if boosted {
                result_boosted
            } else {
                layout_fn(result_with_layout, fn_args).await
            }
        }
    );

    // add HxBoosted input to item_fn
    let hx_boosted_input = template_fn.sig.inputs.first().unwrap().clone();
    item_fn.sig.inputs.push(hx_boosted_input);

    // pop the last statement and wrap it with if-else
    let modify_stmt = item_fn.block.stmts.pop().unwrap();
    let modify_stmt = quote!(#modify_stmt).to_string();
    let modify_args = args.join("");

    let new_fn_str = quote!(#template_fn)
        .to_string()
        .replace("layout_fn", layout_fn.as_str())
        .replace("result_boosted", modify_stmt.as_str())
        .replace("result_with_layout", modify_stmt.as_str())
        .replace(", fn_args", modify_args.as_str());

    let new_fn: ItemFn = parse_str(new_fn_str.as_str()).unwrap();
    let new_fn_stmt = new_fn.block.stmts.first().unwrap().clone();

    // push the new statement to item_fn
    item_fn.block.stmts.push(new_fn_stmt);

    item_fn.to_owned()
}
