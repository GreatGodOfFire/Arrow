use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, GenericParam, Ident, ItemStruct, Lifetime, LifetimeDef, Token,
    Type,
};

trait Packet {}

#[proc_macro_derive(Packet)]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = input.ident.clone();
    let name_str = name.to_string();

    let generics = input.generics.clone();
    let mut impl_generics = generics.clone();
    let lifetime = GenericParam::Lifetime(LifetimeDef::new(Lifetime::new("'d", Span::call_site())));

    if impl_generics.params.len() > 0 {
        impl_generics.params.last_mut();
        impl_generics
            .params
            .push_punct(Token!(,)(Span::call_site()));
    }

    impl_generics.params.push(lifetime.clone());

    let code = deserialize_fields(&input);

    // let lifetime = Lifetime::new("'a", Span::call_site());
    // let lifetime_def = LifetimeDef::new(lifetime);
    // GenericParam::Lifetime(lifetime_def)
    let where_clause = generics.where_clause.clone();

    let visitor = format_ident!("{}Visitor", name);

    let code: TokenStream = quote! {
        impl #impl_generics serde::Deserialize<#lifetime> for #name #generics #where_clause {
            fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<#lifetime>, {
                d.deserialize_seq(#visitor)
            }
        }

        struct #visitor;

        impl<'v> serde::de::Visitor<'v> for #visitor {
            type Value = #name;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(#name_str)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'v>, {
                    #code
            }
        }
    }
    .into();
    code
}

fn deserialize_fields(input: &ItemStruct) -> TokenStream2 {
    let fields = input.fields.clone();

    let struct_name = input.ident.clone();

    let types: Vec<Type> = fields.iter().map(|f| f.ty.clone()).collect();
    let names: Vec<Ident> = fields.iter().map(|f| f.ident.clone().unwrap()).collect();

    quote! {
        #(let #names = seq.next_element::<#types>()?.unwrap();)*
        Ok(#struct_name {#(#names),*})
    }
}