#![feature(proc_macro_diagnostic)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{parse_quote, Item, ItemFn, ItemMod};

#[proc_macro_attribute]
pub fn jni_export(args: TokenStream, input: TokenStream) -> TokenStream {
    jni_export_internal(args, input)
}

fn jni_export_internal(args: TokenStream, input: TokenStream) -> TokenStream {
    match syn::parse::<ItemFn>(input.clone()) {
        Ok(mut function) => {
            let mut func_name = String::from("Java_");
            func_name.push_str(&*args.to_string().replace(".", "_"));

            function.attrs.push(parse_quote! {
                #[allow(non_snake_case)]
            });
            function.attrs.push(parse_quote! {
                #[no_mangle]
            });

            function.sig.abi.replace(parse_quote! {
                extern "C"
            });

            function.sig.inputs.insert(
                0,
                parse_quote! {
                    _: *mut JClass
                },
            );

            function.sig.inputs.insert(
                0,
                parse_quote! {
                    _: *mut JEnv
                },
            );

            // if the name is invalid, this is a big uh-oh. don't be invalid here.
            function.sig.ident = syn::parse_str::<Ident>(&*func_name).unwrap();

            function.to_token_stream().into()
        }
        Err(_) => match syn::parse::<ItemMod>(input.clone()) {
            Ok(mut module) => {
                module.attrs.push(parse_quote! {
                    #[allow(non_snake_case)]
                });
                module.attrs.push(parse_quote! {
                    #[no_mangle]
                });

                for item in &mut (module.content.unwrap().1) {
                    if let Item::Fn(function) = item {
                        let mut fn_args = args.clone();
                        (*function).sig.ident.to_tokens(&mut fn_args.into());

                        *function = syn::parse::<ItemFn>(jni_export_internal(
                            fn_args,
                            function.into_token_stream().into(),
                        ))
                        .unwrap();
                    }
                }

                module.to_token_stream().into()
            }
            Err(err) => TokenStream::from(err.to_compile_error()),
        },
    }
}
