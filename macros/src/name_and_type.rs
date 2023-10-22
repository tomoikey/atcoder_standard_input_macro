use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::{Token, Type};

pub struct NameAndType {
    name: Ident,
    _colon: Token![:],
    ty: Type,
}

impl NameAndType {
    pub fn new(name: Ident, ty: Type) -> Self {
        NameAndType {
            name,
            _colon: Colon::default(),
            ty,
        }
    }
    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Parse for NameAndType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon = input.parse()?;
        let ty = input.parse()?;
        Ok(NameAndType { name, _colon, ty })
    }
}
