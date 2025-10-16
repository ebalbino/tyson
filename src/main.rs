use tyson::eval::resolve;
use tyson::parser::parse;
use tyson::print::print;
use tyson::{List, MemoryBlock};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(define (exp-of-ten x)
  (expt 10 x))

(define (foo x context)
  (print (context x)))

(define (bar list context)
  (for-each
   (lambda (listp) (foo listp context))
   list))

(bar '(rand n 8) exp-of-ten)
";

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();
    let mut modules = List::new(&arena);
    let mut expressions = List::new(&arena);

    for text in &[CODE] {
        modules.push_back(&parse(&arena, text).unwrap());
    }

    println!("Memory use after parsing: {:#?}", arena);

    for module in modules.iter() {
        let count = module.iter().count();
        //print(&module, count, 0, false);
        println!("{:#?}", module);
        expressions.push_back(&resolve(&arena, &module, count, 0));
    }

    for expression in expressions.iter() {
        println!("{:#?}", expression);
    }

    println!("{:#?}", arena);
}
