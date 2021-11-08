use crate::format::Format;
use crate::items::expressions::{Body, Expr, Guard, GuardCondition, IntegerLikeExpr};
use crate::items::generics::{Clauses, Either, Maybe, NonEmptyItems};
use crate::items::keywords::{
    AfterKeyword, BeginKeyword, CaseKeyword, CatchKeyword, EndKeyword, IfKeyword, OfKeyword,
    ReceiveKeyword, TryKeyword,
};
use crate::items::styles::{Block, Child, ColumnIndent, Newline, RightSpace, Space};
use crate::items::symbols::{ColonSymbol, RightArrowSymbol};
use crate::items::tokens::{AtomToken, VariableToken};
use crate::parse::Parse;
use crate::span::Span;

#[derive(Debug, Clone, Span, Parse, Format)]
pub enum BlockExpr {
    Case(Box<CaseExpr>),
    If(Box<IfExpr>),
    Receive(Box<ReceiveExpr>),
    Begin(Box<BeginExpr>),
    Try(Box<TryExpr>),
    Catch(Box<CatchExpr>),
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct CaseExpr {
    case: Space<CaseKeyword>,
    value: Child<Expr>,
    of: Space<OfKeyword>,
    clauses: Newline<Block<Clauses<CaseClause>>>,
    end: EndKeyword,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct CaseClause {
    pattern: Expr,
    guard: Maybe<Guard>,
    arrow: Space<RightArrowSymbol>,
    body: Body,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct IfExpr {
    r#if: IfKeyword,
    clauses: Newline<Block<Clauses<IfClause>>>,
    end: EndKeyword,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct IfClause {
    condigion: GuardCondition,
    arrow: Space<RightArrowSymbol>,
    body: Body,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct BeginExpr {
    begin: BeginKeyword,
    exprs: Newline<Block<NonEmptyItems<Expr>>>,
    end: EndKeyword,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct ReceiveExpr {
    receive: Newline<ReceiveKeyword>,
    clauses: Maybe<Newline<Block<Clauses<CaseClause>>>>,
    timeout: Maybe<Newline<ReceiveTimeout>>,
    end: EndKeyword,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct ReceiveTimeout {
    after: Newline<AfterKeyword>,
    clause: Block<ReceiveTimeoutClause>,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct ReceiveTimeoutClause {
    millis: IntegerLikeExpr,
    arrow: Space<RightArrowSymbol>,
    body: Body,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct TryExpr {
    r#try: TryKeyword,
    body: Newline<Body>,
    clauses: Maybe<(OfKeyword, Newline<Block<Clauses<CaseClause>>>)>,
    catch: Maybe<Newline<TryCatch>>,
    after: Maybe<Newline<TryAfter>>,
    end: EndKeyword,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct TryCatch {
    catch: CatchKeyword,
    clauses: Newline<Block<Clauses<CatchClause>>>,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct CatchClause {
    class: Maybe<(Either<AtomToken, VariableToken>, ColonSymbol)>,
    pattern: Expr,
    stacktrace: Maybe<(ColonSymbol, VariableToken)>,
    guard: Maybe<Guard>,
    arrow: Space<RightArrowSymbol>,
    body: Body,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct TryAfter {
    after: AfterKeyword,
    body: Body,
}

#[derive(Debug, Clone, Span, Parse, Format)]
pub struct CatchExpr {
    catch: RightSpace<CatchKeyword>,
    expr: ColumnIndent<Expr>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::expressions::NonLeftRecursiveExpr;
    use crate::items::styles::Child;
    use crate::parse::parse_text;
    use crate::FormatOptions;

    fn format(text: &str) -> String {
        FormatOptions::<Child<Expr>>::new()
            .max_columns(20)
            .format_text(text)
            .expect("parse or format failed")
    }

    #[test]
    fn case_works() {
        let texts = [
            concat!(
                "case Foo of\n", //
                "    1 ->\n",
                "        2\n",
                "end"
            ),
            concat!(
                "case foo() of\n",
                "    {1, 2} ->\n",
                "        3;\n",
                "    A when is_integer(A),\n",
                "           A > 100 ->\n",
                "        A / 10\n",
                "end"
            ),
        ];
        for text in texts {
            let x = parse_text(text).unwrap();
            if let Expr::NonLeftRecursive(NonLeftRecursiveExpr::Block(x)) = &x {
                assert!(matches!(**x, BlockExpr::Case(_)));
            } else {
                panic!("{:?}", x);
            }
            assert_eq!(format(text), text);
        }
    }

    #[test]
    fn if_works() {
        let texts = [
            indoc::indoc! {"
                if
                    true ->
                        2
                end"},
            indoc::indoc! {"
                if
                    A =:= {1, 2} ->
                        3;
                    is_integer(A),
                    A > 100 ->
                        A / 10
                end"},
        ];
        for text in texts {
            let x = parse_text(text).unwrap();
            if let Expr::NonLeftRecursive(NonLeftRecursiveExpr::Block(x)) = &x {
                assert!(matches!(**x, BlockExpr::If(_)));
            } else {
                panic!("{:?}", x);
            }
            assert_eq!(format(text), text);
        }
    }

    #[test]
    fn receive_works() {
        let texts = [
            indoc::indoc! {"
                receive
                    {A, B} ->
                        [A, B];
                    A when is_integer(A);
                           is_atom(A) ->
                        A
                end"},
            indoc::indoc! {"
                receive
                    A ->
                        A
                after
                    1000 ->
                        timeout
                end"},
            indoc::indoc! {"
                receive
                after
                    N ->
                        timeout
                end"},
        ];
        for text in texts {
            let x = parse_text(text).unwrap();
            if let Expr::NonLeftRecursive(NonLeftRecursiveExpr::Block(x)) = &x {
                assert!(matches!(**x, BlockExpr::Receive(_)));
            } else {
                panic!("{:?}", x);
            }
            assert_eq!(format(text), text);
        }
    }

    #[test]
    fn begin_works() {
        let texts = [
            indoc::indoc! {"
                begin
                    1
                end"},
            indoc::indoc! {"
                begin
                    foo(bar, Baz),
                    {[#{}]}
                end"},
        ];
        for text in texts {
            let x = parse_text(text).unwrap();
            if let Expr::NonLeftRecursive(NonLeftRecursiveExpr::Block(x)) = &x {
                assert!(matches!(**x, BlockExpr::Begin(_)));
            } else {
                panic!("{:?}", x);
            }
            assert_eq!(format(text), text);
        }
    }

    #[test]
    fn try_works() {
        let texts = [
            indoc::indoc! {"
                try
                    1
                after
                    2
                end"},
            indoc::indoc! {"
                try
                    1,
                    2,
                    3
                catch
                    E ->
                        E
                end"},
            indoc::indoc! {"
                try
                    X
                of
                    {_, _} ->
                        1;
                    [_, _] ->
                        2
                catch
                    _:E:Stacktrace when is_atom(E) ->
                        foo
                after
                    bar
                end"},
            indoc::indoc! {"
                try
                    foo()
                catch
                    throw:_ ->
                        throw;
                    error:_ ->
                        error
                end"},
        ];
        for text in texts {
            let x = parse_text(text).unwrap();
            if let Expr::NonLeftRecursive(NonLeftRecursiveExpr::Block(x)) = &x {
                assert!(matches!(**x, BlockExpr::Try(_)));
            } else {
                panic!("{:?}", x);
            }
            assert_eq!(format(text), text);
        }
    }

    #[test]
    fn catch_works() {
        let texts = [
            "catch 1",
            indoc::indoc! {"
                catch foo(bar,
                          Baz,
                          qux) + 3 +
                      4"},
        ];
        for text in texts {
            let x = parse_text(text).unwrap();
            if let Expr::NonLeftRecursive(NonLeftRecursiveExpr::Block(x)) = &x {
                assert!(matches!(**x, BlockExpr::Catch(_)));
            } else {
                panic!("{:?}", x);
            }
            assert_eq!(format(text), text);
        }
    }
}
