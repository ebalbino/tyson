use crate::Node;
use crate::parser::*;

pub fn evaluate<'arena>(node: &Node<Value>, depth: usize) {
    for (i, atom) in node.iter().enumerate() {
        if i != 0 {
            print!("\n");
        }

        let indent = || {
            if depth > 0 {
                for _ in 0..depth {
                    print!("  ");
                }
            }
        };

        match atom {
            Value::List(head, len) => {
                let val = head.value;

                match val {
                    Value::List(inner, _len) => {
                        evaluate(&inner, depth);
                    }
                    Value::Symbol(s) => {
                        indent();
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
                    _ => panic!("Invalid call: {:?}", val),
                }
            }
            Value::Symbol(s) => {
                indent();
                print!("{}", s);
            }
            Value::Void => {
                indent();
                print!("nil");
            }
            Value::True => {
                indent();
                print!("true");
            }
            Value::False => {
                indent();
                print!("false");
            }
            Value::Integer(n) => {
                indent();
                print!("int/{}", n);
            }
            Value::Double(n) => {
                indent();
                print!("double/{}", n);
            }
            Value::String(s) => {
                indent();
                print!("string/\"{}\"", s);
            }
        }

        if depth == 0 {
            print!("\n");
        }
    }
}
