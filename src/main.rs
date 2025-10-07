use tyson::{Arena, Array, List, MemoryBlock, make, strmake};

mod parser;

use crate::parser::{Token, Tokenizer};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "(print 1 3.14159 \"hello, world\")\n (+ (* 2 2) (/ 10 5)";
const JSON: &str = "{ \"type\": \"object\" }";
const RUBY_OBJ: &str = "{ \"type\"=> \"object\" }";

#[derive(Debug, Clone, Copy)]
enum Instance<'arena> {
    Integer(i64),
    Double(f64),
    String(&'arena str),
    Symbol(&'arena str),
    List(Array<Instance<'arena>>),
}

fn parse_list<'arena>(
    arena: &'arena Arena,
    tokens: &mut Array<Token>,
) -> Result<Instance<'arena>, &'static str> {
    let token = tokens.pop();

    if !(matches!(
        token,
        Some(Token::LParen) | Some(Token::LBrace) | Some(Token::LBracket)
    )) {
        return Err("The first token must be an opening parentheses");
    }

    let mut list = make!(arena, Instance, 8)
        .map(Array::new)
        .expect("Unable to allocate list.");

    while !tokens.is_empty() {
        match tokens.pop() {
            None => {
                return Err("Not enough tokens");
            }
            Some(token) => match token {
                Token::Integer(i) => {
                    list.push(&Instance::Integer(*i));
                }
                Token::Float(f) => {
                    list.push(&Instance::Double(*f));
                }
                Token::String(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate string");
                    list.push(&Instance::String(s));
                }
                Token::Symbol(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate symbol");
                    list.push(&Instance::Symbol(s));
                }
                Token::LParen | Token::LBrace | Token::LBracket => {
                    tokens.push(&Token::LParen);
                    let sub_list = parse_list(arena, tokens)?;
                    list.push(&sub_list);
                }
                Token::RParen | Token::RBrace | Token::RBracket => {
                    return Ok(Instance::List(list));
                }
            },
        }
    }

    Ok(Instance::List(list))
}

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();

    for text in &[CODE, JSON, RUBY_OBJ] {
        let token_count = Tokenizer::new(text).into_iter().count();
        let mut tokens = make!(arena, Token, token_count)
            .map(Array::new)
            .expect("Not enough space for tokens");

        for token in Tokenizer::new(text) {
            tokens.push(&token);
        }

        tokens.reverse();

        let mut statements = List::new(&arena);

        while !tokens.is_empty() {
            statements.push_back(&parse_list(&arena, &mut tokens));
        }

        for stmt in statements.iter() {
            println!("{:?}", stmt);
        }
    }
}
