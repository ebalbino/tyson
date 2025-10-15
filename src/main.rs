use tyson::eval::resolver;
use tyson::parser::parse;
use tyson::print::print;
use tyson::value::{evaluate, traverse};
use tyson::{List, MemoryBlock};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(defun m 3)
(defun n 3)
(defun iota (m) (step m)) 
(defun iter (m n lst)
		(cond ((> n m) (reverse lst))
			(t (iter m (+ n step) (cons n lst)))))
";

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();
    let mut modules = List::new(&arena);
    let mut expressions = List::new(&arena);

    for text in &[CODE] {
        modules.push_back(&parse(&arena, text).unwrap());
    }

    println!("Memory use after parsing: {:?}", arena);

    for module in modules.iter() {
        traverse(&module, print);
        expressions.push_back(&evaluate(&arena, &module, resolver));
    }

    for expression in expressions.iter().enumerate() {
        println!("{:#?}", expression);
    }

    println!("{:#?}", arena);
}
