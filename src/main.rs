use tyson::eval::evaluate;
use tyson::parser::{Value, parse};
use tyson::{Arena, MemoryBlock, Node};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(begin
    (define (welcome)
        (displayln \"Welcome to the REPL!\")
        (displayln \"Use Ctrl+D to exit!\")
        (newline))

    (define (tle-eval expr)
        (eval expr (top-level-environment)))

    (define (goodbye)
        (newline)
        (displayln \"Quitting!\"))

    (define (repl)
        (display \">> \")
        (let ((expr (read)))
            (if (not (eof-object? expr))
                (let ((result (tle-eval expr)))
                     (if (not (void? result))
                             (writeln result))
                     (repl)))))

    (welcome)
    (repl)
    (goodbye))
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
            Value::List(list, _len) => {
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
            print!("\n");
        }
    }
}

fn walk<'arena, F, T>(ast: &Value<'arena>, mut visit: F)
where
    F: FnMut(&Node<Value>, usize) -> T,
{
    if let Value::List(node, _len) = ast {
        visit(&node, 0);
    }
}

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();

    for text in &[CODE] {
        let program = parse(&arena, text).expect("Unable to parse program.");
        //visit(&program, print);
        walk(&program, evaluate);
    }
}
