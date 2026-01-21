/*
use incpa_str::StrParser;

use crate::recursive;

#[derive(Debug)]
enum Expr {
    Num(u64),
    Sym(String),
    List(Vec<Expr>),
}

fn expr() -> impl StrParser<Output = Expr> {
    recursive(expr_rec)
}

fn expr_rec(rec: ()) -> impl StrParser<Output = Expr> {
    use Expr::*;

    num().map(Num).or(sym().map(Sym)).or(list(rec).map(List))
}

fn num() -> impl StrParser<Output = u64> {
    // Dummy implementation:
    "42".map(|_| 42)
}

fn sym() -> impl StrParser<Output = String> {
    // Dummy implementation:
    "x".map(|_| "x".to_string())
}

fn list(rec: ()) -> impl StrParser<Output = Vec<Expr>> {
    '['.ignore_then(list_tail(rec))
}

fn list_tail(rec: ()) -> impl StrParser<Output = Vec<Expr>> {
    ']'.map(|_| vec![])
        .or(expr_rec(rec))
        .then_ignore(", ")
        .then(list_tail(rec))
        .map(|x, mut xs| {
            xs.insert(0, x);
            xs
        })
}
*/
