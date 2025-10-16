use tyson::parser::{lexer, parse};
use tyson::print::print;
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
    let mut modules = List::new(&arena);
    let mut expressions = List::new(&arena);

    for text in &[CODE] {
        modules.push_back(&lexer(&arena, text).unwrap());
    }

    println!("Memory use after parsing: {:#?}", arena);

    for module in modules.iter() {
        let count = module.iter().count();
        //print(&module, count, 0, false);
        expressions.push_back(&parse(&arena, &module, count, 0));
    }

    for expression in expressions.iter() {
        println!("{:#?}", expression);
    }

    println!("{:#?}", arena);
}
