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
    if depth == 0 {
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

fn expand_array(
    ident: Ident,
    type_array: TypeArray,
    depth: i8,
    is_parent_iter: bool,
) -> anyhow::Result<TokenStream> {
    if depth >= 3 {
        bail!("Array's maximum depth is 2.")
    }

    let array_element_type = type_array.elem;
    let array_length = type_array.len;
    let child_depth = depth + 1;
    let ident_depth = Ident::new(&format!("{}_{}", ident, depth), ident.span());
    let ident_child_depth = Ident::new(&format!("{}_{}", ident, child_depth), ident.span());

    let token_stream = match array_element_type.deref() {
        Type::Array(type_array) => {
            let type_array = type_array.clone();
            let (child_array_element_type, child_array_length) = (type_array.elem, type_array.len);
            quote! {
                let mut #ident = Vec::new();
                for _ in 0..#array_length {
                    let mut input = String::new();
                    ::std::io::stdin().read_line(&mut input).expect("Failed to read array.");
                    let trimed_string = input.trim().to_string();
                    let input = trimed_string
                        .split(' ')
                        .map(|n| n.parse::<#child_array_element_type>().expect("Failed to cast."))
                        .collect::<Vec<_>>();
                    let input: [#child_array_element_type; #child_array_length] =
                        input.try_into().expect("Filed to cast to an array from Vec.");
                    #ident.push(input);
                }
                let #ident: [#array_element_type; #array_length] = #ident.try_into().expect("Filed to cast to an array from Vec.");
            }
        }
        Type::Tuple(type_tuple) => {
            let tuple_token_stream = expand_tuple(ident.clone(), type_tuple.clone(), child_depth);
            quote! {
                let mut #ident = Vec::new();
                for _ in 0..#array_length {
                    #tuple_token_stream
                    #ident.push(#ident_child_depth);
                }
                let #ident: [#array_element_type; #array_length] =
                    #ident.try_into().expect("Filed to cast to an array from Vec.");
            }
        }
        _ => {
            quote! {
                let mut #ident = Vec::new();
                for _ in 0..#array_length {
                    let mut input = String::new();
                    ::std::io::stdin().read_line(&mut input).expect("failed to read array.");
                    #ident.push(input.trim().to_string().parse::<#array_element_type>().unwrap());
                }
                let #ident: [#array_element_type; #array_length] =
                    #ident.try_into().expect("Filed to cast to an array from Vec.");
            }
        }
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
        .map(|(ident, ty)| match ty {
            Type::Array(type_array) => expand_array(ident.clone(), type_array.clone(), 1, false),
            Type::Tuple(type_tuple) => Ok(expand_tuple(ident.clone(), type_tuple.clone(), 1)),
            _ => Ok(quote! {
                let mut #ident = String::new();
                ::std::io::stdin().read_line(&mut #ident).expect("failed to read.");
                let #ident = #ident.trim().to_string().parse::<#ty>().unwrap();
            }),
        })
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
