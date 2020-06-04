extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(hello_macro)]
pub fn hello_macro(ts: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(ts).unwrap();
    let name = ast.ident;
    let (im, ty, wh) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #im HelloMacro for #name #ty #wh {
            fn hello(&self) -> () {
                println!("Hello, {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}