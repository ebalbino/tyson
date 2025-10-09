use crate::Node;
use crate::parser::*;

pub fn evaluate<'arena>(node: &Node<Value>, depth: usize) {
    for (i, atom) in node.iter().enumerate() {
        if i != 0 {
            print!("\n");
        }

        if depth > 0 {
            for _ in 0..depth {
                print!("  ");
            }
        }

        match atom {
            Value::List(head, len) => {
                let val = head.value;

                match val {
                    Value::Symbol(s) => {
                        print!("{}/{}", s, len - 1);
                        match head.next {
                            None => {}
                            Some(ptr) => {
                                print!("\n");
                                let node = unsafe { ptr.as_ref() };
                                evaluate(node, depth + 1);
                            }
                        }
                    }
                    Value::List(inner, _len) => {
                        evaluate(&inner, depth + 1);
                    }
                    _ => print!("Invalid call: {:?}", atom),
                }
            }
            Value::Symbol(s) => {
                print!("{}", s);
            }
            Value::Void => {
                print!("nil");
            }
            Value::True => {
                print!("true");
            }
            Value::False => {
                print!("false");
            }
            Value::Integer(n) => {
                print!("int/{}", n);
            }
            Value::Double(n) => {
                print!("double/{}", n);
            }
            Value::String(s) => {
                print!("string/\"{}\"", s);
            }
        }

        if depth == 0 {
            print!("\n");
        }
    }
}
