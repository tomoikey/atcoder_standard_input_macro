use crate::input_punctuated::MyPunctuated;
use anyhow::bail;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::VecDeque;
use std::ops::Deref;
use syn::{Type, TypeArray, TypeTuple};

fn expand_tuple(ident: Ident, type_tuple: TypeTuple, depth: i8) -> TokenStream {
    let (tuple_element_token_streams, downcast_token_streams): (Vec<_>, Vec<_>) = type_tuple
        .elems
        .iter()
        .enumerate()
        .map(|(i, ty)| {
            (
                quote! {
                    Box::new(split_input[#i].parse::<#ty>().unwrap())
                },
                quote! {
                    any[#i].downcast_ref::<#ty>().unwrap().clone()
                },
            )
        })
        .unzip();
    let token_stream = quote! {
        let mut input = String::new();
        ::std::io::stdin().read_line(&mut input).expect("failed to read");
        let trimed_string = input.trim().to_string();
        let split_input = trimed_string.split(' ').collect::<Vec<_>>();
        let any: Vec<Box<dyn std::any::Any>> = vec![#(#tuple_element_token_streams),*];
    };
    if depth == 0 {
        quote! {
            #token_stream
            let #ident = (#(#downcast_token_streams),*);
        }
    } else {
        quote! {
            #token_stream
            #ident.push((#(#downcast_token_streams),*));
        }
    }
}

fn expand_array(ident: Ident, type_array: TypeArray, depth: i8) -> anyhow::Result<TokenStream> {
    let (array_element_type, array_length) = (type_array.elem, type_array.len);
    let ident_depth = Ident::new(&format!("{}_{}", ident, depth), ident.span());
    match array_element_type.deref() {
        Type::Array(_) | Type::Tuple(_) => {
            let token_stream =
                expand_several_type(ident.clone(), array_element_type.deref(), depth + 1)?;
            let result = if depth == 0 {
                quote! {
                    let mut #ident = Vec::new();
                    for _ in 0..#array_length {
                        #token_stream
                    }
                    let #ident: [#array_element_type; #array_length] = #ident.try_into().expect("Failed to cast to array from Vec");
                }
            } else {
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                        #token_stream
                    }
                    let #ident_depth: [#array_element_type; #array_length] = #ident_depth.try_into().expect("Failed to cast to array from Vec");
                    #ident.push(#ident_depth);
                }
            };
            Ok(result)
        }
        Type::Path(_) => {
            let input_token_stream = quote! {
                let mut input = String::new();
                ::std::io::stdin().read_line(&mut input).expect("Failed to read");
                let input = input
                    .trim()
                    .split(" ")
                    .map(|n| n.parse::<#array_element_type>().expect("Failed to cast"))
                    .collect::<Vec<_>>();
            };
            let result = if depth == 0 {
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
            };
            Ok(result)
        }
        _ => {
            bail!("Unsupported type")
        }
    }
}

fn expand_several_type(ident: Ident, ty: &Type, depth: i8) -> anyhow::Result<TokenStream> {
    if depth >= 2 {
        bail!("Array's maximum depth reached")
    }
    match ty {
        Type::Array(type_array) => expand_array(ident, type_array.clone(), depth),
        Type::Tuple(type_tuple) => Ok(expand_tuple(ident, type_tuple.clone(), depth)),
        Type::Path(_) => Ok(quote! {
            let mut #ident = String::new();
            ::std::io::stdin().read_line(&mut #ident).expect("failed to read");
            let #ident = #ident.trim().to_string().parse::<#ty>().unwrap();
        }),
        _ => bail!("Unsupported type"),
    }
}

pub fn expand_input(input: MyPunctuated) -> anyhow::Result<TokenStream> {
    let token_streams = input
        .iter()
        .map(|field| (field.name(), field.ty()))
        .rev()
        .map(|(ident, ty)| expand_several_type(ident.clone(), ty, 0))
        .collect::<Vec<_>>();

    // Vec<Option<TokenStream>> => Vec<TokenStream> に変換する
    let mut result = VecDeque::new();
    for token_stream_result in token_streams {
        result.push_front(token_stream_result?);
    }
    let result = result.into_iter().collect::<Vec<_>>();
    Ok(quote! {
        #(#result)*
    })
}
