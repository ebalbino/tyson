use std::fmt;
use std::ops::Deref;
use std::str::{CharIndices, FromStr};
use tyson::{Arena, List, Node};

#[derive(Copy, Clone)]
pub struct StrView {
    data: *const u8,
    len: usize,
}

struct StrIntern<'arena> {
    strings: List<'arena, StrView>,
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Integer(i64),
    Float(f64),
    String(StrView),
    Symbol(StrView),
}

pub struct Tokenizer<'code> {
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

    fn read_number(&mut self) -> Option<StrView> {
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

        Some(StrView::from_str(&self.code[start?..end?]))
    }

    fn read_string(&mut self) -> Option<StrView> {
        let mut start = None;
        let mut end = None;

        self.advance(); // Skip the opening quote
        while let Some((i, c)) = self.current {
            if start == None {
                start = Some(i);
            }

            if c == '"' || c == '\'' {
                end = Some(i);
                self.advance(); // Skip the closing quote
                break;
            }

            self.advance();
        }

        Some(StrView::from_str(&self.code[start?..end?]))
    }

    fn read_symbol(&mut self) -> Option<StrView> {
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

        Some(StrView::from_str(&self.code[start?..end?]))
    }
}

impl<'code> Iterator for Tokenizer<'code> {
    type Item = Token;

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
                    Some(Token::Float(val.parse().ok()?))
                } else {
                    Some(Token::Integer(val.parse().ok()?))
                }
            }
            (_, _c) => self.read_symbol().and_then(|s| Some(Token::Symbol(s))),
        }
    }
}

impl StrView {
    fn from_str(str: &str) -> Self {
        Self {
            data: str.as_ptr(),
            len: str.as_bytes().len(),
        }
    }
}

impl Deref for StrView {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.data, self.len)) }
    }
}

impl fmt::Debug for StrView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self)
    }
}

impl PartialEq for StrView {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl PartialEq<str> for StrView {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}

fn is_surrounding_punctuation(c: char) -> bool {
    c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}'
}
