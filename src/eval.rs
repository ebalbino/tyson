use crate::value::ASTNode;
use crate::{Arena, Array, Node, make};

#[derive(Clone, Copy, Debug)]
pub enum Atom<'arena> {
    True,
    False,
    Void,
    Int { inner: i64 },
    Number { inner: f64 },
    String { inner: &'arena str },
    Buffer { data: &'arena [u8] },
    File { path: &'arena str, lazy: bool },
    Define,
    Cons,
    Head,
    Tail,
    Binding { symbol: &'arena str, arity: usize },
    Statement { body: Array<Expression<'arena>> },
    Code { body: Array<Expression<'arena>> },
}

#[derive(Clone, Copy, Debug)]
pub struct Expression<'arena> {
    depth: usize,
    position: usize,
    payload: Atom<'arena>,
}

pub fn resolve<'arena>(
    arena: &'arena Arena<'arena>,
    root: &'arena Node<ASTNode<'arena>>,
    count: usize,
    depth: usize,
) -> Option<Array<Expression<'arena>>> {
    make!(arena, Expression, count)
        .map(Array::new)
        .map(|mut exprs| {
            for (position, node) in root.iter().enumerate() {
                let payload = match node {
                    ASTNode::List(list, len) => Atom::Statement {
                        body: resolve(arena, list, *len, depth + 1).unwrap(),
                    },
                    ASTNode::Quoted(list, len) => Atom::Code {
                        body: resolve(arena, list, *len, depth + 1).unwrap(),
                    },
                    ASTNode::Symbol(symbol) => match *symbol {
                        "define" | "def" => Atom::Define,
                        "head" | "car" => Atom::Head,
                        "tail" | "cdr" => Atom::Tail,
                        "cons" => Atom::Cons,
                        _ => {
                            let arity = if position == 0 { count - 1 } else { 0 };
                            Atom::Binding { symbol, arity }
                        }
                    },
                    ASTNode::Void => Atom::Void,
                    ASTNode::True => Atom::True,
                    ASTNode::False => Atom::False,
                    ASTNode::Integer(i) => Atom::Int { inner: *i },
                    ASTNode::Double(f) => Atom::Number { inner: *f },
                    ASTNode::String(s) => Atom::String { inner: s },
                };
                exprs.push(&Expression {
                    depth,
                    position,
                    payload,
                });
            }
            exprs
        })
}
