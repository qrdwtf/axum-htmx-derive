#![cfg(test)]

use crate::boosted_by;
use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn first() {
    let before = quote! {
        async fn index(Path(user_id): Path<u32>) -> Html<String> {
            let ctx = PageTemplate {
                locale: "en".to_string(),
            };

            Html(ctx.render_once().unwrap_or(String::new()))
        }
    };
    let expected = quote! {
        async fn index(Path(user_id): Path<u32>, axum_htmx::HxBoosted(boosted): axum_htmx::HxBoosted) -> Html<String> {
            let ctx = PageTemplate {
                locale: "en".to_string(),
            };

            if boosted {
                Html(ctx.render_once().unwrap_or(String::new()))
            } else {
                with_layout(Html(ctx.render_once().unwrap_or(String::new())))
            }
        }
    };

    let after = boosted_by(quote!(with_layout), before);

    assert_tokens_eq(&expected, &after);

    // assert_eq!(after.to_string(), must_be);
}

fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        // println!("expected: {}", &expected);
        // println!("actual  : {}", &actual);
        panic!("expected != actual");
    }
}
