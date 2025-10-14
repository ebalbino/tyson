use crate::env::Env;
use crate::value::{ASTNode, evaluate};
use crate::{Arena, Array, Box as ArenaBox, List, Node, make, strmake};

#[derive(Clone, Copy, Debug)]
pub enum Atom<'arena> {
    True,
    False,
    Void,
    Int(i64),
    Number(f64),
    String(&'arena str),
    Ref(&'arena str, usize),
    Function(Array<Atom<'arena>>),
    Unknown,
}

pub fn resolver<'arena>(
    arena: &'arena Arena<'arena>,
    node: &'arena ASTNode<'arena>,
    depth: usize,
    list_index: usize,
    list_len: usize,
) -> Atom<'arena> {
    match node {
        ASTNode::List(head, len) => {
            let count = *len;
            let mut atoms = make!(arena, Atom, count).map(Array::new).unwrap();

            for (i, node) in head.iter().enumerate() {
                if i == 0 {
                    match node {
                        ASTNode::Symbol(_) => {} // NOOP
                        ASTNode::List(_, n) => {
                            return resolver(arena, node, depth + 1, 0, *n);
                        }
                        _ => {
                            panic!("Head of list must be symbol or list");
                        }
                    }
                }

                atoms.push(&resolver(arena, node, depth + 1, i, *len));
            }

            Atom::Function(atoms)
        }
        ASTNode::Symbol(s) => Atom::Ref(s, list_len - 1),
        ASTNode::Void => Atom::Void,
        ASTNode::True => Atom::True,
        ASTNode::False => Atom::False,
        ASTNode::Integer(n) => Atom::Int(*n),
        ASTNode::Double(n) => Atom::Number(*n),
        ASTNode::String(s) => Atom::String(s),
    }
}
