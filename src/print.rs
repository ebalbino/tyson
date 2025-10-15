use crate::Node;
use crate::value::ASTNode;

pub fn print<'arena>(root: &Node<ASTNode>, depth: usize, list_index: usize, list_len: usize) {
    if depth > 0 || list_index != 0 {
        for _ in 0..depth {
            print!("  ");
        }
    }

    print!("(");

    for (i, stmt) in root.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }

        match stmt {
            ASTNode::List(list, len) => {
                print!("\n");
                print(list, depth + 1, 0, *len);
                print!(")");
            }
            ASTNode::Integer(i) => {
                print!("{}", i);
            }
            ASTNode::Double(d) => {
                print!("{}", d);
            }
            ASTNode::False => {
                print!("false");
            }
            ASTNode::True => {
                print!("true");
            }
            ASTNode::Void => {
                print!("nil");
            }
            ASTNode::String(s) => {
                print!("{}", s);
            }
            ASTNode::Symbol(s) => {
                print!("{}", s);
            }
        }

        if depth == 0 {
            if i == list_len - 1 {
                print!(")\n");
            }
        }
    }
}
