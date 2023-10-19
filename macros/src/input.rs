use crate::input_punctuated::MyPunctuated;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
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
    match depth {
        0 => {
            quote! {
                let mut #ident = String::new();
                ::std::io::stdin().read_line(&mut #ident).expect("failed to read.");
                let trimed_string = #ident.trim().to_string();
                let split_input = trimed_string.split(' ').collect::<Vec<_>>();
                let #ident = (#(#token_streams),*);
            }
        }
        _ => {
            quote! {
                let mut input = String::new();
                ::std::io::stdin().read_line(&mut input).expect("failed to read.");
                let trimed_string = input.trim().to_string();
                let split_input = trimed_string.split(' ').collect::<Vec<_>>();
                let #ident = (#(#token_streams),*);
            }
        }
    }
}

fn expand_array(ident: Ident, type_array: TypeArray, depth: i8) -> TokenStream {
    let array_element_type = type_array.elem;
    let array_length = type_array.len;
    let child_depth = depth + 1;
    let ident_depth = Ident::new(&format!("{}_{}", ident, depth), ident.span());
    let ident_child_depth = Ident::new(&format!("{}_{}", ident, depth + 1), ident.span());

    match array_element_type.deref() {
        Type::Array(type_array) => match depth {
            0 => {
                let array_token_stream =
                    expand_array(ident.clone(), type_array.clone(), child_depth);
                quote! {
                    let mut #ident = Vec::new();
                    for _ in 0..#array_length {
                        #array_token_stream
                        #ident.push(#ident_child_depth)
                    }
                    let #ident = #ident;
                }
            }
            1 => {
                let array_token_stream =
                    expand_array(ident.clone(), type_array.clone(), child_depth);
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                        #array_token_stream
                        #ident_depth.push(#ident_child_depth);
                    }
                }
            }
            _ => {
                let array_token_stream =
                    expand_array(ident.clone(), type_array.clone(), child_depth);
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                        #array_token_stream
                        #ident_depth.push(#ident_child_depth);
                    }
                }
            }
        },
        Type::Tuple(type_tuple) => match depth {
            0 => {
                let tuple_token_stream =
                    expand_tuple(ident.clone(), type_tuple.clone(), child_depth);
                quote! {
                    let mut #ident = Vec::new();
                    for _ in 0..#array_length {
                        #tuple_token_stream
                        #ident.push(#ident #child_depth);
                    }
                }
            }
            1 => {
                let tuple_token_stream =
                    expand_tuple(ident.clone(), type_tuple.clone(), child_depth);
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                        #tuple_token_stream
                        #ident_depth.push(#ident_child_depth);
                    }
                }
            }
            _ => {
                let tuple_token_stream =
                    expand_tuple(ident.clone(), type_tuple.clone(), child_depth);
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                        #tuple_token_stream
                        #ident_depth.push(#ident_child_depth);
                    }
                }
            }
        },
        _ => match depth {
            0 => {
                quote! {
                    let mut #ident = Vec::new();
                    for _ in 0..#array_length {
                        let mut input = String::new();
                        ::std::io::stdin().read_line(&mut input).expect("failed to read array.");
                        #ident.push(input.trim().to_string().parse::<#array_element_type>().unwrap());
                    }
                    let mut #ident = #ident;
                }
            }
            1 => {
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                         let mut input = String::new();
                         ::std::io::stdin().read_line(&mut input).expect("failed to read array.");
                         #ident_depth.push(input.trim().to_string().parse::<#array_element_type>().unwrap());
                    }
                }
            }
            _ => {
                quote! {
                    let mut #ident_depth = Vec::new();
                    for _ in 0..#array_length {
                         let mut input = String::new();
                         ::std::io::stdin().read_line(&mut input).expect("failed to read array.");
                         #ident_depth.push(input.trim().to_string().parse::<#array_element_type>().unwrap());
                    }
                }
            }
        },
    }
}

pub fn expand_input(input: MyPunctuated) -> TokenStream {
    let fields = input.deref();
    let fields = fields
        .iter()
        .map(|field| (field.name(), field.ty()))
        .collect::<Vec<_>>();
    let token_streams = fields
        .into_iter()
        .map(|(ident, ty)| match ty {
            Type::Array(type_array) => expand_array(ident.clone(), type_array.clone(), 0),
            Type::Tuple(type_tuple) => expand_tuple(ident.clone(), type_tuple.clone(), 0),
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
