use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, Meta, parse_macro_input};

#[proc_macro_derive(DebugC, attributes(prefix))]
pub fn debug_c_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut prefix = "OP_".to_string();

    for attr in &input.attrs {
        if attr.path().is_ident("prefix") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(s), ..
                }) = &meta.value
                {
                    prefix = s.value();
                }
            }
        }
    }

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("OpDebug can only be used on enums"),
    };

    let arms = variants.iter().map(|v| {
        let variant_name = &v.ident;
        let formatted_name = format!("{}{}", prefix, variant_name.to_string().to_uppercase());

        quote! {
            #name::#variant_name => write!(f, #formatted_name),
        }
    });

    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(TryFromU8)]
pub fn try_from_u8_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("TryFromU8 can only be used on enums"),
    };

    let arms = variants.iter().enumerate().map(|(index, v)| {
        let variant_name = &v.ident;
        let index = index as u8;

        quote! {
            #index => Ok(#name::#variant_name),
        }
    });

    let expanded = quote! {
        impl std::convert::TryFrom<u8> for #name {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(#arms)*
                    _ => Err(())
                }
            }
        }
    };

    TokenStream::from(expanded)
}
