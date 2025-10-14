use crate::Node;
use crate::value::ASTNode;

pub fn print<'arena>(root: &Node<ASTNode>, depth: usize, list_index: usize, list_len: usize) {
    if depth > 0 {
        for _ in 0..depth {
            print!("  ");
        }
    }

    print!("(");

    for (i, stmt) in root.iter().enumerate() {
        if i > 0 && depth != 0 {
            print!(" ");
        }

        match stmt {
            ASTNode::List(list, len) => {
                if depth != 0 {
                    print!("\n");
                }
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
                print!("void");
            }
            ASTNode::String(s) => {
                print!("{}", s);
            }
            ASTNode::Symbol(s) => {
                print!("{}", s);
            }
        }

        if depth == 0 {
            print!("\n");
        }
    }
}
