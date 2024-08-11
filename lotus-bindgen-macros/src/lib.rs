use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn lotus_bindgen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(input) = syn::parse::<syn::ForeignItemFn>(item.clone()) {
        foreign_item_fn(input)
    } else if let Ok(input) = syn::parse::<syn::ItemFn>(item) {
        item_fn(input)
    } else {
        panic!("lotus_bindgen can only be used on functions");
    }
}

fn foreign_item_fn(input: syn::ForeignItemFn) -> proc_macro::TokenStream {
    let syn::ForeignItemFn {
        attrs, vis, sig, ..
    } = input;

    // let x = sig.inputs.iter().next().unwrap().clone();
    // let x = match x {
    //     syn::FnArg::Typed(x) => x,
    //     _ => panic!("Expected a typed argument"),
    // };

    // let x = x.ty;
    // x.

    let output = quote!(
        #(#attrs)*
        #vis #sig;
    );

    proc_macro::TokenStream::from(output)
}

fn item_fn(input: syn::ItemFn) -> proc_macro::TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
        ..
    } = input;

    let output = quote!(
        #(#attrs)*
        #vis #sig #block
    );

    proc_macro::TokenStream::from(output)
}
