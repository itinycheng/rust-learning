use proc_macro2::Span;
use quote::quote;
use syn::{AttributeArgs, ItemFn, MetaNameValue, NestedMeta};

pub(crate) fn parse(attrs: AttributeArgs, func: ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let fn_vis = &func.vis;
    let fn_body = &func.block;
    let fn_sig = &func.sig;
    let fn_name = &fn_sig.ident;
    let fn_generics = &fn_sig.generics;
    let fn_inputs = &fn_sig.inputs;
    let fn_output = &fn_sig.output;
    let attrs_parsed = parse_attrs(attrs)?
        .into_iter()
        .map(|tuple| {
            format!(
                "{} = {}",
                tuple.0.segments[0].ident.to_string(),
                tuple.1.value()
            )
        })
        .collect::<Vec<String>>();
    let gen = quote! {
        #fn_vis fn #fn_name #fn_generics (#fn_inputs) #fn_output{
            #(println!("{:?} ", #attrs_parsed);)*
            #fn_body
        }
    };
    Ok(gen)
}

fn parse_attrs(attrs: AttributeArgs) -> syn::Result<Vec<(syn::Path, syn::LitStr)>> {
    if attrs.is_empty() {
        Err(syn::Error::new(Span::call_site(), "empty"))
    } else {
        let mut vec = vec![];
        for attr in attrs {
            match attr {
                NestedMeta::Meta(syn::Meta::NameValue(MetaNameValue {
                    path,
                    lit: syn::Lit::Str(str),
                    ..
                })) => {
                    vec.push((path, str));
                }
                _ => panic!("Illegal attrs found in #[parse_attrs(..)]"),
            }
        }
        Ok(vec)
    }
}
