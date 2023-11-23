use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, AttrStyle, Data, DeriveInput, Expr, ExprLit, Fields,
    GenericParam, Generics, Index, Lit, Meta, MetaList, MetaNameValue,
};

#[proc_macro_derive(Tags, attributes(serde))]
pub fn tags_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let Data::Enum(enum_data) = input.data else {
        return quote_spanned! {ident.span()=>
            compile_error!("expected enum");
        }
        .into();
    };

    let mut tags = vec![];
    for variant in enum_data.variants {
        let Fields::Unit = variant.fields else {
            return quote_spanned! {variant.ident.span()=>
                compile_error!("expected unit variant");
            }
            .into();
        };

        let attrs = variant.attrs;

        let mut serde_rename_target = None;
        for attr in attrs {
            if !attr.path().is_ident("serde") {
                continue;
            }

            let Ok(MetaNameValue { path, value, .. }) = attr.parse_args::<MetaNameValue>() else {
                return quote_spanned! {variant.ident.span()=>
                    compile_error!("unexpected serde attribute");
                }
                .into();
            };

            if path.is_ident("rename") {
                let name = match value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) => lit.value(),
                    _ => unreachable!(),
                };

                serde_rename_target = Some(name);

                break;
            }
        }

        if serde_rename_target.is_none() {
            return quote_spanned! {variant.ident.span()=>
                compile_error!("no #[serde(rename = <target>)] attribute defined");
            }
            .into();
        }

        tags.push((variant.ident, serde_rename_target));
    }

    let variant_to_tag_entries = tags
        .iter()
        .map(|(variant, tag)| quote! { #variant => #tag });

    let tag_to_variant_entries = tags
        .iter()
        .map(|(variant, tag)| quote! { #tag => Some(#variant) })
        .chain(std::iter::once(quote! { _ => None }));

    let tags = tags.iter().map(|(_variant, tag)| quote! { #tag });

    quote! {
        impl #ident {
            pub fn tags() -> &'static [&'static str] {
                &[#(#tags),*]
            }

            pub fn tag(&self) -> &'static str {
                use #ident::*;

                match self { #(#variant_to_tag_entries),* }
            }

            pub fn by_tag(tag: &str) -> Option<Self> {
                use #ident::*;

                match tag { #(#tag_to_variant_entries),* }
            }
        }
    }
    .into()
}
