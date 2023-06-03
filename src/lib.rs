#![feature(proc_macro_diagnostic)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn jni_export(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func_name = String::from("Java_");
    func_name.push_str(&*args.to_string().replace(".", "_"));

    let mut input = parse_macro_input!(input as ItemFn);

    input.attrs.push(parse_quote! {
        #[allow(non_snake_case)]
    });
    input.attrs.push(parse_quote! {
        #[no_mangle]
    });

    input.sig.abi.replace(parse_quote! {
        extern "C"
    });

    // if the name is invalid, this is a big uh-oh. don't be invalid here.
    input.sig.ident = syn::parse_str::<Ident>(&*func_name).unwrap();

    input.sig.inputs.insert(
        0,
        parse_quote! {
            _: *mut JClass
        },
    );

    input.sig.inputs.insert(
        0,
        parse_quote! {
            _: *mut JEnv
        },
    );

    input.to_token_stream().into()
}
