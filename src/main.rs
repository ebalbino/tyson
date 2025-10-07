use std::marker::PhantomData;

use tyson::{Arena, Array, Box as ArenaBox, List, MemoryBlock, Node, make, strmake};

mod parser;

use crate::parser::{Token, Tokenizer};

fn megabytes(n: usize) -> usize {
    1024 * 1024 * n
}

const CODE: &str = "
(defpackage :ini-parser
  (:use :common-lisp :anaphora)
  (:export :read-config-from-file))
(in-package :ini-parser)

(defparameter +empty-line+ (format nil \"~%\"))

(defmacro defmatcher (name regexp &rest fields)
  (let ((result-expr (loop for i in fields collect `(aref data ,i))))
    `(defun ,name (string)
         (let ((scaner (ppcre:create-scanner ,regexp)))
           (multiple-value-bind (matched data) (ppcre:scan-to-strings scaner string)
             (if matched (list ,@result-expr)))))))
";

#[derive(Debug, Clone, Copy)]
enum Instance<'arena> {
    Integer(i64),
    Double(f64),
    String(&'arena str),
    Symbol(&'arena str),
    List(ArenaBox<Node<Instance<'arena>>>),
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

    let mut list: List<'arena, Instance> = List::new(arena);

    while !tokens.is_empty() {
        match tokens.pop() {
            None => {
                return Err("Not enough tokens");
            }
            Some(token) => match token {
                Token::Integer(i) => {
                    list.push_back(&Instance::Integer(*i));
                }
                Token::Float(f) => {
                    list.push_back(&Instance::Double(*f));
                }
                Token::String(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate string");
                    list.push_back(&Instance::String(s));
                    //println!("{:?}", list);
                }
                Token::Symbol(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate symbol");
                    list.push_back(&Instance::Symbol(s));
                }
                Token::LParen | Token::LBrace | Token::LBracket => {
                    tokens.push(&Token::LParen);
                    let sub_list = parse_list(arena, tokens)?;
                    list.push_back(&sub_list);
                }
                Token::RParen | Token::RBrace | Token::RBracket => {
                    return make!(arena, Node<Instance>)
                        .map(ArenaBox::new)
                        .map(|mut b| {
                            *b = list.to_node().unwrap();
                            Instance::List(b)
                        })
                        .ok_or("Failed to close list");
                }
            },
        }
    }

    return make!(arena, Node<Instance>)
        .map(ArenaBox::new)
        .map(|mut b| {
            *b = list.to_node().unwrap();
            Instance::List(b)
        })
        .ok_or("Failed to close list");
}

fn print_list(root: &Node<Instance>, depth: usize) {
    for i in root.iter() {
        match i {
            Instance::List(l) => {
                print_list(l, depth + 1);
            }
            _ => {
                if depth > 1 {
                    for _ in 1..depth {
                        print!("  ");
                    }
                }
                println!("{:?}", i);
            }
        }
    }
}

fn main() {
    let block = MemoryBlock::with_capacity(megabytes(32));
    let arena = block.arena(megabytes(16)).unwrap();

    for text in &[CODE] {
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
            statements.push_back(&parse_list(&arena, &mut tokens).unwrap());
        }

        print_list(&statements.to_node().unwrap(), 0);
    }

    println!("{:?}", arena);
}
