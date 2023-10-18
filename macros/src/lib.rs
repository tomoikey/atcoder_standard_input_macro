mod input;
mod input_panctuated;
mod name_and_type;

use crate::input::expand_input;
use input_panctuated::MyPunctuated;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn include_input(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MyPunctuated);
    expand_input(input).into()
}
