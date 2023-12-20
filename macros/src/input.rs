use crate::input_punctuated::MyPunctuated;
use anyhow::bail;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Type, TypeArray, TypeTuple};

/// タプルとして展開する
fn expand_tuple(ident: &Ident, type_tuple: TypeTuple, depth: i8) -> TokenStream {
    let token_streams = type_tuple
        .elems
        .iter()
        .enumerate()
        .map(|(i, ty)| {
            quote! {
                split_input[#i].parse::<#ty>().unwrap()
            }
        })
        .collect::<Vec<_>>();
    let token_stream = quote! {
        let mut input = String::new();
        ::std::io::stdin().read_line(&mut input).expect("failed to read");
        let trimed_string = input.trim().to_string();
        let split_input = trimed_string.split(' ').collect::<Vec<_>>();
    };
    if depth == 0 {
        quote! {
            #token_stream
            let #ident = (#(#token_streams),*);
        }
    } else {
        quote! {
            #token_stream
            #ident.push((#(#token_streams),*));
        }
    }
}

/// 配列として展開する
fn expand_array(ident: &Ident, type_array: TypeArray, depth: i8) -> anyhow::Result<TokenStream> {
    let (array_element_type, array_length) = (type_array.elem, type_array.len);
    match *array_element_type {
        Type::Array(_) | Type::Tuple(_) => {
            let token_stream = expand_several_type(ident, &array_element_type, depth + 1)?;
            Ok(quote! {
                let mut #ident = Vec::new();
                for _ in 0..#array_length {
                    #token_stream
                }
                let #ident: [#array_element_type; #array_length] = #ident.try_into().expect("Failed to cast to an array from Vec");
            })
        }
        Type::Path(_) => {
            let token_stream = quote! {
                .trim()
                .split(" ")
                .map(|n| n.parse::<#array_element_type>().expect("Failed to cast"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Failed to cast to an array from Vec");
            };
            let result = if depth == 0 {
                quote! {
                    let mut #ident = String::new();
                    ::std::io::stdin().read_line(&mut #ident).expect("Failed to read");
                    let #ident: [#array_element_type; #array_length] = #ident #token_stream
                }
            } else {
                quote! {
                    let mut input = String::new();
                    ::std::io::stdin().read_line(&mut input).expect("Failed to read");
                    let input: [#array_element_type; #array_length] = input #token_stream
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

fn expand_several_type(ident: &Ident, ty: &Type, depth: i8) -> anyhow::Result<TokenStream> {
    // サポートするのは2次元までにする
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
        .map(|(ident, ty)| expand_several_type(ident, ty, 0))
        .collect::<Vec<_>>();
    // Vec<Option<TokenStream>> => Vec<TokenStream> に変換する
    let mut result = Vec::new();
    for token_stream_result in token_streams {
        result.push(token_stream_result?);
    }
    Ok(quote! {
        #(#result)*
    })
}
