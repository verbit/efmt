use crate::format::Format;
use crate::items::tokens::AtomToken;
use crate::parse::{self, TokenStream, Parse};
use crate::span::Span;

macro_rules! impl_parse {
    ($name:ident, $value:expr) => {
        impl Parse for $name {
            fn parse(ts: &mut TokenStream) -> parse::Result<Self> {
                let token: AtomToken = ts.parse()?;
                if token.value() == $value {
                    Ok(Self(token))
                } else {
                    Err(parse::Error::unexpected_token(ts, token.into()))
                }
            }
        }
    };
}

#[derive(Debug, Clone, Span, Format)]
pub struct DefineAtom(AtomToken);
impl_parse!(DefineAtom, "define");

#[derive(Debug, Clone, Span, Format)]
pub struct IncludeAtom(AtomToken);
impl_parse!(IncludeAtom, "include");

#[derive(Debug, Clone, Span, Format)]
pub struct IncludeLibAtom(AtomToken);
impl_parse!(IncludeLibAtom, "include_lib");

#[derive(Debug, Clone, Span, Format)]
pub struct SpecAtom(AtomToken);
impl_parse!(SpecAtom, "spec");

#[derive(Debug, Clone, Span, Format)]
pub struct TypeAtom(AtomToken);
impl_parse!(TypeAtom, "type");

#[derive(Debug, Clone, Span, Format)]
pub struct OpaqueAtom(AtomToken);
impl_parse!(OpaqueAtom, "opaque");

#[derive(Debug, Clone, Span, Format)]
pub struct CallbackAtom(AtomToken);
impl_parse!(CallbackAtom, "callback");

#[derive(Debug, Clone, Span, Format)]
pub struct RecordAtom(AtomToken);
impl_parse!(RecordAtom, "record");
