use crate::tokenizer::{Token, tokenize};
use crate::{Arena, Box as ArenaBox, List, Node, make, strmake};
use crate::{Box as ArenaBox, Node};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ASTNode<'arena> {
    Unit,
    Null,
    True,
    False,
    Integer(i64),
    Double(f64),
    String(&'arena str),
    Symbol(&'arena str),
    List(ArenaBox<Node<ASTNode<'arena>>>, usize),
    Quoted(ArenaBox<Node<ASTNode<'arena>>>, usize),
}

pub fn lexer<'arena>(
    arena: &'arena Arena,
    code: &str,
) -> Result<Node<ASTNode<'arena>>, &'static str> {
    let mut tokens = List::new(&arena);

    for token in tokenize(code) {
        tokens.push_back(&token);
    }

    let tree = lex_tokens(arena, &mut tokens, false)?;

    if let ASTNode::List(head, _) = tree {
        return Ok(*head);
    }

    Err("The program couldn't be parsed correctly.")
}

fn lex_tokens<'arena>(
    arena: &'arena Arena,
    tokens: &mut List<Token>,
    quoted: bool,
) -> Result<ASTNode<'arena>, &'static str> {
    let mut list: List<'arena, ASTNode> = List::new(arena);

    while !tokens.is_empty() {
        match tokens.pop_front() {
            None => {
                return Err("Not enough tokens");
            }
            Some(token) => match token {
                Token::Integer(i) => {
                    list.push_back(&ASTNode::Integer(i.parse().unwrap()));
                }
                Token::Float(f) => {
                    list.push_back(&ASTNode::Double(f.parse().unwrap()));
                }
                Token::Nil => {
                    list.push_back(&ASTNode::Null);
                }
                Token::True => {
                    list.push_back(&ASTNode::True);
                }
                Token::False => {
                    list.push_back(&ASTNode::False);
                }
                Token::String(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate string.");
                    list.push_back(&ASTNode::String(s));
                }
                Token::Symbol(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate symbol.");
                    list.push_back(&ASTNode::Symbol(s));
                }
                Token::Quote => match tokens.pop_front().expect("You can't quote nothing.") {
                    Token::LParen | Token::LBrace | Token::LBracket => {
                        let sub_list = parse_list(arena, tokens, true)?;
                        list.push_back(&sub_list);
                    }
                    _ => {
                        panic!("");
                    }
                },
                Token::LParen | Token::LBrace | Token::LBracket => {
                    let sub_list = parse_list(arena, tokens, false)?;
                    list.push_back(&sub_list);
                }
                Token::RParen | Token::RBrace | Token::RBracket => {
                    return make!(arena, Node<ASTNode>)
                        .map(ArenaBox::new)
                        .map(|mut b| {
                            let count = list.len();
                            if count == 0 {
                                return ASTNode::Unit;
                            }

                            *b = list.to_node().unwrap();
                            if quoted {
                                ASTNode::Quoted(b, count)
                            } else {
                                ASTNode::List(b, count)
                            }
                        })
                        .ok_or("Failed to close list");
                }
                Token::Comment(_) => (), // NOTE: Comments aren't used in the AST for now.
            },
        }
    }

    return make!(arena, Node<ASTNode>)
        .map(ArenaBox::new)
        .map(|mut b| {
            let count = list.len();
            *b = list.to_node().unwrap();
            ASTNode::List(b, count)
        })
        .ok_or("Failed to close list");
}
