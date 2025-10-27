use tyson::print::print;
use tyson::read::parse;
use tyson::{List, MemoryBlock};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(define (make-base-pythagoreans upper)
  (define (py3 a b c)
    (define (U a b c)
      (list (+ (* a 1) (* b -2) (* c 2))
            (+ (* a 2) (* b -1) (* c 2))
            (+ (* a 2) (* b -2) (* c 3))))
    (define (A a b c)
      (list (+ (* a 1) (* b 2) (* c 2))
            (+ (* a 2) (* b 1) (* c 2))
            (+ (* a 2) (* b 2) (* c 3))))
    (define (D a b c)
      (list (+ (* a -1) (* b 2) (* c 2))
            (+ (* a -2) (* b 1) (* c 2))
            (+ (* a -2) (* b 2) (* c 3))))
    (values (U a b c) (A a b c) (D a b c)))

  (let1 pythagoreans (make-hash-table 'equal?)
    (let loop ((q '((3 4 5))))
      (if (null? q) pythagoreans
          (let1 tr (car q)
            (if (every (cut <= <> upper) tr)
                (begin
                  (hash-table-put! pythagoreans tr #t)
                  (receive (u a d) (apply py3 tr)
                    (loop (cons* u a d (cdr q)))))
                (loop (cdr q))))))))
";

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();
    let mut expressions = List::new(&arena);

    for source in &[CODE] {
        expressions.push_back(&parse(&arena, source).expect("Unable to parse code!"));
    }

    for (expression, code) in expressions.iter().zip(&[CODE]) {
        let mut string = String::with_capacity(4096);
        println!("count: {}", expression.len());
        let _ = print(&mut string, expression, false);
        println!("{code}");
        println!();
        println!("{string}");

        if *code == string {
            println!("Exact match!");
        } else {
            println!("Does not match!");
        }
    }

    for source in &[CODE] {
        expressions.push_back(&parse(&arena, source).expect("Unable to parse code!"));
    }

    println!("{:#?}", arena);
}
