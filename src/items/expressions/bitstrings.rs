use crate::format2::{Format2, Formatter2, Indent, Newline};
use crate::items::expressions::{BaseExpr, Expr, Qualifier};
use crate::items::generics::{BinaryOpLike, BinaryOpStyle, BitstringLike, Maybe, NonEmptyItems};
use crate::items::symbols::{
    ColonSymbol, DoubleLeftAngleSymbol, DoubleRightAngleSymbol, DoubleVerticalBarSymbol,
    HyphenSymbol, SlashSymbol,
};
use crate::items::tokens::{AtomToken, IntegerToken};
use crate::parse::Parse;
use crate::span::Span;

#[derive(Debug, Clone, Span, Parse, Format2)]
pub enum BitstringExpr {
    Construct(BitstringConstructExpr),
    Comprehension(BitstringComprehensionExpr),
}

/// `<<` (`$SEGMENT` `,`?)* `>>`
///
/// - $SEGMENT: [Expr] `$SIZE`? `$TYPE`?
/// - $SIZE: `:` [Expr]
/// - $TYPE: `/` ([AtomToken] `-`?)+
#[derive(Debug, Clone, Span, Parse, Format2)]
pub struct BitstringConstructExpr(BitstringLike<BitstringSegment>);

/// `<<` [Expr] `||` ([Qualifier] `,`?)+  `>>`
// TODO: Use `ComprehensionLike`
#[derive(Debug, Clone, Span, Parse)]
pub struct BitstringComprehensionExpr {
    open: DoubleLeftAngleSymbol,
    body: ComprehensionBody,
    close: DoubleRightAngleSymbol,
}

impl Format2 for BitstringComprehensionExpr {
    fn format2(&self, fmt: &mut Formatter2) {
        self.open.format2(fmt);
        fmt.subregion(Indent::CurrentColumn, Newline::Never, |fmt| {
            self.body.format2(fmt)
        });
        self.close.format2(fmt);
    }
}

type ComprehensionBody = BinaryOpLike<Expr, ComprehensionDelimiter, NonEmptyItems<Qualifier>>;

#[derive(Debug, Clone, Span, Parse, Format2)]
struct ComprehensionDelimiter(DoubleVerticalBarSymbol);

impl BinaryOpStyle for ComprehensionDelimiter {
    fn indent_offset(&self) -> usize {
        2
    }

    fn allow_newline(&self) -> bool {
        true
    }

    fn should_pack(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Span, Parse, Format2)]
struct BitstringSegment {
    value: BaseExpr,
    size: Maybe<BitstringSegmentSize>,
    ty: Maybe<BitstringSegmentType>,
}

#[derive(Debug, Clone, Span, Parse, Format2)]
struct BitstringSegmentSize {
    colon: ColonSymbol,
    size: BaseExpr,
}

#[derive(Debug, Clone, Span, Parse)]
struct BitstringSegmentType {
    slash: SlashSymbol,
    specifiers: NonEmptyItems<BitstringSegmentTypeSpecifier, HyphenSymbol>,
}

impl Format2 for BitstringSegmentType {
    fn format2(&self, fmt: &mut Formatter2) {
        self.slash.format2(fmt);
        for (item, delimiter) in self
            .specifiers
            .items()
            .iter()
            .zip(self.specifiers.delimiters().iter())
        {
            item.format2(fmt);
            delimiter.format2(fmt);
        }
        self.specifiers
            .items()
            .last()
            .expect("unreachable")
            .format2(fmt);
    }
}

#[derive(Debug, Clone, Span, Parse, Format2)]
struct BitstringSegmentTypeSpecifier {
    name: AtomToken,
    value: Maybe<(ColonSymbol, IntegerToken)>,
}

#[cfg(test)]
mod tests {
    use crate::items::expressions::Expr;

    #[test]
    fn bitstring_construct_works() {
        let texts = [
            "<<>>",
            indoc::indoc! {"
            %---10---|%---20---|
            <<1, 2, 3, 4, 5, 6,
              7, 8, 9>>"},
            "<<1, 2:16, 3>>",
            "<<<<\"foo\">>/binary>>",
            indoc::indoc! {"
            <<(3 bsr 30 + 2):0,
              $k:[]/signed-integer>>"},
            indoc::indoc! {"
            <<1,
              (foo()):4/little-signed-integer-unit:8,
              C/binary>>"},
        ];
        for text in texts {
            crate::assert_format2!(text, Expr);
        }
    }

    #[test]
    fn bitstring_comprehension_works() {
        let texts = [
            indoc::indoc! {"
            %---10---|%---20---|
            <<<<X>> ||
                X <- [1, 2, 3]>>"},
            indoc::indoc! {"
            %---10---|%---20---|
            <<(foo(X,
                   Y,
                   Z,
                   bar(),
                   baz())) ||
                X <- [1, 2, 3,
                      4, 5],
                Y <= Z,
                false>>"},
            indoc::indoc! {"
            %---10---|%---20---|
            <<<<if
                    X < 10 ->
                        X + $0;
                    true ->
                        X - 10 +
                        $A
                end>> ||
                <<X:4>> <= B>>"},
        ];
        for text in texts {
            crate::assert_format2!(text, Expr);
        }
    }
}
