use crate::input_punctuated::MyPunctuated;
use anyhow::bail;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::VecDeque;
use std::ops::Deref;
use syn::{Type, TypeArray, TypeTuple};

fn expand_tuple(ident: Ident, type_tuple: TypeTuple, depth: i8) -> TokenStream {
    let token_streams = type_tuple
        .elems
        .iter()
        .enumerate()
        .map(|(i, t)| {
            quote! {
                split_input[#i].parse::<#t>().unwrap()
            }
        })
        .collect::<Vec<_>>();
    let ident_depth = Ident::new(&format!("{}_{}", ident, depth), ident.span());
    if depth == 1 {
        quote! {
            let mut #ident = String::new();
            ::std::io::stdin().read_line(&mut #ident).expect("failed to read.");
            let trimed_string = #ident.trim().to_string();
            let split_input = trimed_string.split(' ').collect::<Vec<_>>();
            let #ident = (#(#token_streams),*);
        }
    } else {
        quote! {
            let mut input = String::new();
            ::std::io::stdin().read_line(&mut input).expect("failed to read.");
            let trimed_string = input.trim().to_string();
            let split_input = trimed_string.split(' ').collect::<Vec<_>>();
            let #ident_depth = (#(#token_streams),*);
        }
    }
}

fn expand_array(ident: Ident, type_array: TypeArray, depth: i8) -> anyhow::Result<TokenStream> {
    let (array_element_type, array_length) = (type_array.elem, type_array.len);
    let ident_depth = Ident::new(&format!("{}_{}", ident, depth), ident.span());
    let token_stream = match array_element_type.deref() {
        Type::Array(_) | Type::Tuple(_) => {
            let token_stream =
                expand_several_type(ident.clone(), array_element_type.deref(), depth)?;
            if depth == 1 {
                quote! {
                    let mut #ident = Vec::new();
                    for _ in 0..#array_length {
                        #token_stream
                    }
                    let #ident: [#array_element_type; #array_length] = #ident.try_into().expect("");
                }
            } else {
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                        #token_stream
                    }
                    let #ident_depth: [#array_element_type; #array_length] = #ident_depth.try_into().expect("");
                    #ident.push(#ident_depth);
                }
            }
        }
        _ => {
            let input_token_stream = quote! {
                let mut input = String::new();
                ::std::io::stdin().read_line(&mut input).expect("Failed to read");
                let input = input
                    .trim()
                    .split(" ")
                    .map(|n| n.parse::<#array_element_type>().expect("Failed to cast"))
                    .collect::<Vec<_>>();
            };
            if depth == 1 {
                quote! {
                    #input_token_stream
                    let #ident: [#array_element_type; #array_length] = input.try_into().expect("Failed to cast to an array from Vec");
                }
            } else {
                quote! {
                    #input_token_stream
                    let input: [#array_element_type; #array_length] = input.try_into().expect("Failed to cast to an array from Vec");
                    #ident.push(input);
                }
            }
        }
    };
    Ok(token_stream)
}

fn expand_several_type(ident: Ident, ty: &Type, depth: i8) -> anyhow::Result<TokenStream> {
    if depth >= 2 {
        bail!("Array's maximum depth is 2.")
    }

    let token_stream = match ty {
        Type::Array(type_array) => expand_array(ident, type_array.clone(), depth + 1)?,
        Type::Tuple(type_tuple) => expand_tuple(ident, type_tuple.clone(), depth + 1),
        _ => quote! {
            let mut #ident = String::new();
            ::std::io::stdin().read_line(&mut #ident).expect("failed to read.");
            let #ident = #ident.trim().to_string().parse::<#ty>().unwrap();
        },
    };
    Ok(token_stream)
}

pub fn expand_input(input: MyPunctuated) -> anyhow::Result<TokenStream> {
    let fields = input.deref();
    let fields = fields
        .iter()
        .map(|field| (field.name(), field.ty()))
        .collect::<Vec<_>>();
    let token_streams = fields
        .into_iter()
        .map(|(ident, ty)| expand_several_type(ident.clone(), ty, 0))
        .collect::<Vec<_>>();

    // Vec<Option<TokenStream>> => Vec<TokenStream> に変換する
    let mut result = VecDeque::new();
    for token_stream_result in token_streams {
        if let Ok(token_stream) = token_stream_result {
            result.push_front(token_stream);
        } else {
            bail!("token_streams has at least 1 Err.");
        }
    }
    let result = result.into_iter().collect::<Vec<_>>();
    Ok(quote! {
        #(#result)*
    })
}
