use crate::{Arena, Box as ArenaBox, List, Node, make, strmake};
use std::str::CharIndices;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token<'code> {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    True,
    False,
    Nil,
    Integer(&'code str),
    Float(&'code str),
    String(&'code str),
    Symbol(&'code str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<'arena> {
    Void,
    True,
    False,
    Integer(i64),
    Double(f64),
    String(&'arena str),
    Symbol(&'arena str),
    List(ArenaBox<Node<Value<'arena>>>),
}

struct Tokenizer<'code> {
    code: &'code str,
    char_indices: CharIndices<'code>,
    current: Option<(usize, char)>,
}

impl<'code> Tokenizer<'code> {
    pub fn new(code: &'code str) -> Self {
        let mut char_indices = code.char_indices();
        let current = char_indices.next();
        Self {
            code,
            char_indices,
            current,
        }
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        self.current = self.char_indices.next();
        self.current
    }

    fn eat_whitespace(&mut self) {
        while let Some((_, c)) = self.current {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn read_number(&mut self) -> Option<&'code str> {
        let mut start = None;
        let mut end = None;

        while let Some((i, c)) = self.current {
            if start == None {
                start = Some(i);
            }

            if !c.is_numeric() && c != '.' {
                end = Some(i);
                break;
            }

            self.advance();
        }

        Some(&self.code[start?..end?])
    }

    fn read_string(&mut self) -> Option<&'code str> {
        let mut start = None;
        let mut end = None;

        self.advance(); // Skip the opening quote
        while let Some((i, c)) = self.current {
            if start == None {
                start = Some(i);
            }

            if c == '"' {
                end = Some(i);
                self.advance(); // Skip the closing quote
                break;
            }

            self.advance();
        }

        Some(&self.code[start?..end?])
    }

    fn read_symbol(&mut self) -> Option<&'code str> {
        let mut start = None;
        let mut end = None;

        while let Some((i, c)) = self.current {
            if start == None {
                start = Some(i);
            }

            if c.is_whitespace() || is_surrounding_punctuation(c) || c == '\'' {
                end = Some(i);
                break;
            }

            self.advance();
        }

        Some(&self.code[start?..end?])
    }
}

impl<'code> Iterator for Tokenizer<'code> {
    type Item = Token<'code>;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();
        match self.current? {
            (_, '(') => {
                self.advance();
                Some(Token::LParen)
            }
            (_, ')') => {
                self.advance();
                Some(Token::RParen)
            }
            (_, '[') => {
                self.advance();
                Some(Token::LBracket)
            }
            (_, ']') => {
                self.advance();
                Some(Token::RBracket)
            }
            (_, '{') => {
                self.advance();
                Some(Token::LBrace)
            }
            (_, '}') => {
                self.advance();
                Some(Token::RBrace)
            }
            (_, '"') | (_, '\'') => self.read_string().and_then(|s| Some(Token::String(s))),
            (_, c) if c.is_numeric() => {
                let val = self.read_number()?;
                if val.contains('.') {
                    Some(Token::Float(val))
                } else {
                    Some(Token::Integer(val))
                }
            }
            (_, _c) => self.read_symbol().and_then(|s| match s {
                "#f" | "false" => Some(Token::False),
                "#t" | "true" => Some(Token::True),
                "nil" => Some(Token::Nil),
                _ => Some(Token::Symbol(s)),
            }),
        }
    }
}

fn is_surrounding_punctuation(c: char) -> bool {
    c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}'
}

pub fn tokenize<'code>(code: &'code str) -> impl Iterator<Item = Token<'code>> {
    Tokenizer::new(code).into_iter()
}

fn parse_list<'arena>(
    arena: &'arena Arena,
    tokens: &mut List<Token>,
) -> Result<Value<'arena>, &'static str> {
    let mut list: List<'arena, Value> = List::new(arena);

    while !tokens.is_empty() {
        match tokens.pop_front() {
            None => {
                return Err("Not enough tokens");
            }
            Some(token) => match token {
                Token::Integer(i) => {
                    list.push_back(&Value::Integer(i.parse().unwrap()));
                }
                Token::Float(f) => {
                    list.push_back(&Value::Double(f.parse().unwrap()));
                }
                Token::Nil => {
                    list.push_back(&Value::Void);
                }
                Token::True => {
                    list.push_back(&Value::True);
                }
                Token::False => {
                    list.push_back(&Value::False);
                }
                Token::String(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate string");
                    list.push_back(&Value::String(s));
                }
                Token::Symbol(s) => {
                    let s = strmake!(arena, "{}", s).expect("No memory to allocate symbol");
                    list.push_back(&Value::Symbol(s));
                }
                Token::LParen | Token::LBrace | Token::LBracket => {
                    let sub_list = parse_list(arena, tokens)?;
                    list.push_back(&sub_list);
                }
                Token::RParen | Token::RBrace | Token::RBracket => {
                    return make!(arena, Node<Value>)
                        .map(ArenaBox::new)
                        .map(|mut b| {
                            *b = list.to_node().unwrap();
                            Value::List(b)
                        })
                        .ok_or("Failed to close list");
                }
            },
        }
    }

    return make!(arena, Node<Value>)
        .map(ArenaBox::new)
        .map(|mut b| {
            *b = list.to_node().unwrap();
            Value::List(b)
        })
        .ok_or("Failed to close list");
}

pub fn parse<'arena>(arena: &'arena Arena, code: &str) -> Result<Value<'arena>, &'static str> {
    let mut tokens = List::new(&arena);

    for token in tokenize(code) {
        tokens.push_back(&token);
    }

    parse_list(arena, &mut tokens)
}
