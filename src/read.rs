use crate::{Arena, Array, Box as ArenaBox, List, Node, make};
use core::str::CharIndices;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token<'code> {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    True,
    False,
    Nil,
    Quote,
    Quasiquote,
    Integer(&'code str),
    Float(&'code str),
    String(&'code str),
    Symbol(&'code str),
    Comment(&'code str),
}

struct Tokenizer<'code> {
    code: &'code str,
    char_indices: CharIndices<'code>,
    current: Option<(usize, char)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lexeme<'arena> {
    Unit,
    Null,
    True,
    False,
    Integer(&'arena str),
    Double(&'arena str),
    String(&'arena str),
    Symbol(&'arena str, bool),
    Operator(&'arena str),
    List(ArenaBox<Node<Lexeme<'arena>>>, usize),
    Quoted(ArenaBox<Node<Lexeme<'arena>>>, usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Atom<'arena> {
    True,
    False,
    Void,
    Nil,
    Int { inner: i64 },
    Number { inner: f64 },
    String { inner: &'arena str },
    Buffer { data: &'arena [u8] },
    File { path: &'arena str, lazy: bool },
    Define,
    Cons,
    Head,
    Tail,
    Binding { name: &'arena str },
    Statement { body: Array<Expression<'arena>> },
    Code { body: Array<Expression<'arena>> },
    Add,
    Subtract,
    Multiply,
    Divide,
    Eq,
    Neq,
    LT,
    GT,
    LTE,
    GTE,
    ArrowLeft,
    ArrowRight,
    Negate,
    Exp,
    Mod,
    Remainder,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Expression<'arena> {
    pub depth: usize,
    pub payload: Atom<'arena>,
}

pub fn parse<'arena>(
    arena: &'arena Arena,
    code: &'static str,
) -> Option<Array<Expression<'arena>>> {
    let (root, count) = lexer(arena, code).ok()?;
    parse_list(arena, root, count, 0)
}

impl<'code> Tokenizer<'code> {
    fn new(code: &'code str) -> Self {
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
            if start.is_none() {
                start = Some(i);
            }

            if !c.is_numeric() && c != '.' && c != '-' {
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
            if start.is_none() {
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
            if start.is_none() {
                start = Some(i);
            }

            if c.is_whitespace() || is_surrounding_punctuation(c) {
                end = Some(i);
                break;
            }

            self.advance();
        }

        Some(&self.code[start?..end?])
    }

    fn read_comment(&mut self) -> Option<&'code str> {
        let mut start = None;
        let mut end = None;

        while let Some((i, c)) = self.current {
            if start.is_none() {
                start = Some(i);
            }

            if c == '\n' {
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
            (_, '\'') => {
                self.advance();
                Some(Token::Quote)
            }
            (_, '`') => {
                self.advance();
                Some(Token::Quasiquote)
            }
            (_, ';') => {
                self.advance();
                match self.current? {
                    (_, ';') => self.read_comment().map(Token::Comment),
                    _ => {
                        panic!("Malformed comment");
                    }
                }
            }
            (_, '"') => self.read_string().map(Token::String),
            (_, c) if c.is_numeric() => {
                let val = self.read_number()?;
                if val.contains('.') {
                    Some(Token::Float(val))
                } else {
                    Some(Token::Integer(val))
                }
            }
            (_, c) => match c {
                '-' => {
                    if let Some((_, next_char)) = self.char_indices.clone().peekable().peek() && next_char.is_numeric() {
			let val = self.read_number()?;
			if val.contains('.') {
			    return Some(Token::Float(val));
			} else {
			    return Some(Token::Integer(val));
                        }
                    }

                    self.read_symbol().map(|s| match s {
                        "#f" | "false" => Token::False,
                        "#t" | "true" => Token::True,
                        "nil" => Token::Nil,
                        _ => Token::Symbol(s),
                    })
                }
                _ => self.read_symbol().map(|s| match s {
                    "#f" | "false" => Token::False,
                    "#t" | "true" => Token::True,
                    "nil" => Token::Nil,
                    _ => Token::Symbol(s),
                }),
            },
        }
    }
}

fn lexer<'arena>(
    arena: &'arena Arena,
    code: &'arena str,
) -> Result<(Node<Lexeme<'arena>>, usize), &'static str> {
    let mut tokens = List::new(arena);

    for token in tokenize(code) {
        tokens.push_back(&token);
    }

    let tree = lex_tokens(arena, &mut tokens, false)?;

    if let Lexeme::List(head, len) = tree {
        return Ok((*head, len));
    }

    Err("The program couldn't be parsed correctly.")
}

fn lex_tokens<'arena>(
    arena: &'arena Arena<'arena>,
    tokens: &mut List<'arena, Token<'arena>>,
    quoted: bool,
) -> Result<Lexeme<'arena>, &'static str> {
    let mut list: List<'arena, Lexeme> = List::new(arena);

    while !tokens.is_empty() {
        match tokens.pop_front() {
            None => {
                return Err("Not enough tokens");
            }
            Some(token) => match token {
                Token::Integer(i) => {
                    list.push_back(&Lexeme::Integer(i));
                }
                Token::Float(f) => {
                    list.push_back(&Lexeme::Double(f));
                }
                Token::Nil => {
                    list.push_back(&Lexeme::Null);
                }
                Token::True => {
                    list.push_back(&Lexeme::True);
                }
                Token::False => {
                    list.push_back(&Lexeme::False);
                }
                Token::String(s) => {
                    list.push_back(&Lexeme::String(s));
                }
                Token::Symbol(s) => {
                    if is_operator(s) {
                        list.push_back(&Lexeme::Operator(s));
                    } else {
                        list.push_back(&Lexeme::Symbol(s, false));
                    }
                }
                Token::Quote => match tokens.pop_front().expect("You can't quote nothing.") {
                    Token::LParen | Token::LBrace | Token::LBracket => {
                        let sub_list = lex_tokens(arena, tokens, true)?;
                        list.push_back(&sub_list);
                    }
                    Token::Symbol(s) => {
                        list.push_back(&Lexeme::Symbol(s, true));
                    }
                    _ => {
                        panic!("Unable to quote.");
                    }
                },
                Token::Quasiquote => {
                    match tokens.pop_front().expect("You can't quasiquote nothing.") {
                        Token::LParen | Token::LBrace | Token::LBracket => {
                            let sub_list = lex_tokens(arena, tokens, true)?;
                            list.push_back(&sub_list);
                        }
                        Token::Symbol(s) => {
                            list.push_back(&Lexeme::Symbol(s, true));
                        }
                        _ => {
                            panic!("Unable to quasiquote.");
                        }
                    }
                }
                Token::LParen | Token::LBrace | Token::LBracket => {
                    let sub_list = lex_tokens(arena, tokens, false)?;
                    list.push_back(&sub_list);
                }
                Token::RParen | Token::RBrace | Token::RBracket => {
                    return make!(arena, Node<Lexeme>)
                        .map(ArenaBox::new)
                        .map(|mut b| {
                            let count = list.len();
                            if count == 0 {
                                return Lexeme::Unit;
                            }

                            *b = list.to_node().unwrap();
                            if quoted {
                                Lexeme::Quoted(b, count)
                            } else {
                                Lexeme::List(b, count)
                            }
                        })
                        .ok_or("Failed to close list");
                }
                Token::Comment(_) => (), // NOTE: Comments aren't used in the AST for now.
            },
        }
    }

    return make!(arena, Node<Lexeme>)
        .map(ArenaBox::new)
        .map(|mut b| {
            let count = list.len();
            *b = list.to_node().unwrap();
            Lexeme::List(b, count)
        })
        .ok_or("Failed to close list");
}

fn is_operator(s: &str) -> bool {
    match s {
        "+" | "-" | "*" | "/" | "//" | "=" | "!=" | ">" | "<" | ">=" | "<=" | "->" | "<-" | "!"
        | "^" | "%" => true,
        _ => false,
    }
}

fn parse_list<'arena>(
    arena: &'arena Arena<'arena>,
    root: Node<Lexeme<'arena>>,
    count: usize,
    depth: usize,
) -> Option<Array<Expression<'arena>>> {
    make!(arena, Expression, count)
        .map(Array::new)
        .map(|mut exprs| {
            for node in root.iter() {
                let payload = match node {
                    Lexeme::List(list, len) => Atom::Statement {
                        body: parse_list(arena, **list, *len, depth + 1).unwrap(),
                    },
                    Lexeme::Quoted(list, len) => Atom::Code {
                        body: parse_list(arena, **list, *len, depth + 1).unwrap(),
                    },
                    Lexeme::Symbol(name, _) => match *name {
                        "define" | "def" => Atom::Define,
                        "head" | "car" => Atom::Head,
                        "tail" | "cdr" => Atom::Tail,
                        "add" => Atom::Add,
                        "sub" => Atom::Subtract,
                        "mul" => Atom::Multiply,
                        "div" => Atom::Divide,
                        "rem" => Atom::Remainder,
                        "eq" => Atom::Eq,
                        "neq" => Atom::Neq,
                        "lt" => Atom::LT,
                        "gt" => Atom::GT,
                        "lte" => Atom::LTE,
                        "negate" | "neg" => Atom::Negate,
                        "gte" => Atom::GTE,
                        "exp" => Atom::Exp,
                        "mod" => Atom::Mod,
                        "cons" => Atom::Cons,
                        _ => Atom::Binding { name },
                    },
                    Lexeme::Unit => Atom::Void,
                    Lexeme::Null => Atom::Nil,
                    Lexeme::True => Atom::True,
                    Lexeme::False => Atom::False,
                    Lexeme::Integer(i) => Atom::Int {
                        inner: i.parse().expect("Unable to parse integer value."),
                    },
                    Lexeme::Double(f) => Atom::Number {
                        inner: f.parse().expect("Unable to parse floating point value."),
                    },
                    Lexeme::String(s) => Atom::String { inner: s },
                    Lexeme::Operator(s) => match *s {
                        "+" => Atom::Add,
                        "-" => Atom::Subtract,
                        "*" => Atom::Multiply,
                        "/" => Atom::Divide,
                        "//" => Atom::Remainder,
                        "=" => Atom::Eq,
                        "!=" => Atom::Neq,
                        ">" => Atom::GT,
                        "<" => Atom::LT,
                        ">=" => Atom::GTE,
                        "<=" => Atom::LTE,
                        "->" => Atom::ArrowRight,
                        "<-" => Atom::ArrowLeft,
                        "!" => Atom::Negate,
                        "^" => Atom::Exp,
                        "%" => Atom::Mod,
                        _ => panic!("Unsupported operator!"),
                    },
                };
                exprs.push(&Expression { depth, payload });
            }
            exprs
        })
}

fn is_surrounding_punctuation(c: char) -> bool {
    c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}'
}

fn tokenize<'code>(code: &'code str) -> impl Iterator<Item = Token<'code>> {
    Tokenizer::new(code)
}
