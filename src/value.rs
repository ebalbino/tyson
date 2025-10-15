use crate::{Box as ArenaBox, Node};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ASTNode<'arena> {
    Void,
    True,
    False,
    Integer(i64),
    Double(f64),
    String(&'arena str),
    Symbol(&'arena str),
    List(ArenaBox<Node<ASTNode<'arena>>>, usize),
    Quoted(ArenaBox<Node<ASTNode<'arena>>>, usize),
}
