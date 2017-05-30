#![feature(proc_macro)]
#![feature(plugin)]
#![plugin(error_def)]

#![warn(unused,
        missing_debug_implementations, missing_copy_implementations)]
#![deny(bad_style, future_incompatible,
        unsafe_code,
        trivial_casts, trivial_numeric_casts)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, StrLit, StrStyle};

use errors::ParseError;


/// Declares a unit of measurement
///
/// Usage: `#[unit(quantity=A, symbol="c", singular="c", [plural="cs"])]`
// TODO: Write more detailed docs with examples etc.
#[proc_macro_attribute]
pub fn unit(args: TokenStream, item: TokenStream) -> TokenStream {
    let item_tt = syn::parse_derive_input(&item.to_string()).unwrap();
    let properties = UnitProperties::parse(item_tt.ident, &args).unwrap();

    impl_unit(properties).parse().unwrap()
}

#[allow(unused)] // TODO: Remove after implementation
fn impl_unit(properties: UnitProperties) -> quote::Tokens {
    use StrStyle::Cooked;

    let name = properties.name;
    let quantity = properties.quantity;
    let symbol = StrLit {
        value: properties.symbol,
        style: Cooked
    };
    let singular = StrLit {
        value: properties.singular,
        style: Cooked
    };
    let plural = StrLit {
        value: properties.plural,
        style: Cooked
    };

    // There are no traits we could implement yet.
    // TODO: Define them, then return here.
    quote!{};
    unimplemented!()
}


#[derive(Debug)]
struct UnitProperties {
    pub name: Ident,
    pub quantity: Ident,
    pub symbol: String,
    pub singular: String,
    pub plural: String,
}

impl UnitProperties {
    fn parse(name: Ident, stream: &TokenStream) -> Result<Self, ParseError> {
        use ParseError::*;

        let tokens = stream.to_string();
        let mut quantity = Err(ParamMissing { param: "quantity".into() });
        let mut symbol = Err(ParamMissing { param: "symbol".into() });
        let mut singular = Err(ParamMissing { param: "singular".into() });
        let mut plural = None;

        // Get rid of parentheses and surrounding whitespace
        let haystack = &tokens[2 .. tokens.len() - 2];
        let params = haystack
            .split(" , ")
            .map(|s| {
                let mut inner = s.split(" = ");
                let left = inner.next().unwrap();
                let right = inner.next()
                    // Get rid of quotes that may still be present
                    .map(|s| s.trim_matches('"').to_string())
                    .ok_or_else(|| ExpectedStringIdent {
                        after: "=".into()
                    })?;

                if let Some(token) = inner.next() {
                    return Err(UnexpectedToken { token: token.into() })
                }

                Ok((left, right))
            }).collect::<Result<Vec<_>, ParseError>>()?;

        for (left, right) in params {
            match left {
                "quantity" => quantity = Ok(right.into()),
                "symbol" => symbol = Ok(right),
                "singular" => singular = Ok(right),
                "plural" => plural = Some(right),
                other => return Err(UnexpectedParam { param: other.into() })
            }
        }

        let singular = singular?;

        let plural = if let Some(pl) = plural {
            pl
        } else {
            Self::infer_plural(&singular)
        };

        Ok(Self {
            name: name,
            quantity: quantity?,
            symbol: symbol?,
            singular: singular,
            plural: plural
        })
    }

    fn infer_plural(singular: &str) -> String {
        let mut result = String::with_capacity(singular.len() + 1);
        result.push_str(singular);
        result.push('s');
        result
    }
}


mod errors {
    error_def! ParseError {
        ExpectedStringIdent { after: String }
            => "Syntax error: expected string or ident after" ("{}", after),
        UnexpectedToken { token: String }
            => "Syntax error: unexpected token" ("{}", token),
        UnexpectedParam { param: String }
            => "Unexpected parameter" ("{}", param),
        ParamMissing { param: String }
            => "Not all required parameters given" ("{} missing", param)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
