mod internal;

extern crate proc_macro;
extern crate quote;
extern crate syn;

use crate::internal::print_attrs;
use crate::internal::custom_debug;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::ItemFn;

#[proc_macro]
pub fn create_fn(ident: TokenStream) -> TokenStream {
    let fn_name = ident.to_string();
    let fn_identifier = Ident::new(fn_name.as_str(), Span::call_site());
    let gen = quote! {
        fn #fn_identifier<T: std::fmt::Debug>(t: T){
            println!("{:?}", t);
        }
    };
    gen.into()
}

#[proc_macro_derive(hello_macro)]
pub fn hello_macro(ts: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(ts).unwrap();
    let name = ast.ident;
    let (im, ty, wh) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #im HelloMacro for #name #ty #wh {
            fn hello(&self) {
                println!("Hello, {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn custom_debug(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    custom_debug::parse(ast)
        .unwrap_or_else(to_compile_error)
        .into()
}


#[proc_macro_attribute]
pub fn time_cost(_: TokenStream, body: TokenStream) -> TokenStream {
    let func = parse_macro_input!(body as ItemFn);
    let fn_vis = &func.vis;
    let fn_body = &func.block;
    let fn_sig = &func.sig;
    let fn_name = &fn_sig.ident;
    let fn_generics = &fn_sig.generics;
    let fn_inputs = &fn_sig.inputs;
    let fn_output = &fn_sig.output;

    let gen = quote! {
        #fn_vis fn #fn_name #fn_generics (#fn_inputs) #fn_output{
            use std::time::Instant;
            let start_time = Instant::now();
            #fn_body
            println!("time cost {:?}", start_time.elapsed());
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn print_attr(head: TokenStream, body: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(head as syn::AttributeArgs);
    let func = parse_macro_input!(body as ItemFn);
    print_attrs::parse(attr, func)
        .unwrap_or_else(to_compile_error)
        .into()
}

fn to_compile_error(error: syn::Error) -> proc_macro2::TokenStream {
    error.to_compile_error()
}
