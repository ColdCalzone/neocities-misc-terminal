extern crate proc_macro;
use litrs::Literal;
use proc_macro::{TokenStream, TokenTree};
use quote::quote_spanned;
use std::hash::{DefaultHasher, Hash, Hasher};

#[proc_macro]
pub fn hash(in_stream: TokenStream) -> TokenStream {
    let mut hash = DefaultHasher::new();
    for tt in in_stream.into_iter() {
        let span = tt.span();
        if let TokenTree::Punct(p) = tt {
            if p.as_char() != ',' {
                return quote_spanned! {
                    span.into()=>
                    compile_error!("Expected \",\", found {}", p.as_char());
                }
                .into();
            }
            continue;
        }
        let lit = Literal::try_from(tt).expect("Could not parse literal");
        match lit {
            Literal::Bool(inner) => inner.value().hash(&mut hash),
            Literal::Integer(inner) => inner.value::<u128>().hash(&mut hash),
            Literal::Float(..) => {
                return quote_spanned! {
                    span.into()=>
                    compile_error!("Cannot hash floats!");
                }
                .into();
            }
            Literal::Char(inner) => inner.value().hash(&mut hash),
            Literal::String(inner) => inner.value().hash(&mut hash),
            Literal::Byte(inner) => inner.value().hash(&mut hash),
            Literal::ByteString(inner) => inner.value().hash(&mut hash),
        }
    }
    TokenStream::from(TokenTree::Literal(proc_macro::Literal::u64_suffixed(
        hash.finish(),
    )))
}
