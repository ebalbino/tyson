use tyson::parser::{Value, parse};
use tyson::{MemoryBlock, Node};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(defpackage :ini-parser
  (:use :common-lisp :anaphora)
  (:export :read-config-from-file))

(in-package :ini-parser)

(defparameter +empty-line+ (format nil \"~%\"))
";

fn print<'arena>(root: &Node<Value>, depth: usize) {
    if depth > 1 {
        for _ in 1..depth {
            print!("  ");
        }
    }

    if depth != 0 {
        print!("(");
    }

    for (i, stmt) in root.iter().enumerate() {
        if i > 0 && depth != 0 {
            print!(" ");
        }

        match stmt {
            Value::List(list) => {
                if depth != 0 {
                    print!("\n");
                }
                print(list, depth + 1);
                print!(")");
            }
            Value::Integer(i) => {
                print!("{}", i);
            }
            Value::Double(d) => {
                print!("{}", d);
            }
            Value::False => {
                print!("false");
            }
            Value::True => {
                print!("true");
            }
            Value::Void => {
                print!("void");
            }
            Value::String(s) => {
                print!("{}", s);
            }
            Value::Symbol(s) => {
                print!("{}", s);
            }
        }

        if depth == 0 {
            println!();
        }
    }
}

fn evaluate<'arena>(root: &Node<Value>, depth: usize) {}

fn visit<'arena, F>(program: Value<'arena>, mut emit: F)
where
    F: FnMut(&Node<Value>, usize),
{
    if let Value::List(root) = program {
        emit(&root, 0);
    } else {
        panic!("Invalid program");
    }
}

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();

    for text in &[CODE] {
        let program = parse(&arena, text).expect("Unable to parse program.");
        visit(program, print);
    }
}
