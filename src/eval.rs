use crate::value::ASTNode;
use crate::{Arena, Array, Node, make};

#[derive(Clone, Copy, Debug)]
pub enum Atom<'arena> {
    True {
        depth: usize,
        position: usize,
    },
    False {
        depth: usize,
        position: usize,
    },
    Void {
        depth: usize,
        position: usize,
    },
    Int {
        depth: usize,
        position: usize,
        inner: i64,
    },
    Number {
        depth: usize,
        position: usize,
        inner: f64,
    },
    String {
        depth: usize,
        position: usize,
        inner: &'arena str,
    },
    Call {
        depth: usize,
        position: usize,
        symbol: &'arena str,
        arity: usize,
    },
    Identifier {
        depth: usize,
        position: usize,
        symbol: &'arena str,
    },
    Expression {
        depth: usize,
        position: usize,
        body: Array<Atom<'arena>>,
    },
    Code {
        depth: usize,
        position: usize,
        body: Array<Atom<'arena>>,
    },
}

pub fn resolve<'arena>(
    arena: &'arena Arena<'arena>,
    root: &'arena Node<ASTNode<'arena>>,
    list_len: usize,
    depth: usize,
) -> Option<Array<Atom<'arena>>> {
    make!(arena, Atom, list_len)
        .map(Array::new)
        .map(|mut exprs| {
            for (position, node) in root.iter().enumerate() {
                exprs.push(&match node {
                    ASTNode::List(list, len) => Atom::Expression {
                        body: resolve(arena, list, *len, depth + 1).unwrap(),
                        depth,
                        position,
                    },
                    ASTNode::Quoted(list, len) => Atom::Code {
                        body: resolve(arena, list, *len, depth + 1).unwrap(),
                        depth,
                        position,
                    },
                    ASTNode::Symbol(symbol) if position == 0 => {
                        let arity = list_len - 1;
                        Atom::Call {
                            symbol,
                            arity,
                            depth,
                            position,
                        }
                    }
                    ASTNode::Symbol(symbol) => Atom::Identifier {
                        depth,
                        position,
                        symbol,
                    },
                    ASTNode::Void => Atom::Void { depth, position },
                    ASTNode::True => Atom::True { depth, position },
                    ASTNode::False => Atom::False { depth, position },
                    ASTNode::Integer(i) => Atom::Int {
                        inner: *i,
                        depth,
                        position,
                    },
                    ASTNode::Double(f) => Atom::Number {
                        inner: *f,
                        depth,
                        position,
                    },
                    ASTNode::String(s) => Atom::String {
                        inner: s,
                        depth,
                        position,
                    },
                });
            }
            exprs
        })
}
