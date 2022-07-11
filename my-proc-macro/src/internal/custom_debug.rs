use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::FieldsNamed;
use syn::Lit;
use syn::NestedMeta;
use syn::{Attribute, DeriveInput, Meta, MetaNameValue};

pub(crate) fn parse(ast: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast.ident;
    let (im, ty, wh) = &ast.generics.split_for_impl();

    // get fields list
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        return Err(syn::Error::new(
            Span::call_site(),
            "get struct's fields failed",
        ));
    };

    // parse fields
    let debug_fields = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let attrs = &field.attrs;
            let (rename, format) = attrs
                .into_iter()
                .map(|attr| parse_attr(attr))
                .filter(|(rename, ..)| rename.len() > 0)
                .next()
                .unwrap_or_else(|| (field_name.as_ref().unwrap().to_string(), "{}".to_string()));

            quote! {
                .field(#rename, &format_args!(#format, &self.#field_name))
            }
        })
        .collect::<Vec<TokenStream>>();

    let gen = quote! {
        impl #im std::fmt::Debug for #struct_name #ty #wh {
             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                 f.debug_struct(stringify!(#struct_name))
                  #(#debug_fields)*
                  .finish()
             }
        }
    };
    Ok(gen.into())
}

// debug(rename="r_name", format="{:?}")
fn parse_attr(attr: &Attribute) -> (String, String) {
    let ref meta_list = match attr.parse_meta() {
        Ok(Meta::List(meta_list)) => meta_list,
        _ => panic!("unwrap meta_list failed"),
    };
    let mut attr_value = (String::default(), String::default());
    if meta_list.path.segments[0].ident == "debug" {
        meta_list.nested.iter().for_each(|nested_meta| {
            if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(value),
                ..
            })) = nested_meta
            {
                match path.segments[0].ident.to_owned() {
                    ident if ident == "rename" => attr_value.0 = value.value(),
                    ident if ident == "format" => attr_value.1 = value.value(),
                    _ => panic!("error attr key"),
                }
            }
        });
    }
    attr_value
}
