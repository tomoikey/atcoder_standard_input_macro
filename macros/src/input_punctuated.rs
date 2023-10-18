use crate::name_and_type::NameAndType;
use std::ops::Deref;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

pub struct MyPunctuated(Punctuated<NameAndType, Token![,]>);

impl Deref for MyPunctuated {
    type Target = Punctuated<NameAndType, Token![,]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for MyPunctuated {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MyPunctuated(Punctuated::parse_terminated(input)?))
    }
}
