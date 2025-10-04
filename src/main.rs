use tyson::{Arena, List, MemoryBlock, Node};

mod parser;

use crate::parser::Tokenizer;

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "(print 1 3.14159 'hello, world') (+ 1 2)";

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(16));
    let arena = block.arena(megabytes(2)).unwrap();
    let tokenizer = Tokenizer::new(CODE);
    let mut tokens = List::new(&arena);

    for token in tokenizer {
        println!("{:?}", token);
        tokens.push_back(&token);
    }
}
