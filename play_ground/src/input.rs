use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Token, Type, TypeArray, TypeTuple};

pub struct MyPunctuated {
    value: Punctuated<NameAndType, Token![,]>,
}

impl Parse for MyPunctuated {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = Punctuated::parse_terminated(input)?;
        Ok(MyPunctuated { value })
    }
}

#[warn(dead_code)]
pub struct NameAndType {
    name: Ident,
    _colon: Token![:],
    ty: Type,
}

impl Parse for NameAndType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon = input.parse()?;
        let ty = input.parse()?;
        Ok(NameAndType { name, _colon, ty })
    }
}

fn expand_tuple(ident: Ident, type_tuple: TypeTuple) -> TokenStream {
    let token_streams = type_tuple
        .elems
        .iter()
        .enumerate()
        .map(|(i, t)| {
            quote! {
                values[#i].parse::<#t>().unwrap()
            }
        })
        .collect::<Vec<_>>();
    quote! {
        let mut #ident = String::new();
        ::std::io::stdin().read_line(&mut #ident).expect("failed to read.");
        let trim_string = #ident.trim().to_string();
        let values = trim_string.split(' ').collect::<Vec<_>>();
        let #ident = (#(#token_streams),*);
    }
}

fn expand_array(ident: Ident, type_array: TypeArray) -> TokenStream {
    let array_element_type = type_array.elem;
    let array_length = type_array.len;
    quote! {
        let mut #ident = Vec::new();
        for _ in 0..#array_length {
            let mut input = String::new();
            ::std::io::stdin().read_line(&mut input).expect("failed to read array.");
            #ident.push(input.trim().to_string().parse::<#array_element_type>().unwrap());
        }
        let #ident = #ident;
    }
}

pub fn expand_input(input: MyPunctuated) -> TokenStream {
    let fields = input.value;
    let fields = fields
        .iter()
        .map(|field| (field.name.clone(), field.ty.clone()))
        .collect::<Vec<_>>();
    let token_streams = fields
        .into_iter()
        .map(|(ident, ty)| match ty {
            Type::Array(type_array) => expand_array(ident, type_array),
            Type::Tuple(type_tuple) => expand_tuple(ident, type_tuple),
            _ => {
                quote! {
                    let mut #ident = String::new();
                    ::std::io::stdin().read_line(&mut #ident).expect("failed to read.");
                    let #ident = #ident.trim().to_string().parse::<#ty>().unwrap();
                }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #(#token_streams)*
    }
}
