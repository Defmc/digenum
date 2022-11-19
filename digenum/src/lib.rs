extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, Item, Variant};

#[proc_macro_derive(DigEnum)]
pub fn digenum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    let num = if let Item::Enum(num) = input {
        num
    } else {
        panic!("struct isn't enum")
    };
    let name = num.ident;
    let (impl_gen, ty_gen, where_gen) = num.generics.split_for_impl();
    let defs = num.variants.iter().map(gen_fns);

    let expanded = quote! {
        impl #impl_gen #name #ty_gen #where_gen {
            #(#defs)*
        }
    };
    TokenStream::from(expanded)
}

fn gen_fns(v: &Variant) -> TokenStream2 {
    let name = &v.ident;
    let fields = &v.fields;
    let as_name = format_ident!("as_{name}");
    let as_mut_name = format_ident!("as_mut_{name}");
    let into_name = format_ident!("into_{name}");
    if let Fields::Unit = fields {
        return TokenStream2::new();
    }
    let fields = fields.iter().collect::<Vec<_>>();
    let first = fields[0];
    let entries = (0..fields.len())
        .map(|x| format_ident!("_{x}"))
        .collect::<Vec<_>>();

    let ref_fields = if fields.len() == 1 {
        quote! { &#first }
    } else {
        quote! { (#(&#fields), *) }
    };

    let mut_fields = if fields.len() == 1 {
        quote! { &mut #first }
    } else {
        quote! { (#(&mut #fields), *) }
    };

    let fields = if fields.len() == 1 {
        quote! { #first }
    } else {
        quote! { (#(#fields), *) }
    };

    quote! {
        #[allow(non_snake_case)]
        pub fn #as_name(&self) -> Option<#ref_fields> {
            match self {
                Self::#name(#(#entries), *) => Some((#(#entries), *)),
                _ => None
            }
        }

        #[allow(non_snake_case)]
        pub fn #as_mut_name(&mut self) -> Option<#mut_fields> {
            match self {
                Self::#name(#(#entries), *) => Some((#(#entries), *)),
                _ => None
            }
        }

        #[allow(non_snake_case)]
        pub fn #into_name(self) -> Option<#fields> {
            match self {
                Self::#name(#(#entries), *) => Some((#(#entries), *)),
                _ => None
            }
        }
    }
}
