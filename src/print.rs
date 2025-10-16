use crate::Node;
use crate::parser::Lexeme;

pub fn print<'arena>(root: &Node<Lexeme>, count: usize, depth: usize, quoted: bool) {
    if depth > 0 {
        for _ in 0..depth {
            print!("  ");
        }
    }

    if quoted && depth == 0 {
        print!("'(");
    } else if depth == 0 {
        print!("(");
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
            Lexeme::List(list, len) => {
                print!("\n");
                print(list, *len, depth + 1, false);
                print!(")");
            }
            Lexeme::Quoted(list, len) => {
                print!("\n");
                print(list, *len, depth + 1, true);
                print!(")");
            }
            Lexeme::Integer(i) => {
                print!("{}", i);
            }
            Lexeme::Double(d) => {
                print!("{}", d);
            }
            Lexeme::False => {
                print!("#f");
            }
            Lexeme::True => {
                print!("#t");
            }
            Lexeme::Unit => {
                if quoted {
                    print!("'()");
                } else {
                    print!("()");
                }
            }
            Lexeme::Null => {
                print!("#nil");
            }
            Lexeme::String(s) => {
                print!("{}", s);
            }
            Lexeme::Symbol(s, quoted) => {
                if *quoted {
                    print!("'{}", s);
                } else {
                    print!("{}", s);
                }
            }
            Lexeme::Operator(s) => {
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
