mod input;
mod input_punctuated;
mod name_and_type;

use crate::input::expand_input;
use input_punctuated::MyPunctuated;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn include_input(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MyPunctuated);
    expand_input(input)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e.to_string()))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
