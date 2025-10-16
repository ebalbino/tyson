use tyson::eval::resolve;
use tyson::parser::parse;
use tyson::print::print;
use tyson::{List, MemoryBlock};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(define (merge L M)
	(if (null? L) M
		(if (null? M) L
			(if (< (car L) (car M))
				(cons (car L) (merge (cdr L) M))
				(cons (car M) (merge (cdr M) L))))))

(define (odd L)
	(if (null? L) '()
		(if (null? (cdr L)) (list (car L))
			(cons (car L) (odd (cddr L))))))
(define (even L)
	(if (null? L) '()
		(if (null? (cdr L)) '()
			(cons (cadr L) (even (cddr L))))))

(define (split L)
	(cons (odd L) (cons (even L) `())))

(define (mergesort L)
	(if (null? L) L
		(if (null? (cdr L)) L
			(merge
				(mergesort (car (split L)))
				(mergesort (cadr (split L)))))))
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
