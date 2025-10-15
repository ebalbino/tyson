use crate::value::ASTNode;
use crate::{Arena, Array, Box as ArenaBox, List, Node, make, strmake};

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
    Expression {
        depth: usize,
        position: usize,
        body: Array<Atom<'arena>>,
    },
}

pub fn resolver<'arena>(
    arena: &'arena Arena<'arena>,
    node: &'arena ASTNode<'arena>,
    depth: usize,
    position: usize,
    list_len: usize,
) -> Atom<'arena> {
    match node {
        ASTNode::List(head, len) => {
            let count = *len;
            let mut body = make!(arena, Atom, count).map(Array::new).unwrap();

            for (i, node) in head.iter().enumerate() {
                if i == 0 {
                    match node {
                        ASTNode::List(_, n) => {
                            return resolver(arena, node, depth + 1, 0, *n);
                        }
                        _ => {}
                    }
                }

                body.push(&resolver(arena, node, depth + 1, i, *len));
            }

            Atom::Expression {
                body,
                depth,
                position,
            }
        }
        ASTNode::Symbol(s) => Atom::Call {
            symbol: s,
            arity: list_len - 1,
            depth,
            position,
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
    }
}
