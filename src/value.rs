use crate::{Arena, Array, Box as ArenaBox, Node, make};

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
}

pub fn traverse<'arena, F>(root: &Node<ASTNode<'arena>>, mut visit: F)
where
    F: FnMut(&Node<ASTNode>, usize, usize, usize),
{
    for (i, atom) in root.iter().enumerate() {
        if let ASTNode::List(node, len) = atom {
            visit(node, 0, i, *len);
        } else {
            panic!("Only a list of lists can be evaluated.");
        }
    }
}

pub fn evaluate<'arena, F, T>(
    arena: &'arena Arena<'arena>,
    root: &'arena Node<ASTNode<'arena>>,
    mut resolve: F,
) -> Array<T>
where
    F: FnMut(&'arena Arena<'arena>, &'arena ASTNode<'arena>, usize, usize, usize) -> T,
    T: Copy,
{
    let count = root.iter().count();
    let mut expressions = make!(arena, T, count).map(Array::new).unwrap();

    for (i, atom) in root.iter().enumerate() {
        expressions.push(&resolve(arena, atom, 0, i, count));
    }

    return expressions;
}
