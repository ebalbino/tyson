use crate::Node;
use crate::value::ASTNode;

pub fn print<'arena>(root: &Node<ASTNode>, count: usize, depth: usize, quoted: bool) {
    if !quoted && depth > 1 {
        for _ in 0..depth - 1 {
            print!("  ");
        }
    }

    for (i, atom) in root.iter().enumerate() {
        if depth != 0 && i == 0 {
            if quoted {
                print!("'(");
            } else {
                print!("(");
            }
        }

        if i != 0 {
            print!(" ");
        }

        match atom {
            ASTNode::List(list, len) => {
                print!("\n");
                print(list, *len, depth + 1, false);
                print!(")");
            }
            ASTNode::Quoted(list, len) => {
                print(list, *len, depth + 1, true);
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
            if i == count - 1 {
                print!(")\n");
            }
        }
    }
}
