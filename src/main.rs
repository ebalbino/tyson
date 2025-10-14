use tyson::MemoryBlock;
use tyson::eval::{Atom, resolver};
use tyson::parser::parse;
use tyson::print::print;
use tyson::value::{ASTNode, evaluate, traverse};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(define (pop)
  (lambda (stack)
    (let ((element (car stack))
          (new-stack (cdr stack)))
      (list element new-stack))))

(define (stack-of result)
  (cadr result))

(define (value-of result)
  (car result))

(define (>>= stack-action continuation)
  (lambda (stack)
    (let ((result (stack-action stack)))
      ((continuation (value-of result)) (stack-of result)))))

(define (return value)
  (lambda (stack)
    (list value stack)))

(define (run-stack computation stack)
  (computation stack))

(define (eval-stack computation stack)
  (value-of (computation stack)))

(define (exec-stack computation stack)
  (stack-of (computation stack)))

(define (computation-1) (>>= (push 4) (lambda (_)
                      (>>= (push 5) (lambda (_)
                      (>>= (pop)    (lambda (a)
                      (>>= (pop)    (lambda (b)
                      (return (list a b)))))))))))

(define (computation-2) (>>= (push 2) (lambda (_)
                      (>>= (push 3) (lambda (_)
                      (>>= (pop)    (lambda (a)
                      (>>= (pop)    (lambda (b)
                      (return (list a b)))))))))))

(define (main)
  (let ((initial-stack '())
        (composed (>>= computation-1 (lambda (a)
                  (>>= computation-2 (lambda (b)
                  (return (list a b))))))))
    (begin
      (display \"Result: \")
      (display (eval-stack composed initial-stack)))))
";

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();

    for text in &[CODE] {
        let program = parse(&arena, text).expect("Unable to parse program.");
        println!("Memory use after parsing: {:?}", arena);
        traverse(&program, print);

        let results = evaluate(&arena, &program, resolver);
        println!("Memory use after evaluation: {:?}", arena);

        for result in results.iter() {
            match result {
                Atom::Function(atoms) => {
                    for atom in atoms.iter() {
                        println!("{:?}", atom);
                    }
                }
                _ => {
                    println!("{:?}", result);
                }
            }
        }
    }

    println!("{:?}", arena);
}
