use crate::format::Format;
use crate::items::expressions::Expr;
use crate::items::styles::Space;
use crate::items::symbols::{DoubleLeftArrowSymbol, LeftArrowSymbol};
use crate::parse::Parse;
use crate::span::Span;

#[derive(Debug, Clone, Span, Parse, Format)]
pub enum Qualifier {
    Generator(Generator),
    BitstringGenerator(BitstringGenerator),
    Filter(Expr),
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct Generator {
    pattern: Expr,
    arrow: Space<LeftArrowSymbol>,
    expr: Expr,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct BitstringGenerator {
    pattern: Expr,
    arrow: Space<DoubleLeftArrowSymbol>,
    expr: Expr,
}
