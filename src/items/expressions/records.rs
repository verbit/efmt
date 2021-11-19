use crate::format::{self, Format};
use crate::items::expressions::{Either, Expr};
use crate::items::generics::{BinaryOpLike, BinaryOpStyle, TupleLike};
use crate::items::symbols::{DotSymbol, MatchSymbol, SharpSymbol};
use crate::items::tokens::AtomToken;
use crate::items::variables::UnderscoreVariable;
use crate::parse::{self, Parse, ResumeParse};
use crate::span::Span;

#[derive(Debug, Clone, Span, Parse, Format)]
pub enum RecordConstructOrIndexExpr {
    Construct(Box<RecordConstructExpr>),
    Index(Box<RecordIndexExpr>),
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub enum RecordAccessOrUpdateExpr {
    Access(Box<RecordAccessExpr>),
    Update(Box<RecordUpdateExpr>),
}

impl ResumeParse<Expr> for RecordAccessOrUpdateExpr {
    fn resume_parse(ts: &mut parse::TokenStream, value: Expr) -> parse::Result<Self> {
        if ts.peek::<(SharpSymbol, (AtomToken, DotSymbol))>().is_some() {
            ts.resume_parse(value).map(Self::Access)
        } else {
            ts.resume_parse(value).map(Self::Update)
        }
    }
}

/// `#` `$NAME` `{` (`$FIELD` `,`?)* `}`
///
/// - $NAME: [AtomToken]
/// - $FIELD: ([AtomToken] | `_`) `=` [Expr]
#[derive(Debug, Clone, Span, Parse, Format)]
pub struct RecordConstructExpr {
    sharp: SharpSymbol,
    name: AtomToken,
    fields: TupleLike<RecordField>,
}

/// `$VALUE` `#` `$NAME` `.` `$FIELD`
///
/// - $VALUE: [Expr]
/// - $NAME: [AtomToken]
/// - $FIELD: [AtomToken]
#[derive(Debug, Clone, Span, Parse)]
pub struct RecordAccessExpr {
    value: Expr,
    index: RecordIndexExpr,
}

impl ResumeParse<Expr> for RecordAccessExpr {
    fn resume_parse(ts: &mut parse::TokenStream, value: Expr) -> parse::Result<Self> {
        Ok(Self {
            value,
            index: ts.parse()?,
        })
    }
}

impl Format for RecordAccessExpr {
    fn format(&self, fmt: &mut format::Formatter) -> format::Result<()> {
        self.value.format(fmt)?;
        if self.value.is_integer_token() {
            fmt.write_space()?;
        }
        self.index.format(fmt)?;
        Ok(())
    }
}

/// `#` `$NAME` `.` `$FIELD`
///
/// - $NAME: [AtomToken]
/// - $FIELD: [AtomToken]
#[derive(Debug, Clone, Span, Parse, Format)]
pub struct RecordIndexExpr {
    sharp: SharpSymbol,
    name: AtomToken,
    dot: DotSymbol,
    field: AtomToken,
}

/// `$VALUE` `#` `$NAME` `{` (`$FIELD` `,`?)* `}`
///
/// - $VALUE: [Expr]
/// - $NAME: [AtomToken]
/// - $FIELD: [AtomToken]
#[derive(Debug, Clone, Span, Parse)]
pub struct RecordUpdateExpr {
    value: Expr,
    sharp: SharpSymbol,
    name: AtomToken,
    fields: TupleLike<RecordField>,
}

impl ResumeParse<Expr> for RecordUpdateExpr {
    fn resume_parse(ts: &mut parse::TokenStream, value: Expr) -> parse::Result<Self> {
        Ok(Self {
            value,
            sharp: ts.parse()?,
            name: ts.parse()?,
            fields: ts.parse()?,
        })
    }
}

impl Format for RecordUpdateExpr {
    fn format(&self, fmt: &mut format::Formatter) -> format::Result<()> {
        self.value.format(fmt)?;
        if self.value.is_integer_token() {
            fmt.write_space()?;
        }
        self.sharp.format(fmt)?;
        self.name.format(fmt)?;
        self.fields.format(fmt)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Span, Parse, Format)]
struct RecordField(BinaryOpLike<Either<AtomToken, UnderscoreVariable>, RecordFieldDelimiter, Expr>);

#[derive(Debug, Clone, Span, Parse, Format)]
struct RecordFieldDelimiter(MatchSymbol);

impl BinaryOpStyle for RecordFieldDelimiter {
    fn indent_offset(&self) -> usize {
        4
    }

    fn allow_newline(&self) -> bool {
        true
    }

    fn should_pack(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_construct_works() {
        let texts = [
            "#foo{}",
            indoc::indoc! {"
            #foo{module = Mod,
                 _ = '_'}"},
            indoc::indoc! {"
            %---10---|%---20---|
            #foo{bar = 2,
                 baz =
                     {bar, baz}}"},
        ];
        for text in texts {
            crate::assert_format!(text, Expr);
        }
    }

    #[test]
    fn record_index_works() {
        let texts = ["#foo.bar"];
        for text in texts {
            crate::assert_format!(text, Expr);
        }
    }

    #[test]
    fn record_access_works() {
        let texts = [
            "X#foo.bar",
            "(foo())#foo.bar",
            "N2#nrec2.nrec1#nrec1.nrec0#nrec0.name",
            "0 #foo.bar",
        ];
        for text in texts {
            crate::assert_format!(text, Expr);
        }
    }

    #[test]
    fn record_update_works() {
        let texts = [
            "M#foo{}",
            "88 #foo{}",
            "M#baz{qux = 1}#foo.bar",
            indoc::indoc! {"
            %---10---|%---20---|
            M#foo.bar#baz{qux =
                              1}"},
            indoc::indoc! {"
            %---10---|%---20---|
            (foo())#foo{bar = 2,
                        baz =
                            {bar,
                             baz}}"},
        ];
        for text in texts {
            crate::assert_format!(text, Expr);
        }
    }
}
